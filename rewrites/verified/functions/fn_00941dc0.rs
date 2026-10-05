// original: 0x00941dc0 streaming_subobj_init (proposed)

/// Initialise the embedded sub-object at offset 0x14 and return the object.
///
/// Calls the sub-object initialiser (thiscall, no stack arguments) with
/// `this + 0x14` and returns `this` in `eax`.
///
/// Original: 0x00941dc0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00941dc0(this: u32) -> u32 {
    const SUB: u32 = 0x14;
    const CALLEE: u32 = 1;
    let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, this.wrapping_add(SUB));
    this
});
