use crate::d2::packets::PacketFromServer;

pub trait Plugin {
    fn on_load(&self);

    fn on_unload(&self);

    fn on_tick(&self);

    fn on_game_packet_received(&self, _packet: &PacketFromServer) {}
}
