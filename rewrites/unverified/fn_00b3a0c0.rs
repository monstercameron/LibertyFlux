// original: 0x00b3a0c0 task_subsystem_init (proposed)

/// Initialise the task subsystem: zero the counter globals, set the two
/// scaler globals (1.0 and all-bits-set), then run the nine initialisation
/// callees in order (two take a 1.0 word, one takes -1, two take 1, four
/// take nothing), and finally zero the pending-count global. The callees'
/// answers are ignored; the result is the last callee's answer, matching
/// the original's exit register. Original: 0x00b3a0c0 (cdecl, no stack
/// arguments).
lf_checker_rt::export!(cdecl, rw_00b3a0c0() -> u32 {
    unsafe {
        const ONE_BITS: u32 = 0x3f80_0000;
        lf_checker_rt::global::<u32>(0x016624c0).write(0);
        lf_checker_rt::global::<u32>(0x016624c4).write(0);
        lf_checker_rt::global::<u32>(0x01662460).write(0);
        lf_checker_rt::global::<u32>(0x01662464).write(0);
        lf_checker_rt::global::<u32>(0x01045928).write(ONE_BITS);
        lf_checker_rt::global::<u32>(0x01045938).write(0xffff_ffff);
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, ONE_BITS);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, ONE_BITS);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, 0xffff_ffff);
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, 1);
        let _: u32 = lf_checker_rt::callee_cdecl!(8, u32, 1);
        let answer: u32 = lf_checker_rt::callee_cdecl!(9, u32,);
        lf_checker_rt::global::<u32>(0x016624ac).write(0);
        answer
    }
});
