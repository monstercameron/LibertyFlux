// original: 0x0052b010 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race15Standard, player_schema::LeaderboardInfo, 10>::vf14

/// Ranked-race leaderboard column fill for one race table (virtual slot 14).
///
/// Fills one column of a ranked-race leaderboard view: for each of five
/// slots it either copies the selected row's bytes to `row_out` and records
/// the per-slot outcome in `flag_out`, or advances an integer cursor and
/// records the slot bit in the two-word mask at `mask_out`.
///
/// `this` is the leaderboard-info object (vtable slots `VT_SEL`/`VT_ROW`
/// supply the selected slot and the per-slot row index). `cursor` seeds the
/// cursor; `limit` is `base + cursor`. `row_out` takes 8 bytes copied from
/// past the found row object when the slot matches and the row length does
/// not exceed `LEN_LIMIT`. `mask_out` is zeroed on entry, then each taken
/// slot overwrites it with its single bit. `flag_out` is zeroed on entry,
/// then each matching slot overwrites it with its outcome. `ctx` is passed
/// as the object to the four worker callees, while `base` only sets the
/// cursor limit (`base + cursor`). `KIND` is this instance's table key, passed to the query
/// callee; it is the only difference between the eight instantiations.
///
/// The class switch sets the stride to 8 unless the class is 4, -1, 0 or
/// above 5 (a jump table in the original). Iteration stops early once an
/// outcome is 0. Returns the last outcome in the low byte; the upper three
/// bytes are caller leftovers (`(an instruction of the original)` in the original).
///
/// Original: 0x0052b010 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_0052b010(this: u32, cursor: u32, row_out: u32, mask_out: u32, flag_out: u32, ctx: u32, base: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const VT_SEL: u32 = 0x2c;
        const VT_ROW: u32 = 0x30;
        const SLOTS: u32 = 5;
        const STRIDE: u32 = 8;
        const LEN_LIMIT: u32 = 8;
        const ROW_DATA_OFF: u32 = 4;
        const Q_ROWS_OFF: u32 = 8;
        const KIND: u32 = 0x5d;
        const ID_QUERY: u32 = 3;
        const ID_EXCLUDED: u32 = 4;
        const ID_CLASS: u32 = 5;
        const ID_FIND: u32 = 6;
        const ID_LEN: u32 = 7;
        const ID_TAKE: u32 = 8;

        wr32(mask_out, 0);
        wr32(mask_out + 4, 0);
        (flag_out as *mut u8).write(0);
        // Note the split the binary shows: the limit comes from the sixth
        // argument while the worker callees take the fifth as their object.
        let limit = base.wrapping_add(cursor);
        let vt = rd32(this);
        let mut ok: u8 = 1;
        let sel: u32 = {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt + VT_SEL) as usize);
            f(this)
        };
        let mut q = [0u32; 4];
        let query_ok: u32 =
            lf_checker_rt::callee_fastcall!(ID_QUERY, u32, KIND, q.as_mut_ptr() as u32);
        if query_ok & 0xFF == 0 {
            return 0;
        }
        let rows = q[(Q_ROWS_OFF / 4) as usize];
        let mut cur = cursor;
        let mut slot: u32 = 0;
        while slot < SLOTS {
            if ok == 0 {
                break;
            }
            let vt_loop = rd32(this);
            let idx: u32 = {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt_loop + VT_ROW) as usize);
                f(this, slot)
            };
            let skip: u32 = lf_checker_rt::callee_thiscall!(ID_EXCLUDED, u32, ctx, idx);
            if skip & 0xFF != 0 {
                slot += 1;
                continue;
            }
            let obj = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
            let class: u32 = lf_checker_rt::callee_thiscall!(ID_CLASS, u32, obj);
            let mut stride: u32 = 0;
            if class != 0xFFFF_FFFF {
                let t = class.wrapping_sub(1);
                if t <= 4 && t != 3 {
                    stride = STRIDE;
                }
            }
            if sel == slot {
                ok = 0;
                let found: u32 = lf_checker_rt::callee_thiscall!(ID_FIND, u32, ctx, idx);
                if found != 0 {
                    let len: u32 = lf_checker_rt::callee_thiscall!(ID_LEN, u32, found);
                    if len <= LEN_LIMIT {
                        wr32(row_out, rd32(found + ROW_DATA_OFF));
                        wr32(row_out + 4, rd32(found + ROW_DATA_OFF + 4));
                        ok = 1;
                    }
                }
                (flag_out as *mut u8).write(ok);
            } else {
                cur = cur.wrapping_add(stride);
                if cur > limit {
                    ok = 0;
                } else {
                    let old = cur.wrapping_sub(stride);
                    let take: u32 =
                        lf_checker_rt::callee_thiscall!(ID_TAKE, u32, ctx, idx, old, stride);
                    if take & 0xFF == 0 {
                        ok = 0;
                    } else {
                        let mut lo = 1u32 << (slot & 31);
                        let mut hi = 0u32;
                        if slot >= 0x20 {
                            hi = lo;
                        }
                        lo ^= hi;
                        if slot >= 0x40 {
                            hi = lo;
                        }
                        wr32(mask_out, lo);
                        wr32(mask_out + 4, hi);
                        ok = 1;
                    }
                }
            }
            slot += 1;
        }
        ok as u32
    }
});
