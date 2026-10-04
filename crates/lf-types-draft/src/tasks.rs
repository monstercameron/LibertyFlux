//! Tasks: the `CTask` hierarchy and task records.
//!
//! Holds 41 draft layouts: `CTask` and the `CTaskSimple*` and `CTaskComplex*` families, the
//! dummy-ped tasks `CDummyTask*`, the task-info records (`CTaskInfo`, `CComplexGunTaskInfo`,
//! `CScenarioTaskInfo`), `CWeightedTaskSet`, and the `CTaskNode` record seen by the native-handler
//! lanes. Every layout is Inferred; size confidence (the analysis lanes' own rating) is high for 0,
//! medium for 1 and low for 40. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CComplexGunTaskInfo`.
///
/// Size: 0x4c (low). Bases: CStatusAndTargetTaskInfo@0x0.
/// Lanes: c-misc-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CComplexGunTaskInfo {
    /// Unknown bytes (0x0..0x38).
    pub _pad_0000: [u8; 0x38],
    /// field_38 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: word, lanes: c-misc-a).
    pub field_44: u16,
    /// field_46 (confidence: medium, kind: bool_or_byte, lanes: c-misc-a).
    pub field_46: u8,
    /// Unknown bytes (0x47..0x48).
    pub _pad_0047: [u8; 0x1],
    /// field_48 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_48: u32,
}
assert_size!(CComplexGunTaskInfo, 0x4c); // merged size 0x4c rounded to 4
assert_offset!(CComplexGunTaskInfo, field_38, 0x38);
assert_offset!(CComplexGunTaskInfo, field_40, 0x40);
assert_offset!(CComplexGunTaskInfo, field_44, 0x44);
assert_offset!(CComplexGunTaskInfo, field_46, 0x46);
assert_offset!(CComplexGunTaskInfo, field_48, 0x48);

/// Merged layout for `CDummyTask`.
///
/// Size: 0x62 (low). Bases: none.
/// Lanes: c-entities, via:CDummyTask_Dead, via:CDummyTask_FallDownAndGetUp, via:CDummyTask_Flee, via:CDummyTask_PlayAnim, via:CDummyTask_ShockingEventWatch, via:CDummyTask_Stationary, via:CDummyTask_Wander.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDummyTask {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_4: u32,
    /// field_8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x18).
    pub _pad_0010: [u8; 0x8],
    /// field_18 (confidence: high, kind: bool-or-byte, lanes: c-entities,via:CDummyTask_Flee,via:CDummyTask_Wander moved from siblings:CDummyTask_Flee,CDummyTask_Wander).
    pub field_18: u8,
    /// Unknown bytes (0x19..0x1c).
    pub _pad_0019: [u8; 0x3],
    /// field_1c (confidence: high, kind: u32, lanes: c-entities,via:CDummyTask_FallDownAndGetUp,via:CDummyTask_Flee,via:CDummyTask_PlayAnim,via:CDummyTask_ShockingEventWatch,via:CDummyTask_Stationary,via:CDummyTask_Wander moved from siblings:CDummyTask_FallDownAndGetUp,CDummyTask_Flee,CDummyTask_PlayAnim,CDummyTask_ShockingEventWatch,CDummyTask_Stationary,CDummyTask_Wander).
    pub field_1c: u32,
    /// field_20 (confidence: high, kind: bool-or-byte, lanes: c-entities,via:CDummyTask_Dead,via:CDummyTask_Stationary moved from siblings:CDummyTask_PlayAnim,CDummyTask_Stationary).
    pub field_20: u8,
    /// Unknown bytes (0x21..0x2c).
    pub _pad_0021: [u8; 0xb],
    /// field_2c (confidence: high, kind: u32, lanes: c-entities,via:CDummyTask_Flee,via:CDummyTask_Stationary moved from siblings:CDummyTask_Flee,CDummyTask_Stationary).
    pub field_2c: u32,
    /// Unknown bytes (0x30..0x34).
    pub _pad_0030: [u8; 0x4],
    /// field_34 (confidence: high, kind: bool-or-byte, lanes: c-entities,via:CDummyTask_ShockingEventWatch,via:CDummyTask_Stationary moved from siblings:CDummyTask_ShockingEventWatch,CDummyTask_Stationary).
    pub field_34: u8,
    /// Unknown bytes (0x35..0x38).
    pub _pad_0035: [u8; 0x3],
    /// field_38 (confidence: high, kind: u32, lanes: c-entities,via:CDummyTask_Dead,via:CDummyTask_FallDownAndGetUp,via:CDummyTask_Flee,via:CDummyTask_Stationary moved from siblings:CDummyTask_FallDownAndGetUp,CDummyTask_Flee,CDummyTask_PlayAnim,CDummyTask_Stationary).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x48).
    pub _pad_003c: [u8; 0xc],
    /// field_48 (confidence: high, kind: bool-or-byte, lanes: c-entities,via:CDummyTask_Stationary,via:CDummyTask_Wander moved from siblings:CDummyTask_Stationary,CDummyTask_Wander).
    pub field_48: u8,
    /// Unknown bytes (0x49..0x4a).
    pub _pad_0049: [u8; 0x1],
    /// field_4a (confidence: high, kind: int16?, lanes: c-entities,via:CDummyTask_Stationary,via:CDummyTask_Wander moved from siblings:CDummyTask_Stationary,CDummyTask_Wander).
    pub field_4a: u16,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: high, kind: u32, lanes: c-entities,via:CDummyTask_Stationary,via:CDummyTask_Wander moved from siblings:CDummyTask_Stationary,CDummyTask_Wander).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x60).
    pub _pad_0054: [u8; 0xc],
    /// field_60 (confidence: medium, kind: int16?, lanes: c-entities,via:CDummyTask_Stationary,via:CDummyTask_Wander moved from siblings:CDummyTask_Stationary,CDummyTask_Wander).
    pub field_60: u16,
    /// Unknown trailing bytes (0x62..0x64).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CDummyTask, 0x64); // merged size 0x62 rounded to 4
assert_offset!(CDummyTask, vfptr, 0x0);
assert_offset!(CDummyTask, field_4, 0x4);
assert_offset!(CDummyTask, field_8, 0x8);
assert_offset!(CDummyTask, field_c, 0xc);
assert_offset!(CDummyTask, field_18, 0x18);
assert_offset!(CDummyTask, field_1c, 0x1c);
assert_offset!(CDummyTask, field_20, 0x20);
assert_offset!(CDummyTask, field_2c, 0x2c);
assert_offset!(CDummyTask, field_34, 0x34);
assert_offset!(CDummyTask, field_38, 0x38);
assert_offset!(CDummyTask, field_48, 0x48);
assert_offset!(CDummyTask, field_4a, 0x4a);
assert_offset!(CDummyTask, field_50, 0x50);
assert_offset!(CDummyTask, field_60, 0x60);

/// Merged layout for `CDummyTask_FallDownAndGetUp`.
///
/// Size: 0x3c (low). Bases: CDummyTask@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDummyTaskFallDownAndGetUp {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: high, kind: float, lanes: c-entities).
    pub field_10: f32,
    /// field_14 (confidence: high, kind: float, lanes: c-entities).
    pub field_14: f32,
    /// Unknown bytes (0x18..0x24).
    pub _pad_0018: [u8; 0xc],
    /// field_24 (confidence: high, kind: pointer, lanes: c-entities).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: high, kind: u32, lanes: c-entities).
    pub field_28: u32,
    /// Unknown trailing bytes (0x2c..0x3c).
    pub _pad_end: [u8; 0x10],
}
assert_size!(CDummyTaskFallDownAndGetUp, 0x3c); // merged size 0x3c rounded to 4
assert_offset!(CDummyTaskFallDownAndGetUp, field_10, 0x10);
assert_offset!(CDummyTaskFallDownAndGetUp, field_14, 0x14);
assert_offset!(CDummyTaskFallDownAndGetUp, field_24, 0x24);
assert_offset!(CDummyTaskFallDownAndGetUp, field_28, 0x28);

/// Merged layout for `CDummyTask_Flee`.
///
/// Size: 0x42 (low). Bases: CDummyTask@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDummyTaskFlee {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: high, kind: u32, lanes: c-entities).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: u32, lanes: c-entities).
    pub field_14: u32,
    /// Unknown bytes (0x18..0x24).
    pub _pad_0018: [u8; 0xc],
    /// field_24 (confidence: high, kind: float, lanes: c-entities).
    pub field_24: f32,
    /// field_28 (confidence: high, kind: float, lanes: c-entities).
    pub field_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_30: u8,
    /// field_31 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_31: u8,
    /// Unknown bytes (0x32..0x3c).
    pub _pad_0032: [u8; 0xa],
    /// field_3c (confidence: high, kind: int16?, lanes: c-entities).
    pub field_3c: u16,
    /// field_3e (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_3e: u8,
    /// Unknown trailing bytes (0x3f..0x44).
    pub _pad_end: [u8; 0x5],
}
assert_size!(CDummyTaskFlee, 0x44); // merged size 0x42 rounded to 4
assert_offset!(CDummyTaskFlee, field_10, 0x10);
assert_offset!(CDummyTaskFlee, field_14, 0x14);
assert_offset!(CDummyTaskFlee, field_24, 0x24);
assert_offset!(CDummyTaskFlee, field_28, 0x28);
assert_offset!(CDummyTaskFlee, field_30, 0x30);
assert_offset!(CDummyTaskFlee, field_31, 0x31);
assert_offset!(CDummyTaskFlee, field_3c, 0x3c);
assert_offset!(CDummyTaskFlee, field_3e, 0x3e);

/// Merged layout for `CDummyTask_ShockingEventWatch`.
///
/// Size: 0x3a (low). Bases: CDummyTask@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDummyTaskShockingEventWatch {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: high, kind: u32, lanes: c-entities).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: float, lanes: c-entities).
    pub field_14: f32,
    /// Unknown bytes (0x18..0x24).
    pub _pad_0018: [u8; 0xc],
    /// field_24 (confidence: low, kind: u32, lanes: c-entities).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: u32, lanes: c-entities).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: high, kind: pointer, lanes: c-entities).
    pub field_30: Ptr32<u8>,
    /// Unknown trailing bytes (0x34..0x3c).
    pub _pad_end: [u8; 0x8],
}
assert_size!(CDummyTaskShockingEventWatch, 0x3c); // merged size 0x3a rounded to 4
assert_offset!(CDummyTaskShockingEventWatch, field_10, 0x10);
assert_offset!(CDummyTaskShockingEventWatch, field_14, 0x14);
assert_offset!(CDummyTaskShockingEventWatch, field_24, 0x24);
assert_offset!(CDummyTaskShockingEventWatch, field_28, 0x28);
assert_offset!(CDummyTaskShockingEventWatch, field_30, 0x30);

/// Merged layout for `CDummyTask_Stationary`.
///
/// Size: 0x62 (low). Bases: CDummyTask@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDummyTaskStationary {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: high, kind: pointer, lanes: c-entities).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: high, kind: float, lanes: c-entities).
    pub field_14: f32,
    /// Unknown bytes (0x18..0x24).
    pub _pad_0018: [u8; 0xc],
    /// field_24 (confidence: high, kind: u32, lanes: c-entities).
    pub field_24: u32,
    /// field_28 (confidence: high, kind: u32, lanes: c-entities).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: high, kind: u32, lanes: c-entities).
    pub field_30: u32,
    /// Unknown bytes (0x34..0x36).
    pub _pad_0034: [u8; 0x2],
    /// field_36 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_36: u16,
    /// Unknown bytes (0x38..0x3c).
    pub _pad_0038: [u8; 0x4],
    /// field_3c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_3c: u32,
    /// field_40 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x54).
    pub _pad_0048: [u8; 0xc],
    /// field_54 (confidence: high, kind: float, lanes: c-entities).
    pub field_54: f32,
    /// field_58 (confidence: high, kind: float, lanes: c-entities).
    pub field_58: f32,
    /// field_5c (confidence: low, kind: u32, lanes: c-entities).
    pub field_5c: u32,
    /// Unknown trailing bytes (0x60..0x64).
    pub _pad_end: [u8; 0x4],
}
assert_size!(CDummyTaskStationary, 0x64); // merged size 0x62 rounded to 4
assert_offset!(CDummyTaskStationary, field_10, 0x10);
assert_offset!(CDummyTaskStationary, field_14, 0x14);
assert_offset!(CDummyTaskStationary, field_24, 0x24);
assert_offset!(CDummyTaskStationary, field_28, 0x28);
assert_offset!(CDummyTaskStationary, field_30, 0x30);
assert_offset!(CDummyTaskStationary, field_36, 0x36);
assert_offset!(CDummyTaskStationary, field_3c, 0x3c);
assert_offset!(CDummyTaskStationary, field_40, 0x40);
assert_offset!(CDummyTaskStationary, field_44, 0x44);
assert_offset!(CDummyTaskStationary, field_54, 0x54);
assert_offset!(CDummyTaskStationary, field_58, 0x58);
assert_offset!(CDummyTaskStationary, field_5c, 0x5c);

/// Merged layout for `CScenarioTaskInfo`.
///
/// Size: 0x70 (low). Bases: CTaskInfoWithCloneTask@0x0.
/// Lanes: c-misc-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CScenarioTaskInfo {
    /// Unknown bytes (0x0..0x28).
    pub _pad_0000: [u8; 0x28],
    /// field_28 (confidence: low, kind: u32, lanes: c-misc-a).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x34).
    pub _pad_002c: [u8; 0x8],
    /// field_34 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: high, kind: word, lanes: c-misc-a).
    pub field_40: u16,
    /// Unknown bytes (0x42..0x44).
    pub _pad_0042: [u8; 0x2],
    /// field_44 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_44: u32,
    /// Unknown trailing bytes (0x48..0x70).
    pub _pad_end: [u8; 0x28],
}
assert_size!(CScenarioTaskInfo, 0x70); // merged size 0x70 rounded to 4
assert_offset!(CScenarioTaskInfo, field_28, 0x28);
assert_offset!(CScenarioTaskInfo, field_34, 0x34);
assert_offset!(CScenarioTaskInfo, field_38, 0x38);
assert_offset!(CScenarioTaskInfo, field_40, 0x40);
assert_offset!(CScenarioTaskInfo, field_44, 0x44);

