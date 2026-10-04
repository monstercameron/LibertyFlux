// original: 0x008bf180 input_rebind_device_key (proposed)

/// Look up a device's key record and rewrite the current device's binding bytes.
///
/// `dev` selects a row of the per-device table (`T_DEVS`, rows of 24 bytes:
/// record-array pointer at `+0`, record count as a 16-bit word at `+4`).
/// Each record is `REC_LEN` bytes with a signed 16-bit key at `+0x12` and a
/// binding id byte at `+0x15`. `in_ecx` is a slot index into the global slot
/// array `G_SLOTS` (dwords).
///
/// The two answers of the key callee for `(dev, 0x1e)` and `(dev, 0x1d)`
/// (`NO_LINK` answers are replaced by `ALT_A`/`ALT_B`) are searched for in
/// the record keys: the first must match (otherwise the last key read is
/// returned), the second picks a record (record 0 when it matches nothing)
/// whose id byte seeds a scan of the 60-row binding table `T_DEFS` (rows of
/// 12 bytes: id at `+0`, value-list pointer at `+4`, limit as a 16-bit word
/// at `+8`). On a hit whose limit does not exceed the slot value, the slot
/// is cleared and the output id is read from the value list; a second scan
/// resolves the output id back to a limit. Finally the current device's row
/// (selected by `G_CUR_INDEX`) has the found record's id byte replaced and
/// its neighbour byte set to the resolved limit, and stale slots are
/// cleared. An empty record list returns `dev * 3`.
///
/// Original: 0x008bf180 (thiscall, incoming ECX plus one stack word).
lf_checker_rt::export!(thiscall, rw_008bf180(in_ecx: u32, dev: u32) -> u32 {
    unsafe {
        const G_CUR_INDEX: u32 = 0x01160C40;
        const G_SLOTS: u32 = 0x01160C48;
        const T_DEFS: u32 = 0x019D30C0;
        const T_DEF_END: u32 = 0x019D3390;
        const T_DEVS: u32 = 0x019D33A0;
        const REC_LEN: u32 = 0x16;
        const REC_KEY: u32 = 0x12;
        const REC_ID: u32 = 0x15;
        const REC_OUT: u32 = 0x14;
        const NO_LINK: u32 = 0x7FFF_FFFF;
        const ALT_A: u32 = 0x2f;
        const ALT_B: u32 = 0x2e;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let mut pair_a = lf_checker_rt::callee_cdecl!(1, u32, dev, 0x1e_u32);
        if pair_a == NO_LINK {
            pair_a = ALT_A;
        }
        let mut pair_b = lf_checker_rt::callee_cdecl!(2, u32, dev, 0x1d_u32);
        if pair_b == NO_LINK {
            pair_b = ALT_B;
        }
        let dev_row = lf_checker_rt::relocated(T_DEVS).wrapping_add(dev.wrapping_mul(24));
        let recs = rd32(dev_row);
        let n = rd16(dev_row.wrapping_add(4)) as i32;
        if n <= 0 {
            return dev.wrapping_mul(3);
        }
        // The first answer must match a record key; otherwise the last key
        // read is the result.
        let mut j = 0i32;
        let mut w = 0u32;
        loop {
            w = (rd16(recs.wrapping_add((j as u32).wrapping_mul(REC_LEN).wrapping_add(REC_KEY))) as i16 as i32) as u32;
            if pair_a == w {
                break;
            }
            j += 1;
            if j >= n {
                return w;
            }
        }
        // The second answer picks a record, defaulting to record 0.
        let mut pick = 0u32;
        let mut k = 0i32;
        loop {
            let v = (rd16(recs.wrapping_add((k as u32).wrapping_mul(REC_LEN).wrapping_add(REC_KEY))) as i16 as i32) as u32;
            if pair_b == v {
                pick = k as u32;
                break;
            }
            k += 1;
            if k >= n {
                pick = 0;
                break;
            }
        }
        let id1 = rd8(recs.wrapping_add(pick.wrapping_mul(REC_LEN).wrapping_add(REC_ID)));
        let mut gval = rd32(lf_checker_rt::relocated(G_SLOTS).wrapping_add(pair_b.wrapping_mul(4)));
        let mut out_id = 0u32;
        let mut t = lf_checker_rt::relocated(T_DEFS);
        let end = lf_checker_rt::relocated(T_DEF_END);
        loop {
            if rd32(t) == id1 as u32 {
                let lim = rd16(t.wrapping_add(8));
                if (gval as i32) >= (lim as i32) {
                    if pair_b != NO_LINK {
                        wr32(lf_checker_rt::relocated(G_SLOTS).wrapping_add(pair_b.wrapping_mul(4)), 0);
                    }
                    gval = 0;
                }
                let q = rd32(t.wrapping_add(4));
                out_id = rd32(q.wrapping_add(gval.wrapping_mul(24).wrapping_add(0x10)));
                break;
            }
            t = t.wrapping_add(12);
            if t >= end {
                break;
            }
        }
        let mut out_lim = 0u32;
        let mut t2 = lf_checker_rt::relocated(T_DEFS);
        loop {
            if rd32(t2) == out_id {
                out_lim = rd16(t2.wrapping_add(8)) as u32;
                break;
            }
            t2 = t2.wrapping_add(12);
            if t2 >= end {
                out_lim = 0;
                break;
            }
        }
        let gidx = rd32(lf_checker_rt::relocated(G_CUR_INDEX));
        let cur = rd32(lf_checker_rt::relocated(T_DEVS).wrapping_add(gidx.wrapping_mul(24)));
        let id2 = rd8(cur.wrapping_add((j as u32).wrapping_mul(REC_LEN).wrapping_add(REC_ID)));
        if (id2 as u32) != out_id {
            if in_ecx != NO_LINK {
                wr32(lf_checker_rt::relocated(G_SLOTS).wrapping_add(in_ecx.wrapping_mul(4)), 0);
            }
        }
        wr8(cur.wrapping_add((j as u32).wrapping_mul(REC_LEN).wrapping_add(REC_ID)), out_id as u8);
        wr8(cur.wrapping_add((j as u32).wrapping_mul(REC_LEN).wrapping_add(REC_OUT)), out_lim as u8);
        cur
    }
});
