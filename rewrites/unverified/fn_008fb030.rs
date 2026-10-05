// original: 0x008FB030 Text_CopyLiteral
/// Bounded 16-bit string copy with zero fill for a null source.
///
/// Copies at most `count - 1` words from `src` to `dst`, stopping at
/// a source NUL, then writes a NUL terminator; a null source fills
/// `count` words with zero instead. A non-positive `count - 1` skips
/// the loop and just terminates. Returns the number of words copied
/// (before the terminator). Cdecl, three stack arguments.
export!(cdecl, rw_008fb030(dst: u32, src: u32, count: u32) -> u32 {
    unsafe {
        let mut n: u32 = 0;
        let lim = count.wrapping_sub(1);
        if src == 0 {
            if (lim as i32) > 0 {
                let mut i: u32 = 0;
                loop {
                    ((dst + i.wrapping_mul(2)) as *mut u16).write_unaligned(0);
                    n = n.wrapping_add(1);
                    i = n;
                    if !((i as i32) < (lim as i32)) {
                        break;
                    }
                }
            }
        } else if (lim as i32) > 0 {
            let mut i: u32 = 0;
            loop {
                let w = ((src + i.wrapping_mul(2)) as *const u16).read_unaligned();
                if w == 0 {
                    break;
                }
                ((dst + i.wrapping_mul(2)) as *mut u16).write_unaligned(w);
                n = n.wrapping_add(1);
                i = n;
                if !((i as i32) < (lim as i32)) {
                    break;
                }
            }
        }
        ((dst + n.wrapping_mul(2)) as *mut u16).write_unaligned(0);
        n & 0xFFFF
    }
});
