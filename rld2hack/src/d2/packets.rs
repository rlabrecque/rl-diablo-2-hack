use super::types;

/// 0x01
///
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct GameFlagsPacketBody {
    difficulty: types::Difficulty,
    arena_flags: u32,
    expansion: u8,
    ladder: u8,
}

/// 0x8f
/// Response from C->S Ping packet.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct PongPacketBody {
    /// Always 0x00 * 32
    bytes: [u8; 32],
}

/// 0xaf
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ConnectionInfoPacketBody {
    /// Whether compression is enabled or not.
    compression: bool
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum PacketFromServer {
    /// 0x00
    GameLoading,
    /// 0x01
    GameFlags(GameFlagsPacketBody),
    /// 0x02
    LoadSuccessful,
    /// 0x03
    LoadAct,
    /// 0x04
    LoadComplete,
    /// 0x05
    UnloadComplete,
    /// 0x06
    GameExitSuccessful,
    /// 0x07
    MapReveal,
    /// 0x08
    MapHide,
    /// 0x0a
    RemoveObject,
    /// 0x0b
    GameHandshake,
    /// 0x0d
    PlayerStop,
    /// 0x0e
    ObjectState,
    /// 0x0f
    PlayerMove,
    /// 0x15
    ReassignPlayer,
    /// 0x19
    GoldToInv,
    /// 0x1a
    AddExpByte,
    /// 0x1b
    AddExpWord,
    /// 0x1c
    AddExpDword,
    /// 0x1d
    BaseAttributeByte,
    /// 0x1e
    BaseAttributeWord,
    /// 0x1f
    BaseAttributeDword,
    /// 0x22
    UpdateItemSkill,
    /// 0x23
    SetSkill,
    /// 0x26
    GameChat,
    /// 0x27
    NPCInfo,
    /// 0x28
    QuestInfo,
    /// 0x29
    GameQuestInfo,
    /// 0x2a
    NPCTransaction,
    /// 0x37
    Unknown0x37,
    /// 0x3e
    UpdateItemStats,
    /// 0x3f
    UseStackableItem,
    /// 0x42
    ClearCursor,
    /// 0x47
    Relator1,
    /// 0x48
    Relator2,
    /// 0x4c
    UnitSkillOnTarget,
    /// 0x4f
    ClearMercList,
    /// 0x51
    AssignObject,
    /// 0x53
    Darkness,
    /// 0x59
    AssignPlayer,
    /// 0x5a
    EventMessages,
    /// 0x5b
    PlayerInGame,
    /// 0x5d
    QuestItemState,
    /// 0x5e
    GameQuestAvailability,
    /// 0x5f
    Unknown0x5f,
    /// 0x63
    WaypointMenu,
    /// 0x65
    PlayerKillCount,
    /// 0x67
    NpcMove,
    /// 0x68
    NpcMoveToTarget,
    /// 0x69
    NpcState,
    /// 0x6b
    NpcAction,
    /// 0x6c
    NpcAttack,
    /// 0x6d
    NpcStop,
    /// 0x76
    PlayerInProximity,
    /// 0x77
    ButtonActions,
    /// 0x7e
    ReloadCMNCOF,
    /// 0x8a
    NpcWantsToInteract,
    /// 0x8d
    AssignPlayerToParty,
    /// 0x8f
    Pong(PongPacketBody),
    /// 0x91
    SetNpcGossipAct,
    /// 0x94
    BaseSkillLevels,
    /// 0x95
    LifeAndManaUpdate,
    /// 0x96
    WalkVerify,
    /// 0x97
    WeaponSwitch,
    /// 0x9c
    ItemActionWorld,
    /// 0x9d
    ItemActionOwned,
    /// 0xa7
    DelayedState,
    /// 0xa8
    SetState,
    /// 0xa9
    EndState,
    /// 0xaa
    AddUnit,
    /// 0xab
    NpcHeal,
    /// 0xac
    AssignNPC,
    /// 0xaf
    ConnectionInfo(ConnectionInfoPacketBody),
    /// 0xb0
    GameConnectionTerminated,
}

impl PacketFromServer {
    pub fn convert(packet: *const u8, _size: i32) -> Result<Self, ()> {
        let packet_id = unsafe { *packet.offset(0) };
        let packet = unsafe { packet.offset(1) };

        match packet_id {
            0x00 => Ok(PacketFromServer::GameLoading),
            0x01 => Ok(PacketFromServer::GameFlags(unsafe { (packet as *const GameFlagsPacketBody).read() })),
            0x02 => Ok(PacketFromServer::LoadSuccessful),
            0x03 => Ok(PacketFromServer::LoadAct),
            0x04 => Ok(PacketFromServer::LoadComplete),
            0x05 => Ok(PacketFromServer::UnloadComplete),
            0x06 => Ok(PacketFromServer::GameExitSuccessful),
            0x07 => Ok(PacketFromServer::MapReveal),
            0x08 => Ok(PacketFromServer::MapHide),
            0x0a => Ok(PacketFromServer::RemoveObject),
            0x0b => Ok(PacketFromServer::GameHandshake),
            0x0d => Ok(PacketFromServer::PlayerStop),
            0x0e => Ok(PacketFromServer::ObjectState),
            0x0f => Ok(PacketFromServer::PlayerMove),
            0x15 => Ok(PacketFromServer::ReassignPlayer),
            0x19 => Ok(PacketFromServer::GoldToInv),
            0x1a => Ok(PacketFromServer::AddExpByte),
            0x1b => Ok(PacketFromServer::AddExpWord),
            0x1c => Ok(PacketFromServer::AddExpDword),
            0x1d => Ok(PacketFromServer::BaseAttributeByte),
            0x1e => Ok(PacketFromServer::BaseAttributeWord),
            0x1f => Ok(PacketFromServer::BaseAttributeDword),
            0x22 => Ok(PacketFromServer::UpdateItemSkill),
            0x23 => Ok(PacketFromServer::SetSkill),
            0x26 => Ok(PacketFromServer::GameChat),
            0x27 => Ok(PacketFromServer::NPCInfo),
            0x28 => Ok(PacketFromServer::QuestInfo),
            0x29 => Ok(PacketFromServer::GameQuestInfo),
            0x2a => Ok(PacketFromServer::NPCTransaction),
            0x37 => Ok(PacketFromServer::Unknown0x37),
            0x3e => Ok(PacketFromServer::UpdateItemStats),
            0x3f => Ok(PacketFromServer::UseStackableItem),
            0x42 => Ok(PacketFromServer::ClearCursor),
            0x47 => Ok(PacketFromServer::Relator1),
            0x48 => Ok(PacketFromServer::Relator2),
            0x4c => Ok(PacketFromServer::UnitSkillOnTarget),
            0x4f => Ok(PacketFromServer::ClearMercList),
            0x51 => Ok(PacketFromServer::AssignObject),
            0x53 => Ok(PacketFromServer::Darkness),
            0x59 => Ok(PacketFromServer::AssignPlayer),
            0x5a => Ok(PacketFromServer::EventMessages),
            0x5b => Ok(PacketFromServer::PlayerInGame,),
            0x5d => Ok(PacketFromServer::QuestItemState),
            0x5e => Ok(PacketFromServer::GameQuestAvailability),
            0x5f => Ok(PacketFromServer::Unknown0x5f),
            0x63 => Ok(PacketFromServer::WaypointMenu),
            0x65 => Ok(PacketFromServer::PlayerKillCount),
            0x67 => Ok(PacketFromServer::NpcMove),
            0x68 => Ok(PacketFromServer::NpcMoveToTarget),
            0x69 => Ok(PacketFromServer::NpcState),
            0x6b => Ok(PacketFromServer::NpcAction),
            0x6c => Ok(PacketFromServer::NpcAttack),
            0x6d => Ok(PacketFromServer::NpcStop),
            0x76 => Ok(PacketFromServer::PlayerInProximity),
            0x77 => Ok(PacketFromServer::ButtonActions),
            0x7e => Ok(PacketFromServer::ReloadCMNCOF),
            0x8a => Ok(PacketFromServer::NpcWantsToInteract),
            0x8d => Ok(PacketFromServer::AssignPlayerToParty),
            0x8f => Ok(PacketFromServer::Pong(unsafe { (packet as *const PongPacketBody).read() })),
            0x91 => Ok(PacketFromServer::SetNpcGossipAct),
            0x94 => Ok(PacketFromServer::BaseSkillLevels),
            0x95 => Ok(PacketFromServer::LifeAndManaUpdate),
            0x96 => Ok(PacketFromServer::WalkVerify),
            0x97 => Ok(PacketFromServer::WeaponSwitch),
            0x9c => Ok(PacketFromServer::ItemActionWorld),
            0x9d => Ok(PacketFromServer::ItemActionOwned),
            0xa7 => Ok(PacketFromServer::DelayedState),
            0xa8 => Ok(PacketFromServer::SetState),
            0xa9 => Ok(PacketFromServer::EndState),
            0xaa => Ok(PacketFromServer::AddUnit),
            0xab => Ok(PacketFromServer::NpcHeal),
            0xac => Ok(PacketFromServer::AssignNPC),
            0xaf => Ok(PacketFromServer::ConnectionInfo(unsafe { (packet as *const ConnectionInfoPacketBody).read() })),
            0xb0 => Ok(PacketFromServer::GameConnectionTerminated),
            _ => Err(()),
        }
    }
}

/*
CMD	Size	Description	Struct	Comment
0x00	1	Game Loading      	0x00
0x01	8	Game Flags      	0x01 [BYTE Difficulty] [DWORD dwArenaFlags] [BYTE bIsExpansion] [BYTE bIsLadder]	corrected 2020.12.03 - thanks to misiek1294
0x02	1	Load Successful      	0x02
0x03	12	Load Act      	0x03 [BYTE Act] [DWORD Map ID] [WORD Area  Id] [DWORD Unknown]
0x04	1	Load Complete      	0x04
0x05	1	Unload Complete   	0x05
0x06	1	Game Exit Sucessful   	0x06
0x07	6	Map Reveal    	0x07 [WORD Tile X] [WORD Tile Y] [BYTE Area Id]
0x08	6	Map Hide      	0x08 [WORD Tile X] [WORD Tile Y] [BYTE Area Id]
0x09	12	Assign Level Warp	0x09 [BYTE WarpType] [DWORD WarpGid] [BYTE WarpClassId] [WORD WarpX] [WORD WarpY]	corrected 2018.12.05
0x0A	6	Remove Object      	0x0A [BYTE Unit Type] [DWORD Unit Id]
0x0B	6	Game Handshake   	0x0B [BYTE Unit Type] [DWORD Unit Id]
0x0C	9	NPC Hit         	0x0C [BYTE Unit Type] [DWORD Unit Id] [WORD  Animation Id] [BYTE Life]
0x0D	13	Player Stop      	0x0D [BYTE Unit Type] [DWORD Unit Id] [BYTE  Unknown] [WORD Unit X] [WORD Unit Y] [BYTE Unknown] [BYTE Life]
0x0E	12	Object State      	0x0E [BYTE UnitType] [DWORD UnitGUID] [BYTE  PortalFlags] [BYTE FlagIsTargetable] [DWORD  UnitState]	corrected 2019.03.30 - thanks to misiek1294
0x0F	16	Player Move      	0x0F [BYTE Unit Type] [DWORD Unit Id] [BYTE  0x01 = Walk || 0x23 = Run || 0x20 = Knockback] [WORD Target X] [WORD Target Y] 00  [WORD Current X] [WORD Current Y]
0x10	16	Player To Target   	0x10 [BYTE Unit Type] [DWORD Unit Id] [BYTE  0x02 = Walk || 0x24 = Run] [BYTE Target Type] [DWORD Target Id] [WORD Current X]  [WORD Current Y]
0x11	8	Report Kill      	0x11 [BYTE Unit Type] [DWORD Unit Id] [WORD  Unknown]
0x12	0
0x13	0
0x14	0
0x15	11	Reassign Player   	0x15 [BYTE Unit Type] [DWORD Unit Id] [WORD  X] [WORD Y] [BYTE 0x01 = True || 0x00 = False]
0x16	Count * 9 + 4 	<Unknown>         	0x16 [BYTE Unknown] [BYTE Unknown] [BYTE Count] ARRAY[Count] ([BYTE UnitType] [DWORD UnitGid] [WORD x] [WORD y])
0x17	12	<Unknown>         	0x17 [BYTE UnitType] [DWORD UnitGid] [BYTE bUnknown0] [BYTE bUnknown1] [WORD wUnknown2] [WORD wUnknown3]
0x18	15	PlayerHPMP	0x18 [BITS[15] HP] [BITS[15] MP] [BITS[15] Stamina] [BITS[7] HPRegen] [BITS[7] MPRegen] [BITS[16] x] [BITS[16] y] [BITS[8] Vx] [ BITS[8] Vy]	Used to update player
0x19	2	(BYTE)Gold to Inv.   	0x19 [BYTE Amount]
0x1A	2	(BYTE)Add Exp.      	0x1A [BYTE Amount]
0x1B	3	(WORD)Add Exp.   	0x1B [WORD Amount]
0x1C	5	(DWORD)Add Exp.   	0x1C [DWORD Amount]
0x1D	3	(BYTE)Base Attribute   	0x1D [BYTE Attribute] [BYTE Amount]	getPacket(1, 0x1d, 1, stat, 1, value);
0x1E	4	(WORD)Base Attribute   	0x1E [BYTE Attribute] [WORD Amount]
0x1F	6	(DWORD)Base Attribute   	0x1F [BYTE Attribute] [DWORD Amount]
0x20	10	Attribute Update    	0x20 [DWORD Unit Id] [BYTE Attribute] [DWORD  Amount]
0x21	12	Update Item Oskill  	0x21 [WORD Unknown] [DWORD Unit Id] [WORD Skill] [BYTE Base Level] [BYTE Bonus Amount] [BYTE Unknown]
0x22	12	Update Item Skill   	0x22 [WORD Unknown (Unit Type?)] [DWORD Unit Id] [WORD Skill] [BYTE Amount] [WORD Unknown]
0x23	13	Set Skill      	0x23 [BYTE UnitType] [DWORD UnitGid] [BYTE Hand (R=0, L =1)] [WORD Skill] [DWORD ItemGid]
0x24	0
0x25	0
0x26	variable	Game Chat      	0x26 [BYTE ChatType] [BYTE LocaleId] [BYTE UnitType] [DWORD UnitGid] [BYTE ChatColor] [BYTE ChatSubType] [NULLSTRING Nick] [NULLSTRING Message]
0x27	40	NPC Info      	0x27 [BYTE Unit Type] [DWORD Unit Id]  [BYTE Count] [BYTE Unknown] ARRAY[Count] ([BYTE Show] [BYTE Unused] [WORD MessageId] )	Used by game to show messages from npc after interacting with
0x28	103	Quest Info      	0x28 [BYTE UpdateType] [DWORD UnitGid] [BYTE Timer] ARRAY[96] ([BYTE QuestBit])
0x29	97	Game Quest Info   	0x29 [BYTE[96] QuestBit]
0x2A	15	NPC Transaction   	0x2A [BYTE TradeType] [BYTE Result] [DWORD Unknown] [DWORD NpcGid] [DWORD GoldInInventory]	TRADETYPE: 0x00 = to inventory (buying) 0x03 = from inventory/belt (selling(including gambling)) 0x04 = to belt 0x05 = to stackable objects (books/keys) RESULT: 0x00 = Purchased, 0x01 = Sold, 0x0a = No Space, 0x0c = Insuffecient Gold. Thanks to jaenster for TradeType and Result research
0x2B	0
0x2C	8	Play Sound      	0x2C [BYTE Unit Type] [DWORD Unit Id] [WORD  Sound]
0x2D	0
0x2E	0
0x2F	0
0x30	0
0x31	0
0x32	0
0x33	0
0x34	0
0x35	0
0x36	0
0x37	0
0x38	0
0x39	0
0x3A	0
0x3B	0
0x3C	0
0x3D	0
0x3E	variable	Update Item Stats   	0x3E *
0x3F	8	Use Stackable Item   	0x3F [BYTE SellIcon] [DWORD ItemGid] [WORD SkillId]
0x40	13	<Unknown>     	0x40 [DWORD ItemGid] [DWORD Unknown] [DWORD Unknown]
0x41	0
0x42	6	Clear Cursor     	0x42 [BYTE Unit Type] [DWORD Player Id]
0x43	0
0x44	0
0x45	0
0x46	0
0x47	11	Relator 1      	0x47 [BYTE UnitType] [BYTE Gap] [DWORD Unit Id] [BYTE[4] Unused]
0x48	11	Relator 2     	0x48 [BYTE UnitType] [BYTE Gap] [DWORD Unit Id] [BYTE[4] Unused]
0x49	0
0x4A	0
0x4B	0
0x4C	16	Unit Skill on Target    	0x4C [BYTE Unit Type] [DWORD Unit Id] [WORD  Skill] [BYTE Unknown] [BYTE Unknown] [DWORD Target Id] 00 00
0x4D	17	Unit Cast Skill         	0x4D [BYTE Unit Type] [DWORD Unit Id] [DWORD  Skill] [BYTE Unknown] [WORD X] [WORD Y] 00 00
0x4E	7	Merc For Hire      	0x4E [WORD MercNameString] [DWORD Seed]
0x4F	1	Clear Merc List	0x4F
0x50	15	Quest Special	0x50 [WORD MessageType] [WORD[6] Argument]
0x51	14	Assign Object      	0x51 [BYTE Object Type] [DWORD Object Id]  [WORD Object Code] [WORD X] [WORD Y] [BYTE State] [BYTE Interaction Type]
0x52	42	Player Quest Log	0x52 [BYTE[41] Quest]
0x53	10	Darkness	0x53 [DWORD ActTBC] [DWORD AngleTBC]  [BYTE Darkness]	Takes pointer from Act gnAct+4 and setup some viariables in there also do some calcualtions with sin, cos and convertng degrees to radians
0x54	0
0x55	0
0x56	0
0x57	14	NPC Enchants	0x57 [DWORD MonsterGid] [BYTE MonsterType] [WORD MonsterNameIDX] [BYTE anEnchant1] [BYTE anEnchant2] [BYTE anEnchant3] [BYTE Filler] [WORD MonsterIsChampion]	2019.01.11 - Updated
0x58	7	Open User Interface	0x58 [DWORD UnitGid] [BYTE UIType] [BYTE Bool]	thakns to devurandom for sharing notes
0x59	26	Assign Player      	0x59 [DWORD Unit Id] [BYTE Char Type]  [NULLSTRING[16] Char Name] [WORD X] [WORD Y]
0x5A	40	Event Messages          	0x5A [BYTE MessageType] [BYTE Color] [DWORD Arg] [BYTE ArgTypes] [NULLSTRING[16] Name1] [NULLSTRING[16] NAME2]
0x5B	PacketLength	Player In Game          	0x5B [WORD PacketLength] [DWORD Player Id] [BYTE Char Type] [NULLSTRING[16] Char Name] [WORD Char Lvl] [WORD Party Id] 00 00 00 00 00 00 00 00
0x5C	5	Player Left Game        	0x5C [DWORD Player Id]
0x5D	6	Quest Item State        	0x5D [BYTE QuestId] [BYTE AlertFlags] [BYTE FilterStatus] [WORD Extra]
0x5E	38	Game Quest Availability	0x5E [BYTE[37] Quest]	thanks richard for this
0x5F	5	<Unknown>               	0x5F [DWORD Unknown]
0x60	7	Townportal State        	0x60 [BYTE State] [BYTE Area Id] [DWORD Unit Id]
0x61	2	CanGoToAct	0x61 [BYTE Act]	Act = 1-5 (Acts), 7 = CowLevel
0x62	7	<Unknown>               	0x62 [BYTE UnitType] [DWORD UnitGid] [BYTE Unused]	Somehow connected to incoming 0x58
0x63	21	Waypoint Menu           	0x63 [DWORD dwWaypointGid] [WORD wSetOrDel] [BITS[64] Waypoint] [BYTE[6] Zero]	2019.10.13 - If wSetOrDel is equal to 258 it update waypoints list with data from packet, if wSetOrDel is 257 it clears waypoint list - every other value causes game error
0x64	0
0x65	7	Player Kill Count       	0x65 [DWORD Player Id] [WORD Count]
0x66	0
0x67	16	NPC Move                	0x67 [DWORD NpcGid] [BYTE 0x01 = Walk || 0x17 = Run] [WORD X] [WORD Y] [WORD Unknown] [BYTE Unknown] [WORD Unknown] [BYTE Unknown]
0x68	21	NPC Move to Target      	0x68 [DWORD NpcGid] [BYTE 0x00 = Walk || 0x18 = Run] [WORD X] [WORD Y] [BYTE Target Unit Type] [DWORD Target Id] [WORD Unknown] [BYTE Unknown] [WORD Unknown] [BYTE Unknown]
0x69	12	NPC State               	0x69 [DWORD NpcGid] [BYTE State] [WORD X] [WORD Y] [BYTE Unit Life] [BYTE Unknown]
0x6A	12	<UNKNOWN>	0x6A [DWORD NpcGid] [BYTE Unknown0] [BYTE Unknown1] [DWORD Unknown2] [BYTE Unknown3]
0x6B	16	NPC Action              	0x6B [DWORD NpcGid] [BYTE Action] 00 00 00 00 00 00 [WORD X] [WORD Y]
0x6C	16	NPC Attack              	0x6C [DWORD NpcGid] [WORD Attack Type] [DWORD Target Id] [BYTE Target Type] [WORD X] [WORD Y]
0x6D	10	NPC Stop                	0x6D [DWORD NpcGid] [WORD X] [WORD Y] [BYTE Unit Life]
0x6E	0
0x6F	0
0x70	0
0x71	0
0x72	0
0x73	32	<Unknown>         	0x73 [DWORD Unused] [WORD Unknown] [DWORD Unknown] [DWORD Unknown] [DWORD Unknown] [DWORD Unknown] [WORD Unknown] [BYTE OwnerType]  [DWORD OwnerGid] [BYTE Unknown] [BYTE PierceIdxValue]
0x74	10	Player Corpse Assign    	0x74 [BYTE Assign 0x00 = False || 0x01 True] [DWORD Owner Id] [DWORD Corpse Id]
0x75	13	Player Party Info       	0x75 [DWORD Unit Id] [WORD Party Id] [WORD Char Level] [WORD Relationship] [WORD In Your Party? 0x00 = False || 0x01 = True]
0x76	6	Player In Proximity     	0x76 [BYTE Unit Type] [BYTE Unit Id]
0x77	2	Button Actions          	0x77 [BYTE Action]	getPacket(1, 0x77, 1, 0x0C);
0x78	21	Trade Accepted          	0x78 [NULLSTRING[16] Char Name] [DWORD Unit Id]
0x79	6	Gold in Trade           	0x79 [BYTE Gold Owner] [DWORD Amount]
0x7A	13	Pet Action              	0x7A [BYTE 0x00 = Unsummoned/Lost Sight || 0x01 = Summoned/Assign] [BYTE Skill] [WORD Pet Type] [DWORD Owner Id] [DWORD Pet Id]
0x7B	8	Assign Skill Hotkey     	0x7B [BYTE Slot] [BYTE Skill] [BYTE 0x00 = Right || 0x80 = Left] FF FF FF FF	In Game Skill and Hand is actually WORD but later it is AND to separate values
0x7C	6	Use Scroll              	0x7C [BYTE Type] [DWORD Item Id]
0x7D	18	Set Item Flags	0x7D [BYTE UnitType] [DWORD UnitGid] [DWORD ItemGid] [DWORD AndValue] [DWORD dwFlagsAfterAnd]	dwFlagsAfterAnd = item.dwFlags & AndValue
0x7E	5	CMNCOF	0x7E [BYTE[4] Unused]	2019.01.24 Reload CMNCOF file and reassign its 4global variable for current act
0x7F	10	Ally Party Info         	0x7F  [BYTE Unit Type] [WORD Unit Life] [DWORD Unit Id] [WORD Unit Area Id]   	2019.01.28 Last arg is WORD (misiek1294)
0x80	0
0x81	20	Assign Merc      	0x81 [BYTE Unknown] [WORD Merc Kind?] [DWORD Owner Id] [DWORD Merc Id] [DWORD Unknown] [DWORD Unknown]
0x82	29	Portal Ownership        	0x82 [DWORD OwnerGid] [NULLSTRING[16] OwnerName] [DWORD PortalGid] [DWORD DestinationId]
0x83	0
0x84	0
0x85	0
0x86	0
0x87	0
0x88	0
0x89	2	Special Quest Event     	0x89 [BYTE Event Id]
0x8A	6	NPC Wants to Interact   	0x8A [BYTE Unit Type] [DWORD Unit Id]
0x8B	6	Player Relationship     	0x8B [DWORD Unit Id] [BYTE 0x00 = No Party || 0x01 = In Party || 0x02 = Wants to Party]
0x8C	11	Relationship Update     	0x8C [DWORD Player 1 Id] [DWORD Player 2 Id] [WORD Relation State]
0x8D	7	Assign Player To Party  	0x8D [DWORD Player Id] [WORD Party Id]
0x8E	10	Corpse Assign           	0x8E [BYTE 0x00 = Unassign || 0x01 = Assign] [DWORD Owner Id] [DWORD Corpse Id]
0x8F	33	Pong                    	0x8F [BYTES[32] 0x00]
0x90	13	Party Automap Info      	0x90 [DWORD Player Id] [DWORD Player X] [DWORD Player Y]
0x91	26	Set NPC Gossip (Act)	0x91 [BYTE ACT] [WORD[12] Str/NPC ID]	Each str id is for a sep npc
0x92	6	Remove Unit Display	0x92 [BYTE UnitType] [DWORD UnitGid]
0x93	8	<Unknown>               	0x93 [DWORD PlayerGid] [BYTE signedUnknown] [BYTE Unknown] [BYTE Unknown]
0x94	Count * 3 + 6	Base Skill Levels       	0x94 [BYTE Count] [DWORD Player Id] ARRAY[Count] ( [WORD SkillId] [BYTE Level] )
0x95	13	Life and Mana Update    	0x95 [BITS[15] HP] [BITS[15] MP] [BITS[15] Stamina] [BITS[7] HPRegen] [BITS[7] MPRegen] [BITS[16] x] [BITS[16] y] [BITS[8] Vx] [ BITS[8] Vy]
0x96	9	Walk Verify             	0x96 [BITS[15] Stamina] [BITS[16] x] [BITS[16] y] [BITS[8 Vx] [BITS[8] Vy]
0x97	1	Weapon Switch           	0x97	By Default we have slot 0, once packet is received it does slot = !slot
0x98	7	UpdateNPCUnknownField40	0x98 [DWORD UnitGid] [WORD Value]
0x99	16	Skill Cast on Unit	0x99 [BYTE AttackerType] [DWORD AttackerGid] [WORD SkillId] [BYTE SkillLevel] [BYTE TargetType] [DWORD TargetGid] [WORD Unknown]
0x9A	17	Skill Cast on X and Y	0x9A [BYTE AttackerType] [DWORD AttackerGid] [WORD SkillId] [WORD Unused] [BYTE SkillLevel] [WORD TargetX] [WORD TargetY] [WORD Unnown]
0x9B	7	MercReviveCost	0x9B [WORD MercNameId] [DWORD ReviveCost]	2019.02.05 - Confirimed by Szumigajowy that cost is DWORD but game use WORD to display revive cost
0x9C	variable	Item Action (World)     	0x9C *
0x9D	variable	Item Action (Owned)     	0x9D *
0x9E	7	SetMercStat	0x9E [BYTE StatId] [DWORD MercGid] [BYTE NewValue]
0x9F	8	SetMercStat	0x9F [BYTE StatId] [DWORD MercGid] [WORD NewValue]
0xA0	10	SetMercStat	0xA0 [BYTE StatId] [DWORD MercGid] [DWORD NewValue]
0xA1	7	AddMercExp	0xA1 [BYTE StatId] [DWORD MercGid] [BYTE AddValue]
0xA2	8	AddMercExp	0xA2 [BYTE StatId] [DWORD MercGid] [WORD AddValue]
0xA3	24	Skill Aura Stat             	0xA3 [BYTE AuraStat] [WORD SkillId] [WORD SkillLevel] [BYTE UnitType] [DWORD UnitGid] [BYTE TargetType] [DWORD TargetGid] [DWORD TargetX] [DWORD TargetY]	2019.01.20 - Skill + AuraStat set on Unit, Target set to Unit or Coords
0xA4	3	Next Baal Wave NPC ClassId	0xA4 [WORD ClassId]
0xA5	8	State Skill Move	0xA5 [BYTE UnitType] [DWORD UnitGid] [WORD SkillId]
0xA6	variable	<Unknown>               	0xA6
0xA7	7	Delayed State           	0xA7 [BYTE Unit Type] [DWORD Unit Id] [BYTE State]
0xA8	PacketLength	Set State               	0xA8 [BYTE Unit Type] [DWORD Unit Id] [BYTE PacketLength] [BYTE State] [VOID State Effects]
0xA9	7	End State               	0xA9 [BYTE Unit Type] [DWORD Unit Id] [BYTE State]
0xAA	PacketLength	Add Unit                	0xAA [BYTE Unit Type] [DWORD Unit Id] [BYTE PacketLength] [VOID State Info]
0xAB	7	NPC Heal                	0xAB [BYTE Unit Type] [DWORD Unit Id] [BYTE Unit Life]
0xAC	variable	Assign NPC              	0xAC [DWORD Unit Id] [WORD Unit Code] [WORD X] [WORD Y] [BYTE Unit Life] [BYTE Packet Length] [VOID State Info]
0xAD	0
0xAE	LengthNoHeader + 1	Warden Request          	0xAE [WORD LengthNoHeader] [VOID Data]
0xAF	2	Connection Info         	0xAF [BYTE Compression]	1 with compression, 0 without compression
0xB0	1	Game Connection Terminated   	0xB0
0xB1	0
0xB2	53	GamesInfo	0xB2 [NULLSTRING[16] Unk1] [NULLSTRING[16] Unk2] [NULLSTRING[16] Unk3] [WORD nClientsCount] [WORD nGameToken]	Firehawk: "0x6A is sent to receive information about all of the games hosted by that particular GS"
0xB3	ChunkSize + 7	DownloadSave	0xB3 [BYTE ChunkSize] [BOOL FirstPart] [DWORD FullSize] [BYTE[ChunkSize] RawBytes]	send to client on leaving TCP/IP game
0xB4	5	TimeOut	0xB4 [DWORD Reason]
*/
