use super::functions;
use crate::library::Library;
use detour::GenericDetour;

pub struct D2Core {
    pub game: Library,

    pub exit_game_detour: GenericDetour<functions::ExitGameFn>,
    pub print_game_string_detour: GenericDetour<functions::PrintGameStringFn>,
}

impl D2Core {
    /// Constructs a new copy of D2Core.
    /// Note: This also initializes the singleton used for accessing D2Core
    /// from detoured functions. The singleton can be accessed via get().
    pub fn new(game: Library) -> Box<Self> {
        println!("D2Core - Initialize");
        let exit_game_detour = functions::create_hook_exit_game(&game);
        let print_game_string_detour = functions::create_hook_print_game_string(&game);

        let d2core = Box::new(D2Core {
            game: game,
            exit_game_detour: exit_game_detour,
            print_game_string_detour: print_game_string_detour,
        });

        unsafe {
            assert!(INSTANCE == std::ptr::null_mut(), "D2Core has already been initialized!");
            INSTANCE = std::mem::transmute(d2core);

            // Get that instance back out of INSTANCE so that we can return it.
            let d2core: Box<D2Core> = std::mem::transmute(INSTANCE);
            d2core
        }
    }

    pub fn get() -> &'static Self {
        unsafe {
            assert!(INSTANCE != std::ptr::null_mut(), "D2Core is not initialized!");
            &(*INSTANCE)
        }
    }
}

impl Drop for D2Core {
    fn drop(&mut self) {
        println!("D2Core - Drop");

        unsafe {
            INSTANCE = std::ptr::null_mut();
        }

        unsafe {
            self.exit_game_detour.disable().unwrap();
        }
    }
}

static mut INSTANCE: *const D2Core = std::ptr::null_mut();
