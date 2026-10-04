// original: 0x00BE8020 anim_setup_checked_b (proposed)
/// Set up a blend slot when the pilot check accepts, else mark it failed.
///
/// Same shape as 0x00BE7FC0 with the 8.0 weight, the handle at `+0x14`,
/// the failed byte at `+0x20` and the other completion callback.
/// See that function for the argument and call description.
///
/// Original: 0x00BE8020 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00BE8020(obj: u32, target: u32, arg: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x0b;
        const WEIGHT: u32 = 0x4100_0000;
        const HANDLE: u32 = 0x14;
        const FAILED: u32 = 0x20;
        const MOVER: u32 = 0x78;
        const CALLBACK: u32 = 0x00BE4D50;
        const CHECK: u32 = 1;
        const COMPUTE: u32 = 2;
        const ATTACH: u32 = 3;
        let ok: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, KIND, arg);
        if ok == 0 {
            ((obj + FAILED) as *mut u8).write(1);
            return 0;
        }
        let mover = ((target + MOVER) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_thiscall!(
            COMPUTE, u32, mover, KIND, arg, WEIGHT, 0xffff_ffff
        );
        ((obj + HANDLE) as *mut u32).write_unaligned(h);
        let out: u32 = lf_checker_rt::callee_thiscall!(
            ATTACH, u32, h, 1, lf_checker_rt::relocated(CALLBACK), obj
        );
        ((obj + FAILED) as *mut u8).write(0);
        out
    }
});

