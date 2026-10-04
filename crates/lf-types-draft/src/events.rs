//! Events: the `CEvent` hierarchy.
//!
//! Holds 11 draft layouts: `CEvent`, its subclasses and `CEventHandler`, and `CIntermezzoEvent`.
//! Every layout is Inferred; size confidence (the analysis lanes' own rating) is high for 0, medium
//! for 0 and low for 11. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CEvent`.
///
/// Size: 0x4c (low). Bases: none.
/// Lanes: c-events, via:CEventBuildingCollision, via:CEventClimbNavMeshOnRoute, via:CEventCommunicateEvent, via:CEventDamage, via:CEventDeath, via:CEventEditableResponse, via:CEventGivePedTask, via:CEventGunShot, via:CEventImminentDanger, via:CEventNewTask, via:CEventObjectCollision, via:CEventRevived, via:CEventScriptCommand, via:CEventShocking, via:CEventSoundDynamic, via:CEventSwitch2NM, via:CEventVehicleCollision, via:CEventVehicleDamage.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEvent {
    /// __vtbl (confidence: high, kind: vtable_ptr, lanes: c-events).
    pub vtbl: Ptr32<()>,
    /// field_4 (confidence: low, kind: i32, lanes: c-events).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: bool, lanes: c-events).
    pub field_8: u8,
    /// Unknown bytes (0x9..0xc).
    pub _pad_0009: [u8; 0x3],
    /// field_c (confidence: high, kind: bool, lanes: c-events,via:CEventDeath,via:CEventEditableResponse,via:CEventNewTask,via:CEventShocking moved from siblings:CEventDeath,CEventEditableResponse,CEventNewTask,CEventShocking).
    pub field_c: u8,
    /// Unknown bytes (0xd..0xe).
    pub _pad_000d: [u8; 0x1],
    /// field_e (confidence: medium, kind: int16, lanes: c-events,via:CEventBuildingCollision,via:CEventVehicleCollision moved from siblings:CEventBuildingCollision,CEventVehicleCollision).
    pub field_e: u16,
    /// Unknown bytes (0x10..0x14).
    pub _pad_0010: [u8; 0x4],
    /// field_14 (confidence: high, kind: bool, lanes: c-events,via:CEventGivePedTask,via:CEventRevived,via:CEventScriptCommand,via:CEventSwitch2NM moved from siblings:CEventGivePedTask,CEventRevived,CEventScriptCommand,CEventSwitch2NM).
    pub field_14: u8,
    /// Unknown bytes (0x15..0x38).
    pub _pad_0015: [u8; 0x23],
    /// field_38 (confidence: high, kind: bool, lanes: c-events,via:CEventCommunicateEvent,via:CEventDamage,via:CEventGunShot,via:CEventPotentialWalkIntoFire,via:CEventSoundDynamic,via:CEventVehicleDamage moved from siblings:CEventCommunicateEvent,CEventEditableResponse).
    pub field_38: u8,
    /// Unknown bytes (0x39..0x3c).
    pub _pad_0039: [u8; 0x3],
    /// field_3c (confidence: high, kind: i32, lanes: c-events,via:CEventClimbNavMeshOnRoute,via:CEventSoundDynamic moved from siblings:CEventClimbNavMeshOnRoute,CEventEditableResponse).
    pub field_3c: u32,
    /// field_40 (confidence: high, kind: bool, lanes: c-events,via:CEventGunShot,via:CEventObjectCollision,via:CEventVehicleCollision moved from siblings:CEventEditableResponse,CEventObjectCollision,CEventVehicleCollision).
    pub field_40: u8,
    /// Unknown bytes (0x41..0x42).
    pub _pad_0041: [u8; 0x1],
    /// field_42 (confidence: high, kind: bool, lanes: c-events,via:CEventGunShot,via:CEventVehicleCollision moved from siblings:CEventEditableResponse,CEventVehicleCollision).
    pub field_42: u8,
    /// Unknown bytes (0x43..0x44).
    pub _pad_0043: [u8; 0x1],
    /// field_44 (confidence: high, kind: i32, lanes: c-events,via:CEventGunShot,via:CEventImminentDanger,via:CEventVehicleDamage moved from siblings:CEventEditableResponse,CEventImminentDanger).
    pub field_44: u32,
    /// field_48 (confidence: high, kind: i32, lanes: c-events,via:CEventGunShot,via:CEventImminentDanger,via:CEventVehicleDamage moved from siblings:CEventEditableResponse,CEventImminentDanger).
    pub field_48: u32,
}
assert_size!(CEvent, 0x4c); // merged size 0x4c rounded to 4
assert_offset!(CEvent, vtbl, 0x0);
assert_offset!(CEvent, field_4, 0x4);
assert_offset!(CEvent, field_8, 0x8);
assert_offset!(CEvent, field_c, 0xc);
assert_offset!(CEvent, field_e, 0xe);
assert_offset!(CEvent, field_14, 0x14);
assert_offset!(CEvent, field_38, 0x38);
assert_offset!(CEvent, field_3c, 0x3c);
assert_offset!(CEvent, field_40, 0x40);
assert_offset!(CEvent, field_42, 0x42);
assert_offset!(CEvent, field_44, 0x44);
assert_offset!(CEvent, field_48, 0x48);

