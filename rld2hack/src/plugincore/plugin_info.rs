use crate::plugincore::plugin::Plugin;

pub struct PluginInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub plugin: Box<dyn Plugin>,
}
