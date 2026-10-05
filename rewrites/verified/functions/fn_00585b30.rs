// original: 0x00585b30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_204, player_schema::LeaderboardInfo, 10>::vf14

/// Leaderboard column fetch for one ranked episodic race table (race 204).
///
/// `this` is the leaderboard-info object (vtable pointer at `+0`); `store`
/// (or null) is the row store handed to every row callee; `span` added to
/// `acc_in` bounds a running offset. `row_out` receives one 8-byte row,
/// `bits_out` a 64-bit one-hot step mask, `flag_out` a success byte; all
/// three are cleared first.
///
/// The wanted step comes from the virtual slot at `+0x2c`. A lookup callee
/// keyed by this table's id (0x1a4) answers go or no-go and yields the lane
/// array; on no-go the function returns that answer at once. Otherwise up to
/// 19 steps run while a flag stays set. Each step resolves its row through
/// the virtual slot at `+0x30`, may be skipped by a gate callee, then maps a
/// lane kind to an advance (kinds 1, 2, 3 and 5 advance by 8, anything else
/// by 0). The wanted step fetches a row of at most 8 bytes into `row_out`
/// and records success in `flag_out`; every other step advances the offset
/// against the bound, keeps the row through a store callee and sets bit
/// `step` in `bits_out`. The low return byte is the flag; the upper bytes
/// keep whatever the last callee left, so only the low byte is compared.
///
/// The original carries the running offset and the wanted step in its own
/// incoming argument slots; the rewrite uses locals instead, so the stack
/// comparison is off for this function (see the contract).
///
/// Original: 0x00585b30 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00585b30(this: u32, acc_in: u32, row_out: u32, bits_out: u32, flag_out: u32, store: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a4;
        const VT_SELECTED: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const ITERATIONS: u32 = 0x13;
        const WIDE_STEP: u32 = 8;
        const CAL_LOOKUP: u32 = 2;
        const CAL_KIND: u32 = 4;
        const CAL_SKIP: u32 = 5;
        const CAL_FETCH: u32 = 6;
        const CAL_SIZE: u32 = 7;
        const CAL_STORE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        wr32(bits_out, 0);
        wr32(bits_out.wrapping_add(4), 0);
        wr8(flag_out, 0);
        let limit = span.wrapping_add(acc_in);
        let vtable = rd32(this);
        let mut flag: u8 = 1;
        let selected: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_SELECTED)) as usize);
        let want = selected(this);
        let mut probe = [0u32; 3];
        probe[2] = 0;
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(CAL_LOOKUP, u32, LEADERBOARD_ID, probe.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return ok;
        }
        let lanes = probe[2];
        let index_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_INDEX)) as usize);
        let mut acc = acc_in;
        let mut slot = acc_in;
        let mut step = 0u32;
        while flag != 0 && step < ITERATIONS {
            let row = index_of(this, step);
            let skip: u32 = lf_checker_rt::callee_thiscall!(CAL_SKIP, u32, store, row);
            if (skip & 0xff) == 0 {
                let lane = rd32(lanes.wrapping_add(row.wrapping_mul(4)));
                let kind: u32 = lf_checker_rt::callee_thiscall!(CAL_KIND, u32, lane);
                let mut adv = 0u32;
                if kind != 0xffff_ffff {
                    let pick = kind.wrapping_sub(1);
                    if pick <= 4 && pick != 3 {
                        adv = WIDE_STEP;
                    }
                }
                if want == step {
                    flag = 0;
                    let got: u32 = lf_checker_rt::callee_thiscall!(CAL_FETCH, u32, store, row);
                    if got != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(CAL_SIZE, u32, got);
                        if size <= 8 {
                            wr32(row_out, rd32(got.wrapping_add(4)));
                            wr32(row_out.wrapping_add(4), rd32(got.wrapping_add(8)));
                            flag = 1;
                        }
                    }
                    acc = slot;
                    wr8(flag_out, flag);
                } else {
                    acc = acc.wrapping_add(adv);
                    if acc <= limit {
                        let kept: u32 =
                            lf_checker_rt::callee_thiscall!(CAL_STORE, u32, store, row, slot, adv);
                        if (kept & 0xff) != 0 {
                            let bit = 1u32 << (step & 31);
                            if step < 32 {
                                wr32(bits_out, bit);
                                wr32(bits_out.wrapping_add(4), 0);
                            } else if step < 64 {
                                wr32(bits_out, 0);
                                wr32(bits_out.wrapping_add(4), bit);
                            } else {
                                wr32(bits_out, 0);
                                wr32(bits_out.wrapping_add(4), 0);
                            }
                            flag = 1;
                        } else {
                            flag = 0;
                        }
                    } else {
                        flag = 0;
                    }
                    slot = acc;
                }
            }
            step += 1;
        }
        flag as u32
    }
});
