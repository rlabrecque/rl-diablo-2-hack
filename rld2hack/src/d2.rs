pub mod functions;

use crate::library::Library;

pub struct AutomapOffset {
    x: i32,
    y: i32,
}

impl std::fmt::Display for AutomapOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


pub fn get_screensize_x(game: &Library) -> &u32 {
    unsafe {
        game.read(0x31146Cusize) as &u32
    }
}

pub fn get_screensize_y(game: &Library) -> &u32 {
    unsafe {
        game.read(0x311470usize) as &u32
    }
}

pub fn get_cursor_hover_x(game: &Library) -> &i32 {
    unsafe {
        game.read(0x321E4Cusize) as &i32
    }
}

pub fn get_cursor_hover_y(game: &Library) -> &i32 {
    unsafe {
        game.read(0x321E50usize) as &i32
    }
}

pub fn get_mouse_pos_y(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A6AACusize) as &u32
    }
}

pub fn get_mouse_pos_x(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A6AB0usize) as &u32
    }
}

pub fn get_mouse_offset_y(game: &Library) -> &i32 {
    unsafe {
        game.read(0x3A5208usize) as &i32
    }
}

pub fn get_mouse_offset_z(game: &Library) -> &i32 {
    unsafe {
        game.read(0x3A5214usize) as &i32
    }
}

pub fn get_mouse_offset_x(game: &Library) -> &i32 {
    unsafe {
        game.read(0x3A520Cusize) as &i32
    }
}

pub fn get_automap_on(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A27E8usize) as &u32
    }
}

pub fn get_automap_mode(game: &Library) -> &i32 {
    unsafe {
        game.read(0x311254usize) as &i32
    }
}

pub fn get_automap_offset(game: &Library) -> &AutomapOffset {
    unsafe {
        game.read(0x3A5198usize) as &AutomapOffset
    }
}

pub fn get_viewport_x(game: &Library) -> &i32 {
    unsafe {
        game.read(0x3A5208usize) as &i32
    }
}

pub fn get_viewport_y(game: &Library) -> &i32 {
    unsafe {
        game.read(0x3A520Cusize) as &i32
    }
}

pub fn get_gold_dialog_action(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A279Cusize) as &u32
    }
}

pub fn get_gold_dialog_amount(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A2A68usize) as &u32
    }
}

pub fn get_npc_menu_amount(game: &Library) -> &u32 {
    unsafe {
        game.read(0x325A74usize) as &u32
    }
}

pub fn get_regular_cursor_type(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A6AF0usize) as &u32
    }
}

pub fn get_shop_cursor_type(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BCBF0usize) as &u32
    }
}

pub fn get_ping(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A04A4usize) as &u32
    }
}

pub fn get_skip(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A04B0usize) as &u32
    }
}

pub fn get_fps(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BB390usize) as &u32
    }
}

pub fn get_lang(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BB5DCusize) as &u32
    }
}

pub fn get_divisor(game: &Library) -> &i32 {
    unsafe {
        game.read(0x311254usize) as &i32
    }
}

pub fn get_overhead_trigger(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BF20Eusize) as &u32
    }
}


pub fn get_recent_interact_id(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3C0D25usize) as &u32
    }
}

pub fn get_item_price_list(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3C0D43usize) as &u32
    }
}

pub fn get_waypoint_table(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BF081usize) as &u32
    }
}

pub fn get_is_weapon_swapped(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BCC4Cusize) as &u32
    }
}

pub fn get_is_trade_accepted(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BCE18usize) as &u32
    }
}

pub fn get_is_trade_block(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3BCE28usize) as &u32
    }
}

pub fn get_recent_trade_id(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3C0E7Cusize) as &u32
    }
}

pub fn get_exp_char_flag(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A04F4usize) as &u32
    }
}

pub fn get_map_id(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A0638usize) as &u32
    }
}

pub fn get_always_run(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A0660usize) as &u32
    }
}

