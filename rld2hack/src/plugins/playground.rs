use crate::{
    d2::packets::PacketFromServer,
    plugincore::{plugin::Plugin, plugin_info::PluginInfo},
};

pub fn get_info() -> PluginInfo {
    PluginInfo {
        name: "Playground".into(),
        description: "First plugin!".into(),
        author: "Riley Labrecque".into(),
        version: "1.0.0".into(),
        plugin: Box::new(Playground {}),
    }
}

pub struct Playground {}

impl Plugin for Playground {
    fn on_load(&self) {
        println!("Playground Plugin Loaded.");
    }

    fn on_unload(&self) {
        println!("Playground Plugin Unloaded.");
    }

    fn on_tick(&self) {}

    fn on_game_packet_received(&self, packet: &PacketFromServer) {
        println!("{:?}", packet);
    }
}
