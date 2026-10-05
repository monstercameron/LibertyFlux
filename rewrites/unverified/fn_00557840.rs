// original: 0x00557840 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_35, player_schema::LeaderboardInfo, 10>::vf14

/// Leaderboard row pass over this board's 19 slots (virtual slot 14 of the
/// board-info object), specialised to leaderboard id 0xf1.
///
/// `this` points to the board object (vtable pointer at `+0`). `cur0` is the
/// cursor the pass starts from and `span` bounds how far it may advance
/// (`limit = span + cur0`, wrapping). `row_out` receives an 8-byte row copy,
/// `mask_out` a 64-bit slot mask, `flag_out` a 1-byte flag; all three are
/// cleared first. `ctx` is an opaque value forwarded to every direct callee.
///
/// The pass asks the object for its mark (virtual slot `+0x2c`, one `u32`),
/// then probes the row source (id 0xf1, answered through a 3-word scratch
/// buffer whose third word is the cell table). If the probe refuses, the
/// result is 0. Otherwise each slot `i` in `0..19` runs while the keep flag
/// is set: fetch the row handle (virtual slot `+0x30` of `this`, argument
/// `i`), run the gate callee, and when the gate declines, classify the cell
/// (`cells[row]`) into a stride: classes 1, 2, 3 and 5 advance by 8, class 4
/// and anything else (including -1 and 0) advance by 0. The marked slot
/// (`mark == i`) clears the flag, takes the handle callee's pointer, copies
/// 8 bytes from `handle+4` to `row_out` when the measure callee reports a
/// (signed) size of 8 or less, sets the flag on success, and always stores
/// the flag to `flag_out`. Any other slot advances the cursor by the stride;
/// leaving the limit clears the flag, else the commit callee runs with
/// `(row, old cursor, stride)`: refusal clears the flag, success writes bit
/// `i` into `mask_out` (low word for `i < 32`, the bit-test/mod-32 form the
/// original's `bts` sequence computes) and sets the flag. The result is the
/// flag byte.
///
/// Original: 0x00557840 (thiscall, `this` plus six stack words; callee pops
/// `0x18`). Integer arithmetic only; no floating point. The original spills
/// two scratch words into its incoming argument area (slots 0 and 5), which
/// a Rust rewrite cannot reproduce, so the contract compares everything
/// except the stack diff.
lf_checker_rt::export!(thiscall, rw_00557840(this: u32, cur0: u32, row_out: u32, mask_out: u32, flag_out: u32, ctx: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xf1;
        const VF_MARK: u32 = 0x2c;
        const VF_ROW: u32 = 0x30;
        const SLOTS: u32 = 19;
        const STRIDE: u32 = 8;
        const COPY_LIMIT: i32 = 8;
        const CAL_PROBE: u32 = 2;
        const CAL_GATE: u32 = 4;
        const CAL_CLASSIFY: u32 = 5;
        const CAL_HANDLE: u32 = 6;
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

        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        wr8(flag_out, 0);
        let limit = span.wrapping_add(cur0);

        let vtable = rd32(this);
        let mark: u32 = {
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(VF_MARK)) as usize);
            query(this)
        };

        let mut probe = [0u32; 3];
        let ready: u32 =
            lf_checker_rt::callee_fastcall!(CAL_PROBE, u32, LEADERBOARD_ID, probe.as_mut_ptr() as u32);
        if (ready as u8) == 0 {
            return 0;
        }
        let cells = probe[2];

        let mut keep: u8 = 1;
        let mut cursor = cur0;
        let mut slot: u32 = 0;
        while slot < SLOTS {
            if keep == 0 {
                break;
            }
            let row: u32 = {
                let next: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(VF_ROW)) as usize);
                next(this, slot)
            };
            let gate: u32 = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, ctx, row);
            if (gate as u8) == 0 {
                let cell = rd32(cells.wrapping_add(row.wrapping_mul(4)));
                let cls: u32 = lf_checker_rt::callee_thiscall!(CAL_CLASSIFY, u32, cell);
                // The original switches on (cls - 1) with entries {0,1,2,4}
                // taking the stride case: class 4 and anything outside 1..=5
                // (checked unsigned, so -1 and 0 land here too) keep 0.
                let d = cls.wrapping_sub(1);
                let stride: u32 = if cls == 0xffff_ffff || d > 4 || d == 3 {
                    0
                } else {
                    STRIDE
                };
                if mark == slot {
                    keep = 0;
                    let handle: u32 = lf_checker_rt::callee_thiscall!(CAL_HANDLE, u32, ctx, row);
                    if handle != 0 {
                        let size: u32 = lf_checker_rt::callee_thiscall!(CAL_MEASURE, u32, handle);
                        // Signed comparison in the original (`jg`): a huge
                        // size reads as negative and still copies.
                        if (size as i32) <= COPY_LIMIT {
                            wr32(row_out, rd32(handle.wrapping_add(4)));
                            wr32(row_out.wrapping_add(4), rd32(handle.wrapping_add(8)));
                            keep = 1;
                        }
                    }
                    wr8(flag_out, keep);
                } else {
                    let advanced = cursor.wrapping_add(stride);
                    if advanced > limit {
                        keep = 0;
                        cursor = advanced;
                    } else {
                        let done: u32 =
                            lf_checker_rt::callee_thiscall!(CAL_COMMIT, u32, ctx, row, cursor, stride);
                        if (done as u8) == 0 {
                            keep = 0;
                        } else {
                            // The original's bts/cmov sequence: bit `slot`
                            // of a 64-bit mask, low word below 32.
                            let (lo, hi) = if slot < 32 {
                                (1u32 << slot, 0)
                            } else if slot < 64 {
                                (0, 1u32 << (slot - 32))
                            } else {
                                (0, 0)
                            };
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            keep = 1;
                        }
                        cursor = advanced;
                    }
                }
            }
            slot += 1;
        }
        keep as u32
    }
});