pub fn get_no_pickup(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A6A90usize) as &u32
    }
}

pub fn get_chat_message(game: &Library) -> String {
    unsafe {
        let ptr = game.read(0x3BB638usize) as *const u16;
        if ptr.is_null() {
            String::new()
        }
        else {
            let widestr = widestring::WideCStr::from_ptr_str(ptr);
            widestr.to_string_lossy()
        }
    }
}

pub fn get_orifice_id(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3C547Cusize) as &u32
    }
}

pub fn get_cursor_item_mode(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3C5474usize) as &u32
    }
}

/*
VARPTR(D2CLIENT, AutomapLayer, AutomapLayer*, 0x3A5164) // Updated 1.14d //007A5164-BASE
VARPTR(D2CLIENT, MercStrIndex, WORD, 0x325494) //Updated 1.14d //00725494-BASE

VARPTR(D2CLIENT, ServerSideUnitHashTables, UnitHashTable, 0x3A5E70) // Updated 1.14d //007A5E70-BASE
VARPTR(D2CLIENT, ClientSideUnitHashTables, UnitHashTable, 0x3A5270) // Updated 1.14d //007A5270-BASE

VARPTR(D2CLIENT, NPCMenu, NPCMenu*, 0x326C48)    // Updated 1.14d //00726C48-BASE

VARPTR(D2CLIENT, TradeLayout, InventoryLayout*, 0x3BCA30)     // Updated 1.14d //007BCA30-BASE
VARPTR(D2CLIENT, StashLayout, InventoryLayout*, 0x3BCA78)     // Updated 1.14d //007BCA78-BASE
VARPTR(D2CLIENT, StoreLayout, InventoryLayout*, 0x3BCB58)     // Updated 1.14d //007BCB58-BASE
VARPTR(D2CLIENT, CubeLayout, InventoryLayout*, 0x3BCB70)      // Updated 1.14d //007BCB70-BASE
VARPTR(D2CLIENT, InventoryLayout, InventoryLayout*, 0x3BCB88) // Updated 1.14d //007BCB88-BASE
VARPTR(D2CLIENT, MercLayout, InventoryLayout*, 0x3BCD4C)      // Updated 1.14d //007BCD4C-BASE

VARPTR(D2CLIENT, TransactionDialog, void*, 0x3C0D63)                           // Updated 1.14d //007C0D63-BASE
VARPTR(D2CLIENT, TransactionDialogs, DWORD, 0x3C0E5C)                          // Updated 1.14d //007C0E5C-BASE
VARPTR(D2CLIENT, TransactionDialogs_2, DWORD, 0x3C0E58)                        // Updated 1.14d //007C0E58-BASE
VARPTR(D2CLIENT, pTransactionDialogsInfo, TransactionDialogsInfo_t*, 0x3C0E54) // Updated 1.14d //007C0E54-BASE

VARPTR(D2CLIENT, GameInfo, GameStructInfo*, 0x3A0438) // Updated 1.14d //007A0438-BASE

VARPTR(D2CLIENT, PlayerUnit, UnitAny*, 0x3A6A70)      // Updated 1.14d //007A6A70-BASE
VARPTR(D2CLIENT, SelectedInvItem, UnitAny*, 0x3BCBF4) // Updated 1.14d //007BCBF4-BASE
// VARPTR(D2CLIENT, SelectedUnit, UnitAny*, 0x11C4D8) // unused, but can we use it somewhere maybe? // 1.12 still
VARPTR(D2CLIENT, PlayerUnitList, RosterUnit*, 0x3BB5C0) // Updated 1.14d //007BB5C0-BASE

// VARPTR(D2CLIENT, RecentTradeName, wchar_t*, 0x12334C)

// VARPTR(D2CLIENT, ScreenCovered, DWORD, 0x1E8F9) // unused, appears to be an int specifying which screens (if any) are opened...
*/