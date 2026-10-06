// original: 0x00928A00 CRenderPhaseCascadeShadows::vf0 (symbols)

/// Virtual slot 0 of `CRenderPhaseCascadeShadows`: destroying destructor.
///
/// Runs the phase teardown (callee 1) on `this`, then, when bit 0 of
/// `flags` is set, frees `this` through the delete helper (callee 2).
/// Returns `this` either way.
///
/// Original: 0x00928A00 (thiscall, one stack word). Two direct calls.
lf_checker_rt::export!(thiscall, rw_00928A00(this: u32, flags: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
