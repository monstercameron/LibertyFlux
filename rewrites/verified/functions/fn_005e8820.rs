// original: 0x005e8820 CCutsceneObject::vf24

/// Pointer to corner A of the cutscene object's local bounds.
///
/// `this` is the cutscene object. The function returns `this + CORNER_A`
/// unchanged otherwise: no reads, no writes, no calls. Corner A (`+0x2f0`,
/// three words) and corner B (`+0x300`) are the triples the bounding-box
/// methods (same class, verified earlier) transform; this slot hands out the
/// address of the first triple.
///
/// Original: thiscall, no stack arguments, address returned in EAX.
lf_checker_rt::export!(thiscall, rw_005e8820(this: u32) -> u32 {
    const CORNER_A: u32 = 0x2f0;
    this.wrapping_add(CORNER_A)
});
