// original: 0x00cb7220 id_pair_or_neg1
/// Stores the found record's id pair (+0x50/+0x54) or -1/-1; returns as above.
export!(thiscall, rw_cb7220(this_ptr: u32, arg0: u32) -> u32 {
    unsafe {
        let inner = ((arg0.wrapping_add(0xA80)) as *const u32).read();
        let key = ((inner.wrapping_add(0x3C)) as *const u32).read();
        let found: u32 = lf_checker_rt::callee_cdecl!(1, u32, key);
        if found == 0 {
            ((this_ptr.wrapping_add(0x50)) as *mut u32).write(0xFFFF_FFFF);
            ((this_ptr.wrapping_add(0x54)) as *mut u32).write(0xFFFF_FFFF);
            0
        } else {
            let flags = ((found.wrapping_add(0x378)) as *const u32).read();
            if (flags >> 10) & 1 == 0 {
                ((this_ptr.wrapping_add(0x50)) as *mut u32).write(0xFFFF_FFFF);
                ((this_ptr.wrapping_add(0x54)) as *mut u32).write(0xFFFF_FFFF);
                found
            } else {
                let a = (found as *const u32).read();
                let b = ((found.wrapping_add(4)) as *const u32).read();
                ((this_ptr.wrapping_add(0x50)) as *mut u32).write(a);
                ((this_ptr.wrapping_add(0x54)) as *mut u32).write(b);
                b
            }
        }
    }
});
