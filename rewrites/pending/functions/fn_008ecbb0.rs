// original: 0x008ecbb0 NativeImpl_HAS_CAR_STOPPED_BECAUSE_OF_LIGHT
/// Test whether a flagged link joins two tracked node pairs.
///
/// Three guarded blocks scan the link arrays of nodes named by the id
/// fields on `this` (+0xDDC/+0xDE0/+0xDE4, then two sliding pairs); the
/// first link whose far end matches the expected id and carries a
/// `0xC0` flag byte decides true. Returns 1 or 0 in AL.
export!(thiscall, rw_008ecbb0(this: *const u8, flag: u32) -> u32 {
    unsafe {
        let w = |off: u32| *(this.add(off as usize) as *const u32);
        let table1 = |i: u32| *global::<u32>(0x1178284 + i * 4);
        let table2 = |i: u32| *global::<u32>(0x1178384 + i * 4);
        // Scan node's link fan for `target` with a 0xC0 flag. Returns
        // true on the first flagged match.
        let scan = |nbase: u32, nidx: u32, lbase: u32, target: u32| {
            let node = nbase.wrapping_add(nidx.wrapping_mul(32));
            let count = (*((node + 0x1e) as *const u8) & 0xF) as u32;
            if count == 0 {
                return false;
            }
            let base =
                ((node + 0x12) as *const i16).read_unaligned() as i32;
            let mut i = 0u32;
            while i < count {
                let at = (base + i as i32) as u32;
                let slot = lbase.wrapping_add(at.wrapping_mul(8));
                if *(slot as *const u32) == target
                    && *((slot + 5) as *const u8) & 0xC0 != 0
                {
                    return true;
                }
                i += 1;
            }
            false
        };
        if w(0x28) & 0x7c00 != 0x400 {
            return 0;
        }
        let d_e0 = w(0xDE0);
        if d_e0 & 0xFFFF == 0xFFFF {
            return 0;
        }
        if table1(d_e0 & 0xFFFF) == 0 {
            return 0;
        }
        let d_e4 = w(0xDE4);
        if d_e4 & 0xFFFF == 0xFFFF {
            return 0;
        }
        if table1(d_e4 & 0xFFFF) == 0 {
            return 0;
        }
        if scan(
            table1(d_e0 & 0xFFFF),
            d_e0 >> 16,
            table2(d_e0 & 0xFFFF),
            d_e4,
        ) {
            return 1;
        }
        let d_dc = w(0xDDC);
        if d_dc & 0xFFFF != 0xFFFF {
            if table1(d_dc & 0xFFFF) == 0 {
                return 0;
            }
            if scan(
                table1(d_dc & 0xFFFF),
                d_dc >> 16,
                table2(d_dc & 0xFFFF),
                d_e0,
            ) {
                return 1;
            }
        }
        if flag & 0xFF == 0 {
            return 0;
        }
        let mut k = 0u32;
        while k < 2 {
            let a = w(0xDE4 + k * 4);
            let b = w(0xDE8 + k * 4);
            if b & 0xFFFF == 0xFFFF {
                return 0;
            }
            if table1(b & 0xFFFF) == 0 {
                return 0;
            }
            if scan(
                table1(a & 0xFFFF),
                a >> 16,
                table2(a & 0xFFFF),
                b,
            ) {
                return 1;
            }
            k += 1;
        }
        0
    }
});
