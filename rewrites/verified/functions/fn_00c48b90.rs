// original: 0x00c48b90 ccam_forward_124 (proposed)
/// Forward the sub-object at `this + CHILD` to the dispatch routine.
///
/// Pushes the word at `this + CHILD` and calls the dispatcher
/// (callee 1, stdcall), returning its result.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c48b90(this: u32) -> u32 {
    const CHILD: u32 = 0x124;
    const DISPATCH: u32 = 1;
    unsafe {
        let child = ((this + CHILD) as *const u32).read_unaligned();
        lf_checker_rt::callee_stdcall!(DISPATCH, u32, child)
    }
});
