// original: 0x00b791f0 CTaskSimpleMoveGoToPointOnRoute::vf10
/// Report whether the stored value is positive.
///
/// Compares the float at offset `0x18` against 0.0 and returns 1 when it
/// is strictly greater, 0 otherwise (NaN and zeros compare as not greater).
export!(thiscall, rw_00b791f0(this: u32) -> u32 {
    let x = unsafe { *((this + 0x18) as *const f32) };
    (x > 0.0) as u32
});