/// Merged layout for `CEventCommunicateEvent`.
///
/// Size: 0x39 (low). Bases: CEvent@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventCommunicateEvent {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_10: Ptr32<u8>,
    /// Unknown bytes (0x14..0x18).
    pub _pad_0014: [u8; 0x4],
    /// field_18 (confidence: low, kind: i32, lanes: c-events).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: medium, kind: float, lanes: c-events).
    pub field_24: f32,
    /// field_28 (confidence: medium, kind: float, lanes: c-events).
    pub field_28: f32,
    /// field_2c (confidence: low, kind: i32, lanes: c-events).
    pub field_2c: u32,
    /// field_30 (confidence: low, kind: i32, lanes: c-events).
    pub field_30: u32,
    /// field_34 (confidence: medium, kind: float, lanes: c-events).
    pub field_34: f32,
    /// Unknown trailing bytes (0x38..0x3c).
    pub _pad_end: [u8; 0x4],
}
assert_size!(CEventCommunicateEvent, 0x3c); // merged size 0x39 rounded to 4
assert_offset!(CEventCommunicateEvent, field_10, 0x10);
assert_offset!(CEventCommunicateEvent, field_18, 0x18);
assert_offset!(CEventCommunicateEvent, field_20, 0x20);
assert_offset!(CEventCommunicateEvent, field_24, 0x24);
assert_offset!(CEventCommunicateEvent, field_28, 0x28);
assert_offset!(CEventCommunicateEvent, field_2c, 0x2c);
assert_offset!(CEventCommunicateEvent, field_30, 0x30);
assert_offset!(CEventCommunicateEvent, field_34, 0x34);

/// Merged layout for `CEventDamage`.
///
/// Size: 0x39 (low). Bases: CEventEditableResponse@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventDamage {
    /// Unknown bytes (0x0..0x18).
    pub _pad_0000: [u8; 0x18],
    /// field_18 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: i32, lanes: c-events).
    pub field_1c: u32,
    /// Unknown bytes (0x20..0x24).
    pub _pad_0020: [u8; 0x4],
    /// embedded_CPedDamageResponse (confidence: high, kind: embedded-object, lanes: c-events).
    pub embedded_cpeddamageresponse: u32,
    /// field_28 (confidence: medium, kind: bool, lanes: c-events).
    pub field_28: u8,
    /// Unknown bytes (0x29..0x2c).
    pub _pad_0029: [u8; 0x3],
    /// field_2c (confidence: medium, kind: float, lanes: c-events).
    pub field_2c: f32,
    /// field_30 (confidence: medium, kind: float, lanes: c-events).
    pub field_30: f32,
    /// field_34 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_34: Ptr32<u8>,
    /// Unknown trailing bytes (0x38..0x3c).
    pub _pad_end: [u8; 0x4],
}
assert_size!(CEventDamage, 0x3c); // merged size 0x39 rounded to 4
assert_offset!(CEventDamage, field_18, 0x18);
assert_offset!(CEventDamage, field_1c, 0x1c);
assert_offset!(CEventDamage, embedded_cpeddamageresponse, 0x24);
assert_offset!(CEventDamage, field_28, 0x28);
assert_offset!(CEventDamage, field_2c, 0x2c);
assert_offset!(CEventDamage, field_30, 0x30);
assert_offset!(CEventDamage, field_34, 0x34);

