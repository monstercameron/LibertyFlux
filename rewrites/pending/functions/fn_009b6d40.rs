// original: 0x009B6D40 manager_forward_thunk (proposed)
/// Tail-jump to the manager routine with the manager object in place.
///
/// Loads the manager singleton address and jumps to the shared routine; the
/// jump target returns directly to this function's caller. stdcall, no
/// arguments; the rewrite forwards the call and its result.
lf_checker_rt::export!(stdcall, rw_009B6D40() -> u32 {
    unsafe {
        const MGR: u32 = 0x0118D7F0;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(MGR))
    }
});
