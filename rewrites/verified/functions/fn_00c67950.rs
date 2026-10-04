// original: 0x00C67950 cutscene_track_best_pick (proposed)
//
// stdcall (two stack words: table index idx, flag word whose low byte
// matters; callee pops 8). Picks the best entry of a per-index word table.
//
// Layout (all file VAs): COUNT[idx] is the u32 at 0x0169E248+idx*4;
// TABLE[idx] is a u16 array at 0x0169C488+idx*80; T is the pointer table at
// 0x01295CD8; G is the handle word at 0x012B4138. Each T[w] points at a
// record with a u32 threshold at +0x94 and flag bits at +0x120 (bit 1 used).
//
// When the flag byte is zero and idx-0x1e <= 0xb, a first pass sets two mode
// flags f12/f13 (both start 1): for each table word w, when the predicate
// callee P(w, G) accepts and the veto callee Q(w, G) does not, the record's
// bit 1 clears f12 when set and f13 when clear. Afterwards f13 is cleared
// when both survived (or when the count was not positive). A nonzero flag
// byte skips this pass with f12=1, f13=0; an out-of-range idx skips it with
// both 0.
//
// The second pass scans the same table for the best word: a word is skipped
// when P accepts it or when the fallback callee R(w) accepts it; otherwise,
// once a best exists, it is skipped when its threshold is above the best
// threshold (unsigned), when f12 is set but its bit 1 is clear, or when f13
// is set but its bit 1 is set. The surviving word with the lowest threshold
// wins (ties keep the earlier one). Returns the best word, or -1 when the
// table is empty or nothing survived.
lf_checker_rt::export!(stdcall, rw_00C67950(idx: u32, flag: u32) -> u32 {
    unsafe {
        const COUNT_BASE: u32 = 0x0169E248;
        const TABLE_BASE: u32 = 0x0169C488;
        const PTR_TABLE: u32 = 0x01295CD8;
        const HANDLE: u32 = 0x012B4138;
        const ROW_PITCH: u32 = 80;
        const THRESH_OFF: u32 = 0x94;
        const FLAGS_OFF: u32 = 0x120;
        const KEEP_BIT: u32 = 2;
        const RANGE_LO: u32 = 0x1e;
        const RANGE_HI: u32 = 0x0b;
        const NONE: u32 = 0xFFFF_FFFF;
        const PRED: u32 = 1;
        const VETO: u32 = 2;
        const FALLBACK: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        let g = rd32(lf_checker_rt::relocated(HANDLE));
        let count_addr = lf_checker_rt::relocated(COUNT_BASE).wrapping_add(idx.wrapping_mul(4));
        let row_addr = lf_checker_rt::relocated(TABLE_BASE).wrapping_add(idx.wrapping_mul(ROW_PITCH));
        let ptrs = lf_checker_rt::relocated(PTR_TABLE);

        let mut f12 = 0u8;
        let mut f13 = 0u8;
        if (flag as u8) == 0 {
            if idx.wrapping_sub(RANGE_LO) <= RANGE_HI {
                let count = rd32(count_addr) as i32;
                f12 = 1;
                f13 = 1;
                if count > 0 {
                    let mut i = 0i32;
                    loop {
                        let w = rd16(row_addr.wrapping_add((i as u32).wrapping_mul(2)));
                        let p: u8 = lf_checker_rt::callee_cdecl!(PRED, u8, w, g);
                        if p != 0 {
                            let q: u8 = lf_checker_rt::callee_cdecl!(VETO, u8, w, g);
                            if q == 0 {
                                let rec = rd32(ptrs.wrapping_add(w.wrapping_mul(4)));
                                if rd32(rec + FLAGS_OFF) & KEEP_BIT != 0 {
                                    f12 = 0;
                                } else {
                                    f13 = 0;
                                }
                            }
                        }
                        i += 1;
                        if !(i < count) {
                            break;
                        }
                    }
                    if f12 != 0 && f13 != 0 {
                        f13 = 0;
                    }
                } else {
                    f13 = 0;
                }
            }
        } else {
            f12 = 1;
        }

        let mut best_val = NONE;
        let mut best_idx = NONE;
        let count2 = rd32(count_addr) as i32;
        if count2 > 0 {
            let mut j = 0i32;
            loop {
                let w = rd16(row_addr.wrapping_add((j as u32).wrapping_mul(2)));
                let p: u8 = lf_checker_rt::callee_cdecl!(PRED, u8, w, g);
                let mut take = false;
                if p == 0 {
                    let r: u8 = lf_checker_rt::callee_cdecl!(FALLBACK, u8, w);
                    if r == 0 {
                        let rec = rd32(ptrs.wrapping_add(w.wrapping_mul(4)));
                        take = true;
                        if best_val != NONE {
                            if rd32(rec + THRESH_OFF) > best_val {
                                take = false;
                            } else if f12 != 0 && rd32(rec + FLAGS_OFF) & KEEP_BIT == 0 {
                                take = false;
                            } else if f13 != 0 && rd32(rec + FLAGS_OFF) & KEEP_BIT != 0 {
                                take = false;
                            }
                        }
                        if take {
                            best_val = rd32(rec + THRESH_OFF);
                            best_idx = w;
                        }
                    }
                }
                j += 1;
                if !(j < count2) {
                    break;
                }
            }
        }
        best_idx
    }
});