/// Merged layout for `CEventDraggedOutCar`.
///
/// Size: 0x22 (low). Bases: CEventEditableResponse@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventDraggedOutCar {
    /// Unknown bytes (0x0..0x18).
    pub _pad_0000: [u8; 0x18],
    /// field_18 (confidence: medium, kind: i32, lanes: c-events).
    pub field_18: u32,
    /// field_1c (confidence: medium, kind: i32, lanes: c-events).
    pub field_1c: u32,
    /// Unknown bytes (0x20..0x21).
    pub _pad_0020: [u8; 0x1],
    /// field_21 (confidence: medium, kind: bool, lanes: c-events).
    pub field_21: u8,
    /// Unknown trailing bytes (0x22..0x24).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CEventDraggedOutCar, 0x24); // merged size 0x22 rounded to 4
assert_offset!(CEventDraggedOutCar, field_18, 0x18);
assert_offset!(CEventDraggedOutCar, field_1c, 0x1c);
assert_offset!(CEventDraggedOutCar, field_21, 0x21);

/// Merged layout for `CEventGunShot`.
///
/// Size: 0x4c (low). Bases: CEventEditableResponse@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventGunShot {
    /// Unknown bytes (0x0..0x18).
    pub _pad_0000: [u8; 0x18],
    /// field_18 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x24).
    pub _pad_001c: [u8; 0x8],
    /// field_24 (confidence: medium, kind: float, lanes: c-events).
    pub field_24: f32,
    /// field_28 (confidence: medium, kind: float, lanes: c-events).
    pub field_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: i32, lanes: c-events).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: i32, lanes: c-events).
    pub field_34: u32,
    /// Unknown bytes (0x38..0x43).
    pub _pad_0038: [u8; 0xb],
    /// field_43 (confidence: medium, kind: bool, lanes: c-events).
    pub field_43: u8,
    /// Unknown trailing bytes (0x44..0x4c).
    pub _pad_end: [u8; 0x8],
}
assert_size!(CEventGunShot, 0x4c); // merged size 0x4c rounded to 4
assert_offset!(CEventGunShot, field_18, 0x18);
assert_offset!(CEventGunShot, field_24, 0x24);
assert_offset!(CEventGunShot, field_28, 0x28);
assert_offset!(CEventGunShot, field_30, 0x30);
assert_offset!(CEventGunShot, field_34, 0x34);
assert_offset!(CEventGunShot, field_43, 0x43);

/// Merged layout for `CEventHandler`.
///
/// Size: 0x38 (low). Bases: none.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventHandler {
    /// __vtbl (confidence: high, kind: vtable_ptr, lanes: c-events).
    pub vtbl: Ptr32<()>,
    /// field_4 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_4: Ptr32<u8>,
    /// field_8 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-events).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: i32, lanes: c-events).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: i32, lanes: c-events).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-events).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: i32, lanes: c-events).
    pub field_2c: u32,
    /// field_30 (confidence: low, kind: i32, lanes: c-events).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: i32, lanes: c-events).
    pub field_34: u32,
}
assert_size!(CEventHandler, 0x38); // merged size 0x38 rounded to 4
assert_offset!(CEventHandler, vtbl, 0x0);
assert_offset!(CEventHandler, field_4, 0x4);
assert_offset!(CEventHandler, field_8, 0x8);
assert_offset!(CEventHandler, field_c, 0xc);
assert_offset!(CEventHandler, field_10, 0x10);
assert_offset!(CEventHandler, field_14, 0x14);
assert_offset!(CEventHandler, field_18, 0x18);
assert_offset!(CEventHandler, field_20, 0x20);
assert_offset!(CEventHandler, field_24, 0x24);
assert_offset!(CEventHandler, field_28, 0x28);
assert_offset!(CEventHandler, field_2c, 0x2c);
assert_offset!(CEventHandler, field_30, 0x30);
assert_offset!(CEventHandler, field_34, 0x34);

