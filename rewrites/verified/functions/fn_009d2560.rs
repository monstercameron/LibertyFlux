// original: 0x009D2560 record_compare_4key (proposed)
//
/// Four-key record comparison returning -1, 0 or 1.
///
/// Compares in order: dword at `+0x00` UNSIGNED, dword at `+0x04` UNSIGNED,
/// dword at `+0x0c` SIGNED, dword at `+0x08` UNSIGNED. The first differing
/// key decides (1 when `a` is greater, -1 when less); fully equal records
/// return 0. Cdecl, two record pointers.
lf_checker_rt::export!(cdecl, rw_009D2560(a: u32, b: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd(p: u32, off: u32) -> u32 {
            unsafe { (p.wrapping_add(off) as *const u32).read_unaligned() }
        }
        let d = rd(a, 0x00); let s = rd(b, 0x00);
        if d != s { return if d > s { 1 } else { 0xffffffff }; }
        let d = rd(a, 0x04); let s = rd(b, 0x04);
        if d != s { return if d > s { 1 } else { 0xffffffff }; }
        let d = rd(a, 0x0c) as i32; let s = rd(b, 0x0c) as i32;
        if d != s { return if d > s { 1 } else { 0xffffffff }; }
        let d = rd(a, 0x08); let s = rd(b, 0x08);
        if d > s { 1 } else if d < s { 0xffffffff } else { 0 }
    }
});
