// original: 0x00521FD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race42NoHolds, player_schema::LeaderboardInfo, 10>::vf14

/// Read one leaderboard row-set descriptor for leaderboard 0x7f (Leaderboard_Ranked_Race42NoHolds).
///
/// `this` is the leaderboard-info object: dword at `+0` is its vtable, slot
/// `+0x2c` answers a session handle, slot `+0x30` answers a column index for
/// a row number. `cursor`/`limit` form a probe window (`bound` = `limit` +
/// `cursor`, wrapping); `value_out` takes one 8-byte value, `mask_out` two
/// mask dwords, `flag_out` one flag byte, `ctx` is passed through to every
/// direct callee. Returns a full `eax` whose low byte is the success flag
/// and whose upper bytes are whatever the last step left in `eax`.
///
/// Algorithm: clear the outputs, take the session handle, ask callee 1 for
/// the column table of leaderboard 0x7f (nonzero low byte continues,
/// otherwise the callee's `eax` is the result). Then for rows 0..7 while
/// the flag is set: take the row's column index, ask callee 3 whether to
/// skip the row; otherwise classify the column through callee 4 (answers
/// 1, 2, 3 or 5 step the cursor by 8, anything else by 0). When the session
/// handle equals the row number, fetch the row object through callee 5 and,
/// if its callee-6 size is at most 8 (signed compare), copy its 8 bytes at
/// `+4` to
/// `value_out` and set the flag; if the handle differs, advance the cursor
/// by the step and, when still inside the window, ask callee 7 to confirm
/// the step and set bit `row` of the mask dwords.
///
/// Edge cases: a null row object or an oversize one clears the flag and
/// writes it out; a cursor stepped past the window or a refused confirm
/// clears the flag; a skipped row leaves the flag as it was. The callee
/// also rewrites its own incoming cursor and limit slots, which no caller
/// can observe (the proof compares heap, calls and `eax`, not the frame).
///
/// Original: 0x00521FD0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00521fd0(this: u32, cursor: u32, value_out: u32, mask_out: u32, flag_out: u32, ctx: u32, limit: u32) -> u32 {
    unsafe {
        const LB_ID: u32 = 0x7f;
        const VT_SESSION: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const ROWS: u32 = 7;
        const WIDE_STEP: u32 = 8;

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
        let bound = limit.wrapping_add(cursor);
        let vtable = rd32(this);
        let session_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_SESSION)) as usize);
        let mut eax = session_of(this);
        let session = eax;
        let mut desc = [0u32; 3];
        desc[2] = 0;
        eax = lf_checker_rt::callee_fastcall!(1, u32, LB_ID, desc.as_mut_ptr() as u32);
        if eax & 0xFF == 0 {
            return eax;
        }
        let mut bl: u32 = 1;
        let mut cursor = cursor;
        let mut row = 0u32;
        loop {
            if bl == 0 {
                break;
            }
            let vtable = rd32(this);
            let index_of: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(VT_INDEX)) as usize);
            eax = index_of(this, row);
            let index = eax;
            eax = lf_checker_rt::callee_thiscall!(3, u32, ctx, index);
            if eax & 0xFF == 0 {
                let table = desc[2];
                let column = rd32(table.wrapping_add(index.wrapping_mul(4)));
                eax = lf_checker_rt::callee_thiscall!(4, u32, column);
                let step: u32 = match eax {
                    1 | 2 | 3 | 5 => WIDE_STEP,
                    _ => 0,
                };
                if session == row {
                    let mut picked = 0u32;
                    eax = lf_checker_rt::callee_thiscall!(5, u32, ctx, index);
                    let obj = eax;
                    if obj != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, obj);
                        if (eax as i32) <= WIDE_STEP as i32 {
                            let q = (obj.wrapping_add(4) as *const u64).read_unaligned();
                            (value_out as *mut u64).write_unaligned(q);
                            picked = 1;
                        }
                    }
                    bl = picked;
                    eax = flag_out;
                    wr8(flag_out, bl as u8);
                } else {
                    let prev = cursor;
                    cursor = cursor.wrapping_add(step);
                    if cursor > bound {
                        bl = 0;
                    } else {
                        eax = lf_checker_rt::callee_thiscall!(7, u32, ctx, index, prev, step);
                        if eax & 0xFF == 0 {
                            bl = 0;
                        } else {
                            let mut lo = 0u32;
                            lo |= 1u32 << (row & 31);
                            eax = row;
                            let mut hi = 0u32;
                            if row >= 0x20 {
                                hi = lo;
                                lo ^= hi;
                            }
                            if row >= 0x40 {
                                hi = lo;
                            }
                            eax = mask_out;
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            bl = 1;
                        }
                    }
                }
            }
            row += 1;
            if row >= ROWS {
                break;
            }
        }
        (eax & 0xFFFFFF00) | (bl & 0xFF)
    }
});
