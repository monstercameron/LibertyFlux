// original: 0x009fc880 CPlayStatInt::~CPlayStatInt_2

/// Refresh the playstats collector on its timer tick.
///
/// Takes no arguments and no register inputs (cdecl). Reads a tick from
/// callee 0, stamps it odd, and runs up to four timed stages, each guarded by
/// its own last-run slot and interval: callee 1 re-seeds the collector, the
/// callee 2 block re-resolves the active pair, callee 4 refreshes the cache,
/// and the main body below runs only when the body slot expired. The stamp is
/// recorded in the body slot on exit; the function returns the stamp, or the
/// elapsed time since the body slot when it exits early through the gate.
///
/// The callee 2 block resolves a table entry from callee 2's answer (a small
/// integer at `SEL_OFF` selects one of its words) and, when the entry equals
/// the current word but the pair changed, runs callee 3 on it, then stores
/// the pair. The main body has three bounded loops driven by scripted
/// callees: loop 1 walks `FLOAT_N` floats comparing the live table against
/// the shadow array callee 5 points at (an unordered comparison, including
/// either side NaN, proceeds to the update), loop 2
/// walks `INT_N` words comparing a probed array against the live table, and
/// loop 3 walks `STR_N` string slots resolving each through callee 20. Each
/// iteration is filtered by its loop's gatekeeper call; a changed entry is
/// announced through callee 8/14/21 (thiscall), formatted through callee
/// 9/15/23 (cdecl, buffer and length) and released through callee 10/16/24,
/// and the live word is updated. Finally, when the two generation counters
/// differ, callee 25 formats a generation message (thiscall), callee 26
/// sends it and the shadow counter is synced. Callee 28 is the stack-cookie
/// check (no arguments, registers preserved).
///
/// Only scalar call arguments, the live-table writes, and the return value
/// are compared; the per-iteration message buffers are not snapshotted.
lf_checker_rt::export!(cdecl, rw_009fc880() -> u32 {
    unsafe {
        const STAMP_SLOT: u32 = 0x012B_9008;
        const CACHE_SLOT: u32 = 0x012B_900C;
        const BODY_SLOT: u32 = 0x012B_9010;
        const PAIR_SLOT: u32 = 0x012B_9014;
        const PAIR_CUR: u32 = 0x012B_901C;
        const PAIR_NEW: u32 = 0x012B_9020;
        const RESEED_AFTER: u32 = 0x0103_B58C;
        const CACHE_AFTER: u32 = 0x0103_B590;
        const BODY_AFTER: u32 = 0x0103_B594;
        const PAIR_AFTER: u32 = 0x1388;
        const SEL_OFF: u32 = 0x2B0;
        const FLOAT_TAB: u32 = 0x012B_9190;
        const INT_TAB: u32 = 0x012B_9588;
        const INT_FIRST: u32 = 0xFD;
        const STR_TAB: u32 = 0x012B_9BB8;
        const STR_FIRST: u32 = 0x289;
        const GEN_CUR: u32 = 0x0103_B584;
        const GEN_SHADOW: u32 = 0x0103_B588;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let g = |va: u32| lf_checker_rt::relocated(va);
        let tick: u32 = lf_checker_rt::callee_cdecl!(0, u32,);
        let stamp = tick | 1;
        let last = rd32(g(STAMP_SLOT));
        if last != 0 {
            let elapsed = stamp.wrapping_sub(last);
            if elapsed >= rd32(g(RESEED_AFTER)) {
                let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
            }
        }
        let since_pair = rd32(g(PAIR_SLOT));
        if since_pair == 0 || stamp.wrapping_sub(since_pair) >= PAIR_AFTER {
            let sel_base: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            let sel = rd32(sel_base.wrapping_add(SEL_OFF));
            let entry = rd32(sel_base.wrapping_add(SEL_OFF).wrapping_add(
                sel.wrapping_add(3).wrapping_mul(3).wrapping_mul(4),
            ));
            let cur = rd32(g(PAIR_NEW));
            if entry == cur && cur != rd32(g(PAIR_CUR)) {
                let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, entry);
            }
            wr32(g(PAIR_CUR), rd32(g(PAIR_NEW)));
            wr32(g(PAIR_NEW), entry);
        }
        let since_cache = rd32(g(CACHE_SLOT));
        if since_cache == 0 || stamp.wrapping_sub(since_cache) >= rd32(g(CACHE_AFTER)) {
            let _: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        }
        let since_body = rd32(g(BODY_SLOT));
        if since_body != 0 && stamp.wrapping_sub(since_body) < rd32(g(BODY_AFTER)) {
            let _: u32 = lf_checker_rt::callee_cdecl!(28, u32,);
            // The early exit returns the elapsed time still in eax, not the stamp.
            return stamp.wrapping_sub(since_body);
        }
        // Shadow buffers for the three loops (contents unobserved).
        let mut m1 = [0u32; 24];
        let mut m2 = [0u32; 24];
        let mut m3 = [0u32; 24];
        let mut mt = [0u32; 20];
        // Loop 1: floats.
        let shadow: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        let float_n: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
        if (float_n as i32) > 0 {
            let tab = g(FLOAT_TAB);
            let mut i = 0u32;
            while (i as i32) < float_n as i32 {
                let gate: u32 = lf_checker_rt::callee_cdecl!(7, u32, i);
                if gate & 0xFF == 0 {
                    // The original subtracts the table base from the shadow
                    // pointer first, so the shadow side reads the pointer as-is.
                    let a = rdf(shadow.wrapping_add(i.wrapping_mul(4)));
                    let b = rdf(tab.wrapping_add(i.wrapping_mul(4)));
                    if a != b {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            8, u32, m1.as_mut_ptr() as u32, 4u32, 0x1du32
                        );
                        let v = rd32(shadow.wrapping_add(i.wrapping_mul(4)));
                        wr32(tab.wrapping_add(i.wrapping_mul(4)), v);
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            9, u32, m1.as_mut_ptr() as u32, 0x3cu32
                        );
                        let w = rd32(shadow.wrapping_add(i.wrapping_mul(4)));
                        wr32(tab.wrapping_add(i.wrapping_mul(4)), w);
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(10, u32, m1.as_mut_ptr() as u32);
                    }
                }
                i += 1;
            }
        }
        // Loop 2: words.
        let probe: u32 = lf_checker_rt::callee_cdecl!(11, u32,);
        let int_n: u32 = lf_checker_rt::callee_cdecl!(12, u32,);
        if (int_n as i32) > 0 {
            let tab = g(INT_TAB);
            let mut i = 0u32;
            while (i as i32) < int_n as i32 {
                let key = INT_FIRST.wrapping_add(i);
                let gate: u32 = lf_checker_rt::callee_cdecl!(13, u32, key);
                if gate & 0xFF == 0 {
                    let slot = tab.wrapping_add(i.wrapping_mul(4));
                    let fresh = rd32(probe.wrapping_add(i.wrapping_mul(4)));
                    if fresh != rd32(slot) {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            14, u32, m2.as_mut_ptr() as u32, 3u32, 0x1cu32
                        );
                        wr32(slot, fresh);
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            15, u32, m2.as_mut_ptr() as u32, 0x3cu32
                        );
                        wr32(slot, rd32(probe.wrapping_add(i.wrapping_mul(4))));
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(16, u32, m2.as_mut_ptr() as u32);
                    }
                }
                i += 1;
            }
        }
        // Loop 3: strings.
        let str_base: u32 = lf_checker_rt::callee_cdecl!(17, u32,);
        let str_n: u32 = lf_checker_rt::callee_cdecl!(18, u32,);
        if (str_n as i32) > 0 {
            let tab = g(STR_TAB);
            let mut i = 0u32;
            while (i as i32) < str_n as i32 {
                let key = STR_FIRST.wrapping_add(i);
                let gate: u32 = lf_checker_rt::callee_cdecl!(19, u32, key);
                if gate & 0xFF == 0 {
                    let cell = str_base.wrapping_add(i.wrapping_mul(4));
                    let s = rd32(cell);
                    let mut resolved = 0u32;
                    if s != 0 && (s as *const u8).read() != 0 {
                        resolved = lf_checker_rt::callee_cdecl!(20, u32, s, 0u32);
                    }
                    let slot = tab.wrapping_add(i.wrapping_mul(4));
                    if resolved != rd32(slot) {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            21, u32, m3.as_mut_ptr() as u32, 1u32, 0x1eu32
                        );
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            22, u32, m3.as_mut_ptr() as u32, resolved, rd32(cell)
                        );
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            23, u32, m3.as_mut_ptr() as u32, 0x38u32
                        );
                        wr32(slot, resolved);
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(24, u32, m3.as_mut_ptr() as u32);
                    }
                }
                i += 1;
            }
        }
        if rd32(g(GEN_CUR)) != rd32(g(GEN_SHADOW)) {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(25, u32, mt.as_mut_ptr() as u32, 1u32, 0x2du32);
            let _: u32 =
                lf_checker_rt::callee_cdecl!(26, u32, mt.as_mut_ptr() as u32, 0x38u32);
            wr32(g(GEN_SHADOW), rd32(g(GEN_CUR)));
            let _: u32 = lf_checker_rt::callee_thiscall!(27, u32, mt.as_mut_ptr() as u32);
        }
        wr32(g(BODY_SLOT), stamp);
        let _: u32 = lf_checker_rt::callee_cdecl!(28, u32,);
        stamp
    }
});