/// Merged layout for `CTask`.
///
/// Size: 0x2ef (low). Bases: none.
/// Lanes: c-tasks-a, c-tasks-b, n-08, n-10, n-11, via:CTaskComplexAimAndThrowProjectile, via:CTaskComplexArrestPed, via:CTaskComplexAvoidPlayerTargetting, via:CTaskComplexBackOff, via:CTaskComplexCarDrive, via:CTaskComplexCarDriveBasic, via:CTaskComplexCarDriveMission, via:CTaskComplexCarReactToVehicleCollisionGetOut, via:CTaskComplexCombatAdvanceSubtask, via:CTaskComplexCombatClosestTargetInArea, via:CTaskComplexCombatFireSubtask, via:CTaskComplexCombatFlankSubtask, via:CTaskComplexCombatInvestigateSubtask, via:CTaskComplexCombatPersueInCarSubtask, via:CTaskComplexCombatSeekCoverSubtask, via:CTaskComplexCombatSubtask, via:CTaskComplexCop, via:CTaskComplexDestroyCar, via:CTaskComplexDestroyCarArmed, via:CTaskComplexDie, via:CTaskComplexDriveFireTruck, via:CTaskComplexDriveToPoint, via:CTaskComplexEvasiveStep, via:CTaskComplexFleeAndDive, via:CTaskComplexFleeAnyMeans, via:CTaskComplexGangDriveby, via:CTaskComplexGangHasslePed, via:CTaskComplexGetOffBoat, via:CTaskComplexGetOutOfWater, via:CTaskComplexGoToCarDoorAndStandStill, via:CTaskComplexGoToPointAiming, via:CTaskComplexGun, via:CTaskComplexInjuredOnGround, via:CTaskComplexInvestigateDeadPed, via:CTaskComplexJump, via:CTaskComplexLeaveAnyCar, via:CTaskComplexLeaveCarAndFlee, via:CTaskComplexLeaveCarAndWander, via:CTaskComplexMedicPassenger, via:CTaskComplexMedicTreatInjuredPed, via:CTaskComplexMoveAroundCoverPoints, via:CTaskComplexMoveAvoidOtherPedWhileWandering, via:CTaskComplexMoveBeInFormation, via:CTaskComplexMoveFollowNavMeshRoute, via:CTaskComplexMoveGetOntoMainNavMesh, via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>, via:CTaskComplexMoveWaitForTraffic, via:CTaskComplexMoveWander, via:CTaskComplexNewGetInVehicle, via:CTaskComplexNewUseCover, via:CTaskComplexPlayerDrive, via:CTaskComplexPlayerGun, via:CTaskComplexPlayerInCover, via:CTaskComplexPlayerOnFoot, via:CTaskComplexPlayerSettingsTask, via:CTaskComplexReact, via:CTaskComplexScenario, via:CTaskComplexSearchForPedInCar, via:CTaskComplexSearchForPedOnFoot, via:CTaskComplexSearchWander, via:CTaskComplexSeatedScenario, via:CTaskComplexSeekCover, via:CTaskComplexSeekCoverShooting, via:CTaskComplexSeekEntityAnyMeans<CEntitySeekPosCalculatorXYOffset>, via:CTaskComplexShockingEventGoto, via:CTaskComplexShockingEventHurryAway, via:CTaskComplexSitDownThenIdleThenStandUp, via:CTaskComplexSitIdle, via:CTaskComplexSmartFleeEntity, via:CTaskComplexStandGuard, via:CTaskComplexStationaryScenario, via:CTaskComplexStealCar, via:CTaskComplexThrowProjectile, via:CTaskComplexTrackEntity, via:CTaskComplexTurnToFaceEntityOrCoord, via:CTaskComplexUseClimbOnRoute, via:CTaskComplexUseDropDownOnRoute, via:CTaskComplexUseEffect, via:CTaskComplexUseLadderOnRoute, via:CTaskComplexUseMobilePhone, via:CTaskComplexUseSequence, via:CTaskComplexVehicleSubtask, via:CTaskComplexWaitForSteppingOut, via:CTaskComplexWanderStandard, via:CTaskSimpleAffectSecondaryBehaviour, via:CTaskSimpleAimGun, via:CTaskSimpleAnim, via:CTaskSimpleArrestPed, via:CTaskSimpleAssessInjuredPed, via:CTaskSimpleBeHit, via:CTaskSimpleBlendFromNM, via:CTaskSimpleCarAlign, via:CTaskSimpleCarCloseDoorFromInside, via:CTaskSimpleCarCloseDoorFromOutside, via:CTaskSimpleCarGetIn, via:CTaskSimpleCarGetOut, via:CTaskSimpleCarJumpOut, via:CTaskSimpleCarOpenDoorFromOutside, via:CTaskSimpleCarOpenLockedDoorFromOutside, via:CTaskSimpleCarSetPedInVehicle, via:CTaskSimpleCarSetPedOut, via:CTaskSimpleCarShuffle, via:CTaskSimpleCarSlowDragPedOut, via:CTaskSimpleClimb, via:CTaskSimpleClimbLadder, via:CTaskSimpleCreateCarAndGetIn, via:CTaskSimpleDead, via:CTaskSimpleDie, via:CTaskSimpleFireGun, via:CTaskSimpleGetUp, via:CTaskSimpleHitHead, via:CTaskSimpleMeleeActionResult, via:CTaskSimpleMoveAchieveHeading, via:CTaskSimpleMoveMeleeMovement, via:CTaskSimpleMoveSwim, via:CTaskSimpleMoveTrackingEntity, via:CTaskSimpleNM, via:CTaskSimpleNMBrace, via:CTaskSimpleNMExplosion, via:CTaskSimpleNMJumpRollFromRoadVehicle, via:CTaskSimpleNMOnFire, via:CTaskSimpleNMRollUpAndRelax, via:CTaskSimpleNMScriptControl, via:CTaskSimpleNMShot, via:CTaskSimpleNMSit, via:CTaskSimpleOpenDoor, via:CTaskSimplePickUpObject, via:CTaskSimplePlayAnimAndSlideOutOfCover, via:CTaskSimplePlayRandomAmbients, via:CTaskSimplePlayerAimProjectile, via:CTaskSimplePlayerBeArrested, via:CTaskSimplePutDownObject, via:CTaskSimpleReloadGun, via:CTaskSimpleRunNamedAnim, via:CTaskSimpleRunTimedAnim, via:CTaskSimpleSay, via:CTaskSimpleSetCharIgnoreWeaponRangeFlag, via:CTaskSimpleShakeFist, via:CTaskSimpleShovePed, via:CTaskSimpleSmashCarWindow, via:CTaskSimpleStandStill, via:CTaskSimpleStandUp, via:CTaskSimpleThrowGrenadeFromVehicle, via:CTaskSimpleThrowProjectile, via:CTaskSimpleTogglePedThreatScanner, via:CTaskSimpleWaitUntilPedIsOutCar.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTask {
    /// vfptr (confidence: medium, kind: pointer, lanes: c-tasks-a,c-tasks-b).
    pub vfptr: Ptr32<()>,
    /// task_type_id (confidence: high, kind: u32, lanes: c-tasks-a,c-tasks-b,n-10,n-11).
    pub task_type_id: u32,
    /// flags_incl_priority_bits (confidence: high, kind: flags, lanes: c-tasks-a,c-tasks-b,n-10,n-11).
    pub flags_incl_priority_bits: u32,
    /// next_sub_task_link (confidence: high, kind: u32, lanes: c-tasks-a,c-tasks-b,n-10,n-11).
    pub next_sub_task_link: u32,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-tasks-a,c-tasks-b).
    pub field_10: Ptr32<u8>,
    /// f_14_bool (confidence: high, kind: bool/int8, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexArrestPed,via:CTaskComplexCop,via:CTaskComplexDestroyCar,via:CTaskComplexDestroyCarArmed,via:CTaskComplexDriveWanderForTime,via:CTaskComplexFollowPatrolRoute,via:CTaskComplexPlayerOnFoot,via:CTaskComplexPlayerSettingsTask,via:CTaskComplexWaitForDoorToBeOpen,via:CTaskComplexWaitForMyCarToStop,via:CTaskComplexWaitForSeatToBeFree,via:CTaskComplexWaitForSteppingOut,via:CTaskComplexWaitForTime,via:CTaskComplexWaitTillItsOkToStop,via:CTaskSimpleAffectSecondaryBehaviour,via:CTaskSimpleAssessInjuredPed,via:CTaskSimpleBlendFromNM,via:CTaskSimpleCarAlign,via:CTaskSimpleCarCloseDoorFromInside,via:CTaskSimpleCarCloseDoorFromOutside,via:CTaskSimpleCarGetIn,via:CTaskSimpleCarGetOut,via:CTaskSimpleCarJumpOut,via:CTaskSimpleCarOpenDoorFromOutside,via:CTaskSimpleCarOpenLockedDoorFromOutside,via:CTaskSimpleCarShuffle,via:CTaskSimpleCarSlowBeDraggedOut,via:CTaskSimpleCarSlowDragPedOut,via:CTaskSimpleHitHead,via:CTaskSimpleNM,via:CTaskSimpleSay,via:CTaskSimpleSetCharIgnoreWeaponRangeFlag,via:CTaskSimpleShakeFist,via:CTaskSimpleSmashCarWindow,via:CTaskSimpleTogglePedThreatScanner moved from siblings:CTaskComplex,CTaskSimple).
    pub f_14_bool: u8,
    /// Unknown bytes (0x15..0x18).
    pub _pad_0015: [u8; 0x3],
    /// f_18_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexBackOff,via:CTaskComplexCarDriveMission,via:CTaskComplexDriveFireTruck,via:CTaskComplexDriveToPoint,via:CTaskComplexFleeAnyMeans,via:CTaskComplexFollowPatrolRoute,via:CTaskComplexInjuredOnGround,via:CTaskComplexInvestigateDeadPed,via:CTaskComplexLeaveAnyCar,via:CTaskComplexPlayerInCover,via:CTaskComplexPlayerPlaceCarBomb,via:CTaskComplexSearchForPedInCar,via:CTaskComplexSeekCover,via:CTaskComplexTurnToFaceEntityOrCoord,via:CTaskSimpleAnim,via:CTaskSimpleArrestPed,via:CTaskSimpleBeHit,via:CTaskSimpleDead,via:CTaskSimpleWaitUntilPedIsOutCar moved from siblings:CTaskComplex,CTaskSimple).
    pub f_18_bool: u8,
    /// f_19_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexFleeAnyMeans,via:CTaskComplexLeaveAnyCar,via:CTaskComplexPlayerInCover,via:CTaskSimpleAnim,via:CTaskSimpleBeHit moved from siblings:CTaskComplex,CTaskSimple).
    pub f_19_bool: u8,
    /// Unknown bytes (0x1a..0x1c).
    pub _pad_001a: [u8; 0x2],
    /// f_1C_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexAvoidPlayerTargetting,via:CTaskComplexCarDriveMission,via:CTaskComplexCombatSubtask,via:CTaskComplexDriveToPoint,via:CTaskComplexGoToCarDoorAndStandStill,via:CTaskComplexMedicTreatInjuredPed,via:CTaskComplexVehicleSubtask,via:CTaskSimpleCarGetOut,via:CTaskSimpleCarSetPedInVehicle,via:CTaskSimpleCarSetPedOut,via:CTaskSimpleDuck,via:CTaskSimpleNewGangDriveBy,via:CTaskSimplePause,via:CTaskSimplePauseSystemTimer,via:CTaskSimplePlayerBeArrested,via:CTaskSimpleRunAnim,via:CTaskSimpleRunDictAnim,via:CTaskSimpleRunScriptAnim,via:CTaskSimpleRunTimedAnim moved from siblings:CTaskComplex,CTaskSimple).
    pub f_1c_bool: u8,
    /// f_1D_u8 (confidence: high, kind: bool/int8, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatSubtask,via:CTaskSimpleCarGetOut,via:CTaskSimpleCarSetPedInVehicle moved from siblings:CTaskComplex,CTaskSimple).
    pub f_1d_u8: u8,
    /// Unknown bytes (0x1e..0x20).
    pub _pad_001e: [u8; 0x2],
    /// f_20_bool (confidence: high, kind: bool/int8, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarDriveMission,via:CTaskComplexCarDriveTimed,via:CTaskComplexDriveToPoint,via:CTaskComplexLeaveCarAndWander,via:CTaskComplexScenario,via:CTaskComplexStealCar,via:CTaskComplexUseMobilePhone,via:CTaskSimpleGetUp,via:CTaskSimpleMeleeActionResult,via:CTaskSimpleMovePlayer,via:CTaskSimpleNM,via:CTaskSimplePickUpObject,via:CTaskSimpleShovePed,via:CTaskSimpleStandUp moved from siblings:CTaskComplex,CTaskSimple).
    pub f_20_bool: u8,
    /// f_21_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexScenario,via:CTaskComplexStealCar,via:CTaskSimpleCarSetPedOut moved from siblings:CTaskComplex,CTaskSimple).
    pub f_21_bool: u8,
    /// Unknown bytes (0x22..0x24).
    pub _pad_0022: [u8; 0x2],
    /// f_24_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarDriveBasic,via:CTaskComplexDriveWanderForTime,via:CTaskComplexFollowPedFootsteps,via:CTaskComplexInjuredOnGround,via:CTaskComplexNM,via:CTaskComplexPlayerDrive,via:CTaskComplexScreamInCarThenLeave,via:CTaskComplexSeatedScenario,via:CTaskComplexStationaryScenario,via:CTaskComplexUseSequence,via:CTaskComplexWaitForMyCarToStop,via:CTaskComplexWaitForSteppingOut,via:CTaskComplexWaitForTime,via:CTaskComplexWaitTillItsOkToStop,via:CTaskSimpleCarCloseDoorFromOutside,via:CTaskSimpleOpenDoor,via:CTaskSimpleRunAnim,via:CTaskSimpleRunDictAnim,via:CTaskSimpleRunScriptAnim,via:CTaskSimpleRunTimedAnim,via:CTaskSimpleSay,via:CTaskSimpleThrowGrenadeFromVehicle moved from siblings:CTaskComplex,CTaskSimple).
    pub f_24_bool: u8,
    /// f_25_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexSeatedScenario,via:CTaskComplexStationaryScenario,via:CTaskSimpleSay moved from siblings:CTaskComplex,CTaskSimple).
    pub f_25_bool: u8,
    /// Unknown bytes (0x26..0x28).
    pub _pad_0026: [u8; 0x2],
    /// f_28_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarDriveBasic,via:CTaskComplexDie,via:CTaskComplexFollowLeaderInFormation,via:CTaskComplexInvestigateDeadPed,via:CTaskComplexMoveWaitForTraffic,via:CTaskComplexPlayerGun,via:CTaskComplexSeatedScenario,via:CTaskComplexSeekEntityAnyMeans<CEntitySeekPosCalculatorXYOffset>,via:CTaskComplexShockingEventFlee,via:CTaskComplexShockingEventWatch,via:CTaskComplexSitIdle,via:CTaskComplexUseEffect,via:CTaskComplexUseMobilePhone,via:CTaskSimpleMeleeActionResult,via:CTaskSimpleNMScriptControl moved from siblings:CTaskComplex,CTaskSimple).
    pub f_28_bool: u8,
    /// Unknown bytes (0x29..0x2c).
    pub _pad_0029: [u8; 0x3],
    /// f_2C_bool (confidence: high, kind: bool/int8, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatFireSubtask,via:CTaskComplexPlayerDrive,via:CTaskComplexPlayerOnFoot,via:CTaskSimpleCarGetIn,via:CTaskSimpleCarOpenDoorFromOutside,via:CTaskSimpleCarSlowDragPedOut,via:CTaskSimpleMoveAchieveHeading,via:CTaskSimpleMoveGoToPoint,via:CTaskSimpleMoveMeleeMovement,via:CTaskSimpleMoveStandStill,via:CTaskSimpleMoveTrackingEntity,via:CTaskSimplePutDownObject,via:CTaskSimpleRunAnim,via:CTaskSimpleRunScriptAnim,via:CTaskSimpleRunTimedAnim moved from siblings:CTaskComplex,CTaskSimple).
    pub f_2c_bool: u8,
    /// Unknown bytes (0x2d..0x2e).
    pub _pad_002d: [u8; 0x1],
    /// f_2E_u16 (confidence: medium, kind: u16, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarReactToVehicleCollisionGetOut,via:CTaskComplexSmartFleeEntity,via:CTaskComplexUseMobilePhone,via:CTaskSimpleCarCloseDoorFromInside moved from siblings:CTaskComplex,CTaskSimple).
    pub f_2e_u16: u16,
    /// f_30_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexAimAndThrowProjectile,via:CTaskComplexAvoidPlayerTargetting,via:CTaskComplexGangDriveby,via:CTaskComplexGangHasslePed,via:CTaskComplexMedicDriver,via:CTaskComplexMedicPassenger,via:CTaskComplexMobileChatScenario,via:CTaskComplexShockingEventFlee,via:CTaskComplexShockingEventGoto,via:CTaskComplexShockingEventHurryAway,via:CTaskComplexShockingEventWatch,via:CTaskComplexTrackEntity,via:CTaskComplexVehicleSubtask,via:CTaskComplexWaitForSteppingOut,via:CTaskSimpleCarCloseDoorFromOutside,via:CTaskSimpleMoveMeleeMovement,via:CTaskSimpleMoveSwim,via:CTaskSimplePlayAnimAndSlideOutOfCover,via:CTaskSimplePlayerAimProjectile,via:CTaskSimpleRunAnim,via:CTaskSimpleRunScriptAnim,via:CTaskSimpleRunTimedAnim moved from siblings:CTaskComplex,CTaskSimple).
    pub f_30_bool: u8,
    /// Unknown bytes (0x31..0x34).
    pub _pad_0031: [u8; 0x3],
    /// f_34_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarDriveMission,via:CTaskComplexCombatClosestTargetInArea,via:CTaskComplexGun,via:CTaskComplexLeaveCarAndFlee,via:CTaskComplexMoveWaitForTraffic,via:CTaskComplexNewUseCover,via:CTaskComplexSearchForPedInCar,via:CTaskComplexSearchForPedOnFoot,via:CTaskComplexSearchWander,via:CTaskComplexShockingEventFlee,via:CTaskComplexShockingEventGoto,via:CTaskComplexShockingEventHurryAway,via:CTaskComplexShockingEventWatch,via:CTaskComplexWanderMedic,via:CTaskComplexWanderStandard,via:CTaskSimpleCreateCarAndGetIn,via:CTaskSimpleDie,via:CTaskSimpleNMRollUpAndRelax,via:CTaskSimpleRunScriptAnim,via:CTaskSimpleRunTimedAnim,via:CTaskSimpleShunt,via:CTaskSimpleStartCar moved from siblings:CTaskComplex,CTaskSimple).
    pub f_34_bool: u8,
    /// Unknown bytes (0x35..0x38).
    pub _pad_0035: [u8; 0x3],
    /// f_38_u16 (confidence: high, kind: u16, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarDriveMission,via:CTaskComplexDriveToPoint,via:CTaskComplexGun,via:CTaskComplexMoveGoToPointStandStillAchieveHeading,via:CTaskComplexMoveWander,via:CTaskComplexMove_StepAwayFromCollisionObjects,via:CTaskComplexShockingEventFlee,via:CTaskComplexShockingEventGoto,via:CTaskComplexShockingEventHurryAway,via:CTaskComplexShockingEventWatch,via:CTaskComplexSmartFleeEntity,via:CTaskSimpleDuck,via:CTaskSimpleMoveStandStill,via:CTaskSimpleRunTimedAnim moved from siblings:CTaskComplex,CTaskSimple).
    pub f_38_u16: u16,
    /// Unknown bytes (0x3a..0x3e).
    pub _pad_003a: [u8; 0x4],
    /// f_3E_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCombatInvestigateSubtask,via:CTaskComplexSitIdle,via:CTaskComplexStationaryScenario,via:CTaskComplexWalkWithPedScenario,via:CTaskComplexWanderStandard,via:CTaskSimpleBlendFromNM,via:CTaskSimpleNMBalance,via:CTaskSimpleNMHighFall,via:CTaskSimpleNMJumpRollFromRoadVehicle,via:CTaskSimpleNMOnFire moved from siblings:CTaskComplex,CTaskSimple).
    pub f_3e_bool: u8,
    /// Unknown bytes (0x3f..0x40).
    pub _pad_003f: [u8; 0x1],
    /// f_40_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatSeekCoverSubtask,via:CTaskComplexGangDriveby,via:CTaskComplexGetOffBoat,via:CTaskComplexGetOutOfWater,via:CTaskComplexMedicTreatInjuredPed,via:CTaskComplexMelee,via:CTaskComplexNewExitVehicle,via:CTaskComplexNewGetInVehicle,via:CTaskComplexPlayerOnFoot,via:CTaskComplexSeekCoverShooting,via:CTaskComplexTrackEntity,via:CTaskSimpleFireGun,via:CTaskSimpleNMFlinch,via:CTaskSimpleNMHighFall,via:CTaskSimpleNMJumpRollFromRoadVehicle,via:CTaskSimpleNMOnFire,via:CTaskSimpleStandStill moved from siblings:CTaskComplex,CTaskSimple).
    pub f_40_bool: u8,
    /// Unknown bytes (0x41..0x44).
    pub _pad_0041: [u8; 0x3],
    /// f_44_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarReactToVehicleCollisionGetOut,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCombatPersueInCarSubtask,via:CTaskComplexEvasiveStep,via:CTaskComplexNewGetInVehicle,via:CTaskComplexPlayerDrive,via:CTaskComplexReact,via:CTaskComplexSlideIntoCover,via:CTaskComplexUseLadderOnRoute,via:CTaskSimpleFireGun,via:CTaskSimpleMoveGoToPoint,via:CTaskSimpleMoveSlideToCoord,via:CTaskSimpleMoveStandStill,via:CTaskSimpleStandStill moved from siblings:CTaskComplex,CTaskSimple).
    pub f_44_bool: u8,
    /// f_45_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatFireSubtask,via:CTaskComplexEvasiveStep,via:CTaskComplexUseLadderOnRoute,via:CTaskSimpleStandStill moved from siblings:CTaskComplex,CTaskSimple).
    pub f_45_bool: u8,
    /// Unknown bytes (0x46..0x48).
    pub _pad_0046: [u8; 0x2],
    /// f_48_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCarDrive,via:CTaskComplexCombatInvestigateSubtask,via:CTaskComplexFleeAndDive,via:CTaskComplexGoToCarDoorAndStandStill,via:CTaskComplexGoToPointAiming,via:CTaskComplexMedicPassenger,via:CTaskComplexMobileMakeCall,via:CTaskComplexReact,via:CTaskComplexShockingEventFlee,via:CTaskComplexShockingEventGoto,via:CTaskComplexShockingEventHurryAway,via:CTaskComplexShockingEventWatch,via:CTaskComplexThrowProjectile,via:CTaskSimpleNMExplosion,via:CTaskSimpleThrowProjectile moved from siblings:CTaskComplex,CTaskSimple).
    pub f_48_bool: u8,
    /// Unknown bytes (0x49..0x50).
    pub _pad_0049: [u8; 0x7],
    /// f_50_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCop,via:CTaskComplexDie,via:CTaskComplexMedicTreatInjuredPed,via:CTaskComplexMoveWander,via:CTaskComplexNewUseCover,via:CTaskComplexPlayerOnFoot,via:CTaskComplexSmartFleeEntity,via:CTaskSimpleMoveMeleeMovement,via:CTaskSimpleMoveSlideToCoord,via:CTaskSimpleMoveTrackingEntity,via:CTaskSimpleNMBrace,via:CTaskSimpleNMOnFire,via:CTaskSimpleNMShot,via:CTaskSimpleReloadGun moved from siblings:CTaskComplex,CTaskSimple).
    pub f_50_bool: u8,
    /// Unknown bytes (0x51..0x54).
    pub _pad_0051: [u8; 0x3],
    /// f_54_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexDriveToPoint,via:CTaskComplexGoToCarDoorAndStandStill,via:CTaskComplexMoveAroundCoverPoints,via:CTaskComplexMoveGoToPointRelativeToEntityAndStandStill,via:CTaskComplexUseClimbOnRoute,via:CTaskComplexUseDropDownOnRoute,via:CTaskSimpleMoveMeleeMovement,via:CTaskSimpleMoveSlideToCoord,via:CTaskSimpleMoveTrackingEntity,via:CTaskSimpleNMBalance,via:CTaskSimpleNMBrace,via:CTaskSimpleNMShot,via:CTaskSimpleNMSit,via:CTaskSimplePlayAnimAndSlideIntoCover,via:CTaskSimplePlayAnimAndSlideOutOfCover moved from siblings:CTaskComplex,CTaskSimple).
    pub f_54_bool: u8,
    /// Unknown bytes (0x55..0x58).
    pub _pad_0055: [u8; 0x3],
    /// f_58_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatFlankSubtask,via:CTaskComplexNewExitVehicle,via:CTaskComplexNewGetInVehicle,via:CTaskComplexStationaryScenario,via:CTaskSimpleAimGun,via:CTaskSimpleClimb,via:CTaskSimpleClimbLadder,via:CTaskSimpleFireGun,via:CTaskSimpleMeleeActionResult,via:CTaskSimpleMoveMeleeMovement,via:CTaskSimpleMoveTrackingEntity,via:CTaskSimpleNMBrace,via:CTaskSimpleNMShot,via:CTaskSimpleNMSit,via:CTaskSimpleNewGangDriveBy,via:CTaskSimplePlayAnimAndSlideIntoCover,via:CTaskSimplePlayRandomAmbients,via:CTaskSimpleReloadGun,via:CTaskSimpleSitDown moved from siblings:CTaskComplex,CTaskSimple).
    pub f_58_bool: u8,
    /// Unknown bytes (0x59..0x5c).
    pub _pad_0059: [u8; 0x3],
    /// f_5C_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCombatRetreatSubtask,via:CTaskComplexJump,via:CTaskComplexMoveCrossRoadAtTrafficLights,via:CTaskComplexMoveGetOntoMainNavMesh,via:CTaskComplexMoveWander,via:CTaskComplexSeekCover,via:CTaskSimpleMeleeActionResult,via:CTaskSimplePlayRandomAmbients moved from siblings:CTaskComplex,CTaskSimple).
    pub f_5c_bool: u8,
    /// f_5D_u8 (confidence: high, kind: bool/int8, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexShockingEventHurryAway,via:CTaskSimpleMeleeActionResult,via:CTaskSimplePlayRandomAmbients moved from siblings:CTaskComplex,CTaskSimple).
    pub f_5d_u8: u8,
    /// Unknown bytes (0x5e..0x60).
    pub _pad_005e: [u8; 0x2],
    /// f_60_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCombatSeekCoverSubtask,via:CTaskComplexMoveWaitForTraffic,via:CTaskSimpleNMSit moved from siblings:CTaskComplex,CTaskSimple).
    pub f_60_bool: u8,
    /// Unknown bytes (0x61..0x7c).
    pub _pad_0061: [u8; 0x1b],
    /// f_7C_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexGun,via:CTaskComplexMoveAvoidOtherPedWhileWandering,via:CTaskComplexMoveBeInFormation,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexShockingEventGoto,via:CTaskComplexWalkRoundEntity,via:CTaskSimpleAimGun,via:CTaskSimpleClimb,via:CTaskSimpleFireGun,via:CTaskSimpleMeleeActionResult,via:CTaskSimplePlayRandomAmbients moved from siblings:CTaskComplex,CTaskSimple).
    pub f_7c_bool: u8,
    /// Unknown bytes (0x7d..0x80).
    pub _pad_007d: [u8; 0x3],
    /// f_80_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatFlankSubtask,via:CTaskComplexMoveAvoidOtherPedWhileWandering,via:CTaskComplexMoveBeInFormation,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexSitDownThenIdleThenStandUp,via:CTaskSimpleFireGun,via:CTaskSimpleMoveSwim moved from siblings:CTaskComplex,CTaskSimple).
    pub f_80_bool: u8,
    /// Unknown bytes (0x81..0x89).
    pub _pad_0081: [u8; 0x8],
    /// f_89_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatFlankSubtask,via:CTaskComplexMoveAvoidOtherPedWhileWandering,via:CTaskComplexSitDownThenIdleThenStandUp,via:CTaskComplexStandGuard,via:CTaskComplexWanderCriminal,via:CTaskSimpleClimb moved from siblings:CTaskComplex,CTaskSimple).
    pub f_89_bool: u8,
    /// Unknown bytes (0x8a..0x8c).
    pub _pad_008a: [u8; 0x2],
    /// f_8C_bool (confidence: high, kind: bool, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexStandGuard,via:CTaskSimplePlayRandomAmbients moved from siblings:CTaskComplex,CTaskSimple).
    pub f_8c_bool: u8,
    /// Unknown bytes (0x8d..0xa8).
    pub _pad_008d: [u8; 0x1b],
    /// f_A8_u16 (confidence: high, kind: u16, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexMoveBeInFormation,via:CTaskSimpleAimGun,via:CTaskSimpleClimbLadder,via:CTaskSimpleFireGun,via:CTaskSimpleMoveGoToPoint,via:CTaskSimpleRunNamedAnim moved from siblings:CTaskComplex,CTaskSimple).
    pub f_a8_u16: u16,
    /// Unknown bytes (0xaa..0xcc).
    pub _pad_00aa: [u8; 0x22],
    /// f_CC_u16 (confidence: high, kind: u16, lanes: c-tasks-a,c-tasks-b,via:CTaskComplexMoveFollowNavMeshRoute,via:CTaskSimpleClimbLadder moved from siblings:CTaskComplex,CTaskSimple).
    pub f_cc_u16: u16,
    /// Unknown bytes (0xce..0x2ec).
    pub _pad_00ce: [u8; 0x21e],
    /// task_id (confidence: high, kind: u16, lanes: n-08).
    pub task_id: u16,
    /// task_status (confidence: high, kind: u8, lanes: n-08).
    pub task_status: u8,
    /// Unknown trailing bytes (0x2ef..0x2f0).
    pub _pad_end: [u8; 0x1],
}
assert_size!(CTask, 0x2f0); // merged size 0x2ef rounded to 4
assert_offset!(CTask, vfptr, 0x0);
assert_offset!(CTask, task_type_id, 0x4);
assert_offset!(CTask, flags_incl_priority_bits, 0x8);
assert_offset!(CTask, next_sub_task_link, 0xc);
assert_offset!(CTask, field_10, 0x10);
assert_offset!(CTask, f_14_bool, 0x14);
assert_offset!(CTask, f_18_bool, 0x18);
assert_offset!(CTask, f_19_bool, 0x19);
assert_offset!(CTask, f_1c_bool, 0x1c);
assert_offset!(CTask, f_1d_u8, 0x1d);
assert_offset!(CTask, f_20_bool, 0x20);
assert_offset!(CTask, f_21_bool, 0x21);
assert_offset!(CTask, f_24_bool, 0x24);
assert_offset!(CTask, f_25_bool, 0x25);
assert_offset!(CTask, f_28_bool, 0x28);
assert_offset!(CTask, f_2c_bool, 0x2c);
assert_offset!(CTask, f_2e_u16, 0x2e);
assert_offset!(CTask, f_30_bool, 0x30);
assert_offset!(CTask, f_34_bool, 0x34);
assert_offset!(CTask, f_38_u16, 0x38);
assert_offset!(CTask, f_3e_bool, 0x3e);
assert_offset!(CTask, f_40_bool, 0x40);
assert_offset!(CTask, f_44_bool, 0x44);
assert_offset!(CTask, f_45_bool, 0x45);
assert_offset!(CTask, f_48_bool, 0x48);
assert_offset!(CTask, f_50_bool, 0x50);
assert_offset!(CTask, f_54_bool, 0x54);
assert_offset!(CTask, f_58_bool, 0x58);
assert_offset!(CTask, f_5c_bool, 0x5c);
assert_offset!(CTask, f_5d_u8, 0x5d);
assert_offset!(CTask, f_60_bool, 0x60);
assert_offset!(CTask, f_7c_bool, 0x7c);
assert_offset!(CTask, f_80_bool, 0x80);
assert_offset!(CTask, f_89_bool, 0x89);
assert_offset!(CTask, f_8c_bool, 0x8c);
assert_offset!(CTask, f_a8_u16, 0xa8);
assert_offset!(CTask, f_cc_u16, 0xcc);
assert_offset!(CTask, task_id, 0x2ec);
assert_offset!(CTask, task_status, 0x2ee);

