// original: 0x00550690 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_9, player_schema::LeaderboardInfo, 10>::vf14

/// Read one row of a ranked episodic-race leaderboard into caller buffers.
///
/// `this` is the leaderboard-info object: vtable slot `0x2c` returns the
/// selected row `V`, slot `0x30` maps an iteration number to a table index.
/// `a0` is a cursor start and `a5` its range (`limit = a5 + a0`, wrapping);
/// `a1` receives an 8-byte row payload, `a2` two mask words (one bit per
/// written iteration), `a3` one flag byte, `a4` is the row-set object passed
/// to every row callee. Returns 1 while every step succeeded, else 0.
///
/// The query callee (fastcall: `ecx` = leaderboard id `0xC6`, `edx` = a
/// 12-byte scratch struct) fills the scratch; its third word points at the
/// cell table, and a zero low byte returns 0 at once. Otherwise 19
/// iterations run: the per-iteration index comes from slot `0x30`, the test
/// callee may skip the iteration, and the kind callee maps the table cell to
/// a width (8 for kinds 1, 2, 3 and 5, else 0) through a jump table. On the
/// selected iteration the row callee locates the row, the size callee guards
/// the 8-byte copy into `a1`, and `a3` records success; on every other
/// iteration the cursor advances by the width (past the limit fails), the
/// emit callee writes the row, and the iteration's bit lands in `a2`. Any
/// failure clears the flag and ends the loop at the next check.
///
/// Edge cases: `V` past 18 takes the write path every iteration; a null row
/// or a size above 8 fails the selected iteration; `a0 + a5` wraps, so a
/// huge `a5` still bounds the cursor.
///
/// Original: 0x00550690 (thiscall, six stack words; return in `al`).
lf_checker_rt::export!(thiscall, rw_00550690(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xC6;
        const VT_SELECTED: u32 = 0x2c; // vtable slot returning the selected row V
        const VT_INDEX: u32 = 0x30; // vtable slot returning the per-iteration table index
        const ITERATIONS: u32 = 19;
        const WIDE: u32 = 8;
        const CAL_VF11: u32 = 1;
        const CAL_VF12: u32 = 2;
        const CAL_QUERY: u32 = 3;
        const CAL_TEST: u32 = 4;
        const CAL_KIND: u32 = 5;
        const CAL_ROW: u32 = 6;
        const CAL_SIZE: u32 = 7;
        const CAL_EMIT: u32 = 8;

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

        // Zero the outputs: two mask words and one flag byte.
        wr32(a2, 0);
        wr32(a2.wrapping_add(4), 0);
        wr8(a3, 0);

        let limit = a5.wrapping_add(a0);
        let mut cursor = a0;
        let vt = rd32(this);

        let vf11: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_SELECTED)) as usize);
        let selected = vf11(this);

        let mut query = [0u32; 3];
        query[2] = 0;
        let present: u32 = lf_checker_rt::callee_fastcall!(
            CAL_QUERY,
            u32,
            LEADERBOARD_ID,
            query.as_mut_ptr() as u32
        );
        if present & 0xff == 0 {
            return 0;
        }
        let table = query[2];

        let vf12: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_INDEX)) as usize);

        let mut ok: u8 = 1;
        let mut i: u32 = 0;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let index = vf12(this, i);
            let skip: u32 = lf_checker_rt::callee_thiscall!(CAL_TEST, u32, a4, index);
            if skip & 0xff != 0 {
                i += 1;
                continue;
            }
            let cell = rd32(table.wrapping_add(index.wrapping_mul(4)));
            let kind: u32 = lf_checker_rt::callee_thiscall!(CAL_KIND, u32, cell);
            // Jump-table switch over kind - 1: widths of 8 for kinds
            // 1, 2, 3 and 5, otherwise 0 (kind -1 and anything past 5
            // take the default arm too).
            let width: u32 = if kind == 0xffff_ffff {
                0
            } else {
                let d = kind.wrapping_sub(1);
                if d > 4 || d == 3 { 0 } else { WIDE }
            };
            if selected == i {
                let row: u32 = lf_checker_rt::callee_thiscall!(CAL_ROW, u32, a4, index);
                ok = 0;
                if row != 0 {
                    let size: u32 = lf_checker_rt::callee_thiscall!(CAL_SIZE, u32, row);
                    if size <= 8 {
                        wr32(a1, rd32(row.wrapping_add(4)));
                        wr32(a1.wrapping_add(4), rd32(row.wrapping_add(8)));
                        ok = 1;
                    }
                }
                wr8(a3, ok);
            } else {
                cursor = cursor.wrapping_add(width);
                if cursor > limit {
                    ok = 0;
                } else {
                    let done: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_EMIT, u32, a4, index, a1, width);
                    if done & 0xff == 0 {
                        ok = 0;
                    } else {
                        // One bit for this iteration, split across two words
                        // for indexes past 32/64 (never taken: i stays below 19).
                        ok = 1;
                        let mut lo: u32 = 1u32.wrapping_shl(i);
                        let mut hi: u32 = 0;
                        if i >= 0x20 {
                            hi = lo;
                            lo ^= hi;
                        }
                        if i >= 0x40 {
                            hi = lo;
                        }
                        wr32(a2, lo);
                        wr32(a2.wrapping_add(4), hi);
                    }
                }
            }
            i += 1;
        }
        ok as u32
    }
});
