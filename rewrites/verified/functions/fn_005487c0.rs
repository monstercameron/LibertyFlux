// original: 0x005487C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_7, player_schema::LeaderboardInfo, 10>::vf14
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

/// Scan one ranked-episodic leaderboard board over its 24 slots, collecting a
/// presence bitmask and at most one row payload (template instance for one board).
///
/// `this` is the board object. `mask_out` receives a 64-bit presence mask (two
/// words, zeroed up front); `flag_out` receives a one-byte status flag (zeroed
/// up front); `row_out` receives an 8-byte row payload on the selected slot;
/// `ctx` is the context object passed to the eligibility helpers; `acc0` seeds
/// a running accumulator and `limit_base` bounds it (`limit = limit_base + acc0`).
///
/// Behaviour: zero the outputs, then read the vtable hook at slot `0x2c` (the
/// board's selector value `info`) and run query `QUERY_ID` into a 3-word frame
/// buffer whose third word the query fills with the candidate table. If the
/// query answers zero the function returns that answer at once. Otherwise for
/// each slot `sel` in `0..24` while the status flag is set: read slot index
/// `idx` from the hook at slot `0x30`; if the eligibility check on
/// `(ctx, idx)` passes, look the candidate up and derive a step (8 on four of
/// the lookup's five outcomes, else 0). When `info == sel` the slot is the
/// selected one: fetch its row and, when the row exists and its size fits in 8
/// (signed compare), copy the 8 payload bytes at `row+4` to `row_out` and keep
/// the flag set, else clear it; the flag byte is stored either way. For any
/// other slot grow the accumulator by the step and, when it stays within the
/// limit, commit `(ctx, idx, old accumulator, step)`; on success set bit `sel`
/// of the mask (bit-test semantics past bit 31: the high word takes bit
/// `sel-32`, nothing is set past bit 63) and keep the flag, else clear it.
/// The return value is the final flag (low byte only).
///
/// Original: 0x005487C0 (thiscall, six stack words; the incoming sixth-word slot
/// is reused as spill for `info` and the first-word slot accumulates, which is
/// why the stack check is off for this function: both values are still compared
/// as call arguments).
lf_checker_rt::export!(thiscall, rw_005487C0(this: u32, acc0: u32, row_out: u32, mask_out: u32, flag_out: u32, ctx: u32, limit_base: u32) -> u32 {
    unsafe {
        const LOOP_SLOTS: u32 = 0x18;
        const QUERY_ID: u32 = 0xb9;
        const VTABLE_SELECTOR: u32 = 0x2c;
        const VTABLE_SLOT_INDEX: u32 = 0x30;
        const STEP_PRESENT: u32 = 8;
        const PAYLOAD_MAX: i32 = 8;
        const C_QUERY: u32 = 3;
        const C_CHECK: u32 = 4;
        const C_LOOKUP: u32 = 5;
        const C_FETCH: u32 = 6;
        const C_MEASURE: u32 = 7;
        const C_COMMIT: u32 = 8;

        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        wr8(flag_out, 0);
        let limit = limit_base.wrapping_add(acc0);
        let vtable = rd32(this);
        let selector: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_SELECTOR)) as usize);
        let info = selector(this);
        let mut query_buf = [0u32; 3];
        query_buf[2] = 0;
        let query_answer: u32 = lf_checker_rt::callee_fastcall!(
            C_QUERY, u32, QUERY_ID, query_buf.as_mut_ptr() as u32
        );
        if query_answer & 0xff == 0 {
            return query_answer;
        }
        let slot_index: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_SLOT_INDEX)) as usize);
        let table = query_buf[2];
        let mut acc = acc0;
        let mut ok: u8 = 1;
        let mut sel = 0u32;
        while sel < LOOP_SLOTS {
            if ok == 0 {
                break;
            }
            let idx = slot_index(this, sel);
            let eligible: u32 = lf_checker_rt::callee_thiscall!(C_CHECK, u32, ctx, idx);
            if eligible & 0xff == 0 {
                let mut step = 0u32;
                let key: u32 = lf_checker_rt::callee_thiscall!(
                    C_LOOKUP, u32, rd32(table.wrapping_add(idx.wrapping_mul(4)))
                );
                if key != 0xffff_ffff {
                    let case = key.wrapping_sub(1);
                    if case <= 4 && case != 3 {
                        step = STEP_PRESENT;
                    }
                }
                if info == sel {
                    ok = 0;
                    let row: u32 = lf_checker_rt::callee_thiscall!(C_FETCH, u32, ctx, idx);
                    if row != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(C_MEASURE, u32, row);
                        if (size as i32) <= PAYLOAD_MAX {
                            wr32(row_out, rd32(row.wrapping_add(4)));
                            wr32(row_out.wrapping_add(4), rd32(row.wrapping_add(8)));
                            ok = 1;
                        }
                    }
                    wr8(flag_out, ok);
                } else {
                    let grown = acc.wrapping_add(step);
                    if grown <= limit {
                        let done: u32 = lf_checker_rt::callee_thiscall!(
                            C_COMMIT, u32, ctx, idx, acc, step
                        );
                        if done & 0xff != 0 {
                            let bit = 1u32.wrapping_shl(sel);
                            let mut lo = bit;
                            let mut hi = 0u32;
                            if sel >= 0x20 {
                                hi = lo;
                            }
                            lo ^= hi;
                            if sel >= 0x40 {
                                hi = lo;
                            }
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            ok = 1;
                        } else {
                            ok = 0;
                        }
                    } else {
                        ok = 0;
                    }
                    acc = grown;
                }
            }
            sel += 1;
        }
        ok as u32
    }
});
