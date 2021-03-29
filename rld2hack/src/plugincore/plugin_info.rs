use crate::d2::packets::PacketFromServer;

pub struct PluginInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub plugin: Box<dyn Plugin>,
}

pub trait Plugin {
    fn on_load(&self);

    fn on_unload(&self);

    fn on_tick(&self);

    fn on_game_packet_received(&self, _packet: &PacketFromServer) {}
}