/// Merged layout for `CEventPedCollisionWithPed`.
///
/// Size: 0x44 (low). Bases: CEvent@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventPedCollisionWithPed {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: float, lanes: c-events).
    pub field_10: f32,
    /// Unknown bytes (0x14..0x20).
    pub _pad_0014: [u8; 0xc],
    /// field_20 (confidence: medium, kind: float, lanes: c-events).
    pub field_20: f32,
    /// field_24 (confidence: medium, kind: float, lanes: c-events).
    pub field_24: f32,
    /// field_28 (confidence: medium, kind: float, lanes: c-events).
    pub field_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: i32, lanes: c-events).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: i32, lanes: c-events).
    pub field_34: u32,
    /// Unknown trailing bytes (0x38..0x44).
    pub _pad_end: [u8; 0xc],
}
assert_size!(CEventPedCollisionWithPed, 0x44); // merged size 0x44 rounded to 4
assert_offset!(CEventPedCollisionWithPed, field_10, 0x10);
assert_offset!(CEventPedCollisionWithPed, field_20, 0x20);
assert_offset!(CEventPedCollisionWithPed, field_24, 0x24);
assert_offset!(CEventPedCollisionWithPed, field_28, 0x28);
assert_offset!(CEventPedCollisionWithPed, field_30, 0x30);
assert_offset!(CEventPedCollisionWithPed, field_34, 0x34);

/// Merged layout for `CEventShocking`.
///
/// Size: 0x3c (low). Bases: CEvent@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventShocking {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: i32, lanes: c-events).
    pub field_10: u32,
    /// Unknown bytes (0x14..0x20).
    pub _pad_0014: [u8; 0xc],
    /// field_20 (confidence: medium, kind: float, lanes: c-events).
    pub field_20: f32,
    /// field_24 (confidence: medium, kind: float, lanes: c-events).
    pub field_24: f32,
    /// field_28 (confidence: medium, kind: float, lanes: c-events).
    pub field_28: f32,
    /// Unknown bytes (0x2c..0x36).
    pub _pad_002c: [u8; 0xa],
    /// field_36 (confidence: low, kind: byte, lanes: c-events).
    pub field_36: u8,
    /// Unknown trailing bytes (0x37..0x3c).
    pub _pad_end: [u8; 0x5],
}
assert_size!(CEventShocking, 0x3c); // merged size 0x3c rounded to 4
assert_offset!(CEventShocking, field_10, 0x10);
assert_offset!(CEventShocking, field_20, 0x20);
assert_offset!(CEventShocking, field_24, 0x24);
assert_offset!(CEventShocking, field_28, 0x28);
assert_offset!(CEventShocking, field_36, 0x36);

