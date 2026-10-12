// original: 0x00C79C20 copy_line_until_newline

/// Copy a line from src to dst, stopping before a line-feed or NUL (neither is copied), NUL-terminate dst and return the address just past the stop character. The destination always gets its terminator, even for an empty line. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_c79c20(src: u32, dst: u32) -> u32 {
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

        let mut p = src;
                let mut n = 0u32;
                if rd8(p) != 0x0a {
                    loop {
                        let b = rd8(p);
                        if b == 0 { break; }
                        wr8(dst.wrapping_add(n), b);
                        p = p.wrapping_add(1);
                        n += 1;
                        if rd8(p) == 0x0a { break; }
                    }
                }
                wr8(dst.wrapping_add(n), 0);
                p.wrapping_add(1)
        }
});
