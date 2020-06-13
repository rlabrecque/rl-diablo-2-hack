use crate::library::Library;

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

pub fn get_cursor_hover_x(game: &Library) -> &u32 {
    unsafe {
        game.read(0x321E4Cusize) as &u32
    }
}

pub fn get_cursor_hover_y(game: &Library) -> &u32 {
    unsafe {
        game.read(0x321E50usize) as &u32
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

pub fn get_mouse_offset_y(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A5208usize) as &u32
    }
}

pub fn get_mouse_offset_z(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A5214usize) as &u32
    }
}

pub fn get_mouse_offset_x(game: &Library) -> &u32 {
    unsafe {
        game.read(0x3A520Cusize) as &u32
    }
}

/*
VARPTR(D2CLIENT, AutomapOn, DWORD, 0x3A27E8)            // Updated 1.14d //007A27E8-BASE
VARPTR(D2CLIENT, AutomapMode, int, 0x311254)            // Updated 1.14d //00711254-BASE **Divisor**
VARPTR(D2CLIENT, Offset, POINT, 0x3A5198)               // Updated 1.14d //007A5198-BASE
VARPTR(D2CLIENT, AutomapLayer, AutomapLayer*, 0x3A5164) // Updated 1.14d //007A5164-BASE

// VARPTR(D2CLIENT, MercStrIndex, WORD, 0x325494) //Updated 1.14d //00725494-BASE
VARPTR(D2CLIENT, MercReviveCost, DWORD, 0x3C0DD0) // Updated 1.14d //007C0DD0-BASE

VARPTR(D2CLIENT, ServerSideUnitHashTables, UnitHashTable, 0x3A5E70) // Updated 1.14d //007A5E70-BASE
VARPTR(D2CLIENT, ClientSideUnitHashTables, UnitHashTable, 0x3A5270) // Updated 1.14d //007A5270-BASE

VARPTR(D2CLIENT, ViewportY, int, 0x3A5208) // Updated 1.14d //007A5208-BASE **MouseOffsetY
VARPTR(D2CLIENT, ViewportX, int, 0x3A520C) // Updated 1.14d //007A520C-BASE **MouseOffsetX

VARPTR(D2CLIENT, GoldDialogAction, DWORD, 0x3A279C) // Updated 1.14d //007A279C-BASE
VARPTR(D2CLIENT, GoldDialogAmount, DWORD, 0x3A2A68) // Updated 1.14d //007A2A68-BASE

VARPTR(D2CLIENT, NPCMenu, NPCMenu*, 0x326C48)    // Updated 1.14d //00726C48-BASE
*/

pub fn get_npc_menu_amount(game: &Library) -> &u32 {
    unsafe {
        game.read(0x325A74usize) as &u32
    }
}

/*
VARPTR(D2CLIENT, TradeLayout, InventoryLayout*, 0x3BCA30)     // Updated 1.14d //007BCA30-BASE
VARPTR(D2CLIENT, StashLayout, InventoryLayout*, 0x3BCA78)     // Updated 1.14d //007BCA78-BASE
VARPTR(D2CLIENT, StoreLayout, InventoryLayout*, 0x3BCB58)     // Updated 1.14d //007BCB58-BASE
VARPTR(D2CLIENT, CubeLayout, InventoryLayout*, 0x3BCB70)      // Updated 1.14d //007BCB70-BASE
VARPTR(D2CLIENT, InventoryLayout, InventoryLayout*, 0x3BCB88) // Updated 1.14d //007BCB88-BASE
VARPTR(D2CLIENT, MercLayout, InventoryLayout*, 0x3BCD4C)      // Updated 1.14d //007BCD4C-BASE
*/

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