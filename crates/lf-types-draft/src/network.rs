//! Network session tasks, leaderboards and network blenders.
//!
//! Holds 11 draft layouts: RAGE's `rage::sn*` session tasks, `rage::rl*` leaderboard and task
//! templates, and the game's `CNetBlenderDummyPed`. Every layout is Inferred; size confidence (the
//! analysis lanes' own rating) is high for 0, medium for 0 and low for 11. The conventions are
//! those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CNetBlenderDummyPed`.
///
/// Size: 0xdd (low). Bases: CNetBlenderLinInterp@0x0.
/// Lanes: c-network.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CNetBlenderDummyPed {
    /// vfptr (confidence: high, kind: pointer, lanes: c-network).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x20).
    pub _pad_0004: [u8; 0x1c],
    /// field_20 (confidence: low, kind: embedded, lanes: c-network).
    pub field_20: u32,
    /// Unknown bytes (0x24..0x50).
    pub _pad_0024: [u8; 0x2c],
    /// field_50 (confidence: low, kind: embedded, lanes: c-network).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x60).
    pub _pad_0054: [u8; 0xc],
    /// field_60 (confidence: low, kind: embedded, lanes: c-network).
    pub field_60: u32,
    /// Unknown bytes (0x64..0xb0).
    pub _pad_0064: [u8; 0x4c],
    /// field_b0 (confidence: low, kind: integer, lanes: c-network).
    pub field_b0: u32,
    /// m_bDirty? (confidence: medium, kind: boolean, lanes: c-network).
    pub dirty: u8,
    /// Unknown bytes (0xb5..0xc0).
    pub _pad_00b5: [u8; 0xb],
    /// m_pEntity (confidence: high, kind: pointer, lanes: c-network).
    pub entity: Ptr32<u8>,
    /// m_fHeading[0] (confidence: high, kind: float, lanes: c-network).
    pub heading_0: f32,
    /// m_hBlend[0] (confidence: medium, kind: pointer, lanes: c-network).
    pub blend_0: Ptr32<u8>,
    /// m_fHeading[1] (confidence: high, kind: float, lanes: c-network).
    pub heading_1: f32,
    /// m_hBlend[1] (confidence: medium, kind: pointer, lanes: c-network).
    pub blend_1: Ptr32<u8>,
    /// m_fHeading[2] (confidence: high, kind: float, lanes: c-network).
    pub heading_2: f32,
    /// m_hBlend[2] (confidence: medium, kind: pointer, lanes: c-network).
    pub blend_2: Ptr32<u8>,
    /// m_bInitialised (confidence: medium, kind: boolean, lanes: c-network).
    pub initialised: u8,
    /// Unknown trailing bytes (0xdd..0xe0).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CNetBlenderDummyPed, 0xe0); // merged size 0xdd rounded to 4
assert_offset!(CNetBlenderDummyPed, vfptr, 0x0);
assert_offset!(CNetBlenderDummyPed, field_20, 0x20);
assert_offset!(CNetBlenderDummyPed, field_50, 0x50);
assert_offset!(CNetBlenderDummyPed, field_60, 0x60);
assert_offset!(CNetBlenderDummyPed, field_b0, 0xb0);
assert_offset!(CNetBlenderDummyPed, dirty, 0xb4);
assert_offset!(CNetBlenderDummyPed, entity, 0xc0);
assert_offset!(CNetBlenderDummyPed, heading_0, 0xc4);
assert_offset!(CNetBlenderDummyPed, blend_0, 0xc8);
assert_offset!(CNetBlenderDummyPed, heading_1, 0xcc);
assert_offset!(CNetBlenderDummyPed, blend_1, 0xd0);
assert_offset!(CNetBlenderDummyPed, heading_2, 0xd4);
assert_offset!(CNetBlenderDummyPed, blend_2, 0xd8);
assert_offset!(CNetBlenderDummyPed, initialised, 0xdc);

