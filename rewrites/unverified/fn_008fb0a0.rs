// original: 0x008FB0A0 NativeImpl_IS_THIS_PRINT_BEING_DISPLAYED_2
/// Bounded byte-to-word string copy with zero fill for a null source.
///
/// Copies at most `count - 1` bytes from `src` to `dst` widened to
/// 16-bit words, stopping at a source NUL, then writes a NUL word; a
/// null source fills with zero words instead. A non-positive
/// `count - 1` skips the loop and just terminates. Always returns 0.
/// Cdecl, three stack arguments (dst, src, count).
export!(cdecl, rw_008fb0a0(dst: u32, src: u32, count: u32) -> u32 {
    unsafe {
        let lim = count.wrapping_sub(1);
        let mut d: u32 = 0;
        if src == 0 {
            if (lim as i32) > 0 {
                d = lim;
                for i in 0..lim {
                    ((dst + i.wrapping_mul(2)) as *mut u16).write_unaligned(0);
                }
            }
        } else if (lim as i32) > 0 {
            loop {
                let b = ((src + d) as *const u8).read();
                if b == 0 {
                    break;
                }
                ((dst + d.wrapping_mul(2)) as *mut u16).write_unaligned(b as u16);
                d = d.wrapping_add(1);
                if (d as i32) >= (lim as i32) {
                    break;
                }
            }
        }
        ((dst + d.wrapping_mul(2)) as *mut u16).write_unaligned(0);
        0
    }
});
