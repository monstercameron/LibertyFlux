// original: 0x00946dd0 init_default_slot
/// Reset the default slot and clear its latched state.
///
/// Raises the reset flag, zeroes the reset word, runs the slot closer over
/// the looked-up default entry, then clears the entry's latched byte.
lf_checker_rt::export!(cdecl, rw_00946dd0() -> () {
    unsafe {
        *(lf_checker_rt::global::<u8>(0x11d7629)) = 1;
        *(lf_checker_rt::global::<u32>(0x11d762c)) = 0;
        let e = lf_checker_rt::callee_cdecl!(1, u32, 0);
        lf_checker_rt::callee_thiscall!(2, u32, e);
        let e2 = lf_checker_rt::callee_cdecl!(1, u32, 0);
        *((e2.wrapping_add(0x1920)) as *mut u8) = 0;
    }
});
