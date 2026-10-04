// original: 0x00b791d0 CTaskComplexGoToPointAndStandStillTimed::vf10
/// Report whether the stored timer has reached 1.0.
///
/// Compares the float at offset `0x18` against 1.0 and returns 1 when it
/// is greater or equal, 0 otherwise (NaN compares as less).
export!(thiscall, rw_00b791d0(this: u32) -> u32 {
    let x = unsafe { *((this + 0x18) as *const f32) };
    (x >= 1.0) as u32
});
