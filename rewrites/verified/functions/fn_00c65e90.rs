// original: 0x00c65e90 CCutsceneObject::get_max_bounds

/// Pointer to corner B (the maximum corner) of the cutscene object's bounds.
///
/// `this` is the cutscene object. The function returns `this + CORNER_B`
/// unchanged otherwise: no reads, no writes, no calls. Corner B (`+0x300`,
/// three words) is the maximum triple of the local bounds pair whose minimum
/// triple sits at `+0x2f0`.
///
/// Original: thiscall, no stack arguments, address returned in EAX.
lf_checker_rt::export!(thiscall, rw_00c65e90(this: u32) -> u32 {
    const CORNER_B: u32 = 0x300;
    this.wrapping_add(CORNER_B)
});
