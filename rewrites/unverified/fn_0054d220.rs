// original: 0x0054D220 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_24, player_schema::LeaderboardInfo, 10>::vf14

/// Scan one leaderboard's rounds and report whether the scan completed.
///
/// `this` is the leaderboard-info object: dword at `+0` points at its virtual
/// table, slot `+0x2c` answers a setup query (the round the caller selected)
/// and slot `+0x30` maps a round number to a key. `tag` is the cursor base
/// and is passed through to the emit helper on the first round, after which
/// its slot holds the saved cursor; `buf` points at an 8-byte buffer that
/// takes the copied payload; `out_mask`
/// takes an 8-byte round mask; `out_flag` takes one status byte; `ctx` is the
/// helpers' context; `size` is the buffer span added to `buf` as the cursor
/// limit. Returns 1 when the scan ran to the end, 0 otherwise, in `al`.
///
/// The scan zeroes both outputs, reads the selected round, then asks the
/// setup helper about board 0xcf; a refusal ends the scan with 0. Otherwise
/// it runs 24 rounds: each round fetches its key, lets a gate helper skip the
/// round, reads the key's element from the setup table and maps its kind to a
/// step (8 for kinds 1, 2, 3 and 5, 0 for kind 4 and anything else). On the
/// selected round it looks the row up and, when the row exists and its size
/// is at most 8, copies the 8 payload bytes at row offset 4 into `buf`; the
/// status byte records whether that copy happened. On every other round it
/// advances the cursor by the step, fails the scan if the cursor passed the
/// limit, and otherwise emits through the helper, which stores the round's
/// single-bit mask. A failed round ends the scan at the next round's top.
///
/// Edge cases: a gate skip leaves everything untouched; a selected round of
/// 24 or more means every round takes the cursor path; the selected-round
/// path restores the cursor from the tag slot, which still holds the incoming
/// tag when no cursor round has run yet.
///
/// Original: 0x0054D220 (thiscall, six stack words, board id 0xcf).
lf_checker_rt::export!(thiscall, rw_0054d220(this: u32, arg_tag: u32, buf: u32, out_mask: u32, out_flag: u32, ctx: u32, size: u32) -> u32 {
    unsafe {
        const VF_SELECTED: u32 = 0x2c;
        const VF_ROUND_KEY: u32 = 0x30;
        const BOARD_ID: u32 = 0xcf;
        const ROUNDS: u32 = 24;
        const FULL_STEP: u32 = 8;
        const ROW_PAYLOAD: u32 = 4;
        const COPY_OK_SIZE: i32 = 8;
        const CAL_SETUP: u32 = 3;
        const CAL_GATE: u32 = 4;
        const CAL_KIND: u32 = 5;
        const CAL_ROW: u32 = 6;
        const CAL_ROW_SIZE: u32 = 7;
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

        wr32(out_mask, 0);
        wr32(out_mask.wrapping_add(4), 0);
        wr8(out_flag, 0);
        let limit = size.wrapping_add(arg_tag);
        let vtable = rd32(this);
        let selected_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VF_SELECTED)) as usize);
        let selected = selected_fn(this);
        let mut setup_out = [0u32; 3];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CAL_SETUP,
            u32,
            BOARD_ID,
            setup_out.as_mut_ptr() as u32
        );
        if (ok as u8) == 0 {
            return 0;
        }
        let table = setup_out[2];
        let key_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VF_ROUND_KEY)) as usize);
        let mut cursor = arg_tag;
        let mut slot = arg_tag;
        let mut alive = 1u8;
        let mut round = 0u32;
        while round < ROUNDS {
            if alive == 0 {
                break;
            }
            let key = key_fn(this, round);
            let gate: u32 = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, ctx, key);
            if (gate as u8) == 0 {
                let elem = rd32(table.wrapping_add(key.wrapping_mul(4)));
                let kind: u32 = lf_checker_rt::callee_thiscall!(CAL_KIND, u32, elem);
                let step = match kind.wrapping_sub(1) {
                    0 | 1 | 2 | 4 => FULL_STEP,
                    _ => 0,
                };
                if selected == round {
                    alive = 0;
                    let row: u32 = lf_checker_rt::callee_thiscall!(CAL_ROW, u32, ctx, key);
                    if row != 0 {
                        let row_size: u32 =
                            lf_checker_rt::callee_thiscall!(CAL_ROW_SIZE, u32, row);
                        if (row_size as i32) <= COPY_OK_SIZE {
                            let payload =
                                (row.wrapping_add(ROW_PAYLOAD) as *const u64).read_unaligned();
                            (buf as *mut u64).write_unaligned(payload);
                            alive = 1;
                        }
                    }
                    cursor = slot;
                    wr8(out_flag, alive);
                } else {
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        alive = 0;
                    } else {
                        let emit: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_EMIT, u32, ctx, key, slot, step
                        );
                        if (emit as u8) == 0 {
                            alive = 0;
                        } else {
                            wr32(out_mask, 1u32 << round);
                            wr32(out_mask.wrapping_add(4), 0);
                            alive = 1;
                        }
                    }
                    slot = cursor;
                }
            }
            round += 1;
        }
        alive as u32
    }
});
