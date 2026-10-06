// original: 0x00c659c0 CCutsceneObject::get_bound_radius

/// Bound radius of the cutscene object.
///
/// `this` is the cutscene object. One float is loaded from `BOUND_RADIUS
/// (+0x2e0)` and returned on the x87 stack; nothing is written and no calls
/// are made. The bits pass through untouched, so NaNs keep sign and payload.
///
/// Original: thiscall, no stack arguments, float result left in ST0.
lf_checker_rt::export!(thiscall, rw_00c659c0(this: u32) -> f32 {
    const BOUND_RADIUS: u32 = 0x2e0;
    unsafe { f32::from_bits(((this + BOUND_RADIUS) as *const u32).read_unaligned()) }
});