/// Merged layout for `CTaskComplex`.
///
/// Size: 0x14e6 (low). Bases: CTask@0x0.
/// Lanes: c-tasks-a, via:CTaskComplexArrestPed, via:CTaskComplexArrestedAIPedAndDriveAway, via:CTaskComplexAvoidPlayerTargetting, via:CTaskComplexCarDrive, via:CTaskComplexCarDriveBasic, via:CTaskComplexCarDriveMission, via:CTaskComplexCarDriveTimed, via:CTaskComplexCarReactToVehicleCollisionGetOut, via:CTaskComplexCarSetTempAction, via:CTaskComplexChatScenario, via:CTaskComplexClearVehicleSeat, via:CTaskComplexClimbIntoVehicle, via:CTaskComplexCloseVehicleDoor, via:CTaskComplexCombat, via:CTaskComplexCombatAdvanceSubtask, via:CTaskComplexCombatBustPed, via:CTaskComplexCombatChargeSubtask, via:CTaskComplexCombatFireSubtask, via:CTaskComplexCombatInvestigateSubtask, via:CTaskComplexCombatPersueInCarSubtask, via:CTaskComplexCombatPullFromCarSubtask, via:CTaskComplexCombatRetreatSubtask, via:CTaskComplexCombatSeekCoverSubtask, via:CTaskComplexControlMovement, via:CTaskComplexCop, via:CTaskComplexCopHelicopter, via:CTaskComplexDie, via:CTaskComplexDriveFireTruck, via:CTaskComplexDrivePointRoute, via:CTaskComplexDriveWanderForTime, via:CTaskComplexDrivingScenario, via:CTaskComplexEnterAnyCarAsDriver, via:CTaskComplexEvasiveStep, via:CTaskComplexFleeAnyMeans, via:CTaskComplexFleeShooting, via:CTaskComplexFollowLeaderInFormation, via:CTaskComplexFollowPatrolRoute, via:CTaskComplexFollowPedFootsteps, via:CTaskComplexGetOffBoat, via:CTaskComplexGoToCarDoorAndStandStill, via:CTaskComplexGoToPointAnyMeans, via:CTaskComplexGun, via:CTaskComplexHelicopterStrafe, via:CTaskComplexInvestigateDeadPed, via:CTaskComplexJump, via:CTaskComplexLeaveAnyCar, via:CTaskComplexLeaveCarAndFlee, via:CTaskComplexMedicDriver, via:CTaskComplexMedicPassenger, via:CTaskComplexMedicTreatInjuredPed, via:CTaskComplexMedicWandering, via:CTaskComplexMelee, via:CTaskComplexMobileChatScenario, via:CTaskComplexMobileMakeCall, via:CTaskComplexMoveAroundCoverPoints, via:CTaskComplexMoveAvoidOtherPedWhileWandering, via:CTaskComplexMoveBeInFormation, via:CTaskComplexMoveBetweenPointsScenario, via:CTaskComplexMoveCrossRoadAtTrafficLights, via:CTaskComplexMoveCrowdAroundLocation, via:CTaskComplexMoveFollowNavMeshRoute, via:CTaskComplexMoveGetOntoMainNavMesh, via:CTaskComplexMoveGoToPointRelativeToEntityAndStandStill, via:CTaskComplexMoveGoToPointStandStillAchieveHeading, via:CTaskComplexMoveGoToShelterAndWait, via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>, via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>, via:CTaskComplexMoveWaitForTraffic, via:CTaskComplexMoveWander, via:CTaskComplexMove_StepAwayFromCollisionObjects, via:CTaskComplexNM, via:CTaskComplexNewExitVehicle, via:CTaskComplexNewGetInVehicle, via:CTaskComplexOpenVehicleDoor, via:CTaskComplexPickUpAndCarryObject, via:CTaskComplexPickUpObject, via:CTaskComplexPlayerDrive, via:CTaskComplexPlayerPlaceCarBomb, via:CTaskComplexPlayerSettingsTask, via:CTaskComplexReact, via:CTaskComplexReactToRanPedOver, via:CTaskComplexRevive, via:CTaskComplexScreamInCarThenLeave, via:CTaskComplexSearchForPedInCar, via:CTaskComplexSearchForPedOnFoot, via:CTaskComplexSearchWander, via:CTaskComplexSeekCover, via:CTaskComplexSequence, via:CTaskComplexSetAndGuardArea, via:CTaskComplexShockingEventFlee, via:CTaskComplexShockingEventGoto, via:CTaskComplexShockingEventHurryAway, via:CTaskComplexShockingEventWatch, via:CTaskComplexSitDownThenIdleThenStandUp, via:CTaskComplexSitIdle, via:CTaskComplexSlideIntoCover, via:CTaskComplexSmartFleeEntity, via:CTaskComplexSmartFleePoint, via:CTaskComplexStandGuard, via:CTaskComplexStationaryScenario, via:CTaskComplexStealCar, via:CTaskComplexStuckInAir, via:CTaskComplexThrowProjectile, via:CTaskComplexTreatAccident, via:CTaskComplexUseClimbOnRoute, via:CTaskComplexUseDropDownOnRoute, via:CTaskComplexUseMobilePhone, via:CTaskComplexUseSequence, via:CTaskComplexUseWaterCannon, via:CTaskComplexWaitForSteppingOut, via:CTaskComplexWaitTillItsOkToStop, via:CTaskComplexWalkWithPedScenario, via:CTaskComplexWander, via:CTaskComplexWanderCriminal, via:CTaskComplexWanderStandard.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplex {
    /// Unknown bytes (0x0..0x1a).
    pub _pad_0000: [u8; 0x1a],
    /// f_1A_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexLeaveAnyCar,via:CTaskComplexSearchForPedInCar,via:CTaskComplexSeekCover moved from siblings:CTaskComplexLeaveAnyCar,CTaskComplexSearchForPedInCar,CTaskComplexSeekCover).
    pub f_1a_bool: u8,
    /// Unknown bytes (0x1b..0x32).
    pub _pad_001b: [u8; 0x17],
    /// f_32_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexMedicDriver,via:CTaskComplexMobileChatScenario moved from siblings:CTaskComplexMedicDriver,CTaskComplexScenario).
    pub f_32_bool: u8,
    /// Unknown bytes (0x33..0x36).
    pub _pad_0033: [u8; 0x3],
    /// f_36_u16 (confidence: high, kind: u16, lanes: c-tasks-a,via:CTaskComplexCarDriveMission,via:CTaskComplexGun moved from siblings:CTaskComplexCarDrive,CTaskComplexGun).
    pub f_36_u16: u16,
    /// Unknown bytes (0x38..0x3a).
    pub _pad_0038: [u8; 0x2],
    /// f_3A_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexCarDrive,via:CTaskComplexClimbIntoVehicle moved from siblings:CTaskComplexCarDrive,CTaskComplexVehicleSubtask).
    pub f_3a_bool: u8,
    /// f_3B_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexCarDrive,via:CTaskComplexClimbIntoVehicle,via:CTaskComplexDrivePointRoute moved from siblings:CTaskComplexCarDrive,CTaskComplexDrivePointRoute,CTaskComplexVehicleSubtask).
    pub f_3b_bool: u8,
    /// f_3C_u16 (confidence: high, kind: u16, lanes: c-tasks-a,via:CTaskComplexCarDriveMission,via:CTaskComplexDriveToPoint,via:CTaskComplexGoToCarDoorAndStandStill,via:CTaskComplexGun,via:CTaskComplexJump,via:CTaskComplexMedicDriver,via:CTaskComplexMedicPassenger,via:CTaskComplexMedicWandering,via:CTaskComplexMoveGoToShelterAndWait,via:CTaskComplexWanderStandard moved from siblings:CTaskComplexGoToCarDoorAndStandStill,CTaskComplexGun,CTaskComplexJump,CTaskComplexMedicDriver,CTaskComplexMedicPassenger,CTaskComplexMedicWandering,CTaskComplexMove,CTaskComplexWander).
    pub f_3c_u16: u16,
    /// Unknown bytes (0x3e..0x41).
    pub _pad_003e: [u8; 0x3],
    /// f_41_bool (confidence: medium, kind: bool, lanes: c-tasks-a,via:CTaskComplexGetOffBoat,via:CTaskComplexPlayerSettingsTask moved from siblings:CTaskComplexGetOffBoat,CTaskComplexPlayerSettingsTask).
    pub f_41_bool: u8,
    /// Unknown bytes (0x42..0x46).
    pub _pad_0042: [u8; 0x4],
    /// f_46_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexEvasiveStep,via:CTaskComplexReact,via:CTaskComplexShockingEventFlee,via:CTaskComplexShockingEventGoto,via:CTaskComplexShockingEventHurryAway,via:CTaskComplexShockingEventWatch moved from siblings:CTaskComplexEvasiveStep,CTaskComplexReact,CTaskComplexShockingEvent).
    pub f_46_bool: u8,
    /// Unknown bytes (0x47..0x4c).
    pub _pad_0047: [u8; 0x5],
    /// f_4C_u16 (confidence: high, kind: u16, lanes: c-tasks-a,via:CTaskComplexCombatPersueInCarSubtask,via:CTaskComplexGoToPointAnyMeans,via:CTaskComplexMelee,via:CTaskComplexMoveGetOntoMainNavMesh moved from siblings:CTaskComplexCombatSubtask,CTaskComplexGoToPointAnyMeans,CTaskComplexMelee,CTaskComplexMove).
    pub f_4c_u16: u16,
    /// Unknown bytes (0x4e..0x55).
    pub _pad_004e: [u8; 0x7],
    /// f_55_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexMoveAroundCoverPoints,via:CTaskComplexUseClimbOnRoute,via:CTaskComplexUseDropDownOnRoute moved from siblings:CTaskComplexMove,CTaskComplexUseClimbOnRoute,CTaskComplexUseDropDownOnRoute).
    pub f_55_bool: u8,
    /// Unknown bytes (0x56..0x59).
    pub _pad_0056: [u8; 0x3],
    /// f_59_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexMoveCrowdAroundLocation,via:CTaskComplexStationaryScenario moved from siblings:CTaskComplexMove,CTaskComplexScenario).
    pub f_59_bool: u8,
    /// Unknown bytes (0x5a..0x64).
    pub _pad_005a: [u8; 0xa],
    /// f_64_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexCombat,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCombatFlankSubtask,via:CTaskComplexCombatRetreatSubtask,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>,via:CTaskComplexSequence,via:CTaskComplexSmartFleePoint,via:CTaskComplexStationaryScenario,via:CTaskComplexUseSequence moved from siblings:CTaskComplexCombat,CTaskComplexMove,CTaskComplexScenario,CTaskComplexSequence,CTaskComplexSmartFleePoint,CTaskComplexUseSequence).
    pub f_64_bool: u8,
    /// f_65_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>,via:CTaskComplexSmartFleePoint,via:CTaskComplexStationaryScenario moved from siblings:CTaskComplexMove,CTaskComplexScenario,CTaskComplexSmartFleePoint).
    pub f_65_bool: u8,
    /// Unknown bytes (0x66..0x6c).
    pub _pad_0066: [u8; 0x6],
    /// f_6C_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexGun,via:CTaskComplexStationaryScenario moved from siblings:CTaskComplexGun,CTaskComplexScenario).
    pub f_6c_bool: u8,
    /// Unknown bytes (0x6d..0x70).
    pub _pad_006d: [u8; 0x3],
    /// f_70_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCombatFlankSubtask,via:CTaskComplexCombatInvestigateSubtask,via:CTaskComplexFleeShooting,via:CTaskComplexGun,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>,via:CTaskComplexSetAndGuardArea,via:CTaskComplexShockingEventFlee,via:CTaskComplexShockingEventGoto,via:CTaskComplexShockingEventHurryAway,via:CTaskComplexShockingEventWatch moved from siblings:CTaskComplexCombatSubtask,CTaskComplexGun,CTaskComplexMove,CTaskComplexSetAndGuardArea,CTaskComplexShockingEvent,CTaskComplexSmartFleeEntity).
    pub f_70_bool: u8,
    /// Unknown bytes (0x71..0x78).
    pub _pad_0071: [u8; 0x7],
    /// f_78_u16 (confidence: high, kind: u16, lanes: c-tasks-a,via:CTaskComplexMoveAvoidOtherPedWhileWandering,via:CTaskComplexShockingEventGoto,via:CTaskComplexShockingEventHurryAway,via:CTaskComplexShockingEventWatch,via:CTaskComplexStandGuard moved from siblings:CTaskComplexMove,CTaskComplexStandGuard).
    pub f_78_u16: u16,
    /// Unknown bytes (0x7a..0x86).
    pub _pad_007a: [u8; 0xc],
    /// f_86_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatFlankSubtask,via:CTaskComplexNewExitVehicle,via:CTaskComplexSitDownThenIdleThenStandUp,via:CTaskComplexWanderCop,via:CTaskComplexWanderCriminal moved from siblings:CTaskComplexSitDownThenIdleThenStandUp,CTaskComplexVehicleSubtask).
    pub f_86_bool: u8,
    /// Unknown bytes (0x87..0x8a).
    pub _pad_0087: [u8; 0x3],
    /// f_8A_bool (confidence: medium, kind: bool, lanes: c-tasks-a,via:CTaskComplexNewExitVehicle,via:CTaskComplexWaitForSteppingOut moved from siblings:CTaskComplexVehicleSubtask,CTaskComplexWaitForCondition).
    pub f_8a_bool: u8,
    /// Unknown bytes (0x8b..0x94).
    pub _pad_008b: [u8; 0x9],
    /// f_94_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexMoveWander,via:CTaskComplexSeekCover,via:CTaskComplexStandGuard,via:CTaskComplexWalkRoundEntity moved from siblings:CTaskComplexSeekCover,CTaskComplexStandGuard).
    pub f_94_bool: u8,
    /// Unknown bytes (0x95..0x9a).
    pub _pad_0095: [u8; 0x5],
    /// f_9A_u8 (confidence: high, kind: u8, lanes: c-tasks-a,via:CTaskComplexMoveWander,via:CTaskComplexSearchForPedOnFoot,via:CTaskComplexSearchWander,via:CTaskComplexWalkRoundEntity moved from siblings:CTaskComplexSearchForPedOnFoot,CTaskComplexSearchWander).
    pub f_9a_u8: u8,
    /// Unknown bytes (0x9b..0xb8).
    pub _pad_009b: [u8; 0x1d],
    /// f_B8_int32 (confidence: high, kind: i32, lanes: c-tasks-a,via:CTaskComplexMoveFollowNavMeshRoute,via:CTaskComplexSeekCover moved from siblings:CTaskComplexMove,CTaskComplexSeekCover).
    pub f_b8_int32: u32,
    /// Unknown bytes (0xbc..0xf4).
    pub _pad_00bc: [u8; 0x38],
    /// f_F4_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexEnterAnyCarAsDriver,via:CTaskComplexFleeAnyMeans,via:CTaskComplexGoToPointAnyMeans moved from siblings:CTaskComplexEnterAnyCarAsDriver,CTaskComplexFleeAnyMeans,CTaskComplexGoToPointAnyMeans).
    pub f_f4_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xf8..0x1bc).
    pub _pad_00f8: [u8; 0xc4],
    /// f_1BC_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexPickUpAndCarryObject,via:CTaskComplexPickUpObject,via:CTaskComplexSitIdle moved from siblings:CTaskComplexPickUpAndCarryObject,CTaskComplexPickUpObject,CTaskComplexSitIdle).
    pub f_1bc_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x1c0..0x1e2).
    pub _pad_01c0: [u8; 0x22],
    /// f_1E2_bool (confidence: medium, kind: bool, lanes: c-tasks-a,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>,via:CTaskComplexPickUpAndCarryObject,via:CTaskComplexPickUpObject moved from siblings:CTaskComplexMove,CTaskComplexPickUpAndCarryObject,CTaskComplexPickUpObject).
    pub f_1e2_bool: u8,
    /// Unknown bytes (0x1e3..0x211).
    pub _pad_01e3: [u8; 0x2e],
    /// f_211_u8 (confidence: medium, kind: u8, lanes: c-tasks-a,via:CTaskComplexArrestPed,via:CTaskComplexOpenVehicleDoor moved from siblings:CTaskComplexArrestPed,CTaskComplexVehicleSubtask).
    pub f_211_u8: u8,
    /// Unknown bytes (0x212..0x218).
    pub _pad_0212: [u8; 0x6],
    /// f_218_u8 (confidence: medium, kind: u8, lanes: c-tasks-a,via:CTaskComplexCombatPullFromCarSubtask,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexReact moved from siblings:CTaskComplexCombatSubtask,CTaskComplexMove,CTaskComplexReact).
    pub f_218_u8: u8,
    /// f_219_u8 (confidence: medium, kind: u8, lanes: c-tasks-a,via:CTaskComplexCarDrive,via:CTaskComplexCombatPullFromCarSubtask,via:CTaskComplexMelee,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexOpenVehicleDoor,via:CTaskComplexReact moved from siblings:CTaskComplexCarDrive,CTaskComplexCombatSubtask,CTaskComplexMelee,CTaskComplexMove,CTaskComplexReact,CTaskComplexVehicleSubtask).
    pub f_219_u8: u8,
    /// Unknown bytes (0x21a..0x21c).
    pub _pad_021a: [u8; 0x2],
    /// f_21C_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a,via:CTaskComplexCombat,via:CTaskComplexCombatRetreatSubtask,via:CTaskComplexMoveCrossRoadAtTrafficLights,via:CTaskComplexPlayerSettingsTask,via:CTaskComplexStealCar moved from siblings:CTaskComplexCombat,CTaskComplexCombatSubtask,CTaskComplexMove,CTaskComplexPlayerSettingsTask,CTaskComplexStealCar).
    pub f_21c_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x220..0x224).
    pub _pad_0220: [u8; 0x4],
    /// f_224_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a,via:CTaskComplexArrestPed,via:CTaskComplexChatScenario,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCop,via:CTaskComplexFollowPedFootsteps,via:CTaskComplexGoToPointAnyMeans,via:CTaskComplexNewExitVehicle,via:CTaskComplexPlayerSettingsTask,via:CTaskComplexReactToRanPedOver,via:CTaskComplexSearchForPedOnFoot,via:CTaskComplexStuckInAir,via:CTaskComplexTreatAccident,via:CTaskComplexWaitForSteppingOut,via:CTaskComplexWalkWithPedScenario moved from siblings:CTaskComplexArrestPed,CTaskComplexCombatSubtask,CTaskComplexCop,CTaskComplexFollowPedFootsteps,CTaskComplexGoToPointAnyMeans,CTaskComplexPlayerSettingsTask,CTaskComplexReactToRanPedOver,CTaskComplexScenario,CTaskComplexSearchForPedOnFoot,CTaskComplexStuckInAir,CTaskComplexTreatAccident,CTaskComplexVehicleSubtask,CTaskComplexWaitForCondition).
    pub f_224_u32_or_pointer: Ptr32<u8>,
    /// f_228_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexAvoidPlayerTargetting,via:CTaskComplexCombatFireSubtask,via:CTaskComplexCombatPersueInCarSubtask,via:CTaskComplexCombatPullFromCarSubtask,via:CTaskComplexCopHelicopter,via:CTaskComplexStuckInAir moved from siblings:CTaskComplexAvoidPlayerTargetting,CTaskComplexCombatSubtask,CTaskComplexCopHelicopter,CTaskComplexStuckInAir).
    pub f_228_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x22c..0x26c).
    pub _pad_022c: [u8; 0x40],
    /// f_26C_u8 (confidence: high, kind: u8, lanes: c-tasks-a,via:CTaskComplexArrestPed,via:CTaskComplexCarReactToVehicleCollisionGetOut,via:CTaskComplexCombat,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatBustPed,via:CTaskComplexCombatInvestigateSubtask,via:CTaskComplexCombatPersueInCarSubtask,via:CTaskComplexDriveFireTruck,via:CTaskComplexFleeAnyMeans,via:CTaskComplexFollowLeaderInFormation,via:CTaskComplexHelicopterStrafe,via:CTaskComplexSmartFleeEntity,via:CTaskComplexTreatAccident,via:CTaskComplexUseWaterCannon,via:CTaskComplexWander moved from siblings:CTaskComplexArrestPed,CTaskComplexCarReactToVehicleCollisionGetOut,CTaskComplexCombat,CTaskComplexCombatSubtask,CTaskComplexDriveFireTruck,CTaskComplexFleeAnyMeans,CTaskComplexFollowLeaderInFormation,CTaskComplexHelicopterStrafe,CTaskComplexSmartFleeEntity,CTaskComplexTreatAccident,CTaskComplexUseWaterCannon,CTaskComplexWander).
    pub f_26c_u8: u8,
    /// Unknown bytes (0x26d..0x2b0).
    pub _pad_026d: [u8; 0x43],
    /// f_2B0_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a,via:CTaskComplexArrestPed,via:CTaskComplexCombatFireSubtask,via:CTaskComplexMobileChatScenario,via:CTaskComplexPlayerSettingsTask,via:CTaskComplexThrowProjectile moved from siblings:CTaskComplexArrestPed,CTaskComplexCombatSubtask,CTaskComplexPlayerSettingsTask,CTaskComplexScenario,CTaskComplexThrowProjectile).
    pub f_2b0_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x2b4..0x2e0).
    pub _pad_02b4: [u8; 0x2c],
    /// f_2E0_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexCombatRetreatSubtask,via:CTaskComplexDrivingScenario,via:CTaskComplexFollowLeaderInFormation,via:CTaskComplexMoveBetweenPointsScenario,via:CTaskComplexStationaryScenario moved from siblings:CTaskComplexCombatSubtask,CTaskComplexFollowLeaderInFormation,CTaskComplexScenario).
    pub f_2e0_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x2e4..0x570).
    pub _pad_02e4: [u8; 0x28c],
    /// f_570_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a,via:CTaskComplexCombatChargeSubtask,via:CTaskComplexRevive,via:CTaskComplexShockingEventGoto moved from siblings:CTaskComplexCombatSubtask,CTaskComplexRevive,CTaskComplexShockingEvent).
    pub f_570_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x574..0xa60).
    pub _pad_0574: [u8; 0x4ec],
    /// f_A60_u8 (confidence: high, kind: u8, lanes: c-tasks-a,via:CTaskComplexCombat,via:CTaskComplexMoveBetweenPointsScenario moved from siblings:CTaskComplexCombat,CTaskComplexScenario).
    pub f_a60_u8: u8,
    /// Unknown bytes (0xa61..0xb30).
    pub _pad_0a61: [u8; 0xcf],
    /// f_B30_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a,via:CTaskComplexArrestPed,via:CTaskComplexArrestedAIPedAndDriveAway,via:CTaskComplexCarSetTempAction,via:CTaskComplexCombat,via:CTaskComplexCombatBustPed,via:CTaskComplexCombatInvestigateSubtask,via:CTaskComplexCombatPersueInCarSubtask,via:CTaskComplexCombatPullFromCarSubtask,via:CTaskComplexCombatRetreatSubtask,via:CTaskComplexDriveWanderForTime,via:CTaskComplexFleeAnyMeans,via:CTaskComplexFollowLeaderInFormation,via:CTaskComplexHelicopterStrafe,via:CTaskComplexLeaveAnyCar,via:CTaskComplexStealCar,via:CTaskComplexUseWaterCannon,via:CTaskComplexWaitTillItsOkToStop moved from siblings:CTaskComplexArrestPed,CTaskComplexArrestedAIPedAndDriveAway,CTaskComplexCarSetTempAction,CTaskComplexCombat,CTaskComplexCombatSubtask,CTaskComplexFleeAnyMeans,CTaskComplexFollowLeaderInFormation,CTaskComplexHelicopterStrafe,CTaskComplexLeaveAnyCar,CTaskComplexStealCar,CTaskComplexUseWaterCannon,CTaskComplexWaitForCondition).
    pub f_b30_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xb34..0xd68).
    pub _pad_0b34: [u8; 0x234],
    /// f_D68_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexCombatAdvanceSubtask,via:CTaskComplexCombatSeekCoverSubtask,via:CTaskComplexSlideIntoCover moved from siblings:CTaskComplexCombatSubtask,CTaskComplexSlideIntoCover).
    pub f_d68_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xd6c..0xf1c).
    pub _pad_0d6c: [u8; 0x1b0],
    /// f_F1C_u8 (confidence: medium, kind: u8, lanes: c-tasks-a,via:CTaskComplexLeaveAnyCar,via:CTaskComplexMoveFollowNavMeshRoute,via:CTaskComplexSearchForPedInCar moved from siblings:CTaskComplexLeaveAnyCar,CTaskComplexMove,CTaskComplexSearchForPedInCar).
    pub f_f1c_u8: u8,
    /// Unknown bytes (0xf1d..0xf50).
    pub _pad_0f1d: [u8; 0x33],
    /// f_F50_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexCarDriveBasic,via:CTaskComplexClearVehicleSeat,via:CTaskComplexCombatPersueInCarSubtask,via:CTaskComplexCop,via:CTaskComplexFleeAnyMeans,via:CTaskComplexGoToCarDoorAndStandStill,via:CTaskComplexPlayerDrive,via:CTaskComplexScreamInCarThenLeave,via:CTaskComplexSearchForPedOnFoot,via:CTaskComplexUseMobilePhone moved from siblings:CTaskComplexCarDriveBasic,CTaskComplexCombatSubtask,CTaskComplexCop,CTaskComplexFleeAnyMeans,CTaskComplexGoToCarDoorAndStandStill,CTaskComplexPlayerDrive,CTaskComplexScreamInCarThenLeave,CTaskComplexSearchForPedOnFoot,CTaskComplexUseMobilePhone,CTaskComplexVehicleSubtask).
    pub f_f50_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xf54..0x1070).
    pub _pad_0f54: [u8; 0x11c],
    /// f_1070_u8 (confidence: medium, kind: u8, lanes: c-tasks-a,via:CTaskComplexFollowLeaderInFormation,via:CTaskComplexPlayerDrive moved from siblings:CTaskComplexFollowLeaderInFormation,CTaskComplexPlayerDrive).
    pub f_1070_u8: u8,
    /// Unknown bytes (0x1071..0x1300).
    pub _pad_1071: [u8; 0x28f],
    /// f_1300_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexLeaveCarAndFlee,via:CTaskComplexNewGetInVehicle moved from siblings:CTaskComplexLeaveCarAndFlee,CTaskComplexVehicleSubtask).
    pub f_1300_u32_or_pointer: Ptr32<u8>,
    /// f_1304_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a,via:CTaskComplexArrestPed,via:CTaskComplexClearVehicleSeat,via:CTaskComplexClimbIntoVehicle,via:CTaskComplexCloseVehicleDoor,via:CTaskComplexCombatPersueInCarSubtask,via:CTaskComplexControlMovement,via:CTaskComplexCopHelicopter,via:CTaskComplexDie,via:CTaskComplexGoToCarDoorAndStandStill,via:CTaskComplexMoveBeInFormation,via:CTaskComplexNewExitVehicle,via:CTaskComplexNewGetInVehicle,via:CTaskComplexOpenVehicleDoor,via:CTaskComplexPlayerDrive,via:CTaskComplexWaitForSteppingOut moved from siblings:CTaskComplexArrestPed,CTaskComplexCombatSubtask,CTaskComplexControlMovement,CTaskComplexCopHelicopter,CTaskComplexDie,CTaskComplexGoToCarDoorAndStandStill,CTaskComplexMove,CTaskComplexPlayerDrive,CTaskComplexVehicleSubtask,CTaskComplexWaitForCondition).
    pub f_1304_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x1308..0x14e4).
    pub _pad_1308: [u8; 0x1dc],
    /// f_14E4_u8 (confidence: medium, kind: u8, lanes: c-tasks-a,via:CTaskComplexNewGetInVehicle,via:CTaskComplexPlayerDrive moved from siblings:CTaskComplexPlayerDrive,CTaskComplexVehicleSubtask).
    pub f_14e4_u8: u8,
    /// f_14E5_u8 (confidence: medium, kind: u8, lanes: c-tasks-a,via:CTaskComplexNewExitVehicle,via:CTaskComplexPlayerDrive,via:CTaskComplexWaitForSteppingOut moved from siblings:CTaskComplexPlayerDrive,CTaskComplexVehicleSubtask,CTaskComplexWaitForCondition).
    pub f_14e5_u8: u8,
    /// Unknown trailing bytes (0x14e6..0x14e8).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CTaskComplex, 0x14e8); // merged size 0x14e6 rounded to 4
