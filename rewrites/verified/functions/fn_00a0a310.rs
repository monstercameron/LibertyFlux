// original: 0x00a0a310 mission_cleanup_is_armed (proposed)
/// Test whether mission cleanup is armed and its backend agrees.
///
/// Reads the mode global: anything but 3 means no. In mode 3 it asks the
/// backend predicate and returns 1 only when that also succeeds. Only the
/// low byte is set on the success path. Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_00a0a310() -> u32 {
    unsafe {
        const MODE_GLOBAL: u32 = 0x011d6fd4;
        const ARMED_MODE: u32 = 3;
        const BACKEND: u32 = 0;
        let mode = (lf_checker_rt::global::<u32>(MODE_GLOBAL) as *const u32).read_unaligned();
        if mode != ARMED_MODE {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_cdecl!(BACKEND, u32,);
        ((ok & 0xff) != 0) as u32
    }
});
