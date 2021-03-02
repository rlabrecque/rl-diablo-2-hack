use super::d2core::D2Core;
use super::d2library::D2Library;
use super::packets::PacketFromServer;

use detour::GenericDetour;

/*
FUNCPTR(D2CLIENT, GetQuestInfo, void* __stdcall, (void), 0xB32D0) // Updated 1.14d //004B32D0-BASE

FUNCPTR(D2CLIENT, SubmitItem, void __fastcall, (DWORD dwItemId), 0xB2370) // Updated 1.14d //004B2370-BASE
FUNCPTR(D2CLIENT, Transmute, void __fastcall, (void), 0x8A0D0)            // Updated 1.14d //0048A0D0-BASE

FUNCPTR(D2CLIENT, FindClientSideUnit, UnitAny* __fastcall, (DWORD dwId, DWORD dwType), 0x63990) // Updated 1.14d //00463990-BASE
FUNCPTR(D2CLIENT, FindServerSideUnit, UnitAny* __fastcall, (DWORD dwId, DWORD dwType), 0x639B0) // Updated 1.14d //004639B0-BASE
FUNCPTR(D2CLIENT, GetCurrentInteractingNPC, UnitAny* __fastcall, (void), 0xB1620)               // Updated 1.14d //004B1620-BASE
FUNCPTR(D2CLIENT, GetSelectedUnit, UnitAny* __stdcall, (), 0x67A10)                             // Updated 1.14d //00467A10-BASE
FUNCPTR(D2CLIENT, GetCursorItem, UnitAny* __fastcall, (void), 0x680A0)                          // Updated 1.14d //004680A0-BASE
// FUNCPTR(D2CLIENT, GetMercUnit, UnitAny* __fastcall, (void), 0x78A90) //Updated 1.14d //00478A90-BASE 478F20 with 7 0 args
FUNCPTR(D2CLIENT, SetSelectedUnit_I, void __fastcall, (UnitAny * pUnit), 0x66DE0)                           // Updated 1.14d //00466DE0-BASE
FUNCPTR(D2CLIENT, GetItemName, BOOL __fastcall, (UnitAny * pItem, wchar_t* wBuffer, DWORD dwSize), 0x8C060) // Updated 1.14d //0048C060-BASE
FUNCPTR(D2CLIENT, LoadItemDesc, BOOL __stdcall, (UnitAny * pItem, int type), 0x8DD90)                       // Updated 1.14d //0048DD90-BASE
FUNCPTR(D2CLIENT, GetMonsterOwner, DWORD __fastcall, (DWORD nMonsterId), 0x79150)                           // Updated 1.14d //00479150-BASE
FUNCPTR(D2CLIENT, GetUnitHPPercent, DWORD __fastcall, (DWORD dwUnitId), 0x79080)                            // Updated 1.14d //00479080-BASE
FUNCPTR(D2CLIENT, InitInventory, void __fastcall, (void), 0x845A0)                                          // Updated 1.14d //004845A0-BASE
FUNCPTR(D2CLIENT, SetUIVar, DWORD __fastcall, (DWORD varno, DWORD howset, DWORD unknown1), 0x55F20)         // Updated 1.14d //00455F20-BASE
FUNCPTR(D2CLIENT, GetUnitX, int __fastcall, (UnitAny * pUnit), 0x5ADF0)                                     // Updated 1.14d //0045ADF0-BASE
FUNCPTR(D2CLIENT, GetUnitY, int __fastcall, (UnitAny * pUnit), 0x5AE20)                                     // Updated 1.14d //0045AE20-BASE

FUNCPTR(D2CLIENT, ShopAction, void __fastcall, (UnitAny * pNpc, UnitAny* pItem, DWORD dwSell, DWORD unk, DWORD dwItemCost, DWORD dwMode, DWORD _2, DWORD _3),
        0xB3870) // Updated 1.14d //004B3870-BASE
*/

pub fn close_npc_interact(game: &D2Library) {
    type CloseNPCInteractFn = extern "fastcall" fn();

    unsafe {
        std::mem::transmute::<usize, CloseNPCInteractFn>(game.fix_offset(0xB3F10usize))();
    }
}

pub fn close_interact(game: &D2Library) {
    type CloseInteractFn = extern "fastcall" fn();

    unsafe {
        std::mem::transmute::<usize, CloseInteractFn>(game.fix_offset(0x4C6B0usize))();
    }
}

pub fn get_automap_size(game: &D2Library) -> u32 {
    type GetAutomapSizeFn = extern "fastcall" fn() -> u32;

    unsafe { std::mem::transmute::<usize, GetAutomapSizeFn>(game.fix_offset(0x5A710usize))() }
}

