// original: 0x0057DC60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_175, player_schema::LeaderboardInfo, 10>::vf14

/// Ranked-race leaderboard column fetch for board id 391 (0x187).
///
/// `thiscall`: `this` points to the leaderboard-info object whose virtual
/// slots `VT_COUNT` (+0x2c) and `VT_COLUMN` (+0x30) supply a per-board value
/// and, for each column index 0..19, a column id. `cur` is the starting
/// cursor, `budget` the byte budget added to it to form the cursor bound;
/// `ctx` is an opaque context passed to the emit/check/stat callees.
/// Outputs: eight value bytes at `val_out`, a two-word column mask at
/// `mask_out` (bit `i` set when column `i` is emitted on the mask path),
/// and a flag byte at `flag_out` rewritten on every value-path column.
/// Returns the final flag (0 once anything fails).
///
/// Flow: zero the outputs, call vf11, then fetch the column-entry table for
/// this board id through `CAL_FETCH` (writes two words through `info`).
/// When the fetch reports false, return 0. Otherwise run 19 columns: stop
/// early once the flag is clear; fetch the column id via vf12; ask
/// `CAL_CHECK` whether to skip the column; classify the table entry with
/// `CAL_KIND` (answers 1, 2, 3 or 5 mean an 8-byte column, anything else a
/// 0-byte one); on the value path (`vf11 == index`) resolve the stat object
/// with `CAL_STAT`, check its size with `CAL_STAT_SZ` (at most 8) and copy
/// its eight bytes; on the mask path grow the cursor, fail when it passes
/// the bound, and emit one mask bit through `CAL_EMIT`. Float-free: the
/// eight-byte copy is two word moves. Original: 0x0057DC60 (thiscall, six
/// stack words, returns al).
lf_checker_rt::export!(thiscall, rw_0057DC60(this: u32, cur: u32, val_out: u32, mask_out: u32, flag_out: u32, ctx: u32, budget: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x187;
        const VT_COUNT: u32 = 0x2c;
        const VT_COLUMN: u32 = 0x30;
        const N_COLUMNS: u32 = 19;
        const CAL_FETCH: u32 = 2;
        const CAL_CHECK: u32 = 4;
        const CAL_KIND: u32 = 5;
        const CAL_STAT: u32 = 6;
        const CAL_STAT_SIZE: u32 = 7;
        const CAL_EMIT: u32 = 8;
        const WIDE_COLUMN: u32 = 8;
        const MAX_STAT_SIZE: u32 = 8;

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
        wr32(mask_out.wrapping_add(4), 0);
        wr8(flag_out, 0);
        let bound = budget.wrapping_add(cur);
        let mut cursor = cur;
        let vtable = rd32(this);
        let board_value: u32 = {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(VT_COUNT)) as usize);
            f(this)
        };
        let mut info = [0u32; 2];
        info[1] = 0;
        let fetched =
            lf_checker_rt::callee_fastcall!(CAL_FETCH, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if fetched & 0xFF == 0 {
            return fetched & 0xFF;
        }
        let table = info[1];
        let mut flag: u8 = 1;
        let mut index = 0u32;
        while index < N_COLUMNS {
            if flag == 0 {
                break;
            }
            let column: u32 = {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable.wrapping_add(VT_COLUMN)) as usize);
                f(this, index)
            };
            let skip = lf_checker_rt::callee_thiscall!(CAL_CHECK, u32, ctx, column);
            if skip & 0xFF != 0 {
                index += 1;
                continue;
            }
            let entry = rd32(table.wrapping_add(column.wrapping_mul(4)));
            let kind = lf_checker_rt::callee_thiscall!(CAL_KIND, u32, entry);
            let width = if kind == 0xFFFF_FFFF || kind == 0 || kind > 5 || kind == 4 {
                0
            } else {
                WIDE_COLUMN
            };
            if board_value == index {
                let mut column_flag: u8 = 0;
                let stat = lf_checker_rt::callee_thiscall!(CAL_STAT, u32, ctx, column);
                if stat != 0 {
                    let size = lf_checker_rt::callee_thiscall!(CAL_STAT_SIZE, u32, stat);
                    if size <= MAX_STAT_SIZE {
                        wr32(val_out, rd32(stat.wrapping_add(4)));
                        wr32(val_out.wrapping_add(4), rd32(stat.wrapping_add(8)));
                        column_flag = 1;
                    }
                }
                wr8(flag_out, column_flag);
                flag = column_flag;
            } else {
                cursor = cursor.wrapping_add(width);
                if cursor > bound {
                    flag = 0;
                } else {
                    let emitted =
                        lf_checker_rt::callee_thiscall!(CAL_EMIT, u32, ctx, column, cursor, width);
                    if emitted & 0xFF == 0 {
                        flag = 0;
                    } else {
                        flag = 1;
                        let bit = 1u32 << (index & 31);
                        let (lo, hi) = if index < 32 {
                            (bit, 0)
                        } else if index < 64 {
                            (0, bit)
                        } else {
                            (0, 0)
                        };
                        wr32(mask_out, lo);
                        wr32(mask_out.wrapping_add(4), hi);
                    }
                }
            }
            index += 1;
        }
        flag as u32
    }
});
