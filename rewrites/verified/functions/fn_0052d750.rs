// original: 0x0052d750 rl_leaderboard_race24_vf14

/// Scan one leaderboard's five columns, resolving each column to a record
/// copy or a completion bit.
///
/// `this` is the leaderboard-info object (vtable at `+0x00`; slot `0x2c`
/// answers the previously completed column, slot `0x30` maps a column index
/// to a row key). `cur` is the running cursor, `mgr` the table manager
/// passed to the helpers, and `span` the bound added to the cursor.
/// Out parameters: `out_q` takes
/// an 8-byte record copy, `out_m` an 8-byte completion mask, `out_f` a
/// one-byte found flag.
///
/// The table handle for this instantiation's tag is opened first; a zero
/// answer ends the call at once, returning that zero with the mask cleared
/// and the flag cleared. Otherwise
/// each of the five columns is mapped to a row key, gated, and classified:
/// any class from 1 to 5 except 4 advances the cursor by eight bytes, while
/// class 4 and anything unclassified leave it.
/// The column matching the previously completed one resolves through the
/// record/size helpers (a record of at most eight bytes is copied to
/// `out_q` and the flag set); every other column in bounds writes bit
/// `column` into `out_m`. A failed gate skips the column without touching
/// the running state; a column past the bound, a failed write, or an empty
/// record ends the scan with a false result. The returned byte is the final
/// scan state.
///
/// Original: 0x0052d750 (thiscall: object in ECX, six stack words; the last
/// word's slot doubles as scratch once its bound term is read, and the tag
/// constant above is this instantiation's own and differs between siblings).
lf_checker_rt::export!(thiscall, rw_0052d750(this: u32, cur: u32, out_q: u32, out_m: u32, out_f: u32, mgr: u32, span: u32) -> u32 {
    unsafe {
        const VF_PREV: u32 = 0x2c;
        const VF_STEP: u32 = 0x30;
        const LEADERBOARD_TAG: u32 = 0x25;
        const COLUMNS: u32 = 5;
        const WIDE_STEP: u32 = 8;
        const MAX_COPY: u32 = 8;
        const INFO_WORDS: usize = 4;
        const INFO_ARR: usize = 2;
        const CALLEE_SETUP: u32 = 2;
        const CALLEE_GATE: u32 = 4;
        const CALLEE_CLASS: u32 = 5;
        const CALLEE_REC: u32 = 6;
        const CALLEE_SIZE: u32 = 7;
        const CALLEE_WRITE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// 64-bit mask with one bit set, matching the original's bit-test
        /// sequence (bit&31 lands in the low word below bit 32, in the high
        /// word below bit 64, nowhere above).
        #[inline(always)]
        fn completion_bit(bit: u32) -> (u32, u32) {
            if bit < 32 {
                (1u32 << bit, 0)
            } else if bit < 64 {
                (0, 1u32 << (bit & 31))
            } else {
                (0, 0)
            }
        }

        wr32(out_m, 0);
        wr32(out_m.wrapping_add(4), 0);
        (out_f as *mut u8).write(0);
        let limit = span.wrapping_add(cur);

        let prev: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VF_PREV)) as usize);
        let saved = prev(this);

        let mut info = [0u32; INFO_WORDS];
        info[INFO_ARR] = 0;
        let opened: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_SETUP, u32, LEADERBOARD_TAG, info.as_mut_ptr() as u32);
        if opened & 0xff == 0 {
            return opened & 0xff;
        }
        let rows = info[INFO_ARR];

        let mut live: u8 = 1;
        let mut cursor = cur;
        let mut col: u32 = 0;
        while live != 0 {
            let step_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(this).wrapping_add(VF_STEP)) as usize);
            let row = step_fn(this, col);
            let gate: u32 =
                lf_checker_rt::callee_thiscall!(CALLEE_GATE, u32, mgr, row);
            if gate & 0xff == 0 {
                let key = rd32(rows.wrapping_add(row.wrapping_mul(4)));
                let class: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_CLASS, u32, key);
                // The original's jump table advances every class except 4:
                // indexes 0, 1, 2 and 4 land on the wide step, index 3 (class
                // 4) skips it, and anything outside 0..=4 never reaches the
                // table at all.
                let j = class.wrapping_sub(1);
                let adv = if j <= 4 && j != 3 { WIDE_STEP } else { 0 };
                if saved == col {
                    let mut found: u8 = 0;
                    let rec: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_REC, u32, mgr, row);
                    if rec != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(CALLEE_SIZE, u32, rec);
                        if size <= MAX_COPY {
                            wr32(out_q, rd32(rec.wrapping_add(4)));
                            wr32(out_q.wrapping_add(4), rd32(rec.wrapping_add(8)));
                            found = 1;
                        }
                    }
                    (out_f as *mut u8).write(found);
                    live = found;
                } else {
                    // The bound is checked against the advanced cursor, but
                    // the helper receives the pre-advance one (the original
                    // pushes the saved slot, which is only refreshed after
                    // a successful write).
                    let call_cursor = cursor;
                    cursor = cursor.wrapping_add(adv);
                    if cursor > limit {
                        live = 0;
                    } else {
                        let done: u32 = lf_checker_rt::callee_thiscall!(
                            CALLEE_WRITE, u32, mgr, row, call_cursor, adv);
                        if done & 0xff == 0 {
                            live = 0;
                        } else {
                            let (lo, hi) = completion_bit(col);
                            wr32(out_m, lo);
                            wr32(out_m.wrapping_add(4), hi);
                            live = 1;
                        }
                    }
                }
            }
            col += 1;
            if col >= COLUMNS {
                break;
            }
        }
        live as u32
    }
});

