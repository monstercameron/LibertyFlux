// original: 0x0055C6E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_53, player_schema::LeaderboardInfo, 10>::vf14

/// Leaderboard row collector for ranked episodic race 53: walks ranks
/// 0..19 through the leaderboard object, classifying each rank and either
/// copying the selected rank's payload or placing rows into the cursor.
///
/// Arguments (thiscall, six stack words): `this` leaderboard-info object
/// (vtable slots 0x2c/0x30 supply the selected rank and the rank per
/// iteration); `run_start`/`span` bound the placement cursor
/// (`limit = run_start + span`, wrapping); `copy_dst` takes 8 payload bytes;
/// `mask_out` takes the selection mask qword; `flag_out` one flag byte;
/// `ctx` is the helper context. Callee 3 looks the leaderboard id 0x100
/// up and returns the rank table through a frame buffer (its address is
/// skipped in the proof and its contents snapshotted). The original spills
/// the running cursor and the selected rank into its incoming arg slots;
/// the rewrite keeps them in locals, so the stack comparison is off.
/// Returns the low byte only (1 while every step succeeded).
lf_checker_rt::export!(thiscall, rw_0055c6e0(this: u32, run_start: u32, copy_dst: u32, mask_out: u32, flag_out: u32, ctx: u32, span: u32) -> u32 {
    unsafe {
        /// Object slot holding the selected-rank value (called once, no args).
        const SLOT_SELECTED: u32 = 0x2c;
        /// Object slot returning the rank for iteration `i` (called with `i`).
        const SLOT_RANK_AT: u32 = 0x30;
        /// Loop bound: iterations 0..19.
        const ITERATIONS: u32 = 19;
        /// Row stride the classifier assigns to a recognised entry class.
        const STRIDE_SET: u32 = 8;
        /// Largest entry size (signed compare) that still copies its payload.
        const SIZE_LIMIT: i32 = 8;
        /// Width of the copied payload in words (at entry+4).
        const COPY_WORDS: u32 = 2;
        /// This instantiation's leaderboard id (the template parameter).
        const LEADERBOARD_ID: u32 = 0x100;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let limit = run_start.wrapping_add(span);
        let mut run = run_start;
        let vt = rd32(this);
        let selected_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(SLOT_SELECTED)) as usize);
        let selected = selected_fn(this);
        let mut buf = [0u32; 3];
        buf[2] = 0;
        let found: u32 =
            lf_checker_rt::callee_fastcall!(3, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if (found as u8) == 0 {
            return 0;
        }
        let table = buf[2];
        let mut ok: u8 = 1;
        let mut i: u32 = 0;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let vt = rd32(this);
            let rank_at: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(SLOT_RANK_AT)) as usize);
            let rank = rank_at(this, i);
            let skip: u32 = lf_checker_rt::callee_thiscall!(4, u32, ctx, rank);
            if (skip as u8) == 0 {
                let kind = rd32(table.wrapping_add(rank.wrapping_mul(4)));
                let cls: u32 = lf_checker_rt::callee_thiscall!(5, u32, kind);
                // Classifier switch on (cls - 1): 0,1,2,4 take the stride,
                // 3 and anything out of range leave it zero.
                let d = cls.wrapping_sub(1);
                let stride = if d <= 4 && d != 3 { STRIDE_SET } else { 0 };
                if selected == i {
                    ok = 0;
                    let entry: u32 = lf_checker_rt::callee_thiscall!(6, u32, ctx, rank);
                    if entry != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(7, u32, entry);
                        if (size as i32) <= SIZE_LIMIT {
                            for w in 0..COPY_WORDS {
                                wr32(
                                    copy_dst.wrapping_add(w * 4),
                                    rd32(entry.wrapping_add(4 + w * 4)),
                                );
                            }
                            ok = 1;
                        }
                    }
                    (flag_out as *mut u8).write(ok);
                } else {
                    // NOTE: the callee takes the cursor from *before* the
                    // stride is added (it is reloaded from the spill slot);
                    // the bound check and the stored value use the new one.
                    let prev = run;
                    run = run.wrapping_add(stride);
                    if run > limit {
                        ok = 0;
                    } else {
                        let placed: u32 =
                            lf_checker_rt::callee_thiscall!(8, u32, ctx, rank, prev, stride);
                        if (placed as u8) == 0 {
                            ok = 0;
                        } else {
                            // Bit-test chain: bit i of the 64-bit mask.
                            let (lo, hi) = if i < 32 {
                                (1u32 << i, 0u32)
                            } else if i < 64 {
                                (0u32, 1u32 << (i - 32))
                            } else {
                                (0u32, 0u32)
                            };
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            ok = 1;
                        }
                    }
                }
            }
            i += 1;
        }
        ok as u32
    }
});
