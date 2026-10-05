// original: 0x00870040 rage::crmtComposerData::vf2

/// Forward to the shared composer teardown: adjust the object pointer past
/// the 4-byte vtable slot and tail-jump to the common implementation. The
/// original ends in a jump (no return of its own); the rewrite expresses the
/// same transfer as a call that forwards the adjusted pointer and returns
/// the callee's result unchanged.
///
/// Original: 0x00870040 (thiscall, no stack arguments; tail call).
lf_checker_rt::export!(thiscall, rw_00870040(this: u32) -> u32 {
    const VTABLE_SKIP: u32 = 4;
    lf_checker_rt::callee_thiscall!(1, u32, this + VTABLE_SKIP)
});
