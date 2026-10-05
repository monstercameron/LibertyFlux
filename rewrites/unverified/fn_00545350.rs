// original: 0x00545350 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_13, player_schema::LeaderboardInfo, 10>::vf14

/// Fill one ranked episodic leaderboard's info row (virtual slot 14).
///
/// `this` is the leaderboard-info object. `want` (vtable slot `0x2c`, no
/// arguments) chooses which of the 26 rows is wanted, then the resolver
/// callee takes the leaderboard id (`0xad` for this instantiation) in
/// `ecx` and a pointer to a three-word scratch struct in `edx`; the struct's
/// third word is a table base used below. If the resolver answers false the
/// function returns its answer at once.
///
/// Otherwise the outputs are cleared first: two zero words at `mask` and one
/// zero byte at `flag`. Then rows `0..26` are visited. For each row the step
/// callee (vtable slot `0x30`) maps the row index to a row handle; when the
/// filter callee accepts the handle the row is done. When it declines, a key
/// is loaded from `table[row]` and classified: codes 1, 2, 3, 5 and 6 advance
/// the cursor by 8, every other code leaves it. If the row is the wanted one,
/// the fetch callee returns a record (or null); a record whose measured size
/// is at most 8 contributes its eight bytes at `+4` to `buf`, and `flag` is
/// set to whether anything was contributed while the cursor reloads from its
/// saved slot. Any other row moves the cursor from `cursor_in` (bounded above
/// by `cursor_in + span`)
/// and, through the commit callee, leaves exactly bit `row` set in the
/// 64-bit `mask`. The first declined row clears the status and ends the walk.
/// Returns 1 when every visited row was accepted, else 0 (low byte only).
///
/// Original: 0x00545350 (thiscall: object in `ecx`, six stack words, callee pops
/// `0x18`; the upper bytes of the `u32` return are unspecified register
/// leftovers, so only `al` is compared).
lf_checker_rt::export!(thiscall, rw_00545350(this: u32, cursor_in: u32, buf: u32, mask: u32, flag: u32, session: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xad;
        const SLOT_WANT: u32 = 0x2c;
        const SLOT_STEP: u32 = 0x30;
        const ROWS: u32 = 26;
        const STEP_FULL: u32 = 8;
        const RECORD_OFF: u32 = 4;
        const RECORD_MAX: u32 = 8;
        const CAL_RESOLVE: u32 = 3;
        const CAL_FILTER: u32 = 4;
        const CAL_CLASSIFY: u32 = 5;
        const CAL_FETCH: u32 = 6;
        const CAL_MEASURE: u32 = 7;
        const CAL_COMMIT: u32 = 8;

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

        wr32(mask, 0);
        wr32(mask.wrapping_add(4), 0);
        wr8(flag, 0);

        let limit = span.wrapping_add(cursor_in);
        let vt = rd32(this);
        let want_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(SLOT_WANT)) as usize);
        let want = want_of(this);
        let mut aux = [0u32; 3];
        let ready: u32 =
            lf_checker_rt::callee_fastcall!(CAL_RESOLVE, u32, LEADERBOARD_ID, aux.as_mut_ptr() as u32);
        if (ready & 0xff) == 0 {
            return ready;
        }
        let step_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(SLOT_STEP)) as usize);
        let mut cursor = cursor_in;
        let mut pos = cursor_in;
        let mut ok: u8 = 1;
        let mut row = 0u32;
        while row < ROWS {
            if ok == 0 {
                break;
            }
            let handle = step_of(this, row);
            let quick: u32 = lf_checker_rt::callee_thiscall!(CAL_FILTER, u32, session, handle);
            if (quick & 0xff) == 0 {
                let table = aux[2];
                let key = rd32(table.wrapping_add(handle.wrapping_mul(4)));
                let code: u32 = lf_checker_rt::callee_thiscall!(CAL_CLASSIFY, u32, key);
                let adv = match code {
                    1 | 2 | 3 | 5 | 6 => STEP_FULL,
                    _ => 0,
                };
                if want == row {
                    ok = 0;
                    let got: u32 = lf_checker_rt::callee_thiscall!(CAL_FETCH, u32, session, handle);
                    if got != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(CAL_MEASURE, u32, got);
                        if size <= RECORD_MAX {
                            wr32(buf, rd32(got.wrapping_add(RECORD_OFF)));
                            wr32(buf.wrapping_add(4), rd32(got.wrapping_add(RECORD_OFF + 4)));
                            ok = 1;
                        }
                    }
                    wr8(flag, ok);
                    pos = cursor;
                } else {
                    pos = pos.wrapping_add(adv);
                    if pos > limit {
                        ok = 0;
                    } else {
                        let done: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_COMMIT, u32, session, handle, cursor, adv
                        );
                        if (done & 0xff) == 0 {
                            ok = 0;
                        } else {
                            let bit = 1u64 << row;
                            wr32(mask, bit as u32);
                            wr32(mask.wrapping_add(4), (bit >> 32) as u32);
                            ok = 1;
                        }
                    }
                    cursor = pos;
                }
            }
            row = row.wrapping_add(1);
        }
        ok as u32
    }
});
