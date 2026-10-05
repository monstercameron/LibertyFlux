// original: 0x00c080a0 stream_nested_key_search (proposed)

/// Search nested groups for a record whose leading key matches the string.
///
/// `this` points to the set (`GROUPS` holds the group array, `NGROUPS` its
/// 16-bit length; each group is `GROUP_STRIDE` bytes with its record array at
/// `GREC` and 16-bit record count at `GCOUNT`, each record `REC_STRIDE` bytes
/// starting with a NUL-terminated key). A null string, or one starting with a
/// space, skips the search. Returns the string pointer with its low byte
/// replaced by 1 when a match was found, else by 0.
///
/// Original: 0x00c080a0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c080a0(this: u32, key: u32) -> u32 {
    unsafe {
        const GROUPS: u32 = 0x00;
        const NGROUPS: u32 = 0x04;
        const GROUP_FIRST: u32 = 0x20;
        const GROUP_STRIDE: u32 = 0x28;
        const GREC: u32 = 0x00;
        const GCOUNT: u32 = 0x04;
        const REC_STRIDE: u32 = 0x50;
        const SPACE: u8 = 0x20;
        /// Byte-wise equality in the original's read order and width.
        unsafe fn equal(mut a: u32, mut b: u32) -> bool {
            unsafe {
                loop {
                    let c1 = (a as *const u8).read();
                    let d1 = (b as *const u8).read();
                    if c1 != d1 {
                        return false;
                    }
                    if c1 == 0 {
                        return true;
                    }
                    let c2 = (a.wrapping_add(1) as *const u8).read();
                    let d2 = (b.wrapping_add(1) as *const u8).read();
                    if c2 != d2 {
                        return false;
                    }
                    a = a.wrapping_add(2);
                    b = b.wrapping_add(2);
                    if c2 == 0 {
                        return true;
                    }
                }
            }
        }
        if key == 0 {
            return 0;
        }
        if (key as *const u8).read() == SPACE {
            return key & 0xffff_ff00;
        }
        let ng = (this.wrapping_add(NGROUPS) as *const u16).read_unaligned() as u32;
        let mut found = 0u32;
        if (ng as i32) > 0 {
            let groups = (this.wrapping_add(GROUPS) as *const u32).read_unaligned();
            let mut o = 0u32;
            while o < ng {
                let g = groups.wrapping_add(GROUP_FIRST).wrapping_add(o.wrapping_mul(GROUP_STRIDE));
                let nr = (g.wrapping_add(GCOUNT) as *const u16).read_unaligned() as u32;
                if (nr as i32) > 0 {
                    let recs = (g.wrapping_add(GREC) as *const u32).read_unaligned();
                    let mut i = 0u32;
                    while i < nr {
                        if equal(recs.wrapping_add(i.wrapping_mul(REC_STRIDE)), key) {
                            found = 1;
                        }
                        i += 1;
                    }
                }
                o += 1;
            }
        }
        (key & 0xffff_ff00) | found
    }
});