/*
FUNCPTR(D2CLIENT, NewAutomapCell, AutomapCell* __fastcall, (), 0x57C30)                                                 // Updated 1.14d //00457C30-BASE
FUNCPTR(D2CLIENT, AddAutomapCell, void __fastcall, (AutomapCell * aCell, AutomapCell** node), 0x57B00)                  // Updated 1.14d //00457B00-BASE
FUNCPTR(D2CLIENT, RevealAutomapRoom, void __stdcall, (Room1 * pRoom1, DWORD dwClipFlag, AutomapLayer* aLayer), 0x58F40) // Updated 1.14d //00458F40-BASE
FUNCPTR(D2CLIENT, InitAutomapLayer_I, AutomapLayer* __fastcall, (DWORD nLayerNo), 0x58D40)                              // Updated 1.14d //00458D40-BASE

FUNCPTR(D2CLIENT, ClickMap, void __fastcall, (DWORD MouseFlag, DWORD x, DWORD y, DWORD Type), 0x62D00) // Updated 1.14d //00462D00-BASE
FUNCPTR(D2CLIENT, LeftClickItem_I, void __stdcall,
        (UnitAny * pPlayer, Inventory* pInventory, int x, int y, DWORD dwClickType, InventoryLayout* pLayout, DWORD Location),
        0x8FFE0) // Updated 1.14d //0048FFE0-BASE
*/

pub fn get_mouse_x_offset(game: &D2Library) -> u32 {
    type GetMouseXOffsetFn = extern "fastcall" fn() -> u32;

    unsafe { std::mem::transmute::<usize, GetMouseXOffsetFn>(game.fix_offset(0x5AFB0usize))() }
}

pub fn get_mouse_y_offset(game: &D2Library) -> u32 {
    type GetMouseYOffsetFn = extern "fastcall" fn() -> u32;

    unsafe { std::mem::transmute::<usize, GetMouseYOffsetFn>(game.fix_offset(0x5AFC0usize))() }
}

pub type PrintGameStringFn = extern "fastcall" fn(message: *const u16, color: i32);

pub fn create_hook_print_game_string(game: &D2Library) -> GenericDetour<PrintGameStringFn> {
    unsafe {
        let print_game_string_fn: PrintGameStringFn = std::mem::transmute(game.fix_offset(0x9E3A0));

        let print_game_string_detour =
            GenericDetour::<PrintGameStringFn>::new(print_game_string_fn, print_game_string_hook).unwrap();
        print_game_string_detour.enable().unwrap();
        print_game_string_detour
    }
}

extern "fastcall" fn print_game_string_hook(message: *const u16, color: i32) {
    let msg = unsafe { widestring::WideCStr::from_ptr_str(message).to_string_lossy() };

    println!("print_game_string_hook: {} {}", color, msg);

    D2Core::get().print_game_string_detour.call(message, color);
}

pub fn print_game_string(d2core: &D2Core, message: &str, color: i32) {
    unsafe {
        let widestr = widestring::WideCString::from_str(message).unwrap();
        let widestr = widestr.into_raw();
        d2core.print_game_string_detour.call(widestr, color);
        let _ = widestring::WideCString::from_raw(widestr);
    }
}

pub fn print_party_string(game: &D2Library, message: &str, color: i32) {
    type PrintPartyStringFn = extern "fastcall" fn(message: *const u16, color: i32);

    unsafe {
        let widestr = widestring::WideCString::from_str(message).unwrap();
        let widestr = widestr.into_raw();
        std::mem::transmute::<usize, PrintPartyStringFn>(game.fix_offset(0x9E5C0usize))(widestr, color);
        let _ = widestring::WideCString::from_raw(widestr);
    }
}

/*

FUNCPTR(D2CLIENT, LeaveParty, void __fastcall, (void), 0x79FC0) // Updated 1.14d //00479FC0-BASE

FUNCPTR(D2CLIENT, AcceptTrade, void __fastcall, (void), 0xB9070) // Updated 1.14d //004B9070-BASE
FUNCPTR(D2CLIENT, CancelTrade, void __fastcall, (void), 0xB90B0) // Updated 1.14d //004B90B0-BASE
FUNCPTR(D2CLIENT, TradeOK, void __stdcall, (void), 0xB8A30)      // Updated 1.14d //004B8A30-BASE
*/

pub fn get_difficulty(game: &D2Library) -> u8 {
    type GetDifficultyFn = extern "stdcall" fn() -> u8;

    unsafe { std::mem::transmute::<usize, GetDifficultyFn>(game.fix_offset(0x4DCD0usize))() }
}

