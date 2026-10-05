// original: 0x00B3A0C0 subsys_init_counters

/// Reset the float-tuning globals and their flag bytes to start-up values.
///
/// Writes zeros and ones to seven engine globals, then drives nine tiny
/// setter callees: callee 1 takes two words `(1.0, 1.0)` (both are stored
/// by the real callee, so both are compared), callees 2, 3, 7 and 8 take
/// one word each, the rest take none. Returns the last callee's answer.
/// Cdecl, no stack arguments.
///
/// Original: 0x00B3A0C0.

lf_checker_rt::export!(cdecl, rw_00B3A0C0() -> u32 {
    unsafe {
        const ONE: u32 = 0x3F800000;
        (lf_checker_rt::global::<u32>(0x016624C0) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(0x016624C4) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(0x01662460) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(0x01662464) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(0x01045928) as *mut u32).write_unaligned(ONE);
        (lf_checker_rt::global::<u32>(0x01045938) as *mut u32).write_unaligned(0xFFFF_FFFF);
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, ONE, ONE);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, ONE);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, 0xFFFF_FFFF);
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, 1);
        let _: u32 = lf_checker_rt::callee_cdecl!(8, u32, 1);
        let last: u32 = lf_checker_rt::callee_cdecl!(9, u32,);
        (lf_checker_rt::global::<u32>(0x016624AC) as *mut u32).write_unaligned(0);
        last
    }
});