/// Merged layout for `rage::rlConcreteLeaderboardUpdate<player_schema::Leaderboard_Standard_Episodic_8, player_schema::LeaderboardInfo>`.
///
/// Size: 0x5a5 (low). Bases: rage::rlLeaderboardUpdate@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo
{
    /// __vtbl (confidence: high, kind: vtable_ptr, lanes: c-online).
    pub vtbl: Ptr32<()>,
    /// Unknown bytes (0x4..0x4a0).
    pub _pad_0004: [u8; 0x49c],
    /// field_4a0 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_4a0: u32,
    /// field_4a4 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_4a4: u32,
    /// field_4a8 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_4a8: u32,
    /// field_4ac (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_4ac: u32,
    /// field_4b0 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_4b0: u32,
    /// field_4b4 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_4b4: u32,
    /// field_4b8 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_4b8: u32,
    /// Unknown bytes (0x4bc..0x5a4).
    pub _pad_04bc: [u8; 0xe8],
    /// field_5a4 (confidence: low, kind: u8/bool?, lanes: c-online).
    pub field_5a4: u8,
    /// Unknown trailing bytes (0x5a5..0x5a8).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, 0x5a8); // merged size 0x5a5 rounded to 4
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, vtbl, 0x0);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_4a0, 0x4a0);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_4a4, 0x4a4);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_4a8, 0x4a8);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_4ac, 0x4ac);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_4b0, 0x4b0);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_4b4, 0x4b4);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_4b8, 0x4b8);
assert_offset!(RageRlConcreteLeaderboardUpdatePlayerSchemaLeaderboardStandardEpisodic8PlayerSchemaLeaderboardInfo, field_5a4, 0x5a4);

/// Merged layout for `rage::rlFireAndForgetTask<rage::snJoinCompleteTask>`.
///
/// Size: 0x2ec (low). Bases: rage::snJoinCompleteTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageRlFireAndForgetTaskRageSnJoinCompleteTask {
    /// __vtbl (confidence: high, kind: vtable_ptr, lanes: c-online).
    pub vtbl: Ptr32<()>,
    /// Unknown bytes (0x4..0xc).
    pub _pad_0004: [u8; 0x8],
    /// field_c (confidence: low, kind: pointer, lanes: c-online).
    pub field_c: Ptr32<u8>,
    /// Unknown bytes (0x10..0x14).
    pub _pad_0010: [u8; 0x4],
    /// field_14 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-online).
    pub field_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x60).
    pub _pad_001c: [u8; 0x44],
    /// field_60 (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_60: u32,
    /// field_64 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_64: u32,
    /// field_68 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_68: u32,
    /// field_6c (confidence: low, kind: u8/bool?, lanes: c-online).
    pub field_6c: u8,
    /// field_6d (confidence: low, kind: u8/bool?, lanes: c-online).
    pub field_6d: u8,
    /// Unknown bytes (0x6e..0x90).
    pub _pad_006e: [u8; 0x22],
    /// field_90 (confidence: low, kind: pointer, lanes: c-online).
    pub field_90: Ptr32<u8>,
    /// Unknown bytes (0x94..0x2c0).
    pub _pad_0094: [u8; 0x22c],
    /// field_2c0 (confidence: low, kind: pointer, lanes: c-online).
    pub field_2c0: Ptr32<u8>,
    /// field_2c4 (confidence: low, kind: u16?, lanes: c-online).
    pub field_2c4: u16,
    /// Unknown bytes (0x2c6..0x2c8).
    pub _pad_02c6: [u8; 0x2],
    /// field_2c8 (confidence: low, kind: pointer, lanes: c-online).
    pub field_2c8: Ptr32<u8>,
    /// field_2cc (confidence: low, kind: u16?, lanes: c-online).
    pub field_2cc: u16,
    /// Unknown bytes (0x2ce..0x2d0).
    pub _pad_02ce: [u8; 0x2],
    /// field_2d0 (confidence: low, kind: pointer, lanes: c-online).
    pub field_2d0: Ptr32<u8>,
    /// field_2d4 (confidence: low, kind: u16?, lanes: c-online).
    pub field_2d4: u16,
    /// Unknown bytes (0x2d6..0x2d8).
    pub _pad_02d6: [u8; 0x2],
    /// field_2d8 (confidence: low, kind: pointer, lanes: c-online).
    pub field_2d8: Ptr32<u8>,
    /// field_2dc (confidence: low, kind: u16?, lanes: c-online).
    pub field_2dc: u16,
    /// Unknown bytes (0x2de..0x2e0).
    pub _pad_02de: [u8; 0x2],
    /// field_2e0 (confidence: low, kind: pointer, lanes: c-online).
    pub field_2e0: Ptr32<u8>,
    /// field_2e4 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_2e4: u32,
    /// field_2e8 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_2e8: u32,
}
assert_size!(RageRlFireAndForgetTaskRageSnJoinCompleteTask, 0x2ec); // merged size 0x2ec rounded to 4
assert_offset!(RageRlFireAndForgetTaskRageSnJoinCompleteTask, vtbl, 0x0);
assert_offset!(RageRlFireAndForgetTaskRageSnJoinCompleteTask, field_c, 0xc);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_14,
    0x14
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_18,
    0x18
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_60,
    0x60
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_64,
    0x64
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_68,
    0x68
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_6c,
    0x6c
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_6d,
    0x6d
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_90,
    0x90
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2c0,
    0x2c0
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2c4,
    0x2c4
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2c8,
    0x2c8
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2cc,
    0x2cc
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2d0,
    0x2d0
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2d4,
    0x2d4
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2d8,
    0x2d8
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2dc,
    0x2dc
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2e0,
    0x2e0
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2e4,
    0x2e4
);
assert_offset!(
    RageRlFireAndForgetTaskRageSnJoinCompleteTask,
    field_2e8,
    0x2e8
);

