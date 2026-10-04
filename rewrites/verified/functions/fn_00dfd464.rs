// original: 0x00dfd464 strnicmp
/// Case-insensitive comparison of at most `n` bytes of two strings.
///
/// Uses the locale-aware worker when a locale is active; otherwise validates
/// the pointers and length (reporting `EINVAL` and returning `i32::MAX` on
/// failure) and tail-calls the bytewise comparison worker.
export!(cdecl, rw_00dfd464(s1: u32, s2: u32, n: u32) -> u32 {
    unsafe {
        if lf_checker_rt::global::<u32>(0x17ac3c4).read() != 0 {
            return callee_cdecl!(3, u32, s1, s2, n, 0);
        }
        if s1 == 0 || s2 == 0 || n > 0x7FFF_FFFF {
            let slot = callee_cdecl!(1, u32,);
            (slot as *mut u32).write(0x16);
            callee_cdecl!(2, u32,);
            return 0x7FFF_FFFF;
        }
        callee_cdecl!(4, u32, s1, s2, n)
    }
});