/*
FUNCPTR(D2CLIENT, GetUiVar_I, DWORD __fastcall, (DWORD dwVarNo), 0x538D0) // Updated 1.14d //004538D0-BASE

FUNCPTR(D2CLIENT, DrawRectFrame, VOID __fastcall, (DWORD Rect), 0x52E50) // Updated 1.14d //00452E50-BASE

FUNCPTR(D2CLIENT, PerformGoldDialogAction, void __fastcall, (void), 0x54080) // Updated 1.14d //00454080-BASE

FUNCPTR(D2CLIENT, GetPlayerUnit, UnitAny* __stdcall, (), 0x63DD0) // Updated 1.14d //00463DD0-BASE

// FUNCPTR(D2CLIENT, GetLevelName_I, wchar_t* __fastcall, (DWORD levelId), 0x53E70) //Updated 1.14d //00453E70-BASE

FUNCPTR(D2CLIENT, ClearScreen, void __fastcall, (void), 0xB4620) // Updated 1.14d //004B4620-BASE

FUNCPTR(D2CLIENT, CloseNPCTalk, DWORD __stdcall, (void* unk), 0xA17D0) // Updated 1.14d //004A17D0-BASE

FUNCPTR(D2CLIENT, TestPvpFlag, DWORD __fastcall, (DWORD dwUnitId1, DWORD dwUnitId2, DWORD dwFlag), 0xDC440) // Updated 1.14d //004DC440-BASE

*/

pub fn get_game_language_code(game: &D2Library) -> u32 {
    type GetGameLanguageCodeFn = extern "fastcall" fn() -> u32;

    unsafe { std::mem::transmute::<usize, GetGameLanguageCodeFn>(game.fix_offset(0x125150usize))() }
}

pub type ExitGameFn = extern "fastcall" fn();

pub fn create_hook_exit_game(game: &D2Library) -> GenericDetour<ExitGameFn> {
    unsafe {
        let exit_game_fn: ExitGameFn = std::mem::transmute(game.fix_offset(0x4DD60));

        let exit_game_detour = GenericDetour::<ExitGameFn>::new(exit_game_fn, exit_game_hook).unwrap();
        exit_game_detour.enable().unwrap();
        exit_game_detour
    }
}

extern "fastcall" fn exit_game_hook() {
    println!("exit_game_hook");

    exit_game(D2Core::get());
}

pub fn exit_game(d2core: &D2Core) {
    d2core.exit_game_detour.call();
}

/*
    {PatchCall, GetDllOffset("D2Client.dll", 0x5F802), (DWORD)GamePacketReceived_Intercept, 5}, // Updated 1.14d //0045F802-BASE
    {PatchJmp, GetDllOffset("D2Client.dll", 0x12AE5A), (DWORD)GamePacketSent_Interception, 5},  // Updated 1.14d //0052AE5A-BASE
*/

pub type GamePacketReceivedFn = extern "fastcall" fn(packet: *const u8, size: i32);

pub fn create_hook_game_packet_received(game: &D2Library) -> GenericDetour<GamePacketReceivedFn> {
    unsafe {
        let game_packet_received_fn: GamePacketReceivedFn = std::mem::transmute(game.fix_offset(0x5F7B0));

        let hook_game_packet_received_detour =
            GenericDetour::<GamePacketReceivedFn>::new(game_packet_received_fn, game_packet_received_hook).unwrap();
        hook_game_packet_received_detour.enable().unwrap();
        hook_game_packet_received_detour
    }
}

extern "fastcall" fn game_packet_received_hook(packet: *const u8, size: i32) {
    if size != -1 {
        let packet_type = unsafe { *packet.offset(0) };
        println!("game_packet_received_hook: Packet: 0x{:x}", packet_type);

        let packet_enum: PacketFromServer = PacketFromServer::convert(packet, size).unwrap();
        println!(
            "game_packet_received_hook: Packet: 0x{:x} {:?} Size: {}",
            packet_type, packet_enum, size
        );

        match packet_enum {
            PacketFromServer::ConnectionInfo(test) => {
                println!("ConnectionInfo: {:?}", test);
            }
            PacketFromServer::WeaponSwitch => {
                println!("WeaponSwitch");
            }
            _ => {}
        }
    }

    D2Core::get().game_packet_received_detour.call(packet, size);
}

pub fn game_packet_received(d2core: &D2Core, packet: *const u8, size: i32) {
    d2core.game_packet_received_detour.call(packet, size);
}
