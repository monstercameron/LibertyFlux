// original: 0x00dff20b wcsicmp
/// Case-insensitive comparison of two NUL-terminated wide strings.
///
/// Uses the locale-aware worker when a locale is active; otherwise validates
/// the pointers (reporting `EINVAL` and returning `i32::MAX` on failure) and
/// compares ASCII-folded characters until they differ or the terminator.
export!(cdecl, rw_00dff20b(s1: u32, s2: u32) -> u32 {
    unsafe {
        if lf_checker_rt::global::<u32>(0x17ac3c4).read() != 0 {
            return callee_cdecl!(1, u32, s1, s2, 0);
        }
        if s1 == 0 || s2 == 0 {
            let slot = callee_cdecl!(2, u32,);
            (slot as *mut u32).write(0x16);
            callee_cdecl!(3, u32,);
            return 0x7FFF_FFFF;
        }
        let mut p1 = s1;
        let mut p2 = s2;
        loop {
            let w1 = (p1 as *const u16).read();
            let w2 = (p2 as *const u16).read();
            let l1 = if (0x41..=0x5A).contains(&w1) { w1 + 0x20 } else { w1 };
            let l2 = if (0x41..=0x5A).contains(&w2) { w2 + 0x20 } else { w2 };
            p1 = p1.wrapping_add(2);
            p2 = p2.wrapping_add(2);
            if l1 == 0 || l1 != l2 {
                return (l1 as u32).wrapping_sub(l2 as u32);
            }
        }
    }
});
