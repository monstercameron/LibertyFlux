// original: 0x00c073c0 stream_name_remove (proposed)

/// Remove the entry named by the string from the two parallel tables.
///
/// `this` points to the set (`NAMES` the name-pointer array, `NCOUNT` its
/// 16-bit length, `SLOTS` the parallel value array, `SCOUNT` its 16-bit
/// length). The first name comparing equal to `key` (byte-wise, two bytes at
/// a time) is handed to the release helper (callee 1) then dropped: the names
/// table shifts down and both lengths shrink by one, wrapping past zero. When
/// nothing matches, the slot one past the end is released instead and the
/// lengths still shrink. The names-table shift loop is odd: after the first
/// moved entry its counter becomes the low word of the running table pointer
/// (not the index plus one), so with ordinary heap addresses it stops after
/// one entry however long the table is; the slots-table shift loop beside it
/// counts properly. Returns the slots length minus one.
///
/// Original: 0x00c073c0 (thiscall, one stack word; helper is cdecl, 1 arg).
lf_checker_rt::export!(thiscall, rw_00c073c0(this: u32, key: u32) -> u32 {
    unsafe {
        const NAMES: u32 = 0x10;
        const NCOUNT: u32 = 0x14;
        const SLOTS: u32 = 0x18;
        const SCOUNT: u32 = 0x1c;
        const FREE: u32 = 1;
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
        let n = (this.wrapping_add(NCOUNT) as *const u16).read_unaligned() as u32;
        let names = (this.wrapping_add(NAMES) as *const u32).read_unaligned();
        let mut idx = 0u32;
        if (n as i32) > 0 {
            while idx < n {
                let s = (names.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned();
                if equal(s, key) {
                    break;
                }
                idx += 1;
            }
        }
        let victim = (names.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned();
        let _r: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, victim);
        // Names-table shift with the original's pointer-derived counter.
        if (idx as i32) < (n as i32).wrapping_sub(1) {
            let mut cur = idx;
            let mut ptr = names.wrapping_add(idx.wrapping_mul(4));
            loop {
                let base = (this.wrapping_add(NAMES) as *const u32).read_unaligned();
                ptr = ptr.wrapping_add(1);
                let dest = base.wrapping_add(cur.wrapping_mul(4));
                let v = (dest.wrapping_add(4) as *const u32).read_unaligned();
                (dest as *mut u32).write_unaligned(v);
                let n2 = (this.wrapping_add(NCOUNT) as *const u16).read_unaligned() as u32;
                cur = ptr & 0xffff;
                if !((cur as i32) < (n2 as i32).wrapping_sub(1)) {
                    break;
                }
            }
        }
        (this.wrapping_add(NCOUNT) as *mut u16)
            .write_unaligned(n.wrapping_sub(1) as u16);
        // Slots-table shift, counted properly.
        let n2 = (this.wrapping_add(SCOUNT) as *const u16).read_unaligned() as u32;
        let slots = (this.wrapping_add(SLOTS) as *const u32).read_unaligned();
        let mut j = idx;
        while (j as i32) < (n2 as i32).wrapping_sub(1) {
            let dest = slots.wrapping_add(j.wrapping_mul(4));
            let v = (dest.wrapping_add(4) as *const u32).read_unaligned();
            (dest as *mut u32).write_unaligned(v);
            j += 1;
        }
        (this.wrapping_add(SCOUNT) as *mut u16)
            .write_unaligned(n2.wrapping_sub(1) as u16);
        n2.wrapping_sub(1)
    }
});
