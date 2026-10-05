// original: 0x008FAF60 Text_AppendBounded
/// Append one 16-bit string to another within a word budget.
///
/// A null destination or source ends the call at once (the original
/// then returns whatever was in eax, so the return value is not
/// compared). Otherwise the destination length comes from the length
/// routine: a length already at or past `max - 1` truncates by
/// terminating at `max - 1`, else source words are copied until the
/// budget fills or a source NUL, then a terminator is written. Cdecl,
/// three stack arguments (dst, src, max words).
export!(cdecl, rw_008faf60(dst: u32, src: u32, max: u32) -> u32 {
    unsafe {
        if dst == 0 {
            return 0;
        }
        if src == 0 {
            return 0;
        }
        let l: u32 = callee_cdecl!(1, u32, dst);
        let n = l & 0xFFFF;
        let lim = max.wrapping_sub(1);
        if (n as i32) >= (lim as i32) {
            ((dst + max.wrapping_mul(2).wrapping_sub(2)) as *mut u16).write_unaligned(0);
            return 0;
        }
        let mut i = n;
        let mut p = src;
        loop {
            let w = ((p) as *const u16).read_unaligned();
            if w == 0 {
                break;
            }
            ((dst + i.wrapping_mul(2)) as *mut u16).write_unaligned(w);
            i = i.wrapping_add(1);
            p = p.wrapping_add(2);
            if (i as i32) >= (lim as i32) {
                break;
            }
        }
        if (i as i32) > (lim as i32) {
            ((dst + max.wrapping_mul(2).wrapping_sub(2)) as *mut u16).write_unaligned(0);
        } else {
            ((dst + i.wrapping_mul(2)) as *mut u16).write_unaligned(0);
        }
        i
    }
});