/// Merged layout for `CEventSoundDynamic`.
///
/// Size: 0x44 (low). Bases: CEventEditableResponse@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventSoundDynamic {
    /// Unknown bytes (0x0..0x24).
    pub _pad_0000: [u8; 0x24],
    /// field_24 (confidence: medium, kind: float, lanes: c-events).
    pub field_24: f32,
    /// field_28 (confidence: medium, kind: float, lanes: c-events).
    pub field_28: f32,
    /// field_2c (confidence: low, kind: i32, lanes: c-events).
    pub field_2c: u32,
    /// field_30 (confidence: medium, kind: i32, lanes: c-events).
    pub field_30: u32,
    /// field_34 (confidence: medium, kind: float, lanes: c-events).
    pub field_34: f32,
    /// Unknown trailing bytes (0x38..0x44).
    pub _pad_end: [u8; 0xc],
}
assert_size!(CEventSoundDynamic, 0x44); // merged size 0x44 rounded to 4
assert_offset!(CEventSoundDynamic, field_24, 0x24);
assert_offset!(CEventSoundDynamic, field_28, 0x28);
assert_offset!(CEventSoundDynamic, field_2c, 0x2c);
assert_offset!(CEventSoundDynamic, field_30, 0x30);
assert_offset!(CEventSoundDynamic, field_34, 0x34);

/// Merged layout for `CEventVehicleCollision`.
///
/// Size: 0x43 (low). Bases: CEvent@0x0.
/// Lanes: c-events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEventVehicleCollision {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: float, lanes: c-events).
    pub field_10: f32,
    /// Unknown bytes (0x14..0x20).
    pub _pad_0014: [u8; 0xc],
    /// field_20 (confidence: medium, kind: float, lanes: c-events).
    pub field_20: f32,
    /// field_24 (confidence: medium, kind: float, lanes: c-events).
    pub field_24: f32,
    /// field_28 (confidence: medium, kind: float, lanes: c-events).
    pub field_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: i32, lanes: c-events).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: i32, lanes: c-events).
    pub field_34: u32,
    /// Unknown trailing bytes (0x38..0x44).
    pub _pad_end: [u8; 0xc],
}
assert_size!(CEventVehicleCollision, 0x44); // merged size 0x43 rounded to 4
assert_offset!(CEventVehicleCollision, field_10, 0x10);
assert_offset!(CEventVehicleCollision, field_20, 0x20);
assert_offset!(CEventVehicleCollision, field_24, 0x24);
assert_offset!(CEventVehicleCollision, field_28, 0x28);
assert_offset!(CEventVehicleCollision, field_30, 0x30);
assert_offset!(CEventVehicleCollision, field_34, 0x34);

