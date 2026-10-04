// original: 0x00d6b260 FRONTEND_MENU_MONTAGE_NAVIGATE_MT (stage 1: probe chain + table path + epilogue)

/// Menu input dispatcher, stage 1 (original 0x00D6B260).
///
/// Walks a chain of input probes: each step fetches the shared input-state
/// block, tests either two xor-masked bytes against 0x7F or one helper call,
/// and on a hit runs that step's handler and then the shared epilogue. A
/// mode-0/flag path at the head instead drives two table slots and one
/// handler. The epilogue re-checks two global gates and one helper before a
/// final byte probe.
///
/// Stage 1 covers the full probe chain with all probes missing, the head
/// table path, one nested-probe handler, and the short epilogue. Handler
/// branches behind a hitting probe are stage 2 (`unreachable!` markers).
export!(thiscall, rw_00d6b260(this: u32) -> u32 {
    const MODE: u32 = 0x0103_7720;
    const HEAD_FLAG: u32 = 0x0103_7758;
    const CACHED_BIT: u32 = 0x0103_775A;
    const MODE_GATE: u32 = 0x011F_701E;
    const TBL_IDX_A: u32 = 0x0118_EC8C;
    const TBL_BASE: u32 = 0x0118_E7F8;
    const TBL_IDX_B: u32 = 0x0118_EFF0;
    const LIMIT: u8 = 0x7F;
    unsafe {
        let mode = global::<u32>(MODE).read();
        let cached = global::<u8>(CACHED_BIT).read();
        if mode == 2 || mode == 7 {
            if global::<u8>(MODE_GATE).read() != 0 {
                // EAX still holds the mode word on this early exit.
                return mode;
            }
        }
        if global::<u8>(HEAD_FLAG).read() == 0 && cached == 0 {
            let st = callee_cdecl!(1, u32, 1);
            let base = (st as *const u8).add(0x2FF8);
            let mask = (st as *const u8).add(0x2FFC).read();
            if base.add(6).read() ^ mask <= LIMIT {
                return epilogue_00d6b260();
            }
            if base.add(7).read() ^ mask > LIMIT {
                return epilogue_00d6b260();
            }
            if callee_cdecl!(2, u32,) != 0 {
                return epilogue_00d6b260();
            }
            let tbase = relocated(TBL_BASE) as *const u32;
            let ia = global::<u32>(TBL_IDX_A).read() as usize;
            let slot = tbase.add(ia).read();
            callee_thiscall!(3, u32, slot, 1, 0);
            let ia2 = global::<u32>(TBL_IDX_A).read() as usize;
            ((tbase.add(ia2).read() as *mut u8).add(0x12)).write(1);
            let ib = global::<u32>(TBL_IDX_B).read() as usize;
            ((tbase.add(ib).read() as *mut u8).add(0x12)).write(1);
            global::<u8>(HEAD_FLAG).write(1);
            callee_thiscall!(4, u32, this);
            return epilogue_00d6b260();
        }
        // Main probe chain. `hit` runs the step handler (stage 2) and the
        // epilogue; falling through tries the next probe.
        // B1: byte pair at +0x3088 (handler: stage 2).
        let st = callee_cdecl!(1, u32, 1);
        let mask = (st as *const u8).add(0x308C).read();
        let b = (st as *const u8).add(0x3088);
        if !((b.add(6).read() ^ mask <= LIMIT) || (b.add(7).read() ^ mask > LIMIT)) {
            unreachable!("stage 2: B1 handler");
        }
        // B2: single byte + nested helper (handler verified in stage 1).
        let st = callee_cdecl!(1, u32, 1);
        let c = (st as *const u8).add(0x308E).read() ^ (st as *const u8).add(0x308C).read();
        if c > LIMIT {
            let st2 = callee_cdecl!(1, u32, 1);
            let cx = st2.wrapping_add(0x3088);
            if callee_thiscall!(5, u32, cx, 0x1F4, 0x80, 0xFF) != 0 {
                callee_thiscall!(6, u32, this);
                return epilogue_00d6b260();
            }
        }
        // B3: inverted byte pair at +0x3088 (handler: stage 2).
        let st = callee_cdecl!(1, u32, 1);
        let mask = (st as *const u8).add(0x308C).read();
        let b = (st as *const u8).add(0x3088);
        if !((b.add(6).read() ^ mask > LIMIT) || (b.add(7).read() ^ mask <= LIMIT)) {
            unreachable!("stage 2: B3 handler");
        }
        // B4: byte pair at +0x3098 (handler: stage 2).
        let st = callee_cdecl!(1, u32, 1);
        let mask = (st as *const u8).add(0x309C).read();
        let b = (st as *const u8).add(0x3098);
        if !((b.add(6).read() ^ mask <= LIMIT) || (b.add(7).read() ^ mask > LIMIT)) {
            unreachable!("stage 2: B4 handler");
        }
        // B5: single byte + nested helper (handler: stage 2).
        let st = callee_cdecl!(1, u32, 1);
        let c = (st as *const u8).add(0x309E).read() ^ (st as *const u8).add(0x309C).read();
        if c > LIMIT {
            let st2 = callee_cdecl!(1, u32, 1);
            let cx = st2.wrapping_add(0x3098);
            if callee_thiscall!(5, u32, cx, 0x1F4, 0x80, 0xFF) != 0 {
                unreachable!("stage 2: B5 handler");
            }
        }
        // B6..B21: helper-call probes. Each calls its helper with a block
        // pointer; a nonzero answer (plus the cached bit for B14 on) runs
        // the step handler. All answers are 0 in stage 1.
        let probes: [(u32, u32, bool); 16] = [
            (8, 0x3098, false), // B6
            (7, 0x2FF8, false), // B7 (hit: switch table, stage 2)
            (7, 0x3018, false), // B8
            (7, 0x3008, false), // B9
            (7, 0x3158, false), // B10 (hit: inline toggle, stage 2)
            (7, 0x3058, false), // B11 (hit: inline toggle, stage 2)
            (7, 0x30B8, false), // B12
            (7, 0x3168, false), // B13
            (8, 0x30D8, true),  // B14
            (8, 0x30E8, true),  // B15
            (7, 0x3028, true),  // B16
            (7, 0x3068, true),  // B17
            (7, 0x3078, true),  // B18
            (7, 0x30C8, true),  // B19
            (7, 0x3148, true),  // B20
            (7, 0x30A8, true),  // B21
        ];
        let mut i = 0;
        while i < probes.len() {
            let (id, off, gated) = probes[i];
            let st = callee_cdecl!(1, u32, 1);
            let cx = st.wrapping_add(off);
            let ans = if id == 8 {
                callee_thiscall!(8, u32, cx)
            } else {
                callee_thiscall!(7, u32, cx)
            };
            if ans != 0 && (!gated || cached == 0) {
                unreachable!("stage 2: call-probe handler");
            }
            i += 1;
        }
        epilogue_00d6b260()
    }
});


