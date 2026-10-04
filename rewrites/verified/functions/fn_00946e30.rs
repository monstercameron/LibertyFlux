// original: 0x00946e30 release_two_stored_handles
/// Release two stored handles, clearing their slots.
///
/// Each nonzero slot is handed to the releaser with the stored value, then
/// cleared. Zero slots are skipped.
lf_checker_rt::export!(cdecl, rw_00946e30() -> () {
    unsafe {
        let g1 = lf_checker_rt::global::<u32>(0x11d74f4);
        if *g1 != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, *g1, 0);
            *g1 = 0;
        }
        let g2 = lf_checker_rt::global::<u32>(0x11d74f8);
        if *g2 != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, *g2, 0);
            *g2 = 0;
        }
    }
});