assert_offset!(CTaskComplex, f_1a_bool, 0x1a);
assert_offset!(CTaskComplex, f_32_bool, 0x32);
assert_offset!(CTaskComplex, f_36_u16, 0x36);
assert_offset!(CTaskComplex, f_3a_bool, 0x3a);
assert_offset!(CTaskComplex, f_3b_bool, 0x3b);
assert_offset!(CTaskComplex, f_3c_u16, 0x3c);
assert_offset!(CTaskComplex, f_41_bool, 0x41);
assert_offset!(CTaskComplex, f_46_bool, 0x46);
assert_offset!(CTaskComplex, f_4c_u16, 0x4c);
assert_offset!(CTaskComplex, f_55_bool, 0x55);
assert_offset!(CTaskComplex, f_59_bool, 0x59);
assert_offset!(CTaskComplex, f_64_bool, 0x64);
assert_offset!(CTaskComplex, f_65_bool, 0x65);
assert_offset!(CTaskComplex, f_6c_bool, 0x6c);
assert_offset!(CTaskComplex, f_70_bool, 0x70);
assert_offset!(CTaskComplex, f_78_u16, 0x78);
assert_offset!(CTaskComplex, f_86_bool, 0x86);
assert_offset!(CTaskComplex, f_8a_bool, 0x8a);
assert_offset!(CTaskComplex, f_94_bool, 0x94);
assert_offset!(CTaskComplex, f_9a_u8, 0x9a);
assert_offset!(CTaskComplex, f_b8_int32, 0xb8);
assert_offset!(CTaskComplex, f_f4_u32_or_pointer, 0xf4);
assert_offset!(CTaskComplex, f_1bc_u32_or_pointer, 0x1bc);
assert_offset!(CTaskComplex, f_1e2_bool, 0x1e2);
assert_offset!(CTaskComplex, f_211_u8, 0x211);
assert_offset!(CTaskComplex, f_218_u8, 0x218);
assert_offset!(CTaskComplex, f_219_u8, 0x219);
assert_offset!(CTaskComplex, f_21c_u32_or_pointer, 0x21c);
assert_offset!(CTaskComplex, f_224_u32_or_pointer, 0x224);
assert_offset!(CTaskComplex, f_228_u32_or_pointer, 0x228);
assert_offset!(CTaskComplex, f_26c_u8, 0x26c);
assert_offset!(CTaskComplex, f_2b0_u32_or_pointer, 0x2b0);
assert_offset!(CTaskComplex, f_2e0_u32_or_pointer, 0x2e0);
assert_offset!(CTaskComplex, f_570_u32_or_pointer, 0x570);
assert_offset!(CTaskComplex, f_a60_u8, 0xa60);
assert_offset!(CTaskComplex, f_b30_u32_or_pointer, 0xb30);
assert_offset!(CTaskComplex, f_d68_u32_or_pointer, 0xd68);
assert_offset!(CTaskComplex, f_f1c_u8, 0xf1c);
assert_offset!(CTaskComplex, f_f50_u32_or_pointer, 0xf50);
assert_offset!(CTaskComplex, f_1070_u8, 0x1070);
assert_offset!(CTaskComplex, f_1300_u32_or_pointer, 0x1300);
assert_offset!(CTaskComplex, f_1304_u32_or_pointer, 0x1304);
assert_offset!(CTaskComplex, f_14e4_u8, 0x14e4);
assert_offset!(CTaskComplex, f_14e5_u8, 0x14e5);

