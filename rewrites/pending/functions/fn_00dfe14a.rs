// original: 0x00dfe14a wcs_copy_checked
// rs03f13: bounded wide-character copy with error reporting (cdecl/3).
//
// Copies up to `n` wide characters from `src` to `dst`, stopping after an
// embedded NUL. A null destination or a zero count reports 0x16 without
// touching memory; a null source clears the destination word first and then
// reports 0x16. A copy that exhausts the count without finding a NUL clears
// the destination and reports 0x22. Success returns 0. Reports go through
// the errno slot provider (cdecl/0 callee 1) and the handler (cdecl/0
// callee 2).
export!(cdecl, rw_rs03f13(dst: u32, n: u32, src: u32) -> u32 {
    unsafe {
        if dst == 0 || n == 0 {
            *(callee_cdecl!(1, u32,) as *mut u32) = 0x16;
            callee_cdecl!(2, u32,);
            return 0x16;
        }
        if src == 0 {
            *(dst as *mut u16) = 0;
            *(callee_cdecl!(1, u32,) as *mut u32) = 0x16;
            callee_cdecl!(2, u32,);
            return 0x16;
        }
        let mut d = dst as *mut u16;
        let mut s = src as *const u16;
        let mut left = n;
        loop {
            let w = *s;
            *d = w;
            s = s.add(1);
            d = d.add(1);
            if w == 0 {
                return 0;
            }
            left -= 1;
            if left == 0 {
                break;
            }
        }
        *(dst as *mut u16) = 0;
        *(callee_cdecl!(1, u32,) as *mut u32) = 0x22;
        callee_cdecl!(2, u32,);
        0x22
    }
});