/// Merged layout for `CIntermezzoEvent`.
///
/// Size: 0x28 (low). Bases: none.
/// Lanes: c-misc-a, via:CIntermezzoEventFalling, via:CIntermezzoEventFallingThruAir, via:CIntermezzoEventGarage, via:CIntermezzoEventIntro, via:CIntermezzoEventPlaceBomb, via:CIntermezzoEventPlayerJackVehicle, via:CIntermezzoEventPlayerJackedVehicle, via:CIntermezzoEventStuntJump, via:CIntermezzoEventVaultWall.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CIntermezzoEvent {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-misc-a).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventFalling,via:CIntermezzoEventFallingThruAir,via:CIntermezzoEventGarage,via:CIntermezzoEventIntro,via:CIntermezzoEventPlaceBomb,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventPlayerJackedVehicle,via:CIntermezzoEventStuntJump,via:CIntermezzoEventVaultWall moved from siblings:CIntermezzoEventFalling,CIntermezzoEventFallingThruAir,CIntermezzoEventGarage,CIntermezzoEventIntro,CIntermezzoEventPlaceBomb,CIntermezzoEventPlayerJackVehicle,CIntermezzoEventPlayerJackedVehicle,CIntermezzoEventStuntJump,CIntermezzoEventVaultWall).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventFalling,via:CIntermezzoEventFallingThruAir,via:CIntermezzoEventGarage,via:CIntermezzoEventIntro,via:CIntermezzoEventPlaceBomb,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventPlayerJackedVehicle,via:CIntermezzoEventStuntJump,via:CIntermezzoEventVaultWall moved from siblings:CIntermezzoEventFalling,CIntermezzoEventFallingThruAir,CIntermezzoEventGarage,CIntermezzoEventIntro,CIntermezzoEventPlaceBomb,CIntermezzoEventPlayerJackVehicle,CIntermezzoEventPlayerJackedVehicle,CIntermezzoEventStuntJump,CIntermezzoEventVaultWall).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventFalling,via:CIntermezzoEventFallingThruAir,via:CIntermezzoEventGarage,via:CIntermezzoEventIntro,via:CIntermezzoEventPlaceBomb,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventPlayerJackedVehicle,via:CIntermezzoEventStuntJump,via:CIntermezzoEventVaultWall moved from siblings:CIntermezzoEventFalling,CIntermezzoEventFallingThruAir,CIntermezzoEventGarage,CIntermezzoEventIntro,CIntermezzoEventPlaceBomb,CIntermezzoEventPlayerJackVehicle,CIntermezzoEventPlayerJackedVehicle,CIntermezzoEventStuntJump,CIntermezzoEventVaultWall).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: word, lanes: c-misc-a,via:CIntermezzoEventFalling,via:CIntermezzoEventFallingThruAir,via:CIntermezzoEventGarage,via:CIntermezzoEventIntro,via:CIntermezzoEventPlaceBomb,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventPlayerJackedVehicle,via:CIntermezzoEventStuntJump,via:CIntermezzoEventVaultWall moved from siblings:CIntermezzoEventFalling,CIntermezzoEventFallingThruAir,CIntermezzoEventGarage,CIntermezzoEventIntro,CIntermezzoEventPlaceBomb,CIntermezzoEventPlayerJackVehicle,CIntermezzoEventPlayerJackedVehicle,CIntermezzoEventStuntJump,CIntermezzoEventVaultWall).
    pub field_10: u16,
    /// Unknown bytes (0x12..0x14).
    pub _pad_0012: [u8; 0x2],
    /// field_14 (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventGarage,via:CIntermezzoEventPlaceBomb,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventPlayerJackedVehicle,via:CIntermezzoEventStuntJump moved from siblings:CIntermezzoEventGarage,CIntermezzoEventPlaceBomb,CIntermezzoEventPlayerJackVehicle,CIntermezzoEventPlayerJackedVehicle,CIntermezzoEventStuntJump).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventGarage,via:CIntermezzoEventPlaceBomb,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventPlayerJackedVehicle moved from siblings:CIntermezzoEventGarage,CIntermezzoEventPlaceBomb,CIntermezzoEventPlayerJackVehicle,CIntermezzoEventPlayerJackedVehicle).
    pub field_18: u32,
    /// field_1c (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventPlaceBomb,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventPlayerJackedVehicle moved from siblings:CIntermezzoEventPlaceBomb,CIntermezzoEventPlayerJackVehicle,CIntermezzoEventPlayerJackedVehicle).
    pub field_1c: u32,
    /// field_20 (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventPlayerJackedVehicle,via:CIntermezzoEventStuntJump moved from siblings:CIntermezzoEventPlayerJackedVehicle,CIntermezzoEventStuntJump).
    pub field_20: u32,
    /// field_24 (confidence: medium, kind: u32, lanes: c-misc-a,via:CIntermezzoEventPlayerJackVehicle,via:CIntermezzoEventStuntJump moved from siblings:CIntermezzoEventPlayerJackVehicle,CIntermezzoEventStuntJump).
    pub field_24: u32,
}
assert_size!(CIntermezzoEvent, 0x28); // merged size 0x28 rounded to 4
assert_offset!(CIntermezzoEvent, vfptr, 0x0);
assert_offset!(CIntermezzoEvent, field_4, 0x4);
assert_offset!(CIntermezzoEvent, field_8, 0x8);
assert_offset!(CIntermezzoEvent, field_c, 0xc);
assert_offset!(CIntermezzoEvent, field_10, 0x10);
assert_offset!(CIntermezzoEvent, field_14, 0x14);
assert_offset!(CIntermezzoEvent, field_18, 0x18);
assert_offset!(CIntermezzoEvent, field_1c, 0x1c);
assert_offset!(CIntermezzoEvent, field_20, 0x20);
assert_offset!(CIntermezzoEvent, field_24, 0x24);