/// Merged layout for `CTaskComplexCarDrive`.
///
/// Size: 0xe3c (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a, via:CTaskComplexCarDriveMission, via:CTaskComplexCarDriveWander, via:CTaskComplexDriveToPoint.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexCarDrive {
    /// Unknown bytes (0x0..0x3f).
    pub _pad_0000: [u8; 0x3f],
    /// f_3F_bool (confidence: high, kind: bool, lanes: c-tasks-a,via:CTaskComplexCarDriveMission,via:CTaskComplexDriveToPoint moved from siblings:CTaskComplexCarDriveMission,CTaskComplexDriveToPoint).
    pub f_3f_bool: u8,
    /// Unknown bytes (0x40..0x49).
    pub _pad_0040: [u8; 0x9],
    /// f_49_bool (confidence: medium, kind: bool, lanes: c-tasks-a).
    pub f_49_bool: u8,
    /// Unknown bytes (0x4a..0xe38).
    pub _pad_004a: [u8; 0xdee],
    /// f_E38_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a,via:CTaskComplexCarDriveMission,via:CTaskComplexCarDriveWander,via:CTaskComplexDriveToPoint moved from siblings:CTaskComplexCarDriveMission,CTaskComplexCarDriveWander,CTaskComplexDriveToPoint).
    pub f_e38_u32_or_pointer: Ptr32<u8>,
}
assert_size!(CTaskComplexCarDrive, 0xe3c); // merged size 0xe3c rounded to 4
assert_offset!(CTaskComplexCarDrive, f_3f_bool, 0x3f);
assert_offset!(CTaskComplexCarDrive, f_49_bool, 0x49);
assert_offset!(CTaskComplexCarDrive, f_e38_u32_or_pointer, 0xe38);

/// Merged layout for `CTaskComplexClimbLadder`.
///
/// Size: 0x290 (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexClimbLadder {
    /// Unknown bytes (0x0..0x74).
    pub _pad_0000: [u8; 0x74],
    /// f_74_bool (confidence: medium, kind: bool, lanes: c-tasks-a).
    pub f_74_bool: u8,
    /// Unknown bytes (0x75..0x76).
    pub _pad_0075: [u8; 0x1],
    /// f_76_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_76_u16: u16,
    /// Unknown bytes (0x78..0x90).
    pub _pad_0078: [u8; 0x18],
    /// f_90_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_90_int32: u32,
    /// Unknown bytes (0x94..0xd8).
    pub _pad_0094: [u8; 0x44],
    /// f_D8_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_d8_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xdc..0x28c).
    pub _pad_00dc: [u8; 0x1b0],
    /// f_28C_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_28c_u32_or_pointer: Ptr32<u8>,
}
assert_size!(CTaskComplexClimbLadder, 0x290); // merged size 0x290 rounded to 4
assert_offset!(CTaskComplexClimbLadder, f_74_bool, 0x74);
assert_offset!(CTaskComplexClimbLadder, f_76_u16, 0x76);
assert_offset!(CTaskComplexClimbLadder, f_90_int32, 0x90);
assert_offset!(CTaskComplexClimbLadder, f_d8_u32_or_pointer, 0xd8);
assert_offset!(CTaskComplexClimbLadder, f_28c_u32_or_pointer, 0x28c);

/// Merged layout for `CTaskComplexGoToCarDoorAndStandStill`.
///
/// Size: 0x1308 (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexGoToCarDoorAndStandStill {
    /// Unknown bytes (0x0..0x68).
    pub _pad_0000: [u8; 0x68],
    /// f_68_int32 (confidence: low, kind: i32, lanes: c-tasks-a).
    pub f_68_int32: u32,
    /// Unknown bytes (0x6c..0x74).
    pub _pad_006c: [u8; 0x8],
    /// f_74_u8 (confidence: high, kind: u8, lanes: c-tasks-a).
    pub f_74_u8: u8,
    /// f_75_bool (confidence: medium, kind: bool, lanes: c-tasks-a).
    pub f_75_bool: u8,
    /// f_76_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_76_bool: u8,
    /// f_77_bool (confidence: medium, kind: bool, lanes: c-tasks-a).
    pub f_77_bool: u8,
    /// Unknown bytes (0x78..0xaa8).
    pub _pad_0078: [u8; 0xa30],
    /// f_AA8_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_aa8_u32_or_pointer: Ptr32<u8>,
    /// Unknown trailing bytes (0xaac..0x1308).
    pub _pad_end: [u8; 0x85c],
}
assert_size!(CTaskComplexGoToCarDoorAndStandStill, 0x1308); // merged size 0x1308 rounded to 4
assert_offset!(CTaskComplexGoToCarDoorAndStandStill, f_68_int32, 0x68);
assert_offset!(CTaskComplexGoToCarDoorAndStandStill, f_74_u8, 0x74);
assert_offset!(CTaskComplexGoToCarDoorAndStandStill, f_75_bool, 0x75);
assert_offset!(CTaskComplexGoToCarDoorAndStandStill, f_76_bool, 0x76);
assert_offset!(CTaskComplexGoToCarDoorAndStandStill, f_77_bool, 0x77);
assert_offset!(
    CTaskComplexGoToCarDoorAndStandStill,
    f_aa8_u32_or_pointer,
    0xaa8
);

/// Merged layout for `CTaskComplexGun`.
///
/// Size: 0x3e8 (medium). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexGun {
    /// Unknown bytes (0x0..0x4e).
    pub _pad_0000: [u8; 0x4e],
    /// f_4E_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_4e_u16: u16,
    /// Unknown bytes (0x50..0x52).
    pub _pad_0050: [u8; 0x2],
    /// f_52_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_52_u16: u16,
    /// Unknown bytes (0x54..0x56).
    pub _pad_0054: [u8; 0x2],
    /// f_56_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_56_u16: u16,
    /// Unknown bytes (0x58..0x5a).
    pub _pad_0058: [u8; 0x2],
    /// f_5A_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_5a_u16: u16,
    /// Unknown bytes (0x5c..0x68).
    pub _pad_005c: [u8; 0xc],
    /// f_68_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_68_int32: u32,
    /// Unknown bytes (0x6c..0x6d).
    pub _pad_006c: [u8; 0x1],
    /// f_6D_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_6d_u8: u8,
    /// f_6E_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_6e_u8: u8,
    /// f_6F_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_6f_u8: u8,
    /// Unknown bytes (0x70..0x71).
    pub _pad_0070: [u8; 0x1],
    /// f_71_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_71_u8: u8,
    /// f_72_bool (confidence: low, kind: bool, lanes: c-tasks-a).
    pub f_72_bool: u8,
    /// f_73_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_73_bool: u8,
    /// f_74_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_74_u32: u32,
    /// Unknown bytes (0x78..0x7d).
    pub _pad_0078: [u8; 0x5],
    /// f_7D_u8 (confidence: low, kind: u8, lanes: c-tasks-a).
    pub f_7d_u8: u8,
    /// Unknown bytes (0x7e..0x90).
    pub _pad_007e: [u8; 0x12],
    /// f_90_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a).
    pub f_90_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x94..0x9c).
    pub _pad_0094: [u8; 0x8],
    /// f_9C_u32 (confidence: medium, kind: u32, lanes: c-tasks-a).
    pub f_9c_u32: u32,
    /// f_A0_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_a0_int32: u32,
    /// f_A4_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_a4_int32: u32,
    /// Unknown trailing bytes (0xa8..0x3e8).
    pub _pad_end: [u8; 0x340],
}
assert_size!(CTaskComplexGun, 0x3e8); // merged size 0x3e8 rounded to 4
assert_offset!(CTaskComplexGun, f_4e_u16, 0x4e);
assert_offset!(CTaskComplexGun, f_52_u16, 0x52);
assert_offset!(CTaskComplexGun, f_56_u16, 0x56);
assert_offset!(CTaskComplexGun, f_5a_u16, 0x5a);
assert_offset!(CTaskComplexGun, f_68_int32, 0x68);
assert_offset!(CTaskComplexGun, f_6d_u8, 0x6d);
assert_offset!(CTaskComplexGun, f_6e_u8, 0x6e);
assert_offset!(CTaskComplexGun, f_6f_u8, 0x6f);
assert_offset!(CTaskComplexGun, f_71_u8, 0x71);
assert_offset!(CTaskComplexGun, f_72_bool, 0x72);
assert_offset!(CTaskComplexGun, f_73_bool, 0x73);
assert_offset!(CTaskComplexGun, f_74_u32, 0x74);
assert_offset!(CTaskComplexGun, f_7d_u8, 0x7d);
assert_offset!(CTaskComplexGun, f_90_u32_or_pointer, 0x90);
assert_offset!(CTaskComplexGun, f_9c_u32, 0x9c);
assert_offset!(CTaskComplexGun, f_a0_int32, 0xa0);
assert_offset!(CTaskComplexGun, f_a4_int32, 0xa4);

/// Merged layout for `CTaskComplexMelee`.
///
/// Size: 0x21a (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexMelee {
    /// Unknown bytes (0x0..0xb4).
    pub _pad_0000: [u8; 0xb4],
    /// f_B4_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a).
    pub f_b4_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xb8..0xc0).
    pub _pad_00b8: [u8; 0x8],
    /// f_C0_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a).
    pub f_c0_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xc4..0xd0).
    pub _pad_00c4: [u8; 0xc],
    /// f_D0_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_d0_int32: u32,
    /// f_D4_int32 (confidence: low, kind: i32, lanes: c-tasks-a).
    pub f_d4_int32: u32,
    /// f_D8_int32 (confidence: low, kind: i32, lanes: c-tasks-a).
    pub f_d8_int32: u32,
    /// f_DC_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_dc_int32: u32,
    /// f_E0_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_e0_int32: u32,
    /// f_E4_u16 (confidence: low, kind: u16, lanes: c-tasks-a).
    pub f_e4_u16: u16,
    /// Unknown bytes (0xe6..0xe8).
    pub _pad_00e6: [u8; 0x2],
    /// f_E8_int32 (confidence: low, kind: i32, lanes: c-tasks-a).
    pub f_e8_int32: u32,
    /// f_EC_int32 (confidence: low, kind: i32, lanes: c-tasks-a).
    pub f_ec_int32: u32,
    /// f_F0_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_f0_bool: u8,
    /// f_F1_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_f1_u8: u8,
    /// Unknown trailing bytes (0xf2..0x21c).
    pub _pad_end: [u8; 0x12a],
}
assert_size!(CTaskComplexMelee, 0x21c); // merged size 0x21a rounded to 4
assert_offset!(CTaskComplexMelee, f_b4_u32_or_pointer, 0xb4);
assert_offset!(CTaskComplexMelee, f_c0_u32_or_pointer, 0xc0);
assert_offset!(CTaskComplexMelee, f_d0_int32, 0xd0);
assert_offset!(CTaskComplexMelee, f_d4_int32, 0xd4);
assert_offset!(CTaskComplexMelee, f_d8_int32, 0xd8);
assert_offset!(CTaskComplexMelee, f_dc_int32, 0xdc);
assert_offset!(CTaskComplexMelee, f_e0_int32, 0xe0);
assert_offset!(CTaskComplexMelee, f_e4_u16, 0xe4);
assert_offset!(CTaskComplexMelee, f_e8_int32, 0xe8);
assert_offset!(CTaskComplexMelee, f_ec_int32, 0xec);
assert_offset!(CTaskComplexMelee, f_f0_bool, 0xf0);
assert_offset!(CTaskComplexMelee, f_f1_u8, 0xf1);

/// Merged layout for `CTaskComplexMove`.
///
/// Size: 0xa0 (low). Bases: CTaskComplex@0x0, CTaskMoveInterface@0x14.
/// Lanes: c-tasks-a, via:CTaskComplexMoveAvoidOtherPedWhileWandering, via:CTaskComplexMoveBeInFormation, via:CTaskComplexMoveCrossRoadAtTrafficLights, via:CTaskComplexMoveFollowNavMeshRoute, via:CTaskComplexMoveGoToPointStandStillAchieveHeading, via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>, via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>, via:CTaskComplexMoveWander, via:CTaskComplexMove_StepAwayFromCollisionObjects, via:CTaskComplexWalkRoundEntity.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexMove {
    /// Unknown bytes (0x0..0x71).
    pub _pad_0000: [u8; 0x71],
    /// f_71_bool (confidence: medium, kind: bool, lanes: c-tasks-a,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard> moved from siblings:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>).
    pub f_71_bool: u8,
    /// Unknown bytes (0x72..0x74).
    pub _pad_0072: [u8; 0x2],
    /// f_74_int32 (confidence: high, kind: i32, lanes: c-tasks-a,via:CTaskComplexMoveAvoidOtherPedWhileWandering,via:CTaskComplexMoveCrossRoadAtTrafficLights,via:CTaskComplexMoveFollowNavMeshRoute,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,via:CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>,via:CTaskComplexWalkRoundEntity moved from siblings:CTaskComplexMoveAvoidOtherPedWhileWandering,CTaskComplexMoveCrossRoadAtTrafficLights,CTaskComplexMoveFollowNavMeshRoute,CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>,CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>,CTaskComplexWalkRoundEntity).
    pub f_74_int32: u32,
    /// Unknown bytes (0x78..0x84).
    pub _pad_0078: [u8; 0xc],
    /// f_84_u16 (confidence: high, kind: u16, lanes: c-tasks-a,via:CTaskComplexMoveAvoidOtherPedWhileWandering,via:CTaskComplexMoveBeInFormation moved from siblings:CTaskComplexMoveAvoidOtherPedWhileWandering,CTaskComplexMoveBeInFormation).
    pub f_84_u16: u16,
    /// Unknown bytes (0x86..0x9c).
    pub _pad_0086: [u8; 0x16],
    /// f_9C_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a,via:CTaskComplexMoveFollowNavMeshRoute,via:CTaskComplexWalkRoundEntity moved from siblings:CTaskComplexMoveFollowNavMeshRoute,CTaskComplexWalkRoundEntity).
    pub f_9c_u32_or_pointer: Ptr32<u8>,
}
assert_size!(CTaskComplexMove, 0xa0); // merged size 0xa0 rounded to 4
assert_offset!(CTaskComplexMove, f_71_bool, 0x71);
assert_offset!(CTaskComplexMove, f_74_int32, 0x74);
assert_offset!(CTaskComplexMove, f_84_u16, 0x84);
assert_offset!(CTaskComplexMove, f_9c_u32_or_pointer, 0x9c);

/// Merged layout for `CTaskComplexMoveBeInFormation`.
///
/// Size: 0x1308 (low). Bases: CTaskComplexMove@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexMoveBeInFormation {
    /// Unknown bytes (0x0..0xa0).
    pub _pad_0000: [u8; 0xa0],
    /// f_A0_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_a0_int32: u32,
    /// f_A4_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_a4_int32: u32,
    /// Unknown bytes (0xa8..0xb4).
    pub _pad_00a8: [u8; 0xc],
    /// f_B4_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_b4_u32: u32,
    /// Unknown trailing bytes (0xb8..0x1308).
    pub _pad_end: [u8; 0x1250],
}
assert_size!(CTaskComplexMoveBeInFormation, 0x1308); // merged size 0x1308 rounded to 4
assert_offset!(CTaskComplexMoveBeInFormation, f_a0_int32, 0xa0);
assert_offset!(CTaskComplexMoveBeInFormation, f_a4_int32, 0xa4);
assert_offset!(CTaskComplexMoveBeInFormation, f_b4_u32, 0xb4);

