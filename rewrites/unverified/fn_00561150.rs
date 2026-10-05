// original: 0x00561150 vf14_00561150

/// Scan one leaderboard's rows for a matching entry slot (virtual slot 14 of
/// this leaderboard's info object; one near-identical instantiation exists
/// per ranked episodic race).
///
/// `this` points to the info object (vtable at `+0x00`). `cursor` is the
/// current write position and `limit` its budget: entries may advance the
/// cursor up to `cursor + limit`. `quad_out` receives an 8-byte row payload,
/// `mask_out` two words with one bit set for the row taken through the
/// slow path, `flag_out` one byte holding the final live flag, and `ctx` is
/// an opaque context passed through to every helper call.
///
/// The count helper (vtable slot `0x2c`) says how many rows exist; the gate
/// helper (first direct callee, passed `0x110` and a scratch struct whose
/// third word it fills with the row-pointer array) vetoes the scan when its
/// low byte is zero. Otherwise up to 19 rows (`0x13`) are visited: the
/// index helper (vtable slot `0x30`) maps the row number to a table index,
/// the filter helper (second direct callee) skips the row when its low byte
/// is nonzero, and the class helper (third direct callee) maps the row
/// pointer to a size class. Class 1, 2, 3 or 5 advances the cursor by 8
/// bytes per row; class 4, class 0, a missing row (-1) and anything above 5
/// leave it still. The row whose number equals the count takes the direct
/// path (resolve helper, then size helper: the payload is copied only when
/// the resolved pointer is non-null and its size is at most 8, and the
/// flag byte records whether it was); every other row takes the bounded
/// path (the cursor must stay within budget, then the commit helper runs
/// and sets the row's bit in the mask). The scan stops early when the live
/// flag clears. Returns the live flag in the low byte.
///
/// Original: 0x00561150 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00561150(this: u32, cursor: u32, quad_out: u32, mask_out: u32, flag_out: u32, ctx: u32, limit: u32) -> u32 {
    unsafe {
        const GATE_ARG: u32 = 0x110;
        const SLOT_COUNT: u32 = 0x2c;
        const SLOT_INDEX: u32 = 0x30;
        const SCAN_ITERS: u32 = 0x13;
        const STEP: u32 = 8;
        const COPY_LIMIT: u32 = 8;
        const MISSING: u32 = 0xFFFF_FFFF;
        // Callee ids 1 (count) and 3 (index) are reached through the
        // object's virtual slots, not the stub table, so they need no constant here.
        const CAL_GATE: u32 = 2;
        const CAL_FILTER: u32 = 4;
        const CAL_CLASS: u32 = 5;
        const CAL_RESOLVE: u32 = 6;
        const CAL_SIZE: u32 = 7;
        const CAL_COMMIT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        // Output slots start cleared. (The original also parks the row
        // count and the updated cursor in its incoming argument slots; a
        // Rust rewrite cannot address those, so they stay in locals and
        // the contract leaves the stack check off.)
        wr32(mask_out, 0);
        wr32(mask_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);

        let mut pos = cursor;
        let bound = cursor.wrapping_add(limit);

        // Row count from the object's virtual slot 0x2c.
        let vtable = rd32(this);
        let count_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(SLOT_COUNT)) as usize);
        let count = count_fn(this);

        // Gate helper fills the row-pointer array behind the scratch struct.
        let mut info = [0u32; 3];
        info[2] = 0;
        let gate: u32 =
            lf_checker_rt::callee_fastcall!(CAL_GATE, u32, GATE_ARG, info.as_mut_ptr() as u32);
        if (gate as u8) == 0 {
            return gate;
        }
        let rows = info[2];

        let mut live = 1u8;
        let mut row = 0u32;
        loop {
            if live == 0 {
                break;
            }
            let vtable = rd32(this);
            let index_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(SLOT_INDEX)) as usize);
            let idx = index_fn(this, row);
            let go: u32 = lf_checker_rt::callee_thiscall!(CAL_FILTER, u32, ctx, idx);
            if (go as u8) != 0 {
                row += 1;
                if row >= SCAN_ITERS {
                    break;
                }
                continue;
            }
            let elem = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
            let class: u32 = lf_checker_rt::callee_thiscall!(CAL_CLASS, u32, elem);
            // Size class to cursor step: classes 1, 2, 3 and 5 advance by
            // a full entry; anything else (missing row, 0, 4, above 5)
            // holds the cursor. (The original dispatches through a jump
            // table with this exact shape.)
            let mut step = 0u32;
            if class != MISSING {
                let d = class.wrapping_sub(1);
                if d <= 4 && d != 3 {
                    step = STEP;
                }
            }
            if row == count {
                // Direct path: resolve and copy the payload.
                live = 0;
                let p: u32 = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, ctx, idx);
                if p != 0 {
                    let size: u32 = lf_checker_rt::callee_thiscall!(CAL_SIZE, u32, p);
                    if size <= COPY_LIMIT {
                        let lo = rd32(p.wrapping_add(4));
                        let hi = rd32(p.wrapping_add(8));
                        wr32(quad_out, lo);
                        wr32(quad_out.wrapping_add(4), hi);
                        live = 1;
                    }
                }
                (flag_out as *mut u8).write(live);
            } else {
                // Bounded path: advance the cursor and commit the row.
                pos = pos.wrapping_add(step);
                if pos > bound {
                    live = 0;
                } else {
                    let done: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_COMMIT, u32, ctx, idx, pos, step);
                    if (done as u8) == 0 {
                        live = 0;
                    } else {
                        // Set the row's bit across the two mask words.
                        // Reachable rows are below 32, so the high word
                        // stays zero; the general form matches the
                        // original's bit-test dance for every row.
                        let (lo, hi) = if row < 32 {
                            (1u32 << row, 0u32)
                        } else if row < 64 {
                            (0u32, 1u32 << (row - 32))
                        } else {
                            (0u32, 0u32)
                        };
                        wr32(mask_out, lo);
                        wr32(mask_out.wrapping_add(4), hi);
                        live = 1;
                    }
                }
            }
            row += 1;
            if row >= SCAN_ITERS {
                break;
            }
        }
        live as u32
    }
});
