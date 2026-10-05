// original: 0x0057b0a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_165, player_schema::LeaderboardInfo, 10>::vf14

/// Parse one ranked leaderboard's column descriptors into caller buffers.
///
/// `this` is the leaderboard-info object (vtable pointer at `+0`): slot
/// `COUNT_SLOT` reports how many columns the table holds, slot `INDEX_SLOT`
/// maps a column number to its row index. `mask_out` receives a two-word bit
/// mask (cleared first), `flag_out` a single status byte (cleared first),
/// `blob_out` an 8-byte row payload when the counted column is the current
/// one. `ctx` is an opaque cursor handed to the row helpers untouched.
/// `cursor` and `length` bound a span cursor: each processed column advances
/// it by 0 or 8, and running past `cursor + length` fails the run.
///
/// Behaviour: resolve the column table through the lookup helper (fails the
/// run when it answers false, leaving the cleared outputs). Then, for each
/// of the 19 columns in order: stop if a previous column failed; ask the
/// decided-helper whether the column is already settled and skip it when it
/// is; otherwise classify the row (kinds 1, 2, 3 and 5 take an 8-wide span,
/// anything else a 0-wide one). When the counted column equals the current
/// one, fetch its row: with a row present that decodes to 8 bytes or fewer,
/// copy the 8 payload bytes past its header to `blob_out` and record success
/// in `flag_out`, else record failure. Otherwise advance the span cursor and,
/// when it still fits, store the span and set `mask_out` to the single bit
/// of the current column (a plain store, not an accumulation).
///
/// Returns 1 when every processed column succeeded, 0 otherwise; the empty
/// run (lookup refused) returns the lookup's own zero byte. Original:
/// thiscall, six stack words, callee-cleaned.
lf_checker_rt::export!(thiscall, rw_0057b0a0(this: u32, cursor: u32, blob_out: u32, mask_out: u32, flag_out: u32, ctx: u32, length: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17d;
        const COUNT_SLOT: u32 = 0x2c;
        const INDEX_SLOT: u32 = 0x30;
        const N_COLUMNS: u32 = 0x13;
        const WIDE_SPAN: u32 = 8;
        const MAX_INLINE_ROW: u32 = 8;
        const ROW_HEADER: u32 = 4;
        const CAL_COUNT: u32 = 1;
        const CAL_LOOKUP: u32 = 2;
        const CAL_INDEX: u32 = 3;
        const CAL_DECIDED: u32 = 4;
        const CAL_KIND: u32 = 5;
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
        /// The original's `al`-answered helpers keep the caller's upper
        /// `eax` bits (residue differs per side), so only the low byte
        /// carries the answer on either side.
        #[inline(always)]
        fn answered_yes(raw: u32) -> bool {
            raw & 0xff != 0
        }
        /// Single-bit two-word mask the original builds with `bts`/`cmovae`:
        /// bit `col` of a 64-bit value, all zero past bit 63.
        #[inline(always)]
        fn single_bit(col: u32) -> (u32, u32) {
            if col < 32 {
                (1u32 << col, 0)
            } else if col < 64 {
                (0, 1u32 << (col - 32))
            } else {
                (0, 0)
            }
        }

        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        wr8(flag_out, 0);
        let end = length.wrapping_add(cursor);
        let vtable = rd32(this);
        let count_rows: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(COUNT_SLOT)) as usize);
        let counted = count_rows(this);
        let mut table_out = [0u32; 3];
        let looked_up = lf_checker_rt::callee_fastcall!(
            CAL_LOOKUP, u32, LEADERBOARD_ID, table_out.as_mut_ptr() as u32);
        if !answered_yes(looked_up) {
            // The early exit returns the lookup's own low byte (zero here),
            // not the status flag: no `(an instruction of the original)` on this path.
            return looked_up & 0xff;
        }
        let table = table_out[2];
        let row_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(INDEX_SLOT)) as usize);
        let mut ok: u8 = 1;
        let mut span_cursor = cursor;
        let mut col: u32 = 0;
        while col < N_COLUMNS {
            if ok == 0 {
                break;
            }
            let index = row_of(this, col);
            let settled =
                lf_checker_rt::callee_thiscall!(CAL_DECIDED, u32, ctx, index);
            if !answered_yes(settled) {
                let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
                let kind = lf_checker_rt::callee_thiscall!(CAL_KIND, u32, entry);
                let span = match kind {
                    1 | 2 | 3 | 5 => WIDE_SPAN,
                    _ => 0,
                };
                if counted == col {
                    ok = 0;
                    let row = lf_checker_rt::callee_thiscall!(CAL_FETCH, u32, ctx, index);
                    if row != 0 {
                        let size = lf_checker_rt::callee_thiscall!(CAL_SIZE, u32, row);
                        if size <= MAX_INLINE_ROW {
                            let payload = ((row.wrapping_add(ROW_HEADER)) as *const u64)
                                .read_unaligned();
                            (blob_out as *mut u64).write_unaligned(payload);
                            ok = 1;
                        }
                    }
                    wr8(flag_out, ok);
                } else {
                    let before = span_cursor;
                    span_cursor = span_cursor.wrapping_add(span);
                    if span_cursor > end {
                        ok = 0;
                    } else {
                        let stored = lf_checker_rt::callee_thiscall!(
                            CAL_STORE, u32, ctx, index, before, span);
                        if !answered_yes(stored) {
                            ok = 0;
                        } else {
                            let (lo, hi) = single_bit(col);
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            ok = 1;
                        }
                    }
                }
            }
            col += 1;
        }
        ok as u32
    }
});