/// Merged layout for `CTaskComplexMoveFollowNavMeshRoute`.
///
/// Size: 0xfa0 (low). Bases: CTaskComplexMove@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexMoveFollowNavMeshRoute {
    /// Unknown bytes (0x0..0x68).
    pub _pad_0000: [u8; 0x68],
    /// f_68_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_68_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x6c..0x98).
    pub _pad_006c: [u8; 0x2c],
    /// f_98_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_98_bool: u8,
    /// Unknown bytes (0x99..0xa0).
    pub _pad_0099: [u8; 0x7],
    /// f_A0_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_a0_int32: u32,
    /// f_A4_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_a4_int32: u32,
    /// Unknown bytes (0xa8..0xac).
    pub _pad_00a8: [u8; 0x4],
    /// f_AC_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_ac_int32: u32,
    /// f_B0_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_b0_int32: u32,
    /// f_B4_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_b4_u16: u16,
    /// Unknown bytes (0xb6..0xbc).
    pub _pad_00b6: [u8; 0x6],
    /// f_BC_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_bc_int32: u32,
    /// f_C0_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_c0_u16: u16,
    /// Unknown bytes (0xc2..0xc4).
    pub _pad_00c2: [u8; 0x2],
    /// f_C4_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_c4_int32: u32,
    /// f_C8_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_c8_int32: u32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// f_D0_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a).
    pub f_d0_u32_or_pointer: Ptr32<u8>,
    /// f_D4_u16 (confidence: low, kind: u16, lanes: c-tasks-a).
    pub f_d4_u16: u16,
    /// Unknown bytes (0xd6..0xd8).
    pub _pad_00d6: [u8; 0x2],
    /// f_D8_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_d8_u32: u32,
    /// f_DC_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_dc_u8: u8,
    /// Unknown bytes (0xdd..0xe0).
    pub _pad_00dd: [u8; 0x3],
    /// f_E0_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a).
    pub f_e0_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xe4..0x264).
    pub _pad_00e4: [u8; 0x180],
    /// f_264_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_264_int32: u32,
    /// Unknown trailing bytes (0x268..0xfa0).
    pub _pad_end: [u8; 0xd38],
}
assert_size!(CTaskComplexMoveFollowNavMeshRoute, 0xfa0); // merged size 0xfa0 rounded to 4
assert_offset!(
    CTaskComplexMoveFollowNavMeshRoute,
    f_68_u32_or_pointer,
    0x68
);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_98_bool, 0x98);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_a0_int32, 0xa0);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_a4_int32, 0xa4);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_ac_int32, 0xac);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_b0_int32, 0xb0);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_b4_u16, 0xb4);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_bc_int32, 0xbc);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_c0_u16, 0xc0);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_c4_int32, 0xc4);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_c8_int32, 0xc8);
assert_offset!(
    CTaskComplexMoveFollowNavMeshRoute,
    f_d0_u32_or_pointer,
    0xd0
);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_d4_u16, 0xd4);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_d8_u32, 0xd8);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_dc_u8, 0xdc);
assert_offset!(
    CTaskComplexMoveFollowNavMeshRoute,
    f_e0_u32_or_pointer,
    0xe0
);
assert_offset!(CTaskComplexMoveFollowNavMeshRoute, f_264_int32, 0x264);

/// Merged layout for `CTaskComplexMoveWander`.
///
/// Size: 0x126 (low). Bases: CTaskComplexMove@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexMoveWander {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// f_90_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_90_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x94..0xa0).
    pub _pad_0094: [u8; 0xc],
    /// f_A0_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a).
    pub f_a0_u32_or_pointer: Ptr32<u8>,
    /// f_A4_u32-or-pointer (confidence: medium, kind: pointer, lanes: c-tasks-a).
    pub f_a4_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xa8..0xb0).
    pub _pad_00a8: [u8; 0x8],
    /// f_B0_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_b0_u32: u32,
    /// Unknown trailing bytes (0xb4..0x128).
    pub _pad_end: [u8; 0x74],
}
assert_size!(CTaskComplexMoveWander, 0x128); // merged size 0x126 rounded to 4
assert_offset!(CTaskComplexMoveWander, f_90_u32_or_pointer, 0x90);
assert_offset!(CTaskComplexMoveWander, f_a0_u32_or_pointer, 0xa0);
assert_offset!(CTaskComplexMoveWander, f_a4_u32_or_pointer, 0xa4);
assert_offset!(CTaskComplexMoveWander, f_b0_u32, 0xb0);

/// Merged layout for `CTaskComplexPlayerOnFoot`.
///
/// Size: 0xa4 (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexPlayerOnFoot {
    /// Unknown bytes (0x0..0x68).
    pub _pad_0000: [u8; 0x68],
    /// f_68_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_68_int32: u32,
    /// Unknown bytes (0x6c..0x74).
    pub _pad_006c: [u8; 0x8],
    /// f_74_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_74_int32: u32,
    /// Unknown bytes (0x78..0x90).
    pub _pad_0078: [u8; 0x18],
    /// f_90_int32 (confidence: low, kind: i32, lanes: c-tasks-a).
    pub f_90_int32: u32,
    /// Unknown bytes (0x94..0xa0).
    pub _pad_0094: [u8; 0xc],
    /// f_A0_u32 (confidence: medium, kind: u32, lanes: c-tasks-a).
    pub f_a0_u32: u32,
}
assert_size!(CTaskComplexPlayerOnFoot, 0xa4); // merged size 0xa4 rounded to 4
assert_offset!(CTaskComplexPlayerOnFoot, f_68_int32, 0x68);
assert_offset!(CTaskComplexPlayerOnFoot, f_74_int32, 0x74);
assert_offset!(CTaskComplexPlayerOnFoot, f_90_int32, 0x90);
assert_offset!(CTaskComplexPlayerOnFoot, f_a0_u32, 0xa0);

/// Merged layout for `CTaskComplexSeekCover`.
///
/// Size: 0xc0 (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexSeekCover {
    /// Unknown bytes (0x0..0x74).
    pub _pad_0000: [u8; 0x74],
    /// f_74_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_74_u32: u32,
    /// Unknown bytes (0x78..0x90).
    pub _pad_0078: [u8; 0x18],
    /// f_90_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_90_u32: u32,
    /// Unknown bytes (0x94..0xa0).
    pub _pad_0094: [u8; 0xc],
    /// f_A0_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_a0_int32: u32,
    /// f_A4_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_a4_int32: u32,
    /// Unknown bytes (0xa8..0xac).
    pub _pad_00a8: [u8; 0x4],
    /// f_AC_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_ac_u32_or_pointer: Ptr32<u8>,
    /// f_B0_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_b0_u32_or_pointer: Ptr32<u8>,
    /// f_B4_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_b4_u32: u32,
    /// Unknown bytes (0xb8..0xbc).
    pub _pad_00b8: [u8; 0x4],
    /// f_BC_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_bc_int32: u32,
}
assert_size!(CTaskComplexSeekCover, 0xc0); // merged size 0xc0 rounded to 4
assert_offset!(CTaskComplexSeekCover, f_74_u32, 0x74);
assert_offset!(CTaskComplexSeekCover, f_90_u32, 0x90);
assert_offset!(CTaskComplexSeekCover, f_a0_int32, 0xa0);
assert_offset!(CTaskComplexSeekCover, f_a4_int32, 0xa4);
assert_offset!(CTaskComplexSeekCover, f_ac_u32_or_pointer, 0xac);
assert_offset!(CTaskComplexSeekCover, f_b0_u32_or_pointer, 0xb0);
assert_offset!(CTaskComplexSeekCover, f_b4_u32, 0xb4);
assert_offset!(CTaskComplexSeekCover, f_bc_int32, 0xbc);

/// Merged layout for `CTaskComplexSeperate`.
///
/// Size: 0xc0 (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexSeperate {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// f_90_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_90_int32: u32,
    /// Unknown bytes (0x94..0xa4).
    pub _pad_0094: [u8; 0x10],
    /// f_A4_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a).
    pub f_a4_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0xa8..0xac).
    pub _pad_00a8: [u8; 0x4],
    /// f_AC_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a).
    pub f_ac_u32_or_pointer: Ptr32<u8>,
    /// f_B0_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a).
    pub f_b0_u32_or_pointer: Ptr32<u8>,
    /// f_B4_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_b4_int32: u32,
    /// Unknown bytes (0xb8..0xbc).
    pub _pad_00b8: [u8; 0x4],
    /// f_BC_u32 (confidence: medium, kind: u32, lanes: c-tasks-a).
    pub f_bc_u32: u32,
}
assert_size!(CTaskComplexSeperate, 0xc0); // merged size 0xc0 rounded to 4
assert_offset!(CTaskComplexSeperate, f_90_int32, 0x90);
assert_offset!(CTaskComplexSeperate, f_a4_u32_or_pointer, 0xa4);
assert_offset!(CTaskComplexSeperate, f_ac_u32_or_pointer, 0xac);
assert_offset!(CTaskComplexSeperate, f_b0_u32_or_pointer, 0xb0);
assert_offset!(CTaskComplexSeperate, f_b4_int32, 0xb4);
assert_offset!(CTaskComplexSeperate, f_bc_u32, 0xbc);

/// Merged layout for `CTaskComplexSitDownThenIdleThenStandUp`.
///
/// Size: 0x8a (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexSitDownThenIdleThenStandUp {
    /// Unknown bytes (0x0..0x68).
    pub _pad_0000: [u8; 0x68],
    /// f_68_u32-or-pointer (confidence: high, kind: pointer, lanes: c-tasks-a).
    pub f_68_u32_or_pointer: Ptr32<u8>,
    /// Unknown bytes (0x6c..0x74).
    pub _pad_006c: [u8; 0x8],
    /// f_74_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_74_int32: u32,
    /// Unknown bytes (0x78..0x81).
    pub _pad_0078: [u8; 0x9],
    /// f_81_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_81_bool: u8,
    /// f_82_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_82_bool: u8,
    /// f_83_bool (confidence: medium, kind: bool, lanes: c-tasks-a).
    pub f_83_bool: u8,
    /// f_84_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_84_bool: u8,
    /// Unknown bytes (0x85..0x87).
    pub _pad_0085: [u8; 0x2],
    /// f_87_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_87_u8: u8,
    /// f_88_u8 (confidence: low, kind: u8, lanes: c-tasks-a).
    pub f_88_u8: u8,
    /// Unknown trailing bytes (0x89..0x8c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CTaskComplexSitDownThenIdleThenStandUp, 0x8c); // merged size 0x8a rounded to 4
assert_offset!(
    CTaskComplexSitDownThenIdleThenStandUp,
    f_68_u32_or_pointer,
    0x68
);
assert_offset!(CTaskComplexSitDownThenIdleThenStandUp, f_74_int32, 0x74);
assert_offset!(CTaskComplexSitDownThenIdleThenStandUp, f_81_bool, 0x81);
assert_offset!(CTaskComplexSitDownThenIdleThenStandUp, f_82_bool, 0x82);
assert_offset!(CTaskComplexSitDownThenIdleThenStandUp, f_83_bool, 0x83);
assert_offset!(CTaskComplexSitDownThenIdleThenStandUp, f_84_bool, 0x84);
assert_offset!(CTaskComplexSitDownThenIdleThenStandUp, f_87_u8, 0x87);
assert_offset!(CTaskComplexSitDownThenIdleThenStandUp, f_88_u8, 0x88);

/// Merged layout for `CTaskComplexStandGuard`.
///
/// Size: 0x95 (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexStandGuard {
    /// Unknown bytes (0x0..0x68).
    pub _pad_0000: [u8; 0x68],
    /// f_68_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_68_int32: u32,
    /// Unknown bytes (0x6c..0x74).
    pub _pad_006c: [u8; 0x8],
    /// f_74_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_74_u32: u32,
    /// Unknown bytes (0x78..0x90).
    pub _pad_0078: [u8; 0x18],
    /// f_90_u32 (confidence: high, kind: u32, lanes: c-tasks-a).
    pub f_90_u32: u32,
    /// Unknown trailing bytes (0x94..0x98).
    pub _pad_end: [u8; 0x4],
}
assert_size!(CTaskComplexStandGuard, 0x98); // merged size 0x95 rounded to 4
assert_offset!(CTaskComplexStandGuard, f_68_int32, 0x68);
assert_offset!(CTaskComplexStandGuard, f_74_u32, 0x74);
assert_offset!(CTaskComplexStandGuard, f_90_u32, 0x90);

/// Merged layout for `CTaskComplexUseMobilePhone`.
///
/// Size: 0x107c (low). Bases: CTaskComplex@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexUseMobilePhone {
    /// Unknown bytes (0x0..0x29).
    pub _pad_0000: [u8; 0x29],
    /// f_29_u8 (confidence: medium, kind: u8, lanes: c-tasks-a).
    pub f_29_u8: u8,
    /// f_2A_bool (confidence: high, kind: bool, lanes: c-tasks-a).
    pub f_2a_bool: u8,
    /// f_2B_bool (confidence: medium, kind: bool, lanes: c-tasks-a).
    pub f_2b_bool: u8,
    /// Unknown bytes (0x2c..0x1078).
    pub _pad_002c: [u8; 0x104c],
    /// f_1078_u32-or-pointer (confidence: low, kind: pointer, lanes: c-tasks-a).
    pub f_1078_u32_or_pointer: Ptr32<u8>,
}
assert_size!(CTaskComplexUseMobilePhone, 0x107c); // merged size 0x107c rounded to 4
assert_offset!(CTaskComplexUseMobilePhone, f_29_u8, 0x29);
assert_offset!(CTaskComplexUseMobilePhone, f_2a_bool, 0x2a);
assert_offset!(CTaskComplexUseMobilePhone, f_2b_bool, 0x2b);
assert_offset!(CTaskComplexUseMobilePhone, f_1078_u32_or_pointer, 0x1078);

/// Merged layout for `CTaskComplexWanderCop`.
///
/// Size: 0xa8 (low). Bases: CTaskComplexWander@0x0.
/// Lanes: c-tasks-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskComplexWanderCop {
    /// Unknown bytes (0x0..0x90).
    pub _pad_0000: [u8; 0x90],
    /// f_90_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_90_int32: u32,
    /// Unknown bytes (0x94..0x98).
    pub _pad_0094: [u8; 0x4],
    /// f_98_u16 (confidence: medium, kind: u16, lanes: c-tasks-a).
    pub f_98_u16: u16,
    /// Unknown bytes (0x9a..0x9c).
    pub _pad_009a: [u8; 0x2],
    /// f_9C_int32 (confidence: high, kind: i32, lanes: c-tasks-a).
    pub f_9c_int32: u32,
    /// f_A0_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_a0_int32: u32,
    /// f_A4_int32 (confidence: medium, kind: i32, lanes: c-tasks-a).
    pub f_a4_int32: u32,
}
assert_size!(CTaskComplexWanderCop, 0xa8); // merged size 0xa8 rounded to 4
assert_offset!(CTaskComplexWanderCop, f_90_int32, 0x90);
assert_offset!(CTaskComplexWanderCop, f_98_u16, 0x98);
assert_offset!(CTaskComplexWanderCop, f_9c_int32, 0x9c);
assert_offset!(CTaskComplexWanderCop, f_a0_int32, 0xa0);
assert_offset!(CTaskComplexWanderCop, f_a4_int32, 0xa4);

/// Merged layout for `CTaskInfo`.
///
/// Size: 0x1a (low). Bases: none.
/// Lanes: c-misc-a, c-misc-b, c-tasks-b, via:CSequenceTaskInfo, via:CTargetTaskInfo.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskInfo {
    /// vfptr (confidence: high, kind: pointer, lanes: c-entities,c-misc-a,c-misc-b,c-tasks-b,c-ui).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: pointer, lanes: c-misc-a,c-misc-b,c-tasks-b).
    pub field_4: Ptr32<u8>,
    /// field_8 (confidence: medium, kind: pointer, lanes: c-misc-a,c-tasks-b).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-misc-a,c-tasks-b).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_10: Ptr32<u8>,
    /// Unknown bytes (0x14..0x18).
    pub _pad_0014: [u8; 0x4],
    /// field_18 (confidence: high, kind: word?, lanes: c-misc-a,c-misc-b,c-tasks-b,c-ui,via:CSequenceTaskInfo,via:CTargetTaskInfo moved from siblings:CSequenceTaskInfo,CTargetTaskInfo).
    pub field_18: u16,
    /// Unknown trailing bytes (0x1a..0x1c).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CTaskInfo, 0x1c); // merged size 0x1a rounded to 4
assert_offset!(CTaskInfo, vfptr, 0x0);
assert_offset!(CTaskInfo, field_4, 0x4);
assert_offset!(CTaskInfo, field_8, 0x8);
assert_offset!(CTaskInfo, field_c, 0xc);
assert_offset!(CTaskInfo, field_10, 0x10);
assert_offset!(CTaskInfo, field_18, 0x18);

