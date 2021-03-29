use crate::plugincore::plugin_info::{Plugin, PluginInfo};

pub fn get_info() -> PluginInfo {
    PluginInfo {
        name: "Plugin2".into(),
        description: "Second plugin!".into(),
        author: "Riley Labrecque".into(),
        version: "1.0.1".into(),
        plugin: Box::new(Plugin2 {}),
    }
}

struct Plugin2 {}

impl Plugin for Plugin2 {
    fn on_load(&self) {}

    fn on_unload(&self) {}

    fn on_tick(&self) {
        /*println!("");
        println!(
            "ScreenSize: {}x{}",
            d2::variables::get_screensize_x(&d2core.game),
            d2::variables::get_screensize_y(&d2core.game)
        );
        println!(
            "Cursor Hover: ({}, {})",
            d2::variables::get_cursor_hover_x(&d2core.game),
            d2::variables::get_cursor_hover_y(&d2core.game)
        );
        println!(
            "Mouse Pos: ({}, {})",
            d2::variables::get_mouse_pos_x(&d2core.game),
            d2::variables::get_mouse_pos_y(&d2core.game)
        );
        println!(
            "Mouse Offset: ({}, {}, {})",
            d2::variables::get_mouse_offset_y(&d2core.game),
            d2::variables::get_mouse_offset_z(&d2core.game),
            d2::variables::get_mouse_offset_x(&d2core.game)
        );
        println!(
            "Automap: On: {} Mode: {} Offset: {}",
            d2::variables::get_automap_on(&d2core.game),
            d2::variables::get_automap_mode(&d2core.game),
            d2::variables::get_automap_offset(&d2core.game)
        );
        println!(
            "Viewport: ({}, {})",
            d2::variables::get_viewport_x(&d2core.game),
            d2::variables::get_viewport_y(&d2core.game)
        );
        println!(
            "Gold Dialog: Action: {} Amount: {}",
            d2::variables::get_gold_dialog_action(&d2core.game),
            d2::variables::get_gold_dialog_amount(&d2core.game)
        );
        println!("NPC Menu Amount: {}", d2::variables::get_npc_menu_amount(&d2core.game));
        println!(
            "Regular Cursor Type: {}",
            d2::variables::get_regular_cursor_type(&d2core.game)
        );
        println!(
            "Shop Cursor Type: {}",
            d2::variables::get_shop_cursor_type(&d2core.game)
        );
        println!("FPS: {}", d2::variables::get_fps(&d2core.game));
        println!("Skip: {}", d2::variables::get_skip(&d2core.game));
        println!("Ping: {}", d2::variables::get_ping(&d2core.game));
        println!("Lang: {}", d2::variables::get_lang(&d2core.game));
        println!("Divisor: {}", d2::variables::get_divisor(&d2core.game));
        println!(
            "Overhead Trigger: {}",
            d2::variables::get_overhead_trigger(&d2core.game)
        );
        println!(
            "Recent Interact Id: {}",
            d2::variables::get_recent_interact_id(&d2core.game)
        );
        println!("Item Price List: {}", d2::variables::get_item_price_list(&d2core.game));
        println!("Waypoint Table: {}", d2::variables::get_waypoint_table(&d2core.game));
        println!(
            "Is Weapon Swapped: {}",
            d2::variables::get_is_weapon_swapped(&d2core.game)
        );
        println!(
            "Trade: Accepted: {} Blocked: {} Recent Trade Id: {}",
            d2::variables::get_is_trade_accepted(&d2core.game),
            d2::variables::get_is_trade_block(&d2core.game),
            d2::variables::get_recent_trade_id(&d2core.game)
        );
        println!("Exp Char Flag: {}", d2::variables::get_exp_char_flag(&d2core.game));
        println!("Map Id: {}", d2::variables::get_map_id(&d2core.game));
        println!("Always Run: {}", d2::variables::get_always_run(&d2core.game));
        println!("No Pickup: {}", d2::variables::get_no_pickup(&d2core.game));
        println!("Chat Message: {}", d2::variables::get_chat_message(&d2core.game));
        println!("Orfice Id: {}", d2::variables::get_orifice_id(&d2core.game));
        println!(
            "Cursor Item Mode: {}",
            d2::variables::get_cursor_item_mode(&d2core.game)
        );

        println!("");

        println!("Automap Size: {}", d2::functions::get_automap_size(&d2core.game));
        println!("Difficulty: {}", d2::functions::get_difficulty(&d2core.game));
        println!(
            "Game Language Code: {}",
            d2::functions::get_game_language_code(&d2core.game)
        );
        println!(
            "Mouse Offset: ({}, {})",
            d2::functions::get_mouse_x_offset(&d2core.game),
            d2::functions::get_mouse_y_offset(&d2core.game)
        );
        //d2::functions::print_game_string(&d2core, "I love you", 0);
        //d2::functions::print_party_string(&d2core.game, "Spam", 1);

        //d2::functions::close_npc_interact(&d2core.game);
        //d2::functions::close_interact(&d2core.game);
        //d2::functions::exit_game(&d2core);*/
    }
}
