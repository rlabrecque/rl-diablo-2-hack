use crate::library::Library;

use super::functions;
use detour::GenericDetour;

pub struct D2Core {
    pub game: Library,
    //pub plugins:
    //
    pub hook: GenericDetour<functions::ExitGameFn>,
}

impl D2Core {
    pub fn new(game: Library) -> Self {
        let hook = functions::create_hook_exit_game(&game);

        D2Core { game: game, hook: hook }
    }
}

/*
unsafe {
    hook.disable().unwrap();
}
*/
