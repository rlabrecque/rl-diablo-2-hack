use super::functions;
use super::{d2library::D2Library, packets::PacketFromServer};
use retour::GenericDetour;

pub type OnGamePacketReceivedFn = fn(packet: &PacketFromServer);

pub struct D2Core<'a> {
    pub game: D2Library,

    pub exit_game_detour: GenericDetour<functions::ExitGameFn>,
    pub print_game_string_detour: GenericDetour<functions::PrintGameStringFn>,
    pub game_packet_received_detour: GenericDetour<functions::GamePacketReceivedFn>,
    pub game_packet_received_listener: Option<Box<dyn 'a + Fn(&PacketFromServer)>>,
}

impl<'a> D2Core<'a> {
    /// Constructs a new copy of D2Core.
    /// Note: This also initializes the singleton used for accessing D2Core
    /// from detoured functions. The singleton can be accessed via get().
    pub fn new() -> Box<Self> {
        println!("D2Core - Initialize");

        let game = D2Library::new("Game.exe".to_owned());

        let exit_game_detour = functions::create_hook_exit_game(&game);
        let print_game_string_detour = functions::create_hook_print_game_string(&game);
        let game_packet_received_detour = functions::create_hook_game_packet_received(&game);

        let d2core = Box::new(D2Core {
            game: game,
            exit_game_detour: exit_game_detour,
            print_game_string_detour: print_game_string_detour,
            game_packet_received_detour: game_packet_received_detour,
            game_packet_received_listener: None,
        });

        unsafe {
            assert!(INSTANCE == std::ptr::null_mut(), "D2Core has already been initialized!");
            INSTANCE = std::mem::transmute(d2core);

            // Get that instance back out of INSTANCE so that we can return it.
            std::mem::transmute(INSTANCE)
        }
    }

    // Gets the global D2Core instance, this should only ever be used inside callbacks
    pub fn get() -> &'static Self {
        unsafe {
            assert!(INSTANCE != std::ptr::null_mut(), "D2Core is not initialized!");
            &(*INSTANCE)
        }
    }

    pub fn set_callback(&mut self, c: Box<dyn 'a + Fn(&PacketFromServer)>) {
        self.game_packet_received_listener = Some(c);
    }
}

impl<'a> Drop for D2Core<'a> {
    fn drop(&mut self) {
        println!("D2Core - Drop");

        unsafe {
            INSTANCE = std::ptr::null_mut();
        }

        unsafe {
            self.exit_game_detour.disable().unwrap();
            self.print_game_string_detour.disable().unwrap();
            self.game_packet_received_detour.disable().unwrap();
        }

        self.game_packet_received_listener = None;
    }
}

static mut INSTANCE: *const D2Core = std::ptr::null_mut();
