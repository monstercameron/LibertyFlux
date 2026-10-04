// original: 0x008d81c0 threshold_update
/// Refresh a threshold global, then forward to a worker with a fixed tag.
///
/// Writes `HI - LO` (two global float constants) into the threshold cell,
/// loads `this` from its cell, and calls the callee as
/// `callee(this, arg, TAG)` where TAG is a relocated code address.
export!(cdecl, rw_008d81c0(arg: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x00A91C80;
        let hi = *global::<f32>(0x00FE8B08);
        let lo = *global::<f32>(0x0103E8A0);
        *global::<f32>(0x012FB264) = if hi.is_nan() {
            hi
        } else if lo.is_nan() {
            lo
        } else {
            // Pinned dest/src roles: hi is the original's subss dest, so its
            // NaN payload must win over lo's (see the clamp_lerp note).
            hi - lo
        };
        let this_ = *global::<u32>(0x012FB260);
        callee_thiscall!(1, u32, this_, arg, relocated(TAG))
    }
});
