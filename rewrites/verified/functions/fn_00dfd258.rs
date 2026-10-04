// original: 0x00dfd258 tokenize_next
// rs03f02: delimiter tokenizer over a caller string (cdecl/2).
//
// Splits the string at `state` (or at the saved position in the shared
// record, provided by cdecl/0 callee 1, when `state` is null) using the
// NUL-terminated `delims` set plus NUL itself. Leading delimiters are
// skipped, the token is cut with an in-place NUL when a delimiter follows
// it, the new position is stored back at record offset 0x18, and the token
// start is returned, or null when nothing but delimiters remains.
export!(cdecl, rw_rs03f02(state: u32, delims: *const u8) -> u32 {
    unsafe {
        let rec = callee_cdecl!(1, u32,);
        let mut bits = [0u8; 32];
        let mut d = delims;
        loop {
            let b = *d;
            bits[(b >> 3) as usize] |= 1 << (b & 7);
            d = d.add(1);
            if b == 0 {
                break;
            }
        }
        let mut s = if state == 0 {
            *((rec + 0x18) as *const u32)
        } else {
            state
        };
        loop {
            let b = *(s as *const u8);
            if bits[(b >> 3) as usize] & (1 << (b & 7)) == 0 {
                break;
            }
            if b == 0 {
                break;
            }
            s = s.wrapping_add(1);
        }
        let token = s;
        loop {
            let b = *(s as *const u8);
            if b == 0 {
                break;
            }
            if bits[(b >> 3) as usize] & (1 << (b & 7)) != 0 {
                *(s as *mut u8) = 0;
                s = s.wrapping_add(1);
                break;
            }
            s = s.wrapping_add(1);
        }
        *((rec + 0x18) as *mut u32) = s;
        if s == token {
            0
        } else {
            token
        }
    }
});
