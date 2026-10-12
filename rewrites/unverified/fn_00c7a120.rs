// original: 0x00C7A120 read_line_advance

/// Copy the line at *cursor to dst, stopping before a line-feed or NUL (neither is copied), NUL-terminate dst and advance *cursor past the consumed characters plus one. A leading line-feed copies nothing but still advances. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_c7a120(cursor: u32, dst: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        let mut p = rd32(cursor);
                if rd8(p) == 0x0a {
                    wr8(dst, 0);
                    wr32(cursor, p.wrapping_add(1));
                    return cursor;
                }
                let mut n = 0u32;
                loop {
                    p = rd32(cursor);
                    let b = rd8(p);
                    if b == 0 { break; }
                    wr8(dst.wrapping_add(n), b);
                    wr32(cursor, p.wrapping_add(1));
                    n += 1;
                    p = rd32(cursor);
                    if rd8(p) == 0x0a { break; }
                }
                wr8(dst.wrapping_add(n), 0);
                wr32(cursor, rd32(cursor).wrapping_add(1));
                cursor
        }
});
