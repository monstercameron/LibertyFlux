//! Script threads.
//!
//! Holds 3 draft layouts: `rage::scrThread`, the game's `GtaThread`, and the `script_thread` record
//! seen by the native-handler lanes. Every layout is Inferred; size confidence (the analysis lanes'
//! own rating) is high for 0, medium for 0 and low for 3. The conventions are those of the crate
//! root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `GtaThread`.
///
/// Size: 0xb0 (low). Bases: rage::scrThread@0x0.
/// Lanes: c-core, n-04, n-11, n-13, n-22.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GtaThread {
    /// Unknown bytes (0x0..0x28).
    pub _pad_0000: [u8; 0x28],
    /// wait_time_accumulator_ms (confidence: high, kind: float, lanes: n-22).
    pub wait_time_accumulator_ms: f32,
    /// Unknown bytes (0x2c..0x70).
    pub _pad_002c: [u8; 0x44],
    /// field_+0x70 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x70: u32,
    /// Unknown bytes (0x74..0x87).
    pub _pad_0074: [u8; 0x13],
    /// flag_+0x87 (confidence: low, kind: u8/bool?, lanes: c-core).
    pub flag_0x87: u8,
    /// Unknown bytes (0x88..0x94).
    pub _pad_0088: [u8; 0xc],
    /// script_thread_id_allocate (confidence: low, kind: u8, lanes: n-11,n-13).
    pub script_thread_id_allocate: u8,
    /// safe_network_game (confidence: high, kind: flags, lanes: n-22).
    pub safe_network_game: u8,
    /// script_should_saved (confidence: high, kind: flags, lanes: n-22).
    pub script_should_saved: u8,
    /// Unknown bytes (0x97..0x99).
    pub _pad_0097: [u8; 0x2],
    /// minigame_script (confidence: high, kind: flags, lanes: n-04,n-11).
    pub minigame_script: u8,
    /// current_script_non_minigame (confidence: medium, kind: flags, lanes: n-04).
    pub current_script_non_minigame: u8,
    /// Unknown bytes (0x9b..0x9c).
    pub _pad_009b: [u8; 0x1],
    /// paused_pause_game (confidence: high, kind: flags, lanes: c-core,n-13).
    pub paused_pause_game: u8,
    /// pausable (confidence: medium, kind: flags, lanes: n-13).
    pub pausable: u8,
    /// flag_+0x9e (confidence: low, kind: u8/bool?, lanes: c-core).
    pub flag_0x9e: u8,
    /// Unknown bytes (0x9f..0xa0).
    pub _pad_009f: [u8; 0x1],
    /// brain_slot_index_activation (confidence: medium, kind: i32, lanes: c-core,n-11).
    pub brain_slot_index_activation: u32,
    /// Unknown bytes (0xa4..0xa8).
    pub _pad_00a4: [u8; 0x4],
    /// field_+0xa8 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0xa8: u32,
    /// Unknown trailing bytes (0xac..0xb0).
    pub _pad_end: [u8; 0x4],
}
assert_size!(GtaThread, 0xb0); // merged size 0xb0 rounded to 4
assert_offset!(GtaThread, wait_time_accumulator_ms, 0x28);
assert_offset!(GtaThread, field_0x70, 0x70);
assert_offset!(GtaThread, flag_0x87, 0x87);
assert_offset!(GtaThread, script_thread_id_allocate, 0x94);
assert_offset!(GtaThread, safe_network_game, 0x95);
assert_offset!(GtaThread, script_should_saved, 0x96);
assert_offset!(GtaThread, minigame_script, 0x99);
assert_offset!(GtaThread, current_script_non_minigame, 0x9a);
assert_offset!(GtaThread, paused_pause_game, 0x9c);
assert_offset!(GtaThread, pausable, 0x9d);
assert_offset!(GtaThread, flag_0x9e, 0x9e);
assert_offset!(GtaThread, brain_slot_index_activation, 0xa0);
assert_offset!(GtaThread, field_0xa8, 0xa8);