/// Merged layout for `rage::snAddRemoteGamerTask`.
///
/// Size: 0x81c (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnAddRemoteGamerTask {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// field_90 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_94: Ptr32<u8>,
    /// Unknown bytes (0x98..0xd8).
    pub _pad_0098: [u8; 0x40],
    /// field_d8 (confidence: low, kind: pointer, lanes: c-online).
    pub field_d8: Ptr32<u8>,
    /// field_dc (confidence: low, kind: pointer, lanes: c-online).
    pub field_dc: Ptr32<u8>,
    /// field_e0 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_e0: u32,
    /// field_e4 (confidence: low, kind: pointer, lanes: c-online).
    pub field_e4: Ptr32<u8>,
    /// Unknown bytes (0xe8..0x544).
    pub _pad_00e8: [u8; 0x45c],
    /// field_544 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_544: Ptr32<u8>,
    /// Unknown bytes (0x548..0x818).
    pub _pad_0548: [u8; 0x2d0],
    /// field_818 (confidence: low, kind: pointer, lanes: c-online).
    pub field_818: Ptr32<u8>,
}
assert_size!(RageSnAddRemoteGamerTask, 0x81c); // merged size 0x81c rounded to 4
assert_offset!(RageSnAddRemoteGamerTask, field_90, 0x90);
assert_offset!(RageSnAddRemoteGamerTask, field_94, 0x94);
assert_offset!(RageSnAddRemoteGamerTask, field_d8, 0xd8);
assert_offset!(RageSnAddRemoteGamerTask, field_dc, 0xdc);
assert_offset!(RageSnAddRemoteGamerTask, field_e0, 0xe0);
assert_offset!(RageSnAddRemoteGamerTask, field_e4, 0xe4);
assert_offset!(RageSnAddRemoteGamerTask, field_544, 0x544);
assert_offset!(RageSnAddRemoteGamerTask, field_818, 0x818);

/// Merged layout for `rage::snConnectToPeerTask`.
///
/// Size: 0x5a5 (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnConnectToPeerTask {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// field_90 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-online).
    pub field_94: Ptr32<u8>,
    /// Unknown bytes (0x98..0x108).
    pub _pad_0098: [u8; 0x70],
    /// field_108 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_108: Ptr32<u8>,
    /// field_10c (confidence: medium, kind: u16?, lanes: c-online).
    pub field_10c: u16,
    /// Unknown bytes (0x10e..0x110).
    pub _pad_010e: [u8; 0x2],
    /// field_110 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_110: Ptr32<u8>,
    /// field_114 (confidence: medium, kind: u16?, lanes: c-online).
    pub field_114: u16,
    /// Unknown bytes (0x116..0x2c0).
    pub _pad_0116: [u8; 0x1aa],
    /// field_2c0 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_2c0: u32,
    /// Unknown bytes (0x2c4..0x2d0).
    pub _pad_02c4: [u8; 0xc],
    /// field_2d0 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_2d0: u32,
    /// Unknown trailing bytes (0x2d4..0x5a8).
    pub _pad_end: [u8; 0x2d4],
}
assert_size!(RageSnConnectToPeerTask, 0x5a8); // merged size 0x5a5 rounded to 4
assert_offset!(RageSnConnectToPeerTask, field_90, 0x90);
assert_offset!(RageSnConnectToPeerTask, field_94, 0x94);
assert_offset!(RageSnConnectToPeerTask, field_108, 0x108);
assert_offset!(RageSnConnectToPeerTask, field_10c, 0x10c);
assert_offset!(RageSnConnectToPeerTask, field_110, 0x110);
assert_offset!(RageSnConnectToPeerTask, field_114, 0x114);
assert_offset!(RageSnConnectToPeerTask, field_2c0, 0x2c0);
assert_offset!(RageSnConnectToPeerTask, field_2d0, 0x2d0);

