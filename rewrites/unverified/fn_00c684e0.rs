// original: 0x00C684E0 gather_flagged_ids_or_probe_tables (proposed)

/// Gather flagged ids from two slot tables, else scan hash-seeded tables
/// for a flagged word.
///
/// `this` holds two 64-slot id arrays at `+0x404` and `+0x504` (the stack
/// argument is unread). Each round resets a 64-slot scratch array, counts
/// the table's leading ids, and inserts every id whose pointer-table entry
/// (`[id*4 + table]`, flag word at `+0x124`) has bit 0x400 set. If the round
/// predicate accepts the scratch set, the round is done; otherwise a
/// fallback value is requested and a NON-NEGATIVE one (SIGNED test) is
/// returned at once. If both rounds fall through with negative fallbacks,
/// a hash is formed from two global dwords and a heap byte
/// (`((a + 2*b) * 0x47 + (byte & 0x7f)) * 0x2a + marker_base`) and two walks
/// run over 42 table rows: the first scans rows whose marker byte is
/// nonzero, the second rows whose marker is zero; each row with a positive
/// (SIGNED) count contributes that many words, and the first word whose
/// table entry has bit 0x400 is returned. If nothing matches, `-1`.
///
/// Original: 0x00C684E0 (thiscall, one stack word, unread).
lf_checker_rt::export!(thiscall, rw_00C684E0(this: u32, _arg0: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x0129_5CD8;
        const HASH_A: u32 = 0x0169_C47C;
        const HASH_B: u32 = 0x0169_C478;
        const HASH_HOLDER: u32 = 0x0169_E3DC;
        const WORD_BASE: u32 = 0x0169_C488;
        const COUNT_BASE: u32 = 0x0169_E248;
        const COUNT_END: u32 = 0x0169_E2F0;
        const MARKER_BASE: u32 = 0x0168_ACE8;
        const FLAG_BIT: u32 = 0x400;
        const ROW_STRIDE: u32 = 0x50;
        const CK_ID: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn flagged(table: u32, idx: u32) -> bool {
            unsafe {
                let ptr = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                rd32(ptr.wrapping_add(0x124)) & FLAG_BIT != 0
            }
        }

        let slots = [0u32; 64];
        let slots_p = slots.as_ptr() as u32;
        let table = g32(TABLE_PTR);

        // Round 1 over +0x404.
        lf_checker_rt::callee_thiscall!(1, u32, slots_p);
        let count = lf_checker_rt::callee_thiscall!(2, u32, this + 0x404) as i32;
        if count > 0 {
            let mut i = 0i32;
            while i < count {
                let id = lf_checker_rt::callee_thiscall!(3, u32, this + 0x404, i as u32);
                if flagged(table, id) {
                    lf_checker_rt::callee_thiscall!(4, u32, slots_p, id);
                }
                i += 1;
            }
        }
        if lf_checker_rt::callee_thiscall!(5, u32, slots_p) as u8 == 0 {
            let r = lf_checker_rt::callee_thiscall!(6, u32, slots_p) as i32;
            if r >= 0 {
                lf_checker_rt::callee_cdecl!(CK_ID, u32,);
                return r as u32;
            }
        }
        // Round 2 over +0x504.
        lf_checker_rt::callee_thiscall!(1, u32, slots_p);
        let count2 = lf_checker_rt::callee_thiscall!(2, u32, this + 0x504) as i32;
        if count2 > 0 {
            let mut i = 0i32;
            while i < count2 {
                let id = lf_checker_rt::callee_thiscall!(3, u32, this + 0x504, i as u32);
                if flagged(table, id) {
                    lf_checker_rt::callee_thiscall!(4, u32, slots_p, id);
                }
                i += 1;
            }
        }
        if lf_checker_rt::callee_thiscall!(7, u32, slots_p) as u8 == 0 {
            let r = lf_checker_rt::callee_thiscall!(8, u32, slots_p) as i32;
            if r >= 0 {
                lf_checker_rt::callee_cdecl!(CK_ID, u32,);
                return r as u32;
            }
        }
        // Hash-seeded table probe.
        let a = g32(HASH_A);
        let b = g32(HASH_B);
        let hp = g32(HASH_HOLDER);
        let c = (rd8(hp.wrapping_add(0x20)) & 0x7f) as u32;
        let h = a
            .wrapping_add(b.wrapping_mul(2))
            .wrapping_mul(0x47)
            .wrapping_add(c)
            .wrapping_mul(0x2a)
            .wrapping_add(lf_checker_rt::relocated(MARKER_BASE));
        let mut p = h;
        let mut tb = lf_checker_rt::relocated(COUNT_BASE);
        let tb_end = lf_checker_rt::relocated(COUNT_END);
        let mut wb = lf_checker_rt::relocated(WORD_BASE);
        while (tb as i32) < (tb_end as i32) {
            if rd8(p) != 0 {
                let n = rd32(tb) as i32;
                if n > 0 {
                    let mut k = 0i32;
                    while k < n {
                        let w = rd16(wb.wrapping_add((k as u32).wrapping_mul(2)));
                        if flagged(table, w) {
                            lf_checker_rt::callee_cdecl!(CK_ID, u32,);
                            return w;
                        }
                        k += 1;
                    }
                }
            }
            p = p.wrapping_add(1);
            tb = tb.wrapping_add(4);
            wb = wb.wrapping_add(ROW_STRIDE);
        }
        let mut p2 = h;
        let mut tb2 = lf_checker_rt::relocated(COUNT_BASE);
        let mut wb2 = lf_checker_rt::relocated(WORD_BASE);
        while (tb2 as i32) < (tb_end as i32) {
            if rd8(p2) == 0 {
                let n = rd32(tb2) as i32;
                if n > 0 {
                    let mut k = 0i32;
                    while k < n {
                        let w = rd16(wb2.wrapping_add((k as u32).wrapping_mul(2)));
                        if flagged(table, w) {
                            lf_checker_rt::callee_cdecl!(CK_ID, u32,);
                            return w;
                        }
                        k += 1;
                    }
                }
            }
            p2 = p2.wrapping_add(1);
            tb2 = tb2.wrapping_add(4);
            wb2 = wb2.wrapping_add(ROW_STRIDE);
        }
        lf_checker_rt::callee_cdecl!(CK_ID, u32,);
        0xffff_ffff
    }
});
