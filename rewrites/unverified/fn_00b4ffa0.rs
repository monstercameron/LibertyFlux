// original: 0x00b4ffa0 CDummyPed::vf14

/// Forward to the ped spawn initialiser with a default parent.
///
/// Calls the sibling initialiser (callee 1, thiscall on `this`) with the
/// caller's argument and -1 as the parent slot, returning its result.
///
/// Original: 0x00b4ffa0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b4ffa0(this: u32, arg: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        const NO_PARENT: u32 = 0xffff_ffff;
        lf_checker_rt::callee_thiscall!(INIT, u32, this, arg, NO_PARENT)
    }
});