/// Merged layout for `rage::snHandleJoinRequestTask`.
///
/// Size: 0x529 (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnHandleJoinRequestTask {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_8: u32,
    /// Unknown bytes (0xc..0x90).
    pub _pad_000c: [u8; 0x84],
    /// field_90 (confidence: low, kind: pointer, lanes: c-online).
    pub field_90: Ptr32<u8>,
    /// Unknown bytes (0x94..0xd0).
    pub _pad_0094: [u8; 0x3c],
    /// field_d0 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_d0: u32,
    /// field_d4 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_d4: Ptr32<u8>,
    /// Unknown bytes (0xd8..0x2c0).
    pub _pad_00d8: [u8; 0x1e8],
    /// field_2c0 (confidence: low, kind: pointer, lanes: c-online).
    pub field_2c0: Ptr32<u8>,
    /// Unknown bytes (0x2c4..0x2d0).
    pub _pad_02c4: [u8; 0xc],
    /// field_2d0 (confidence: low, kind: pointer, lanes: c-online).
    pub field_2d0: Ptr32<u8>,
    /// Unknown bytes (0x2d4..0x320).
    pub _pad_02d4: [u8; 0x4c],
    /// field_320 (confidence: low, kind: pointer, lanes: c-online).
    pub field_320: Ptr32<u8>,
    /// Unknown bytes (0x324..0x528).
    pub _pad_0324: [u8; 0x204],
    /// field_528 (confidence: medium, kind: u8/bool?, lanes: c-online).
    pub field_528: u8,
    /// Unknown trailing bytes (0x529..0x52c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RageSnHandleJoinRequestTask, 0x52c); // merged size 0x529 rounded to 4
assert_offset!(RageSnHandleJoinRequestTask, field_8, 0x8);
assert_offset!(RageSnHandleJoinRequestTask, field_90, 0x90);
assert_offset!(RageSnHandleJoinRequestTask, field_d0, 0xd0);
assert_offset!(RageSnHandleJoinRequestTask, field_d4, 0xd4);
assert_offset!(RageSnHandleJoinRequestTask, field_2c0, 0x2c0);
assert_offset!(RageSnHandleJoinRequestTask, field_2d0, 0x2d0);
assert_offset!(RageSnHandleJoinRequestTask, field_320, 0x320);
assert_offset!(RageSnHandleJoinRequestTask, field_528, 0x528);

/// Merged layout for `rage::snHostSessionTask`.
///
/// Size: 0x4c4 (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnHostSessionTask {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// field_90 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_94: u32,
    /// Unknown bytes (0x98..0x9c).
    pub _pad_0098: [u8; 0x4],
    /// field_9c (confidence: medium, kind: pointer, lanes: c-online).
    pub field_9c: Ptr32<u8>,
    /// field_a0 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_a0: u32,
    /// field_a4 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_a4: Ptr32<u8>,
    /// Unknown bytes (0xa8..0x4bc).
    pub _pad_00a8: [u8; 0x414],
    /// field_4bc (confidence: low, kind: pointer, lanes: c-online).
    pub field_4bc: Ptr32<u8>,
    /// field_4c0 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_4c0: Ptr32<u8>,
}
assert_size!(RageSnHostSessionTask, 0x4c4); // merged size 0x4c4 rounded to 4
assert_offset!(RageSnHostSessionTask, field_90, 0x90);
assert_offset!(RageSnHostSessionTask, field_94, 0x94);
assert_offset!(RageSnHostSessionTask, field_9c, 0x9c);
assert_offset!(RageSnHostSessionTask, field_a0, 0xa0);
assert_offset!(RageSnHostSessionTask, field_a4, 0xa4);
assert_offset!(RageSnHostSessionTask, field_4bc, 0x4bc);
assert_offset!(RageSnHostSessionTask, field_4c0, 0x4c0);