/// Merged layout for `CTaskInfoWithCloneTask`.
///
/// Size: 0x70 (low). Bases: CTaskInfo@0x0.
/// Lanes: c-misc-a, c-tasks-b, c-ui, via:CComplexGunTaskInfo, via:CComplexNewExitVehicleTaskInfo, via:CComplexNewUseCoverInfo, via:CScenarioTaskInfo, via:CSimpleMeleeActionResultTaskInfo, via:CSimpleNMBraceTaskInfo, via:CSimpleNMFlinchTaskInfo, via:CSimpleNMShotTaskInfo, via:CSitDownIdleThenStandTaskInfo, via:CSitIdleTaskInfo, via:CStatusAndTargetTaskInfo.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskInfoWithCloneTask {
    /// Unknown bytes (0x0..0x14).
    pub _pad_0000: [u8; 0x14],
    /// field_14 (confidence: high, kind: pointer, lanes: c-misc-a,c-tasks-b).
    pub field_14: Ptr32<u8>,
    /// Unknown bytes (0x18..0x1c).
    pub _pad_0018: [u8; 0x4],
    /// field_1c (confidence: high, kind: pointer, lanes: c-entities,c-misc-a,c-tasks-b,c-ui).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: high, kind: bool_or_byte, lanes: c-misc-a,c-ui,via:CComplexNewUseCoverInfo,via:CSimpleMeleeActionResultTaskInfo,via:CSitIdleTaskInfo moved from siblings:CComplexNewUseCoverInfo,CSimpleMeleeActionResultTaskInfo,CSitIdleTaskInfo).
    pub field_20: u8,
    /// Unknown bytes (0x21..0x24).
    pub _pad_0021: [u8; 0x3],
    /// field_24 (confidence: high, kind: bool_or_byte, lanes: c-entities,c-misc-a,via:CScenarioTaskInfo,via:CSimpleNMShotTaskInfo moved from siblings:CScenarioTaskInfo,CSimpleNMShotTaskInfo).
    pub field_24: u8,
    /// Unknown bytes (0x25..0x30).
    pub _pad_0025: [u8; 0xb],
    /// field_30 (confidence: high, kind: u32, lanes: c-misc-a,via:CComplexGunTaskInfo,via:CComplexNewExitVehicleTaskInfo,via:CScenarioTaskInfo,via:CSimpleNMFlinchTaskInfo,via:CSitDownIdleThenStandTaskInfo,via:CStatusAndTargetTaskInfo moved from siblings:CScenarioTaskInfo,CSimpleNMFlinchTaskInfo,CSitDownIdleThenStandTaskInfo,CStatusTaskInfo).
    pub field_30: u32,
    /// Unknown bytes (0x34..0x6c).
    pub _pad_0034: [u8; 0x38],
    /// field_6c (confidence: high, kind: u32, lanes: c-misc-a,c-ui,via:CScenarioTaskInfo,via:CSimpleNMBraceTaskInfo,via:CSimpleNMFlinchTaskInfo,via:CSimpleNMShotTaskInfo,via:CStatusAndTargetTaskInfo moved from siblings:CScenarioTaskInfo,CSimpleNMBraceTaskInfo,CSimpleNMFlinchTaskInfo,CSimpleNMShotTaskInfo,CStatusTaskInfo).
    pub field_6c: u32,
}
assert_size!(CTaskInfoWithCloneTask, 0x70); // merged size 0x70 rounded to 4
assert_offset!(CTaskInfoWithCloneTask, field_14, 0x14);
assert_offset!(CTaskInfoWithCloneTask, field_1c, 0x1c);
assert_offset!(CTaskInfoWithCloneTask, field_20, 0x20);
assert_offset!(CTaskInfoWithCloneTask, field_24, 0x24);
assert_offset!(CTaskInfoWithCloneTask, field_30, 0x30);
assert_offset!(CTaskInfoWithCloneTask, field_6c, 0x6c);

/// Merged layout for `CTaskMoveInterface`.
///
/// Size: 0x20 (low). Bases: none.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskMoveInterface {
    /// vfptr (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: float, lanes: c-tasks-b).
    pub field_4: f32,
    /// field_8 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_8: Ptr32<u8>,
    /// Unknown bytes (0xc..0x14).
    pub _pad_000c: [u8; 0x8],
    /// field_14 (confidence: high, kind: pointer, lanes: c-tasks-a,c-tasks-b).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: high, kind: float, lanes: c-tasks-a,c-tasks-b).
    pub field_18: f32,
    /// field_1c (confidence: high, kind: pointer, lanes: c-tasks-a,c-tasks-b).
    pub field_1c: Ptr32<u8>,
}
assert_size!(CTaskMoveInterface, 0x20); // merged size 0x20 rounded to 4
assert_offset!(CTaskMoveInterface, vfptr, 0x0);
assert_offset!(CTaskMoveInterface, field_4, 0x4);
assert_offset!(CTaskMoveInterface, field_8, 0x8);
assert_offset!(CTaskMoveInterface, field_14, 0x14);
assert_offset!(CTaskMoveInterface, field_18, 0x18);
assert_offset!(CTaskMoveInterface, field_1c, 0x1c);

/// Merged layout for `CTaskNode`.
///
/// Size: 0x1a (low). Bases: none.
/// Lanes: n-08.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskNode {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// sequence_id (confidence: medium, kind: u32, lanes: n-08).
    pub sequence_id: u32,
    /// flags (confidence: medium, kind: flags, lanes: n-08).
    pub flags: u32,
    /// next (confidence: high, kind: pointer, lanes: n-08).
    pub next: Ptr32<u8>,
    /// Unknown bytes (0x10..0x18).
    pub _pad_0010: [u8; 0x8],
    /// mode (confidence: medium, kind: flags, lanes: n-08).
    pub mode: u8,
    /// mode (confidence: medium, kind: flags, lanes: n-08).
    pub mode_2: u8,
    /// Unknown trailing bytes (0x1a..0x1c).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CTaskNode, 0x1c); // merged size 0x1a rounded to 4
assert_offset!(CTaskNode, sequence_id, 0x4);
assert_offset!(CTaskNode, flags, 0x8);
assert_offset!(CTaskNode, next, 0xc);
assert_offset!(CTaskNode, mode, 0x18);
assert_offset!(CTaskNode, mode_2, 0x19);

/// Merged layout for `CTaskSimple`.
///
/// Size: 0xe8 (low). Bases: CTask@0x0.
/// Lanes: c-tasks-b, via:CTaskSimpleAimGun, via:CTaskSimpleBlendFromNM, via:CTaskSimpleCarGetIn, via:CTaskSimpleCarGetOut, via:CTaskSimpleCarOpenDoorFromOutside, via:CTaskSimpleCarSlowBeDraggedOut, via:CTaskSimpleClimb, via:CTaskSimpleClimbLadder, via:CTaskSimpleDuck, via:CTaskSimpleFireGun, via:CTaskSimpleMeleeActionResult, via:CTaskSimpleMoveGoToPoint, via:CTaskSimpleMoveGoToPointOnRoute, via:CTaskSimpleMoveMeleeMovement, via:CTaskSimpleMovePlayer, via:CTaskSimpleMoveStandStill, via:CTaskSimpleMoveSwim, via:CTaskSimpleNM, via:CTaskSimpleNMBalance, via:CTaskSimpleNMBrace, via:CTaskSimpleNMFlinch, via:CTaskSimpleNMHighFall, via:CTaskSimpleNMPose, via:CTaskSimpleNMShot, via:CTaskSimpleNMSit, via:CTaskSimpleNewGangDriveBy, via:CTaskSimplePause, via:CTaskSimplePauseSystemTimer, via:CTaskSimplePlayAnimAndSlideIntoCover, via:CTaskSimplePlayRandomAmbients, via:CTaskSimpleReloadGun, via:CTaskSimpleRunAnim, via:CTaskSimpleRunNamedAnim, via:CTaskSimpleSay, via:CTaskSimpleSitDown, via:CTaskSimpleSlideToCoord, via:CTaskSimpleStandStill, via:CTaskSimpleTogglePedThreatScanner, via:CTaskSimpleTriggerLookAt.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimple {
    /// Unknown bytes (0x0..0x15).
    pub _pad_0000: [u8; 0x15],
    /// field_15 (confidence: high, kind: bool/int8, lanes: c-tasks-b,via:CTaskSimpleBlendFromNM,via:CTaskSimpleTogglePedThreatScanner moved from siblings:CTaskSimpleBlendFromNM,CTaskSimpleTogglePedThreatScanner).
    pub field_15: u8,
    /// Unknown bytes (0x16..0x2d).
    pub _pad_0016: [u8; 0x17],
    /// field_2d (confidence: high, kind: bool/int8, lanes: c-tasks-b,via:CTaskSimpleCarGetIn,via:CTaskSimpleCarOpenDoorFromOutside moved from siblings:CTaskSimpleCarGetIn,CTaskSimpleCarOpenDoorFromOutside).
    pub field_2d: u8,
    /// Unknown bytes (0x2e..0x3c).
    pub _pad_002e: [u8; 0xe],
    /// field_3c (confidence: high, kind: bool/int8, lanes: c-tasks-b,via:CTaskSimpleBlendFromNM,via:CTaskSimpleNMPose,via:CTaskSimpleTriggerLookAt moved from siblings:CTaskSimpleBlendFromNM,CTaskSimpleNM,CTaskSimpleTriggerLookAt).
    pub field_3c: u8,
    /// Unknown bytes (0x3d..0x56).
    pub _pad_003d: [u8; 0x19],
    /// field_56 (confidence: high, kind: bool/int8, lanes: c-tasks-b,via:CTaskSimpleNMBalance,via:CTaskSimplePlayAnimAndSlideIntoCover moved from siblings:CTaskSimpleNM,CTaskSimplePlayAnimAndSlideIntoCover).
    pub field_56: u8,
    /// Unknown bytes (0x57..0x88).
    pub _pad_0057: [u8; 0x31],
    /// field_88 (confidence: medium, kind: bool/int8, lanes: c-tasks-b,via:CTaskSimpleClimb,via:CTaskSimpleMoveSwim moved from siblings:CTaskSimpleClimb,CTaskSimpleMoveSwim).
    pub field_88: u8,
    /// Unknown bytes (0x89..0xa4).
    pub _pad_0089: [u8; 0x1b],
    /// field_a4 (confidence: high, kind: pointer, lanes: c-tasks-b,via:CTaskSimpleAimGun,via:CTaskSimpleClimbLadder,via:CTaskSimpleFireGun,via:CTaskSimpleMoveGoToPoint,via:CTaskSimpleRunNamedAnim moved from siblings:CTaskSimpleAimGun,CTaskSimpleAnim,CTaskSimpleClimbLadder,CTaskSimpleFireGun,CTaskSimpleMove).
    pub field_a4: Ptr32<u8>,
    /// Unknown bytes (0xa8..0xac).
    pub _pad_00a8: [u8; 0x4],
    /// field_ac (confidence: high, kind: pointer, lanes: c-tasks-b,via:CTaskSimpleAimGun,via:CTaskSimpleFireGun,via:CTaskSimpleMoveGoToPoint,via:CTaskSimpleRunNamedAnim moved from siblings:CTaskSimpleAimGun,CTaskSimpleAnim,CTaskSimpleFireGun,CTaskSimpleMove).
    pub field_ac: Ptr32<u8>,
    /// Unknown bytes (0xb0..0xb8).
    pub _pad_00b0: [u8; 0x8],
    /// field_b8 (confidence: high, kind: pointer, lanes: c-tasks-b,via:CTaskSimpleClimbLadder,via:CTaskSimpleFireGun,via:CTaskSimpleMoveGoToPoint moved from siblings:CTaskSimpleClimbLadder,CTaskSimpleFireGun,CTaskSimpleMove).
    pub field_b8: Ptr32<u8>,
    /// field_bc (confidence: high, kind: i32, lanes: c-tasks-b,via:CTaskSimpleFireGun,via:CTaskSimpleMoveGoToPoint moved from siblings:CTaskSimpleFireGun,CTaskSimpleMove).
    pub field_bc: u16,
    /// Unknown bytes (0xbe..0xd0).
    pub _pad_00be: [u8; 0x12],
    /// field_d0 (confidence: high, kind: float, lanes: c-tasks-b,via:CTaskSimpleMoveGoToPointOnRoute,via:CTaskSimpleSlideToCoord moved from siblings:CTaskSimpleAnim,CTaskSimpleMove).
    pub field_d0: f32,
    /// field_d4 (confidence: high, kind: float, lanes: c-tasks-b,via:CTaskSimpleMoveGoToPointOnRoute,via:CTaskSimpleSlideToCoord moved from siblings:CTaskSimpleAnim,CTaskSimpleMove).
    pub field_d4: f32,
    /// Unknown bytes (0xd8..0xe4).
    pub _pad_00d8: [u8; 0xc],
    /// field_e4 (confidence: high, kind: float, lanes: c-tasks-b,via:CTaskSimpleNMBalance,via:CTaskSimpleSlideToCoord moved from siblings:CTaskSimpleAnim,CTaskSimpleNM).
    pub field_e4: f32,
}
assert_size!(CTaskSimple, 0xe8); // merged size 0xe8 rounded to 4
assert_offset!(CTaskSimple, field_15, 0x15);
assert_offset!(CTaskSimple, field_2d, 0x2d);
assert_offset!(CTaskSimple, field_3c, 0x3c);
assert_offset!(CTaskSimple, field_56, 0x56);
assert_offset!(CTaskSimple, field_88, 0x88);
assert_offset!(CTaskSimple, field_a4, 0xa4);
assert_offset!(CTaskSimple, field_ac, 0xac);
assert_offset!(CTaskSimple, field_b8, 0xb8);
assert_offset!(CTaskSimple, field_bc, 0xbc);
assert_offset!(CTaskSimple, field_d0, 0xd0);
assert_offset!(CTaskSimple, field_d4, 0xd4);
assert_offset!(CTaskSimple, field_e4, 0xe4);

/// Merged layout for `CTaskSimpleAimGun`.
///
/// Size: 0xb0 (low). Bases: CTaskSimple@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleAimGun {
    /// Unknown bytes (0x0..0x4c).
    pub _pad_0000: [u8; 0x4c],
    /// field_4c (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_4c: Ptr32<u8>,
    /// Unknown bytes (0x50..0x64).
    pub _pad_0050: [u8; 0x14],
    /// field_64 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_64: Ptr32<u8>,
    /// field_68 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_68: Ptr32<u8>,
    /// field_6c (confidence: medium, kind: float, lanes: c-tasks-b).
    pub field_6c: f32,
    /// field_70 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_78: Ptr32<u8>,
    /// Unknown bytes (0x7c..0x84).
    pub _pad_007c: [u8; 0x8],
    /// field_84 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_84: Ptr32<u8>,
    /// Unknown bytes (0x88..0x90).
    pub _pad_0088: [u8; 0x8],
    /// field_90 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_94: Ptr32<u8>,
    /// field_98 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_98: Ptr32<u8>,
    /// field_9c (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_9c: Ptr32<u8>,
    /// Unknown bytes (0xa0..0xa1).
    pub _pad_00a0: [u8; 0x1],
    /// field_a1 (confidence: medium, kind: bool/int8, lanes: c-tasks-b).
    pub field_a1: u8,
    /// Unknown trailing bytes (0xa2..0xb0).
    pub _pad_end: [u8; 0xe],
}
assert_size!(CTaskSimpleAimGun, 0xb0); // merged size 0xb0 rounded to 4
assert_offset!(CTaskSimpleAimGun, field_4c, 0x4c);
assert_offset!(CTaskSimpleAimGun, field_64, 0x64);
assert_offset!(CTaskSimpleAimGun, field_68, 0x68);
assert_offset!(CTaskSimpleAimGun, field_6c, 0x6c);
assert_offset!(CTaskSimpleAimGun, field_70, 0x70);
assert_offset!(CTaskSimpleAimGun, field_74, 0x74);
assert_offset!(CTaskSimpleAimGun, field_78, 0x78);
assert_offset!(CTaskSimpleAimGun, field_84, 0x84);
assert_offset!(CTaskSimpleAimGun, field_90, 0x90);
assert_offset!(CTaskSimpleAimGun, field_94, 0x94);
assert_offset!(CTaskSimpleAimGun, field_98, 0x98);
assert_offset!(CTaskSimpleAimGun, field_9c, 0x9c);
assert_offset!(CTaskSimpleAimGun, field_a1, 0xa1);

/// Merged layout for `CTaskSimpleClimbLadder`.
///
/// Size: 0xcf (low). Bases: CTaskSimple@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleClimbLadder {
    /// Unknown bytes (0x0..0x70).
    pub _pad_0000: [u8; 0x70],
    /// field_70 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_70: f32,
    /// field_74 (confidence: medium, kind: float, lanes: c-tasks-b).
    pub field_74: f32,
    /// field_78 (confidence: medium, kind: float, lanes: c-tasks-b).
    pub field_78: f32,
    /// Unknown bytes (0x7c..0x84).
    pub _pad_007c: [u8; 0x8],
    /// field_84 (confidence: medium, kind: float, lanes: c-tasks-b).
    pub field_84: f32,
    /// Unknown bytes (0x88..0x90).
    pub _pad_0088: [u8; 0x8],
    /// field_90 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_94: Ptr32<u8>,
    /// field_98 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_98: Ptr32<u8>,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// field_a0 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_a0: Ptr32<u8>,
    /// Unknown bytes (0xa4..0xb0).
    pub _pad_00a4: [u8; 0xc],
    /// field_b0 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_b0: Ptr32<u8>,
    /// field_b4 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_b4: Ptr32<u8>,
    /// Unknown bytes (0xb8..0xc0).
    pub _pad_00b8: [u8; 0x8],
    /// field_c0 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_c0: Ptr32<u8>,
    /// field_c4 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_c4: Ptr32<u8>,
    /// field_c8 (confidence: medium, kind: bool/int8, lanes: c-tasks-b).
    pub field_c8: u16,
    /// field_ca (confidence: medium, kind: bool/int8, lanes: c-tasks-b).
    pub field_ca: u8,
    /// field_cb (confidence: medium, kind: bool/int8, lanes: c-tasks-b).
    pub field_cb: u8,
    /// Unknown bytes (0xcc..0xce).
    pub _pad_00cc: [u8; 0x2],
    /// field_ce (confidence: low, kind: bool/int8, lanes: c-tasks-b).
    pub field_ce: u8,
    /// Unknown trailing bytes (0xcf..0xd0).
    pub _pad_end: [u8; 0x1],
}
assert_size!(CTaskSimpleClimbLadder, 0xd0); // merged size 0xcf rounded to 4
assert_offset!(CTaskSimpleClimbLadder, field_70, 0x70);
assert_offset!(CTaskSimpleClimbLadder, field_74, 0x74);
assert_offset!(CTaskSimpleClimbLadder, field_78, 0x78);
assert_offset!(CTaskSimpleClimbLadder, field_84, 0x84);
assert_offset!(CTaskSimpleClimbLadder, field_90, 0x90);
assert_offset!(CTaskSimpleClimbLadder, field_94, 0x94);
assert_offset!(CTaskSimpleClimbLadder, field_98, 0x98);
assert_offset!(CTaskSimpleClimbLadder, field_a0, 0xa0);
assert_offset!(CTaskSimpleClimbLadder, field_b0, 0xb0);
assert_offset!(CTaskSimpleClimbLadder, field_b4, 0xb4);
assert_offset!(CTaskSimpleClimbLadder, field_c0, 0xc0);
assert_offset!(CTaskSimpleClimbLadder, field_c4, 0xc4);
assert_offset!(CTaskSimpleClimbLadder, field_c8, 0xc8);
assert_offset!(CTaskSimpleClimbLadder, field_ca, 0xca);
assert_offset!(CTaskSimpleClimbLadder, field_cb, 0xcb);
assert_offset!(CTaskSimpleClimbLadder, field_ce, 0xce);

