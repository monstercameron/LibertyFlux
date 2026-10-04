// original: 0x0053BA40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamCarSteal, player_schema::LeaderboardInfo, 10>::vf14

/// Fetch this leaderboard's row table, then walk its statistic slots,
/// copying the selected entry's key and recording which slots stored.
///
/// `this` is the leaderboard-info object: dword at `+0` is its vtable,
/// slot `0x2c` picks the selected slot index, slot `0x30` maps a loop
/// index to a row index. `cursor` is the running write offset, `limit`
/// the bound it must not pass (`bound = limit + cursor`, wrapping).
/// `key_out` receives 8 bytes (the selected entry's key), `mask_out`
/// two dwords (single-bit mask of the last stored slot), `flag_out` one
/// byte (1 when the selected slot copied, else 0). `rows` is the row
/// collection the helper calls query. `LEADERBOARD_ID` (0x2e) names
/// which leaderboard's table is fetched; the loop runs 9 slots.
///
/// Algorithm: zero the mask and flag; ask slot `0x2c` for the selected
/// index; fetch the row table (callee 3, fastcall id + out-pointer, table
/// pointer landed at out+8); return 0 early when the fetch fails. Per
/// slot: map to a row (slot `0x30`); skip the slot when the row-status
/// call (callee 5) says so; classify the row key (callee 4) into a width
/// of 8 for kinds 1, 2, 3, 5 and 0 otherwise. The selected slot copies
/// the entry key (callees 6 and 7 give the entry and its size; the copy
/// happens only for a non-null entry of size <= 8) and records the flag.
/// Any other slot advances a copy of the cursor by the width; when the
/// advanced copy is inside the bound it stores through callee 8 (which
/// sees the un-advanced cursor) and writes the one-bit mask, then keeps
/// the advanced copy either way. A zero outcome
/// ends the walk after that slot. Returns the final outcome byte.
///
/// Original: 0x0053BA40 (thiscall, ECX = this, six stack words, callee
/// pops 0x18). Two indirect vtable calls plus six direct helpers, all
/// answered by script; the two helpers inside the encrypted first
/// megabyte never execute (their call sites are patched).
lf_checker_rt::export!(thiscall, rw_0053ba40(this: u32, cursor: u32, key_out: u32, mask_out: u32, flag_out: u32, rows: u32, limit: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 46;
        const ITERATIONS: u32 = 9;
        const VT_SELECT: u32 = 0x2c;
        const VT_ROW_OF: u32 = 0x30;
        const ENTRY_KEY_OFF: u32 = 4;
        const MAX_COPY: u32 = 8;
        const WIDE: u32 = 8;

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

        wr32(mask_out, 0);
        wr32(mask_out + 4, 0);
        wr8(flag_out, 0);
        let bound = limit.wrapping_add(cursor);
        let mut pos = cursor;
        let vt = rd32(this);
        let select: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_SELECT));
        let selected = select(this);
        let mut query = [0u32; 3];
        query[2] = 0;
        let fetched: u32 =
            lf_checker_rt::callee_fastcall!(3, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if fetched & 0xff == 0 {
            return 0;
        }
        let table = query[2];
        let row_of: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_ROW_OF));
        let mut done: u8 = 1;
        let mut slot: u32 = 0;
        while slot < ITERATIONS {
            if done == 0 {
                break;
            }
            let idx = row_of(this, slot);
            let skip: u32 = lf_checker_rt::callee_thiscall!(5, u32, rows, idx);
            if skip & 0xff != 0 {
                slot += 1;
                continue;
            }
            let key = rd32(table.wrapping_add(idx.wrapping_mul(4)));
            let kind: u32 = lf_checker_rt::callee_thiscall!(4, u32, key);
            let width = match kind {
                1 | 2 | 3 | 5 => WIDE,
                _ => 0,
            };
            if selected == slot {
                done = 0;
                let entry: u32 = lf_checker_rt::callee_thiscall!(6, u32, rows, idx);
                if entry != 0 {
                    let size: u32 = lf_checker_rt::callee_thiscall!(7, u32, entry);
                    if size <= MAX_COPY {
                        let src = (entry + ENTRY_KEY_OFF) as *const u64;
                        (key_out as *mut u64).write_unaligned(src.read_unaligned());
                        done = 1;
                    }
                }
                wr8(flag_out, done);
            } else {
                let advanced = pos.wrapping_add(width);
                if advanced > bound {
                    done = 0;
                } else {
                    let stored: u32 =
                        lf_checker_rt::callee_thiscall!(8, u32, rows, idx, pos, width);
                    if stored & 0xff == 0 {
                        done = 0;
                    } else {
                        let bit = 1u32.wrapping_shl(slot);
                        let (lo, hi) = if slot < 32 {
                            (bit, 0)
                        } else if slot < 64 {
                            (0, bit)
                        } else {
                            (0, 0)
                        };
                        wr32(mask_out, lo);
                        wr32(mask_out + 4, hi);
                        done = 1;
                    }
                }
                pos = advanced;
            }
            slot += 1;
        }
        done as u32
    }
});
