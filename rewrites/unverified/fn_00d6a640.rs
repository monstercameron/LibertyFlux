// original: 0x00d6a640 FRONTEND_MENU_MONTAGE_CYCLE_MT
/// Cycle the montage selector in the requested direction (original 0x00D6A640,
/// thiscall/1).
///
/// Logs the fixed token (callee 1 on the constant object), then moves the
/// member at `this+4` through callee 2 when the low byte of `dir` is zero
/// and through callee 3 otherwise, keeping the low byte of the answer.
/// Advances the selector (callee 4 with 2), polls completion (callee 5 on
/// the member) and, only when that low byte is zero while the move reported
/// nonzero, confirms through callee 6 with 1. All tests are equality or
/// test-byte checks. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a640(this_ptr: u32, dir: u32) -> u32 {
    unsafe {
        const MEMBER_OFF: u32 = 4;
        const LOG_OBJ: u32 = 0x01176888;
        const LOG_TOKEN: u32 = 0x00EEABF4;
        let member = ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(LOG_OBJ),
            lf_checker_rt::relocated(LOG_TOKEN));
        let moved: u32 = if (dir & 0xFF) == 0 {
            lf_checker_rt::callee_thiscall!(2, u32, member)
        } else {
            lf_checker_rt::callee_thiscall!(3, u32, member)
        };
        lf_checker_rt::callee_thiscall!(4, u32, this_ptr, 2);
        let done: u32 = lf_checker_rt::callee_thiscall!(5, u32, member);
        if (done & 0xFF) == 0 && (moved & 0xFF) != 0 {
            lf_checker_rt::callee_thiscall!(6, u32, member, 1);
        }
        0
    }
});