/// Merged layout for `CTaskSimpleFireGun`.
///
/// Size: 0xbe (low). Bases: CTaskSimple@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleFireGun {
    /// Unknown bytes (0x0..0x42).
    pub _pad_0000: [u8; 0x42],
    /// field_42 (confidence: low, kind: i32, lanes: c-tasks-b).
    pub field_42: u16,
    /// Unknown bytes (0x44..0x64).
    pub _pad_0044: [u8; 0x20],
    /// field_64 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_64: f32,
    /// field_68 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_68: Ptr32<u8>,
    /// field_6c (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_6c: Ptr32<u8>,
    /// field_70 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_78: Ptr32<u8>,
    /// Unknown bytes (0x7c..0x84).
    pub _pad_007c: [u8; 0x8],
    /// field_84 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_84: Ptr32<u8>,
    /// Unknown bytes (0x88..0x90).
    pub _pad_0088: [u8; 0x8],
    /// field_90 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_94: Ptr32<u8>,
    /// field_98 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_98: Ptr32<u8>,
    /// field_9c (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_9c: Ptr32<u8>,
    /// field_a0 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_a0: Ptr32<u8>,
    /// Unknown bytes (0xa4..0xb0).
    pub _pad_00a4: [u8; 0xc],
    /// field_b0 (confidence: low, kind: bool/int8, lanes: c-tasks-b).
    pub field_b0: u8,
    /// Unknown bytes (0xb1..0xb4).
    pub _pad_00b1: [u8; 0x3],
    /// field_b4 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_b4: Ptr32<u8>,
    /// Unknown trailing bytes (0xb8..0xc0).
    pub _pad_end: [u8; 0x8],
}
assert_size!(CTaskSimpleFireGun, 0xc0); // merged size 0xbe rounded to 4
assert_offset!(CTaskSimpleFireGun, field_42, 0x42);
assert_offset!(CTaskSimpleFireGun, field_64, 0x64);
assert_offset!(CTaskSimpleFireGun, field_68, 0x68);
assert_offset!(CTaskSimpleFireGun, field_6c, 0x6c);
assert_offset!(CTaskSimpleFireGun, field_70, 0x70);
assert_offset!(CTaskSimpleFireGun, field_74, 0x74);
assert_offset!(CTaskSimpleFireGun, field_78, 0x78);
assert_offset!(CTaskSimpleFireGun, field_84, 0x84);
assert_offset!(CTaskSimpleFireGun, field_90, 0x90);
assert_offset!(CTaskSimpleFireGun, field_94, 0x94);
assert_offset!(CTaskSimpleFireGun, field_98, 0x98);
assert_offset!(CTaskSimpleFireGun, field_9c, 0x9c);
assert_offset!(CTaskSimpleFireGun, field_a0, 0xa0);
assert_offset!(CTaskSimpleFireGun, field_b0, 0xb0);
assert_offset!(CTaskSimpleFireGun, field_b4, 0xb4);

/// Merged layout for `CTaskSimpleIK`.
///
/// Size: 0x68 (low). Bases: none.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleIK {
    /// vfptr (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x14).
    pub _pad_0004: [u8; 0x10],
    /// field_14 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_34: f32,
    /// field_38 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_38: f32,
    /// field_3c (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_3c: f32,
    /// field_40 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_48: Ptr32<u8>,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_50: f32,
    /// field_54 (confidence: medium, kind: float, lanes: c-tasks-b).
    pub field_54: f32,
    /// field_58 (confidence: medium, kind: bool/int8, lanes: c-tasks-b).
    pub field_58: u32,
    /// field_5c (confidence: high, kind: bool/int8, lanes: c-tasks-b).
    pub field_5c: u32,
    /// field_60 (confidence: low, kind: bool/int8, lanes: c-tasks-b).
    pub field_60: u8,
    /// Unknown bytes (0x61..0x64).
    pub _pad_0061: [u8; 0x3],
    /// field_64 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_64: Ptr32<u8>,
}
assert_size!(CTaskSimpleIK, 0x68); // merged size 0x68 rounded to 4
assert_offset!(CTaskSimpleIK, vfptr, 0x0);
assert_offset!(CTaskSimpleIK, field_14, 0x14);
assert_offset!(CTaskSimpleIK, field_18, 0x18);
assert_offset!(CTaskSimpleIK, field_1c, 0x1c);
assert_offset!(CTaskSimpleIK, field_20, 0x20);
assert_offset!(CTaskSimpleIK, field_24, 0x24);
assert_offset!(CTaskSimpleIK, field_28, 0x28);
assert_offset!(CTaskSimpleIK, field_2c, 0x2c);
assert_offset!(CTaskSimpleIK, field_30, 0x30);
assert_offset!(CTaskSimpleIK, field_34, 0x34);
assert_offset!(CTaskSimpleIK, field_38, 0x38);
assert_offset!(CTaskSimpleIK, field_3c, 0x3c);
assert_offset!(CTaskSimpleIK, field_40, 0x40);
assert_offset!(CTaskSimpleIK, field_44, 0x44);
assert_offset!(CTaskSimpleIK, field_48, 0x48);
assert_offset!(CTaskSimpleIK, field_50, 0x50);
assert_offset!(CTaskSimpleIK, field_54, 0x54);
assert_offset!(CTaskSimpleIK, field_58, 0x58);
assert_offset!(CTaskSimpleIK, field_5c, 0x5c);
assert_offset!(CTaskSimpleIK, field_60, 0x60);
assert_offset!(CTaskSimpleIK, field_64, 0x64);

/// Merged layout for `CTaskSimpleMoveGoToPoint`.
///
/// Size: 0xc8 (low). Bases: CTaskSimpleMoveBase@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleMoveGoToPoint {
    /// Unknown bytes (0x0..0x4c).
    pub _pad_0000: [u8; 0x4c],
    /// field_4c (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_4c: Ptr32<u8>,
    /// Unknown bytes (0x50..0x70).
    pub _pad_0050: [u8; 0x20],
    /// field_70 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_70: f32,
    /// field_74 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_74: f32,
    /// field_78 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_78: f32,
    /// Unknown bytes (0x7c..0x84).
    pub _pad_007c: [u8; 0x8],
    /// field_84 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_84: f32,
    /// Unknown bytes (0x88..0x90).
    pub _pad_0088: [u8; 0x8],
    /// field_90 (confidence: medium, kind: float, lanes: c-tasks-b).
    pub field_90: f32,
    /// field_94 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_94: f32,
    /// field_98 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_98: f32,
    /// field_9c (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_9c: f32,
    /// field_a0 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_a0: f32,
    /// Unknown bytes (0xa4..0xb0).
    pub _pad_00a4: [u8; 0xc],
    /// field_b0 (confidence: high, kind: bool/int8, lanes: c-tasks-b).
    pub field_b0: u32,
    /// field_b4 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_b4: Ptr32<u8>,
    /// Unknown bytes (0xb8..0xc0).
    pub _pad_00b8: [u8; 0x8],
    /// field_c0 (confidence: low, kind: bool/int8, lanes: c-tasks-b).
    pub field_c0: u8,
    /// field_c1 (confidence: low, kind: bool/int8, lanes: c-tasks-b).
    pub field_c1: u8,
    /// Unknown bytes (0xc2..0xc4).
    pub _pad_00c2: [u8; 0x2],
    /// field_c4 (confidence: high, kind: bool/int8, lanes: c-tasks-b).
    pub field_c4: u32,
}
assert_size!(CTaskSimpleMoveGoToPoint, 0xc8); // merged size 0xc8 rounded to 4
assert_offset!(CTaskSimpleMoveGoToPoint, field_4c, 0x4c);
assert_offset!(CTaskSimpleMoveGoToPoint, field_70, 0x70);
assert_offset!(CTaskSimpleMoveGoToPoint, field_74, 0x74);
assert_offset!(CTaskSimpleMoveGoToPoint, field_78, 0x78);
assert_offset!(CTaskSimpleMoveGoToPoint, field_84, 0x84);
assert_offset!(CTaskSimpleMoveGoToPoint, field_90, 0x90);
assert_offset!(CTaskSimpleMoveGoToPoint, field_94, 0x94);
assert_offset!(CTaskSimpleMoveGoToPoint, field_98, 0x98);
assert_offset!(CTaskSimpleMoveGoToPoint, field_9c, 0x9c);
assert_offset!(CTaskSimpleMoveGoToPoint, field_a0, 0xa0);
assert_offset!(CTaskSimpleMoveGoToPoint, field_b0, 0xb0);
assert_offset!(CTaskSimpleMoveGoToPoint, field_b4, 0xb4);
assert_offset!(CTaskSimpleMoveGoToPoint, field_c0, 0xc0);
assert_offset!(CTaskSimpleMoveGoToPoint, field_c1, 0xc1);
assert_offset!(CTaskSimpleMoveGoToPoint, field_c4, 0xc4);

/// Merged layout for `CTaskSimpleNM`.
///
/// Size: 0xe4 (low). Bases: CTaskSimple@0x0.
/// Lanes: c-tasks-b, via:CTaskSimpleNMBalance, via:CTaskSimpleNMBrace, via:CTaskSimpleNMFallDown, via:CTaskSimpleNMHighFall, via:CTaskSimpleNMJumpRollFromRoadVehicle, via:CTaskSimpleNMOnFire, via:CTaskSimpleNMShot, via:CTaskSimpleNMSit.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleNM {
    /// Unknown bytes (0x0..0x22).
    pub _pad_0000: [u8; 0x22],
    /// field_22 (confidence: low, kind: i32, lanes: c-tasks-b).
    pub field_22: u16,
    /// Unknown bytes (0x24..0x4c).
    pub _pad_0024: [u8; 0x28],
    /// field_4c (confidence: high, kind: pointer, lanes: c-tasks-b,via:CTaskSimpleNMBalance,via:CTaskSimpleNMBrace,via:CTaskSimpleNMFallDown,via:CTaskSimpleNMOnFire,via:CTaskSimpleNMShot moved from siblings:CTaskSimpleNMBalance,CTaskSimpleNMBrace,CTaskSimpleNMFallDown,CTaskSimpleNMOnFire,CTaskSimpleNMShot).
    pub field_4c: Ptr32<u8>,
    /// Unknown bytes (0x50..0xc0).
    pub _pad_0050: [u8; 0x70],
    /// field_c0 (confidence: high, kind: pointer, lanes: c-tasks-b,via:CTaskSimpleNMBalance,via:CTaskSimpleNMHighFall moved from siblings:CTaskSimpleNMBalance,CTaskSimpleNMHighFall).
    pub field_c0: Ptr32<u8>,
    /// Unknown bytes (0xc4..0xe0).
    pub _pad_00c4: [u8; 0x1c],
    /// field_e0 (confidence: high, kind: pointer, lanes: c-tasks-b,via:CTaskSimpleNMBalance,via:CTaskSimpleNMHighFall moved from siblings:CTaskSimpleNMBalance,CTaskSimpleNMHighFall).
    pub field_e0: Ptr32<u8>,
}
assert_size!(CTaskSimpleNM, 0xe4); // merged size 0xe4 rounded to 4
assert_offset!(CTaskSimpleNM, field_22, 0x22);
assert_offset!(CTaskSimpleNM, field_4c, 0x4c);
assert_offset!(CTaskSimpleNM, field_c0, 0xc0);
assert_offset!(CTaskSimpleNM, field_e0, 0xe0);

/// Merged layout for `CTaskSimpleNMShot`.
///
/// Size: 0xa4 (low). Bases: CTaskSimpleNM@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleNMShot {
    /// Unknown bytes (0x0..0x64).
    pub _pad_0000: [u8; 0x64],
    /// field_64 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_64: f32,
    /// field_68 (confidence: low, kind: float, lanes: c-tasks-b).
    pub field_68: f32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_78: Ptr32<u8>,
    /// Unknown bytes (0x7c..0x84).
    pub _pad_007c: [u8; 0x8],
    /// field_84 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_84: Ptr32<u8>,
    /// Unknown bytes (0x88..0x90).
    pub _pad_0088: [u8; 0x8],
    /// field_90 (confidence: low, kind: pointer, lanes: c-tasks-b).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_94: Ptr32<u8>,
    /// field_98 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_98: Ptr32<u8>,
    /// field_9c (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_9c: Ptr32<u8>,
    /// field_a0 (confidence: medium, kind: bool/int8, lanes: c-tasks-b).
    pub field_a0: u32,
}
assert_size!(CTaskSimpleNMShot, 0xa4); // merged size 0xa4 rounded to 4
assert_offset!(CTaskSimpleNMShot, field_64, 0x64);
assert_offset!(CTaskSimpleNMShot, field_68, 0x68);
assert_offset!(CTaskSimpleNMShot, field_70, 0x70);
assert_offset!(CTaskSimpleNMShot, field_74, 0x74);
assert_offset!(CTaskSimpleNMShot, field_78, 0x78);
assert_offset!(CTaskSimpleNMShot, field_84, 0x84);
assert_offset!(CTaskSimpleNMShot, field_90, 0x90);
assert_offset!(CTaskSimpleNMShot, field_94, 0x94);
assert_offset!(CTaskSimpleNMShot, field_98, 0x98);
assert_offset!(CTaskSimpleNMShot, field_9c, 0x9c);
assert_offset!(CTaskSimpleNMShot, field_a0, 0xa0);

/// Merged layout for `CTaskSimpleRunNamedAnim`.
///
/// Size: 0xb8 (low). Bases: CTaskSimpleAnim@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleRunNamedAnim {
    /// Unknown bytes (0x0..0x98).
    pub _pad_0000: [u8; 0x98],
    /// field_98 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_98: f32,
    /// field_9c (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_9c: Ptr32<u8>,
    /// field_a0 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_a0: Ptr32<u8>,
    /// Unknown bytes (0xa4..0xb0).
    pub _pad_00a4: [u8; 0xc],
    /// field_b0 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_b0: Ptr32<u8>,
    /// field_b4 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_b4: f32,
}
assert_size!(CTaskSimpleRunNamedAnim, 0xb8); // merged size 0xb8 rounded to 4
assert_offset!(CTaskSimpleRunNamedAnim, field_98, 0x98);
assert_offset!(CTaskSimpleRunNamedAnim, field_9c, 0x9c);
assert_offset!(CTaskSimpleRunNamedAnim, field_a0, 0xa0);
assert_offset!(CTaskSimpleRunNamedAnim, field_b0, 0xb0);
assert_offset!(CTaskSimpleRunNamedAnim, field_b4, 0xb4);

/// Merged layout for `CTaskSimpleSitDown`.
///
/// Size: 0x76 (low). Bases: CTaskSimple@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleSitDown {
    /// Unknown bytes (0x0..0x64).
    pub _pad_0000: [u8; 0x64],
    /// field_64 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_64: Ptr32<u8>,
    /// field_68 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_68: Ptr32<u8>,
    /// field_6c (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_6c: Ptr32<u8>,
    /// field_70 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_70: f32,
    /// field_74 (confidence: medium, kind: bool/int8, lanes: c-tasks-b).
    pub field_74: u16,
    /// Unknown trailing bytes (0x76..0x78).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CTaskSimpleSitDown, 0x78); // merged size 0x76 rounded to 4
assert_offset!(CTaskSimpleSitDown, field_64, 0x64);
assert_offset!(CTaskSimpleSitDown, field_68, 0x68);
assert_offset!(CTaskSimpleSitDown, field_6c, 0x6c);
assert_offset!(CTaskSimpleSitDown, field_70, 0x70);
assert_offset!(CTaskSimpleSitDown, field_74, 0x74);

/// Merged layout for `CTaskSimpleSlideToCoord`.
///
/// Size: 0xed (low). Bases: CTaskSimpleRunNamedAnim@0x0.
/// Lanes: c-tasks-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTaskSimpleSlideToCoord {
    /// Unknown bytes (0x0..0xc0).
    pub _pad_0000: [u8; 0xc0],
    /// field_c0 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_c0: f32,
    /// field_c4 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_c4: f32,
    /// field_c8 (confidence: medium, kind: pointer, lanes: c-tasks-b).
    pub field_c8: Ptr32<u8>,
    /// Unknown bytes (0xcc..0xd8).
    pub _pad_00cc: [u8; 0xc],
    /// field_d8 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_d8: f32,
    /// field_dc (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_dc: Ptr32<u8>,
    /// field_e0 (confidence: high, kind: float, lanes: c-tasks-b).
    pub field_e0: f32,
    /// Unknown bytes (0xe4..0xe8).
    pub _pad_00e4: [u8; 0x4],
    /// field_e8 (confidence: high, kind: pointer, lanes: c-tasks-b).
    pub field_e8: Ptr32<u8>,
    /// field_ec (confidence: high, kind: bool/int8, lanes: c-tasks-b).
    pub field_ec: u8,
    /// Unknown trailing bytes (0xed..0xf0).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CTaskSimpleSlideToCoord, 0xf0); // merged size 0xed rounded to 4
assert_offset!(CTaskSimpleSlideToCoord, field_c0, 0xc0);
assert_offset!(CTaskSimpleSlideToCoord, field_c4, 0xc4);
assert_offset!(CTaskSimpleSlideToCoord, field_c8, 0xc8);
assert_offset!(CTaskSimpleSlideToCoord, field_d8, 0xd8);
assert_offset!(CTaskSimpleSlideToCoord, field_dc, 0xdc);
assert_offset!(CTaskSimpleSlideToCoord, field_e0, 0xe0);
assert_offset!(CTaskSimpleSlideToCoord, field_e8, 0xe8);
assert_offset!(CTaskSimpleSlideToCoord, field_ec, 0xec);

/// Merged layout for `CWeightedTaskSet`.
///
/// Size: 0xb34 (low). Bases: CTaskSet@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CWeightedTaskSet {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x28).
    pub _pad_001c: [u8; 0xc],
    /// field_28 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x3c).
    pub _pad_002c: [u8; 0x10],
    /// field_3c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3c: u32,
    /// Unknown bytes (0x40..0x90).
    pub _pad_0040: [u8; 0x50],
    /// field_90 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_90: u32,
    /// Unknown bytes (0x94..0xd0).
    pub _pad_0094: [u8; 0x3c],
    /// field_d0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d0: u32,
    /// field_d4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d4: u32,
    /// field_d8 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_d8: u32,
    /// field_dc (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_dc: u32,
    /// Unknown bytes (0xe0..0x26c).
    pub _pad_00e0: [u8; 0x18c],
    /// field_26c (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_26c: u8,
    /// Unknown bytes (0x26d..0xb30).
    pub _pad_026d: [u8; 0x8c3],
    /// field_b30 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b30: u32,
}
assert_size!(CWeightedTaskSet, 0xb34); // merged size 0xb34 rounded to 4
assert_offset!(CWeightedTaskSet, field_10, 0x10);
assert_offset!(CWeightedTaskSet, field_14, 0x14);
assert_offset!(CWeightedTaskSet, field_18, 0x18);
assert_offset!(CWeightedTaskSet, field_28, 0x28);
assert_offset!(CWeightedTaskSet, field_3c, 0x3c);
assert_offset!(CWeightedTaskSet, field_90, 0x90);
assert_offset!(CWeightedTaskSet, field_d0, 0xd0);
assert_offset!(CWeightedTaskSet, field_d4, 0xd4);
assert_offset!(CWeightedTaskSet, field_d8, 0xd8);
assert_offset!(CWeightedTaskSet, field_dc, 0xdc);
assert_offset!(CWeightedTaskSet, field_26c, 0x26c);
assert_offset!(CWeightedTaskSet, field_b30, 0xb30);
