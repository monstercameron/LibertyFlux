// original: 0x008b78a0 helper_text_format
/// Helper-text resolver.
///
/// Looks up the display strings for `key`: a fast-path hit sets `flag0` and
/// clears `flag1`; a secondary hit clears `flag0` and sets `flag1`; otherwise
/// the primary string is copied to `buf2` (truncated at 14 characters when it
/// is exactly 17 long with a colon at index 14) and the secondary string to
/// `buf1`, clearing both flags. Null flags are skipped. Returns `flag1`.
export!(cdecl, rw_008b78a0(key: u32, buf1: u32, buf2: u32, flag0: u32, flag1: u32) -> u32 {
    unsafe {
        let hit: u32 = callee_cdecl!(2, u32, key, 1, 0);
        if (hit as u8) != 0 {
            if flag0 != 0 {
                (flag0 as *mut u8).write(1);
            }
        } else {
            let alt: u32 = callee_cdecl!(3, u32, key);
            if (alt as u8) != 0 {
                if flag0 != 0 {
                    (flag0 as *mut u8).write(0);
                }
                if flag1 != 0 {
                    (flag1 as *mut u8).write(1);
                }
                return flag1;
            }
            let src1: u32 = callee_cdecl!(4, u32, key);
            let mut src = src1;
            let mut dst = buf2;
            loop {
                let b = (src as *const u8).read();
                (dst as *mut u8).write(b);
                if b == 0 {
                    break;
                }
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
            }
            let mut len: u32 = 0;
            while ((buf2.wrapping_add(len)) as *const u8).read() != 0 {
                len = len.wrapping_add(1);
            }
            if len == 0x11 && ((buf2.wrapping_add(0xE)) as *const u8).read() == 0x3A {
                ((buf2.wrapping_add(0xE)) as *mut u8).write(0);
            }
            let tmp = 0u32;
            let src2: u32 = callee_cdecl!(5, u32, key, 0, &tmp as *const u32 as u32);
            src = src2;
            dst = buf1;
            loop {
                let b = (src as *const u8).read();
                (dst as *mut u8).write(b);
                if b == 0 {
                    break;
                }
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
            }
            if flag0 != 0 {
                (flag0 as *mut u8).write(0);
            }
        }
        if flag1 != 0 {
            (flag1 as *mut u8).write(0);
        }
        flag1
    }
});
