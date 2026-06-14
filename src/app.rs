use crate::client::{ClusterResource, ProxmoxClient};
use crate::config::Config;

#[derive(Debug, Clone)]
pub enum Modal {
    Help,
    Filter,
    Confirm(ConfirmAction),
    Details,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    Stop { node: String, vmid: u32 },
    Reboot { node: String, vmid: u32 },
}

pub struct App {
    pub resources: Vec<ClusterResource>,
    pub selected_index: usize,
    pub filter: String,
    pub display_resources: Vec<ClusterResource>,
    pub modal: Option<Modal>,
    pub status_message: Option<String>,
    pub connected: bool,
    pub config: Config,
    pub client: Option<ProxmoxClient>,
    pub quit: bool,
}

impl App {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let client = if let (Some(host), Some(token_id), Some(token)) =
            (&config.host, &config.token_id, &config.token)
        {
            Some(ProxmoxClient::new(host, token_id, token, config.insecure)?)
        } else {
            None
        };

        let filter = config.filter.clone().unwrap_or_default();
        let mut app = Self {
            resources: Vec::new(),
            selected_index: 0,
            filter,
            display_resources: Vec::new(),
            modal: None,
            status_message: None,
            connected: false,
            config,
            client,
            quit: false,
        };
        app.update_display_resources();
        Ok(app)
    }

    pub fn filtered_resources(&self) -> &[ClusterResource] {
        &self.display_resources
    }

    pub fn selected_resource(&self) -> Option<&ClusterResource> {
        self.display_resources.get(self.selected_index)
    }

    pub fn update_display_resources(&mut self) {
        let f = self.filter.to_lowercase();
        if f.is_empty() {
            self.display_resources = self.resources.clone();
        } else {
            self.display_resources = self
                .resources
                .iter()
                .filter(|r| {
                    r.name.to_lowercase().contains(&f)
                        || r.r#type.to_lowercase().contains(&f)
                        || r.node
                            .as_ref()
                            .map(|n| n.to_lowercase().contains(&f))
                            .unwrap_or(false)
                })
                .cloned()
                .collect();
        }
        self.selected_index = self
            .selected_index
            .min(self.display_resources.len().saturating_sub(1));
    }

    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
        self.selected_index = 0;
        self.update_display_resources();
    }

    pub fn set_resources(&mut self, resources: Vec<ClusterResource>) {
        self.resources = resources;
        self.update_display_resources();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_resource(name: &str, rtype: &str, node: Option<&str>) -> ClusterResource {
        ClusterResource {
            id: format!("{}/{}", rtype, name),
            r#type: rtype.to_string(),
            name: name.to_string(),
            node: node.map(|n| n.to_string()),
            status: "running".to_string(),
            cpu: None,
            maxcpu: None,
            mem: None,
            maxmem: None,
            disk: None,
            maxdisk: None,
            uptime: None,
        }
    }

    fn mock_config() -> Config {
        Config {
            host: None,
            token_id: None,
            token: None,
            insecure: false,
            refresh_interval: 5,
            filter: None,
            no_color: false,
            config: None,
        }
    }

    #[test]
    fn test_empty_filter_returns_all() {
        let config = mock_config();
        let mut app = App::new(config).unwrap();
        app.set_resources(vec![
            mock_resource("web1", "qemu", Some("pve1")),
            mock_resource("db1", "lxc", Some("pve2")),
            mock_resource("storage1", "storage", None),
        ]);
        app.set_filter("".to_string());
        assert_eq!(app.filtered_resources().len(), 3);
        assert_eq!(app.selected_resource().unwrap().name, "web1");
    }

    #[test]
    fn test_filter_subset_by_name() {
        let config = mock_config();
        let mut app = App::new(config).unwrap();
        app.set_resources(vec![
            mock_resource("web1", "qemu", Some("pve1")),
            mock_resource("web2", "qemu", Some("pve1")),
            mock_resource("db1", "lxc", Some("pve2")),
            mock_resource("cache1", "qemu", Some("pve2")),
            mock_resource("storage1", "storage", None),
        ]);
        app.set_filter("web".to_string());
        assert_eq!(app.filtered_resources().len(), 2);
        assert!(app
            .filtered_resources()
            .iter()
            .all(|r| r.name.starts_with("web")));
    }

    #[test]
    fn test_filter_subset_by_type() {
        let config = mock_config();
        let mut app = App::new(config).unwrap();
        app.set_resources(vec![
            mock_resource("vm1", "qemu", Some("pve1")),
            mock_resource("ct1", "lxc", Some("pve1")),
            mock_resource("vm2", "qemu", Some("pve2")),
        ]);
        app.set_filter("lxc".to_string());
        assert_eq!(app.filtered_resources().len(), 1);
        assert_eq!(app.filtered_resources()[0].name, "ct1");
    }

    #[test]
    fn test_filter_subset_by_node() {
        let config = mock_config();
        let mut app = App::new(config).unwrap();
        app.set_resources(vec![
            mock_resource("vm1", "qemu", Some("pve1")),
            mock_resource("vm2", "qemu", Some("pve2")),
            mock_resource("vm3", "qemu", Some("pve1")),
        ]);
        app.set_filter("pve2".to_string());
        assert_eq!(app.filtered_resources().len(), 1);
        assert_eq!(app.filtered_resources()[0].name, "vm2");
    }

    #[test]
    fn test_filter_case_insensitive() {
        let config = mock_config();
        let mut app = App::new(config).unwrap();
        app.set_resources(vec![mock_resource("WebServer", "qemu", Some("PVE1"))]);
        app.set_filter("web".to_string());
        assert_eq!(app.filtered_resources().len(), 1);
        app.set_filter("PVE".to_string());
        assert_eq!(app.filtered_resources().len(), 1);
    }

    #[test]
    fn test_selected_bounds_after_filter() {
        let config = mock_config();
        let mut app = App::new(config).unwrap();
        app.set_resources(vec![
            mock_resource("alpha", "qemu", Some("pve1")),
            mock_resource("beta", "qemu", Some("pve1")),
            mock_resource("gamma", "qemu", Some("pve1")),
        ]);
        app.selected_index = 2;
        app.set_filter("alpha".to_string());
        assert_eq!(app.filtered_resources().len(), 1);
        assert_eq!(app.selected_index, 0);
        assert_eq!(app.selected_resource().unwrap().name, "alpha");
    }

    #[test]
    fn test_selected_resource_none_when_empty() {
        let config = mock_config();
        let app = App::new(config).unwrap();
        assert!(app.selected_resource().is_none());
    }

    #[test]
    fn test_filter_no_match_returns_empty() {
        let config = mock_config();
        let mut app = App::new(config).unwrap();
        app.set_resources(vec![mock_resource("vm1", "qemu", Some("pve1"))]);
        app.set_filter("nonexistent".to_string());
        assert!(app.filtered_resources().is_empty());
        assert!(app.selected_resource().is_none());
    }
}
