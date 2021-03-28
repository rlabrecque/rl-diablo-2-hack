use enumflags2::BitFlags;

use super::types;

/// 0x01
/// Response from C->S GameFlags packet.
#[repr(C, packed(1))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GameFlagsPacketBody {
    pub packet_id: u8,
    pub difficulty: types::Difficulty,
    pub arena_flags: BitFlags<types::ArenaFlags>,
    pub expansion: bool,
    pub ladder: bool,
}

/// 0x19
/// Response from C->S GoldToInv packet.
#[repr(C, packed(1))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoldToInvPacketBody {
    pub packet_id: u8,
    pub amount: u8,
}


/// 0x2c
/// Response from C->S PlaySound packet.
#[repr(C, packed(1))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaySoundPacketBody {
    pub packet_id: u8,
    pub unit_type: u8,
    pub unit_id: u32,
    pub sound: u16,
}

/// 0x8f
/// Response from C->S Ping packet.
#[repr(C, packed(1))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PongPacketBody {
    pub packet_id: u8,
    /// Always 0x00 * 32
    pub bytes: [u8; 32],
}

/// 0xaf
#[repr(C, packed(1))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConnectionInfoPacketBody {
    pub packet_id: u8,
    /// Whether compression is enabled or not.
    pub compression: bool,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
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
    /// 0x09
    AssignLevelWarp,
    /// 0x0a
    RemoveObject,
    /// 0x0b
    GameHandshake,
    /// 0x0c
    NpcHit,
    /// 0x0d
    PlayerStop,
    /// 0x0e
    ObjectState,
    /// 0x0f
    PlayerMove,
    /// 0x15
    ReassignPlayer,
    /// 0x19
    GoldToInv(GoldToInvPacketBody),
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
    /// 0x21
    UpdateItemOSkill,
    /// 0x22
    UpdateItemSkill,
    /// 0x23
    SetSkill,
    /// 0x26
    GameChat,
    /// 0x27
    NpcInfo,
    /// 0x28
    QuestInfo,
    /// 0x29
    GameQuestInfo,
    /// 0x2a
    NpcTransaction,
    /// 0x2c
    PlaySound(PlaySoundPacketBody),
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
    /// 0x4d
    UnitCastSkill,
    /// 0x4e
    MercForHire,
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
    /// 0x75
    PlayerPartyInfo,
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
    pub fn convert(packet: *const u8, size: i32) -> Result<Self, ()> {
        let packet_id = unsafe { *packet.offset(0) };

        match packet_id {
            0x00 => Ok(PacketFromServer::GameLoading),
            0x01 => Ok(PacketFromServer::GameFlags(
                unsafe {
                    debug_assert_eq!(std::mem::size_of::<GameFlagsPacketBody>(), size as usize);
                    (packet as *const GameFlagsPacketBody).read()
                }
            )),
            0x02 => Ok(PacketFromServer::LoadSuccessful),
            0x03 => Ok(PacketFromServer::LoadAct),
            0x04 => Ok(PacketFromServer::LoadComplete),
            0x05 => Ok(PacketFromServer::UnloadComplete),
            0x06 => Ok(PacketFromServer::GameExitSuccessful),
            0x07 => Ok(PacketFromServer::MapReveal),
            0x08 => Ok(PacketFromServer::MapHide),
            0x09 => Ok(PacketFromServer::AssignLevelWarp),
            0x0a => Ok(PacketFromServer::RemoveObject),
            0x0b => Ok(PacketFromServer::GameHandshake),
            0x0c => Ok(PacketFromServer::NpcHit),
            0x0d => Ok(PacketFromServer::PlayerStop),
            0x0e => Ok(PacketFromServer::ObjectState),
            0x0f => Ok(PacketFromServer::PlayerMove),
            0x15 => Ok(PacketFromServer::ReassignPlayer),
            0x19 => Ok(PacketFromServer::GoldToInv(
                unsafe {
                    debug_assert_eq!(std::mem::size_of::<GoldToInvPacketBody>(), size as usize);
                    (packet as *const GoldToInvPacketBody).read()
                }
            )),
            0x1a => Ok(PacketFromServer::AddExpByte),
            0x1b => Ok(PacketFromServer::AddExpWord),
            0x1c => Ok(PacketFromServer::AddExpDword),
            0x1d => Ok(PacketFromServer::BaseAttributeByte),
            0x1e => Ok(PacketFromServer::BaseAttributeWord),
            0x1f => Ok(PacketFromServer::BaseAttributeDword),
            0x21 => Ok(PacketFromServer::UpdateItemOSkill),
            0x22 => Ok(PacketFromServer::UpdateItemSkill),
            0x23 => Ok(PacketFromServer::SetSkill),
            0x26 => Ok(PacketFromServer::GameChat),
            0x27 => Ok(PacketFromServer::NpcInfo),
            0x28 => Ok(PacketFromServer::QuestInfo),
            0x29 => Ok(PacketFromServer::GameQuestInfo),
            0x2a => Ok(PacketFromServer::NpcTransaction),
            0x2c => Ok(PacketFromServer::PlaySound(
                unsafe {
                    debug_assert_eq!(std::mem::size_of::<PlaySoundPacketBody>(), size as usize);
                    (packet as *const PlaySoundPacketBody).read()
                }
            )),
            0x37 => Ok(PacketFromServer::Unknown0x37),
            0x3e => Ok(PacketFromServer::UpdateItemStats),
            0x3f => Ok(PacketFromServer::UseStackableItem),
            0x42 => Ok(PacketFromServer::ClearCursor),
            0x47 => Ok(PacketFromServer::Relator1),
            0x48 => Ok(PacketFromServer::Relator2),
            0x4c => Ok(PacketFromServer::UnitSkillOnTarget),
            0x4d => Ok(PacketFromServer::UnitCastSkill),
            0x4e => Ok(PacketFromServer::MercForHire),
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
            0x75 => Ok(PacketFromServer::PlayerPartyInfo),
            0x76 => Ok(PacketFromServer::PlayerInProximity),
            0x77 => Ok(PacketFromServer::ButtonActions),
            0x7e => Ok(PacketFromServer::ReloadCMNCOF),
            0x8a => Ok(PacketFromServer::NpcWantsToInteract),
            0x8d => Ok(PacketFromServer::AssignPlayerToParty),
            0x8f => Ok(PacketFromServer::Pong(
                unsafe {
                    debug_assert_eq!(std::mem::size_of::<PongPacketBody>(), size as usize);
                    (packet as *const PongPacketBody).read()
                }
            )),
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
            0xaf => Ok(PacketFromServer::ConnectionInfo(
                unsafe {
                    debug_assert_eq!(std::mem::size_of::<ConnectionInfoPacketBody>(), size as usize);
                    (packet as *const ConnectionInfoPacketBody).read()
                }
            )),
            0xb0 => Ok(PacketFromServer::GameConnectionTerminated),
            _ => Err(()),
        }
    }
}
