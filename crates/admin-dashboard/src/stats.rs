use serde::Serialize;
use wassel_plugin_component::PluginMeta;

#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub system: SystemStats,
    pub plugins: Vec<PluginStats>,
}

impl Stats {
    pub fn new(plugins: Vec<PluginMeta>) -> Stats {
        Stats {
            system: SystemStats::load(),
            plugins: plugins.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStats {
    pub memory: u64,
    pub virtual_memory: u64,
    pub cpu_usage: f32,
    pub start_time: u64,
}

impl SystemStats {
    pub fn load() -> Self {
        let pid = sysinfo::get_current_pid().unwrap();
        let info = sysinfo::System::new_all();
        let pinfo = info.process(pid).unwrap();

        Self {
            memory: pinfo.memory(),
            virtual_memory: pinfo.virtual_memory(),
            cpu_usage: pinfo.cpu_usage(),
            start_time: pinfo.start_time(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginStats {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub endpoint: String,
}

impl From<PluginMeta> for PluginStats {
    fn from(value: PluginMeta) -> Self {
        Self {
            id: value.id,
            name: value.name,
            version: value.version,
            description: value.description,
            endpoint: value.endpoint,
        }
    }
}
