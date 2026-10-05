// original: 0x00B39570 task_init_float

/// Initialise one float task slot and notify four callees.
///
/// Writes two engine globals, calls callee 1 as `(slot, f2)`, and unless
/// bit 1 of `flags` is set calls callee 2 with the same pair. Then calls
/// callee 3 (no arguments) and callee 4 with eleven words
/// `(slot, f1, 0, f2, 0, f2, 0, 0, 0, w3, flags)`. Returns callee 4's
/// answer. Cdecl, five stack words (the tested flag word is the fifth).
///
/// Original: 0x00B39570.

lf_checker_rt::export!(cdecl, rw_00B39570(slot: u32, f1: u32, f2: u32, w3: u32, flags: u32) -> u32 {
    unsafe {
        const G_MODE: u32 = 0x010459B4;
        const G_COUNT: u32 = 0x016624B0;
        const MODE_VAL: u32 = 0x400;
        const SKIP_BIT: u32 = 2;
        (lf_checker_rt::global::<u32>(G_MODE) as *mut u32).write_unaligned(MODE_VAL);
        (lf_checker_rt::global::<u32>(G_COUNT) as *mut u32).write_unaligned(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, slot, f2);
        if flags & SKIP_BIT == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, slot, f2);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        lf_checker_rt::callee_cdecl!(
            4, u32, slot, f1, 0, f2, 0, f2, 0, 0, 0, w3, flags
        )
    }
});
