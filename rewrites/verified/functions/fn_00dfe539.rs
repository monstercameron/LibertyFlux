// original: 0x00dfe539 str_to_lower
// rs03f16: locale-sensitive in-place lowercasing (cdecl/1).
//
// When the locale override flag is set, delegates to the locale-aware
// converter (cdecl/3 with trailing arguments -1 and 0) and returns the input
// unchanged. Otherwise a null input reports 0x16 through the errno slot and
// returns zero, while a real string is folded from ASCII uppercase to
// lowercase in place up to its NUL and returned unchanged.
export!(cdecl, rw_rs03f16(s: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x17AC3C4) != 0 {
            callee_cdecl!(1, u32, s, 0xFFFFFFFF, 0);
            return s;
        }
        if s == 0 {
            let slot = callee_cdecl!(2, u32,);
            *(slot as *mut u32) = 0x16;
            callee_cdecl!(3, u32,);
            return 0;
        }
        let mut p = s as *mut u8;
        while *p != 0 {
            let b = *p;
            if (0x41..=0x5A).contains(&b) {
                *p = b.wrapping_add(0x20);
            }
            p = p.add(1);
        }
        s
    }
});
