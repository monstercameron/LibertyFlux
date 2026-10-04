// original: 0x009467e0 guarded_object_register
/// Register an object with the global registrar when probes allow.
///
/// Returns silently unless at least one of two readiness probes reports
/// nonzero. Otherwise compares a limit from a third probe against a tag byte
/// in the object (unsigned); registers only when the tag is below the limit.
lf_checker_rt::export!(cdecl, rw_009467e0(a1: u32) -> () {
    unsafe {
        let x = lf_checker_rt::callee_cdecl!(1, u32,);
        if (x & 0xff) == 0 {
            let y = lf_checker_rt::callee_cdecl!(2, u32,);
            if (y & 0xff) == 0 {
                return;
            }
        }
        let lim = lf_checker_rt::callee_cdecl!(3, u32,);
        let tag = *(a1.wrapping_add(6) as *const u8) as u32;
        if tag >= lim {
            return;
        }
        lf_checker_rt::callee_thiscall!(
            4,
            u32,
            lf_checker_rt::relocated(0x18ecfb0),
            a1
        );
    }
});
