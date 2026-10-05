// original: 0x0x005411B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_TeamMafya, player_schema::LeaderboardInfo, 10>::vf14

/// Leaderboard-row fan-out for one concrete leaderboard schema (virtual slot
/// 0x14 of `rlConcreteLeaderboardInfo<Schema, LeaderboardInfo, 10>`).
///
/// Walks `ROW_COUNT` rows for this schema (`SCHEMA_CONST` identifies the
/// schema to the probe callee). For each row it asks the row source (vtable
/// slot `VT_ROW` on `this`) for the row key, then asks the gate callee
/// whether the row needs the slow path. Skipped rows keep the running `live`
/// flag; a row on the slow path is classified by the classifier callee into
/// a block size (8 for classes 1, 2, 3 and 5, else 0) and then either matches
/// the wanted row (looked up through the context object: a hit whose entry
/// size is at most 8 in the signed sense copies its 8 payload bytes to
/// `entry_out` and keeps the walk alive, anything else kills it and records
/// the flag) or advances the cursor by the block size inside `base + limit`
/// and records the row bit on success.
///
/// Arguments (thiscall: `this` in ECX, six words on the stack, callee pops
/// 0x18): `this` is the leaderboard-info object (vtable at `+0`); `base` is
/// the cursor start; `entry_out` receives one 8-byte payload; `bits_out`
/// receives the 64-bit row mask (replaced, not accumulated, by each
/// successful range write); `flag_out` receives the final `live` byte after
/// every matched row; `ctx` is the lookup context passed in ECX to the
/// direct callees; `limit` bounds the cursor range. Returns the `live` flag
/// in AL (1 = the walk is still alive). The probe callee answers in AL too:
/// a zero low byte returns early with that value.
///
/// Edge cases: a null lookup result kills the walk; an entry size above 8
/// (signed) skips the copy and kills it; a cursor step past the cap kills
/// it; a failed range write kills it; class 4 takes the default block size
/// 0 like every class outside 1, 2, 3 and 5.
lf_checker_rt::export!(thiscall, rw_005411b0(this: u32, base: u32, entry_out: u32, bits_out: u32, flag_out: u32, ctx: u32, limit: u32) -> u32 {
    unsafe {
        const SCHEMA_CONST: u32 = 0x3F;
        const ROW_COUNT: u32 = 5;
        const VT_MATCH: u32 = 0x2c;
        const VT_ROW: u32 = 0x30;
        const BLOCK_FULL: u32 = 8;
        const ENTRY_SIZE_MAX: i32 = 8;
        const C_PROBE: u32 = 3;
        const C_GATE: u32 = 4;
        const C_CLASS: u32 = 5;
        const C_LOOKUP: u32 = 6;
        const C_SIZE: u32 = 7;
        const C_WRITE: u32 = 8;

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
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn vt_call0(this: u32, slot: u32) -> u32 {
            unsafe {
                let tgt = rd32(rd32(this).wrapping_add(slot));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(this)
            }
        }
        #[inline(always)]
        unsafe fn vt_call1(this: u32, slot: u32, arg: u32) -> u32 {
            unsafe {
                let tgt = rd32(rd32(this).wrapping_add(slot));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(this, arg)
            }
        }

        wr32(bits_out, 0);
        wr32(bits_out + 4, 0);
        wr8(flag_out, 0);
        let cap = limit.wrapping_add(base);
        let matched = vt_call0(this, VT_MATCH);
        let mut probe = [0u32; 3];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(C_PROBE, u32, SCHEMA_CONST, probe.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return ok;
        }
        let rows = probe[2];
        let mut live: u8 = 1;
        let mut cursor = base;
        let mut row: u32 = 0;
        while row < ROW_COUNT {
            if live == 0 {
                break;
            }
            let key = vt_call1(this, VT_ROW, row);
            let gate: u32 = lf_checker_rt::callee_thiscall!(C_GATE, u32, ctx, key);
            if (gate as u8) == 0 {
                let class: u32 = lf_checker_rt::callee_thiscall!(
                    C_CLASS,
                    u32,
                    rd32(rows.wrapping_add(key.wrapping_mul(4)))
                );
                let blk = match class {
                    1 | 2 | 3 | 5 => BLOCK_FULL,
                    _ => 0,
                };
                if matched == row {
                    live = 0;
                    let ent: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, ctx, key);
                    if ent != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(C_SIZE, u32, ent);
                        if (size as i32) <= ENTRY_SIZE_MAX {
                            wr64(entry_out, rd64(ent.wrapping_add(4)));
                            live = 1;
                        }
                    }
                    wr8(flag_out, live);
                } else {
                    cursor = cursor.wrapping_add(blk);
                    if cursor > cap {
                        live = 0;
                    } else {
                        let w: u32 = lf_checker_rt::callee_thiscall!(C_WRITE, u32, ctx, key, cursor, blk);
                        if (w as u8) == 0 {
                            live = 0;
                        } else {
                            live = 1;
                            // One bit in a 64-bit mask, replaced each time:
                            // rows 0..31 land in the low word, 32..63 in the
                            // high word (the original's bts addresses the bit
                            // modulo 32), row 64 and up clear both words.
                            let bit = 1u32.wrapping_shl(row);
                            let (lo, hi) = if row < 32 {
                                (bit, 0)
                            } else if row < 64 {
                                (0, bit)
                            } else {
                                (0, 0)
                            };
                            wr32(bits_out, lo);
                            wr32(bits_out + 4, hi);
                        }
                    }
                }
            }
            row += 1;
        }
        live as u32
    }
});