/// Merged layout for `rage::snJoinSessionTask`.
///
/// Size: 0x8c0 (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnJoinSessionTask {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// field_90 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-online).
    pub field_94: Ptr32<u8>,
    /// Unknown bytes (0x98..0x108).
    pub _pad_0098: [u8; 0x70],
    /// field_108 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_108: u32,
    /// field_10c (confidence: low, kind: pointer, lanes: c-online).
    pub field_10c: Ptr32<u8>,
    /// field_110 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_110: u32,
    /// field_114 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_114: u32,
    /// Unknown bytes (0x118..0x150).
    pub _pad_0118: [u8; 0x38],
    /// field_150 (confidence: low, kind: pointer, lanes: c-online).
    pub field_150: Ptr32<u8>,
    /// field_154 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_154: u32,
    /// field_158 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_158: Ptr32<u8>,
    /// Unknown bytes (0x15c..0x35c).
    pub _pad_015c: [u8; 0x200],
    /// field_35c (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_35c: u32,
    /// field_360 (confidence: low, kind: pointer, lanes: c-online).
    pub field_360: Ptr32<u8>,
    /// field_364 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_364: Ptr32<u8>,
    /// Unknown bytes (0x368..0x568).
    pub _pad_0368: [u8; 0x200],
    /// field_568 (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_568: u32,
    /// field_56c (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_56c: u32,
    /// field_570 (confidence: low, kind: pointer, lanes: c-online).
    pub field_570: Ptr32<u8>,
    /// field_574 (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_574: u32,
    /// field_578 (confidence: low, kind: pointer, lanes: c-online).
    pub field_578: Ptr32<u8>,
    /// field_57c (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_57c: u32,
    /// field_580 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_580: u32,
    /// field_584 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_584: u32,
    /// Unknown bytes (0x588..0x788).
    pub _pad_0588: [u8; 0x200],
    /// field_788 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_788: u32,
    /// field_78c (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_78c: u32,
    /// field_790 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_790: u32,
    /// Unknown bytes (0x794..0x798).
    pub _pad_0794: [u8; 0x4],
    /// field_798 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_798: u32,
    /// Unknown bytes (0x79c..0x7a4).
    pub _pad_079c: [u8; 0x8],
    /// field_7a4 (confidence: low, kind: pointer, lanes: c-online).
    pub field_7a4: Ptr32<u8>,
    /// Unknown bytes (0x7a8..0x8b8).
    pub _pad_07a8: [u8; 0x110],
    /// field_8b8 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_8b8: Ptr32<u8>,
    /// field_8bc (confidence: medium, kind: pointer, lanes: c-online).
    pub field_8bc: Ptr32<u8>,
}
assert_size!(RageSnJoinSessionTask, 0x8c0); // merged size 0x8c0 rounded to 4
assert_offset!(RageSnJoinSessionTask, field_90, 0x90);
assert_offset!(RageSnJoinSessionTask, field_94, 0x94);
assert_offset!(RageSnJoinSessionTask, field_108, 0x108);
assert_offset!(RageSnJoinSessionTask, field_10c, 0x10c);
assert_offset!(RageSnJoinSessionTask, field_110, 0x110);
assert_offset!(RageSnJoinSessionTask, field_114, 0x114);
assert_offset!(RageSnJoinSessionTask, field_150, 0x150);
assert_offset!(RageSnJoinSessionTask, field_154, 0x154);
assert_offset!(RageSnJoinSessionTask, field_158, 0x158);
assert_offset!(RageSnJoinSessionTask, field_35c, 0x35c);
assert_offset!(RageSnJoinSessionTask, field_360, 0x360);
assert_offset!(RageSnJoinSessionTask, field_364, 0x364);
assert_offset!(RageSnJoinSessionTask, field_568, 0x568);
assert_offset!(RageSnJoinSessionTask, field_56c, 0x56c);
assert_offset!(RageSnJoinSessionTask, field_570, 0x570);
assert_offset!(RageSnJoinSessionTask, field_574, 0x574);
assert_offset!(RageSnJoinSessionTask, field_578, 0x578);
assert_offset!(RageSnJoinSessionTask, field_57c, 0x57c);
assert_offset!(RageSnJoinSessionTask, field_580, 0x580);
assert_offset!(RageSnJoinSessionTask, field_584, 0x584);
assert_offset!(RageSnJoinSessionTask, field_788, 0x788);
assert_offset!(RageSnJoinSessionTask, field_78c, 0x78c);
assert_offset!(RageSnJoinSessionTask, field_790, 0x790);
assert_offset!(RageSnJoinSessionTask, field_798, 0x798);
assert_offset!(RageSnJoinSessionTask, field_7a4, 0x7a4);
assert_offset!(RageSnJoinSessionTask, field_8b8, 0x8b8);
assert_offset!(RageSnJoinSessionTask, field_8bc, 0x8bc);

