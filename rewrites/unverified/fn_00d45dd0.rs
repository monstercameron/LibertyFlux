// original: 0x00D45DD0 peds_task_collect_candidates (proposed)

/// Collect up to 32 candidate values, then publish one of them.
///
/// `kind_obj` selects a table row (its `+0x2e` word indexes the kind table;
/// bit 1 of that row's `+0x120` word becomes the check flag passed on to the
/// filter helper). `index` chooses the collection path: non-negative collects
/// through the indexed table (global at `G2`, stride `index * 0x58`), while
/// `-1` (or an empty indexed table) collects through the sequential table
/// rooted at `slot_a[0]`. `slot_a` receives the resolve helper's answer on
/// the indexed path; `slot_b` receives the published value; `wanted`, when
/// non-zero, names a value that ends the search immediately when produced.
///
/// Behaviour: on the indexed path each entry is mapped, filter-tested with
/// the check flag, and resolved; a resolved value equal to a non-zero
/// `wanted` is published at once with success. Otherwise values accumulate
/// (at most 32). On the sequential path each entry is fetched twice (the
/// first fetch is what is kept) and filter-tested the same way. When at least
/// one value was collected, a random one is picked by
/// `trunc(rand16 * K * count)` with `K = 1/32768` and published with success;
/// with none collected the result is failure. Only the low return byte is
/// meaningful (1 success, 0 failure); the upper bytes keep stale eax.
///
/// Original: 0x00D45DD0 (cdecl, five stack words, returns u32, al significant).
lf_checker_rt::export!(cdecl, rw_00D45DD0(kind_obj: u32, index: u32, slot_a: u32, slot_b: u32, wanted: u32) -> u32 {
    unsafe {
        const KIND_TABLE: u32 = 0x0129_5CD8;
        const KIND_WORD: u32 = 0x120;
        const MODEL_ID: u32 = 0x2e;
        const SINGLETON: u32 = 0x016D_D63C;
        const INDEX_BASE: u32 = 0x016D_D640;
        const INDEX_OFF: u32 = 0x44;
        const STRIDE: u32 = 0x58;
        const ENTRY_STEP: u32 = 12;
        const COUNT_OFF: u32 = 0x38;
        const MAX_CANDS: usize = 32;
        const PICK_SCALE: f32 = f32::from_bits(0x3800_0000);
        const RESOLVE_CALLEE: u32 = 1;
        const OPEN_CALLEE: u32 = 2;
        const MAP_CALLEE: u32 = 3;
        const FILTER_CALLEE: u32 = 4;
        const PRODUCE_CALLEE: u32 = 5;
        const SEQ_COUNT_CALLEE: u32 = 6;
        const SEQ_FETCH_CALLEE: u32 = 7;
        const SEQ_MAP_CALLEE: u32 = 8;
        const RAND_CALLEE: u32 = 9;
        const COOKIE_CALLEE: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,) };
        }
        #[inline(always)]
        unsafe fn singleton() -> u32 {
            unsafe { lf_checker_rt::global::<u32>(SINGLETON).read_unaligned() }
        }

        let row = rd32(
            (lf_checker_rt::global::<u32>(KIND_TABLE) as u32)
                .wrapping_add((rd16(kind_obj.wrapping_add(MODEL_ID)) as i16 as i32 as u32).wrapping_mul(4)),
        );
        let flag = if rd32(row.wrapping_add(KIND_WORD)) & 2 != 0 { 1u32 } else { 0u32 };
        let mut cands = [0u32; MAX_CANDS];
        let mut n = 0usize;
        let mut indexed_ran = false;
        if index != 0xFFFF_FFFF {
            let opened = lf_checker_rt::callee_thiscall!(RESOLVE_CALLEE, u32, singleton(), index);
            wr32(slot_a, opened);
            let obj = lf_checker_rt::callee_thiscall!(OPEN_CALLEE, u32, singleton(), index);
            if rd32(obj.wrapping_add(COUNT_OFF)) as i32 > 0 {
                let index_table =
                    lf_checker_rt::global::<u32>(INDEX_BASE).read_unaligned();
                let base = rd32(
                    index_table
                        .wrapping_add(index.wrapping_mul(STRIDE))
                        .wrapping_add(INDEX_OFF),
                );
                let mut i = 0u32;
                loop {
                    let item = rd32(base.wrapping_add(i.wrapping_mul(ENTRY_STEP)));
                    let mapped = lf_checker_rt::callee_cdecl!(MAP_CALLEE, u32, index, item);
                    if mapped != 0 {
                        let ok = lf_checker_rt::callee_cdecl!(FILTER_CALLEE, u32, mapped, flag);
                        if ok & 0xFF != 0 {
                            let v = lf_checker_rt::callee_thiscall!(
                                PRODUCE_CALLEE, u32, singleton(), index, item
                            );
                            if wanted != 0 && v == wanted {
                                wr32(slot_b, wanted);
                                cookie();
                                return 1;
                            }
                            if n < MAX_CANDS {
                                cands[n] = v;
                                n += 1;
                            }
                        }
                    }
                    i += 1;
                    if i as i32 >= rd32(obj.wrapping_add(COUNT_OFF)) as i32 {
                        break;
                    }
                }
                indexed_ran = true;
            }
        }
        if !indexed_ran {
            let total = lf_checker_rt::callee_cdecl!(SEQ_COUNT_CALLEE, u32, rd32(slot_a));
            if total as i32 <= 0 {
                cookie();
                return 0;
            }
            let mut i = 0u32;
            while (i as i32) < (total as i32) {
                let fetched =
                    lf_checker_rt::callee_cdecl!(SEQ_FETCH_CALLEE, u32, rd32(slot_a), i);
                let mapped =
                    lf_checker_rt::callee_cdecl!(SEQ_MAP_CALLEE, u32, rd32(slot_a), fetched);
                if mapped != 0 {
                    let ok = lf_checker_rt::callee_cdecl!(FILTER_CALLEE, u32, mapped, flag);
                    if ok & 0xFF != 0 && n < MAX_CANDS {
                        cands[n] = fetched;
                        n += 1;
                    }
                }
                i += 1;
            }
        }
        if n == 0 {
            cookie();
            return 0;
        }
        let rand = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
        let scaled = mul(mul((rand & 0xFFFF) as f32, PICK_SCALE), n as f32);
        let pick = scaled as i32 as u32;
        wr32(slot_b, cands[pick as usize]);
        cookie();
        1
    }
});
