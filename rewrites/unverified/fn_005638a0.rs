// original: 0x005638A0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_79, player_schema::LeaderboardInfo, 10>::vf14
///
/// Poll one ranked leaderboard's rows and reduce them into three outputs.
///
/// `this` is the leaderboard-info object (its virtual table supplies the
/// handle slot at `+0x2c` and the row-index slot at `+0x30`). The six stack
/// arguments are: `base` (the running cursor's start), `payload_out` (8
/// bytes receiving one row's payload), `bits_out` (8 bytes receiving a
/// one-hot row mask), `flag_out` (one byte receiving the last row-test
/// outcome), `ctx` (an opaque context handed to the row callees) and
/// `span` (the cursor may advance at most this far past `base`).
///
/// Algorithm: clear `bits_out` and `flag_out`, fetch the query handle,
/// open query `LEADERBOARD_ID` (which yields the row-handle table), then
/// run 19 iterations. Iteration `i` maps to a row index through the index
/// slot; a skip test against `ctx` may pass it over. Otherwise the row's
/// kind decides the
/// cursor step (8 for kinds 1, 2, 3 and 5, else 0). When the handle equals
/// the iteration number the row object is fetched and, if its signed
/// size is at most 8 (a negative size copies), its payload copied to
/// `payload_out` and `flag_out` set.
/// Otherwise the cursor advances by the step: it must stay within
/// `base + span`, and a place call must accept the row, whose bit is then
/// written into `bits_out` (overwriting the previous iteration's bit).
/// The loop stops early when an iteration reports failure. Returns the
/// last outcome byte in `al` (upper bytes are the last callee's leftover).
///
/// Edge cases: a rejected query returns 0 with the outputs cleared; a null
/// row or an oversize one clears the flag; a wrapped `base + span` fails
/// the bound check. The kind switch reads a table of code addresses; the
/// five entries collapse to the two step values above.
///
/// Original: 0x005638A0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_005638A0(this: u32, base: u32, payload_out: u32, bits_out: u32, flag_out: u32, ctx_arg: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x11A;
        const ITERATIONS: u32 = 19;
        const STEP_WIDE: u32 = 8;
        const VT_HANDLE: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const Q_ROWS_WORD: usize = 2;
        const ROW_PAYLOAD_OFF: u32 = 4;
        const PAYLOAD_MAX: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }

        wr32(bits_out, 0);
        wr32(bits_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let limit = span.wrapping_add(base);
        let vtable = rd32(this);
        let handle_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_HANDLE)) as usize);
        let handle = handle_of(this);
        // Query block shared with the open callee: it stores the row table
        // at word 2, the only word either side reads afterwards.
        let mut query = [0u32, 0u32, 0u32];
        let opened: u32 = lf_checker_rt::callee_fastcall!(
            3, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if (opened & 0xff) == 0 {
            return 0;
        }
        let rows = query[Q_ROWS_WORD];
        let index_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_INDEX)) as usize);
        let mut ok: u8 = 1;
        let mut cursor = base;
        let mut i = 0u32;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let idx = index_of(this, i);
            let skipped: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, ctx_arg, idx);
            if (skipped & 0xff) == 0 {
                ok = 0;
                let row_handle = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
                let kind: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, row_handle);
                let step = if matches!(kind, 1 | 2 | 3 | 5) { STEP_WIDE } else { 0 };
                if handle == i {
                    let row: u32 =
                        lf_checker_rt::callee_thiscall!(6, u32, ctx_arg, idx);
                    ok = 0;
                    if row != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(7, u32, row);
                        // Signed compare (jg): a negative size copies.
                        if (size as i32) <= PAYLOAD_MAX as i32 {
                            wr64(payload_out, rd64(row.wrapping_add(ROW_PAYLOAD_OFF)));
                            ok = 1;
                        }
                    }
                    (flag_out as *mut u8).write(ok);
                } else {
                    // The place call sees the cursor from before this
                    // iteration's step (the original passes its spill slot,
                    // which the loop end refreshes only afterwards).
                    let prev = cursor;
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        ok = 0;
                    } else {
                        let placed: u32 = lf_checker_rt::callee_thiscall!(
                            8, u32, ctx_arg, idx, prev, step);
                        if (placed & 0xff) == 0 {
                            ok = 0;
                        } else {
                            ok = 1;
                            let bit = 1u32 << (i & 31);
                            let (lo, hi) = if i < 0x20 {
                                (bit, 0)
                            } else if i < 0x40 {
                                (0, bit)
                            } else {
                                (0, 0)
                            };
                            wr32(bits_out, lo);
                            wr32(bits_out.wrapping_add(4), hi);
                        }
                    }
                }
            }
            i += 1;
        }
        ok as u32
    }
});