/// Merged layout for `rage::scrThread`.
///
/// Size: 0x70 (low). Bases: none.
/// Lanes: c-core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageScrThread {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr: Ptr32<()>,
    /// field_+0x4 (confidence: medium, kind: u32?, lanes: c-core,n-11,n-13).
    pub field_0x4: u32,
    /// field_+0x8 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x8: u32,
    /// field_+0xc (confidence: high, kind: u32?, lanes: c-core,n-11,n-22).
    pub field_0xc: u32,
    /// field_+0x10 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x10: u32,
    /// Unknown bytes (0x14..0x18).
    pub _pad_0014: [u8; 0x4],
    /// ptr_+0x18 (confidence: low, kind: pointer, lanes: c-core).
    pub ptr_0x18: Ptr32<u8>,
    /// field_+0x1c (confidence: medium, kind: u32?, lanes: c-core,n-22).
    pub field_0x1c: u32,
    /// field_+0x20 (confidence: medium, kind: u32?, lanes: c-core,n-22).
    pub field_0x20: u32,
    /// field_+0x24 (confidence: medium, kind: u32?, lanes: c-core,n-22).
    pub field_0x24: u32,
    /// Unknown bytes (0x28..0x2c).
    pub _pad_0028: [u8; 0x4],
    /// field_+0x2c (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x2c: u32,
    /// field_+0x30 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x30: u32,
    /// Unknown bytes (0x34..0x44).
    pub _pad_0034: [u8; 0x10],
    /// field_+0x44 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x44: u32,
    /// Unknown bytes (0x48..0x54).
    pub _pad_0048: [u8; 0xc],
    /// field_+0x54 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x54: u32,
    /// field_+0x58 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x58: u32,
    /// field_+0x5c (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x5c: u32,
    /// field_+0x60 (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x60: u32,
    /// field_+0x64 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x64: u32,
    /// field_+0x68 (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x68: u32,
    /// field_+0x6c (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x6c: u32,
}
assert_size!(RageScrThread, 0x70); // merged size 0x70 rounded to 4
assert_offset!(RageScrThread, vfptr, 0x0);
assert_offset!(RageScrThread, field_0x4, 0x4);
assert_offset!(RageScrThread, field_0x8, 0x8);
assert_offset!(RageScrThread, field_0xc, 0xc);
assert_offset!(RageScrThread, field_0x10, 0x10);
assert_offset!(RageScrThread, ptr_0x18, 0x18);
assert_offset!(RageScrThread, field_0x1c, 0x1c);
assert_offset!(RageScrThread, field_0x20, 0x20);
assert_offset!(RageScrThread, field_0x24, 0x24);
assert_offset!(RageScrThread, field_0x2c, 0x2c);
assert_offset!(RageScrThread, field_0x30, 0x30);
assert_offset!(RageScrThread, field_0x44, 0x44);
assert_offset!(RageScrThread, field_0x54, 0x54);
assert_offset!(RageScrThread, field_0x58, 0x58);
assert_offset!(RageScrThread, field_0x5c, 0x5c);
assert_offset!(RageScrThread, field_0x60, 0x60);
assert_offset!(RageScrThread, field_0x64, 0x64);
assert_offset!(RageScrThread, field_0x68, 0x68);
assert_offset!(RageScrThread, field_0x6c, 0x6c);

/// Merged layout for `script_thread`.
///
/// Size: 0x28 (low). Bases: none.
/// Lanes: n-15.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScriptThread {
    /// Unknown bytes (0x0..0x1c).
    pub _pad_0000: [u8; 0x1c],
    /// timer (confidence: high, kind: u32, lanes: n-15).
    pub timer: u32,
    /// timer (confidence: high, kind: u32, lanes: n-15).
    pub timer_2: u32,
    /// timer (confidence: high, kind: u32, lanes: n-15).
    pub timer_3: u32,
}
assert_size!(ScriptThread, 0x28); // merged size 0x28 rounded to 4
assert_offset!(ScriptThread, timer, 0x1c);
assert_offset!(ScriptThread, timer_2, 0x20);
assert_offset!(ScriptThread, timer_3, 0x24);
