// original: 0x00dfdeb3 wcsstr
/// `wcsstr`: address of the first `needle` occurrence in `haystack`.
///
/// An empty needle matches at the start; no match returns null. The
/// original dispatches on a CPU flag between SSE4.2, SSE2 and scalar
/// scans; all compute this same function.
export!(cdecl, rw_00dfdeb3(haystack: *const u16, needle: *const u16) -> u32 {
    unsafe {
        if *needle == 0 {
            return haystack as u32;
        }
        let mut h = haystack;
        while *h != 0 {
            let mut a = h;
            let mut b = needle;
            while *b != 0 && *a == *b {
                a = a.add(1);
                b = b.add(1);
            }
            if *b == 0 {
                return h as u32;
            }
            h = h.add(1);
        }
        0
    }
});
