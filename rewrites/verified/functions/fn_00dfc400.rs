// original: 0x00dfc400 strchr
/// `strchr`: address of the first `ch` in `s`, or null.
///
/// Only the low byte of `ch` is significant. A NUL search matches the
/// string terminator. The original dispatches on a CPU flag between an SSE
/// and a scalar scan; both compute this same function, so the rewrite is
/// the single scan.
export!(cdecl, rw_00dfc400(s: *const u8, ch: u32) -> u32 {
    unsafe {
        let target = ch as u8;
        let mut p = s;
        loop {
            let b = *p;
            if b == target {
                return p as u32;
            }
            if b == 0 {
                return 0;
            }
            p = p.add(1);
        }
    }
});
