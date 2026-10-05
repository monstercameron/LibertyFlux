// original: 0x005543a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_23, player_schema::LeaderboardInfo, 10>::vf14

/// Nineteen-round leaderboard probe for one ranked episodic race table.
///
/// `this` is the leaderboard-info object. The routine reads a selector
/// through virtual slot `+0x2c` (answer `first`), then runs rounds `i` in
/// `0..19`, fetching a row index per round through virtual slot `+0x30`.
/// Each round: the `CHECK` callee may skip the round; otherwise the
/// `CLASSIFY` callee maps the row word to a step (answer 1, 2, 3 or 5
/// steps 8, anything else steps 0). When `first == i`, the `LOOKUP`
/// callee resolves the row and, if its `ROWSIZE` is at most 8, the 8
/// bytes at row `+4` are copied to `row_out` and the low byte of
/// `mask_out` records success. Otherwise the running total (starting at
/// `base`, capped at `base + budget` unsigned) advances by the step and
/// the `COMMIT` callee records `(index, previous total, step)`; on
/// success `mask_out` gets the one-hot mask `1 << i` (low word) and
/// zero (high word). A failed round clears the sticky status, and a
/// cleared status ends the loop at the next round top.
///
/// `mask_out` (8 bytes) and `flag_out` (1 byte) are zeroed on entry.
/// Every match round rewrites `flag_out` with the round status. Returns the sticky status: 1
/// unless a round failed or the filler refused. The filler callee takes
/// a frame-local area and fills the word at `+8` with the row-table
/// pointer; only that word is ever read back. The one-hot store's
/// high-bit branches are dead for `i < 19`.
///
/// Original: thiscall, six stack words, byte result in AL. The routine
/// also scratches its incoming `base`/`budget` argument slots; those
/// caller-stack writes are not reproduced here (contract checks.stack
/// is off, recorded as narrowed).
lf_checker_rt::export!(thiscall, rw_005543a0(this: u32, base: u32, row_out: u32, mask_out: u32, flag_out: u32, store: u32, budget: u32) -> u32 {
    unsafe {
        // Indirect callee ids 0 (vtable+0x2c) and 2 (vtable+0x30) are
        // reached through the fabricated object, not the stub table.
        const FILL: u32 = 1; // fastcall(0xE0, &frame) -> al ok?
        const CHECK: u32 = 3; // thiscall(store, idx) -> al skip?
        const CLASSIFY: u32 = 4; // thiscall(word) -> step code
        const LOOKUP: u32 = 5; // thiscall(store, idx) -> row pointer
        const ROWSIZE: u32 = 6; // thiscall(row) -> size
        const COMMIT: u32 = 7; // thiscall(store, idx, total, step) -> al ok?
        const VT_INFO: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const FILL_ARG: u32 = 0xE0;
        const ROUNDS: u32 = 19;
        const STEP: u32 = 8;
        const ROW_COPY_OFF: u32 = 4;
        const TABLE_WORD: usize = 2; // frame[2] is the area's word at +8

        #[inline(always)]
        unsafe fn vcall0(vtab: u32, slot: u32, this: u32) -> u32 {
            unsafe {
                let addr = (vtab.wrapping_add(slot) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(this)
            }
        }
        #[inline(always)]
        unsafe fn vcall1(vtab: u32, slot: u32, this: u32, arg: u32) -> u32 {
            unsafe {
                let addr = (vtab.wrapping_add(slot) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(this, arg)
            }
        }

        (mask_out as *mut u32).write_unaligned(0);
        (mask_out.wrapping_add(4) as *mut u32).write_unaligned(0);
        (flag_out as *mut u8).write(0);
        let limit = base.wrapping_add(budget);
        let mut total = base;

        let vtab = (this as *const u32).read_unaligned();
        let first = vcall0(vtab, VT_INFO, this);

        let mut frame = [0u32; 4];
        let filled =
            lf_checker_rt::callee_fastcall!(FILL, u32, FILL_ARG, frame.as_mut_ptr() as u32);
        if (filled & 0xFF) == 0 {
            return 0;
        }

        let mut status: u8 = 1;
        let mut round: u32 = 0;
        while round < ROUNDS {
            if status == 0 {
                break;
            }
            let vtab = (this as *const u32).read_unaligned();
            let idx = vcall1(vtab, VT_INDEX, this, round);
            let skip = lf_checker_rt::callee_thiscall!(CHECK, u32, store, idx);
            if (skip & 0xFF) == 0 {
                let table = frame[TABLE_WORD];
                let word =
                    (table.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned();
                let code = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, word);
                // The original's switch: (code - 1) in {0, 1, 2, 4}
                // steps 8; every other value (including code 4, which is
                // case 3 falling through, and the default) steps 0.
                let step = match code.wrapping_sub(1) {
                    0 | 1 | 2 | 4 => STEP,
                    _ => 0,
                };
                if first == round {
                    let row = lf_checker_rt::callee_thiscall!(LOOKUP, u32, store, idx);
                    status = 0;
                    if row != 0 {
                        let size = lf_checker_rt::callee_thiscall!(ROWSIZE, u32, row);
                        if size <= 8 {
                            let chunk = (row.wrapping_add(ROW_COPY_OFF) as *const u64)
                                .read_unaligned();
                            (row_out as *mut u64).write_unaligned(chunk);
                            status = 1;
                        }
                    }
                    (flag_out as *mut u8).write(status);
                } else {
                    let old = total;
                    total = total.wrapping_add(step);
                    // Unsigned comparison against the cap, like the original's `ja`.
                    if total > limit {
                        status = 0;
                    } else {
                        let ok =
                            lf_checker_rt::callee_thiscall!(COMMIT, u32, store, idx, old, step);
                        if (ok & 0xFF) == 0 {
                            status = 0;
                        } else {
                            // One-hot mask of the round; the original's
                            // high-bit carry dance is dead for round < 32.
                            (mask_out as *mut u32).write_unaligned(1u32 << round);
                            (mask_out.wrapping_add(4) as *mut u32).write_unaligned(0);
                            status = 1;
                        }
                    }
                }
            }
            round += 1;
        }
        status as u32
    }
});