/// Merged layout for `rage::snMigrateSessionTask`.
///
/// Size: 0x1698 (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnMigrateSessionTask {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// field_90 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_94: u32,
    /// Unknown bytes (0x98..0x904).
    pub _pad_0098: [u8; 0x86c],
    /// field_904 (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_904: u32,
    /// field_908 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_908: Ptr32<u8>,
    /// Unknown bytes (0x90c..0x918).
    pub _pad_090c: [u8; 0xc],
    /// field_918 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_918: u32,
    /// field_91c (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_91c: u32,
    /// Unknown bytes (0x920..0x1658).
    pub _pad_0920: [u8; 0xd38],
    /// field_1658 (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_1658: u32,
    /// field_165c (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_165c: u32,
    /// field_1660 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_1660: u32,
    /// field_1664 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_1664: u32,
    /// field_1668 (confidence: low, kind: pointer, lanes: c-online).
    pub field_1668: Ptr32<u8>,
    /// field_166c (confidence: low, kind: pointer, lanes: c-online).
    pub field_166c: Ptr32<u8>,
    /// Unknown bytes (0x1670..0x1694).
    pub _pad_1670: [u8; 0x24],
    /// field_1694 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_1694: u32,
}
assert_size!(RageSnMigrateSessionTask, 0x1698); // merged size 0x1698 rounded to 4
assert_offset!(RageSnMigrateSessionTask, field_90, 0x90);
assert_offset!(RageSnMigrateSessionTask, field_94, 0x94);
assert_offset!(RageSnMigrateSessionTask, field_904, 0x904);
assert_offset!(RageSnMigrateSessionTask, field_908, 0x908);
assert_offset!(RageSnMigrateSessionTask, field_918, 0x918);
assert_offset!(RageSnMigrateSessionTask, field_91c, 0x91c);
assert_offset!(RageSnMigrateSessionTask, field_1658, 0x1658);
assert_offset!(RageSnMigrateSessionTask, field_165c, 0x165c);
assert_offset!(RageSnMigrateSessionTask, field_1660, 0x1660);
assert_offset!(RageSnMigrateSessionTask, field_1664, 0x1664);
assert_offset!(RageSnMigrateSessionTask, field_1668, 0x1668);
assert_offset!(RageSnMigrateSessionTask, field_166c, 0x166c);
assert_offset!(RageSnMigrateSessionTask, field_1694, 0x1694);

/// Merged layout for `rage::snModifyPresenceFlagsTask`.
///
/// Size: 0x768 (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnModifyPresenceFlagsTask {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// field_90 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_90: u32,
    /// field_94 (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_94: u32,
    /// Unknown bytes (0x98..0x9c).
    pub _pad_0098: [u8; 0x4],
    /// field_9c (confidence: medium, kind: pointer, lanes: c-online).
    pub field_9c: Ptr32<u8>,
    /// Unknown trailing bytes (0xa0..0x768).
    pub _pad_end: [u8; 0x6c8],
}
assert_size!(RageSnModifyPresenceFlagsTask, 0x768); // merged size 0x768 rounded to 4
assert_offset!(RageSnModifyPresenceFlagsTask, field_90, 0x90);
assert_offset!(RageSnModifyPresenceFlagsTask, field_94, 0x94);
assert_offset!(RageSnModifyPresenceFlagsTask, field_9c, 0x9c);

/// Merged layout for `rage::snSendInvitesTask`.
///
/// Size: 0xa0 (low). Bases: rage::snTask@0x0.
/// Lanes: c-online.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSnSendInvitesTask {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: medium, kind: pointer, lanes: c-online).
    pub field_4: Ptr32<u8>,
    /// Unknown bytes (0x8..0x90).
    pub _pad_0008: [u8; 0x88],
    /// field_90 (confidence: medium, kind: embedded-object?, lanes: c-online).
    pub field_90: u32,
    /// field_94 (confidence: low, kind: zero-inited?, lanes: c-online).
    pub field_94: u32,
    /// Unknown bytes (0x98..0x9c).
    pub _pad_0098: [u8; 0x4],
    /// field_9c (confidence: medium, kind: zero-inited?, lanes: c-online).
    pub field_9c: u32,
}
assert_size!(RageSnSendInvitesTask, 0xa0); // merged size 0xa0 rounded to 4
assert_offset!(RageSnSendInvitesTask, field_4, 0x4);
assert_offset!(RageSnSendInvitesTask, field_90, 0x90);
assert_offset!(RageSnSendInvitesTask, field_94, 0x94);
assert_offset!(RageSnSendInvitesTask, field_9c, 0x9c);
