// original: 0x00c48ce0 CCamCinematic::vf5 (symbols)
/// Ensure a slot exists unless one is already active.
///
/// Asks the member (callee 1, called with 1) whether a slot is
/// active; when it reports none, opens one (callee 2). Returns 1 in
/// the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c48ce0(this: u32) -> u32 {
    const IS_ACTIVE: u32 = 1;
    const ENSURE_SLOT: u32 = 2;
    unsafe {
        if lf_checker_rt::callee_thiscall!(IS_ACTIVE, u32, this, 1) == 0 {
            lf_checker_rt::callee_thiscall!(ENSURE_SLOT, u32, this);
        }
    }
    1
});
