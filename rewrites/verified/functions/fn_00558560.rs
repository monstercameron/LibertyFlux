// original: 0x00558560 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_38, player_schema::LeaderboardInfo, 10>::vf14

/// Refresh one ranked episodic-race leaderboard's cached columns.
///
/// `this` is the leaderboard-info object. `cursor` is the starting write
/// cursor and `limit_off` its allowed range (`cursor + limit_off` is the
/// limit, wrapping). `qword_out` receives one 8-byte cell, `mask_out` an
/// 8-byte bit mask with one bit per processed row, `flag_out` a success
/// byte. `ctx` is an opaque context handed to the row helpers.
///
/// The object answers two virtual queries: slot `0x2c` yields a generation
/// id, slot `0x30` maps a loop counter (0..19) to a row index. A resolver
/// call keyed by `LEADERBOARD_ID` fills a 3-word descriptor whose third
/// word points at the row table; each row's first word is classified by a
/// type query into a cursor stride of 0 (type 4, -1 or out of range) or 8
/// (types 1, 2, 3 and 5).
///
/// The loop runs 19 rows while the success flag stays set. A row the skip
/// query accepts is left alone. When the generation id equals the loop
/// counter the row is read back instead: a lookup call plus a size call,
/// copying the 8 bytes at `row + 4` into `qword_out` when the size is 8 or
/// less, and always storing the outcome byte into `flag_out`. Otherwise the
/// cursor advances by the stride and must stay within the limit, a consume
/// call takes (row, cursor, stride), and on success bit `counter` is set in
/// `mask_out` (low dword for counters below 32, high dword above, following
/// the original's bit-test sequence exactly).
///
/// Returns 0 when the resolver declines, else the low byte holds the final
/// success flag while the upper three bytes are whatever the last
/// register write left (an out-pointer or the last helper answer), which
/// the rewrite tracks in `eax` exactly like the original.
///
/// Original: 0x00558560 (thiscall, `this` in ECX, six stack words, callee
/// cleans 0x18).
lf_checker_rt::export!(thiscall, rw_00558560(this: u32, cursor: u32, qword_out: u32, mask_out: u32, flag_out: u32, ctx: u32, limit_off: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xf4;
        const ITERATIONS: u32 = 19;
        const SLOT_GENERATION: u32 = 0x2c;
        const SLOT_ROW_INDEX: u32 = 0x30;
        const ENTRY_STRIDE: u32 = 8;
        const COPY_LIMIT: u32 = 8;
        const ROW_CELL: u32 = 4;
        const BIT_HI_HALF: u32 = 0x20;
        const BIT_PAST_END: u32 = 0x40;
        const CAL_RESOLVE: u32 = 2;
        const CAL_SKIP: u32 = 4;
        const CAL_TYPE: u32 = 5;
        const CAL_LOOKUP: u32 = 6;
        const CAL_SIZE: u32 = 7;
        const CAL_CONSUME: u32 = 8;

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
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }

        wr64(mask_out, 0);
        wr8(flag_out, 0);
        let limit = cursor.wrapping_add(limit_off);
        // Tracks the original's EAX so the returned upper bytes match: the
        // original ends with `(an instruction of the original)`, keeping earlier bits.
        let mut eax: u32;
        let vtab = rd32(this);
        // Contract callee 1 answers through the planted slot.
        let generation: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab.wrapping_add(SLOT_GENERATION)) as usize);
        let gen = generation(this);
        eax = gen;
        let mut info = [0u32; 3];
        info[1] = 0;
        let resolved: u32 = lf_checker_rt::callee_fastcall!(
            CAL_RESOLVE,
            u32,
            LEADERBOARD_ID,
            info.as_mut_ptr() as u32
        );
        eax = resolved;
        if resolved == 0 {
            return 0;
        }
        let table = info[2];
        // Contract callee 3 answers through the planted slot.
        let row_index: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtab.wrapping_add(SLOT_ROW_INDEX)) as usize);
        let mut ok: u8 = 1;
        let mut cur = cursor;
        let mut iter: u32 = 0;
        while ok != 0 {
            let idx: u32 = row_index(this, iter);
            eax = idx;
            let skip: u32 = lf_checker_rt::callee_thiscall!(CAL_SKIP, u32, ctx, idx);
            eax = skip;
            if skip == 0 {
                let ty: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_TYPE,
                    u32,
                    rd32(table.wrapping_add(idx.wrapping_mul(4)))
                );
                eax = ty;
                let stride = if ty as i32 == -1 {
                    0
                } else {
                    let t = ty.wrapping_sub(1);
                    if t > 4 || t == 3 {
                        0
                    } else {
                        ENTRY_STRIDE
                    }
                };
                if gen == iter {
                    ok = 0;
                    let row: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, ctx, idx);
                    eax = row;
                    if row != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(CAL_SIZE, u32, row);
                        eax = size;
                        if size <= COPY_LIMIT {
                            wr64(qword_out, rd64(row.wrapping_add(ROW_CELL)));
                            ok = 1;
                        }
                    }
                    wr8(flag_out, ok);
                    eax = flag_out;
                } else if cur.wrapping_add(stride) > limit {
                    cur = cur.wrapping_add(stride);
                    ok = 0;
                } else {
                    // The consume call takes the pre-increment cursor: the
                    // original reads it back from its argument slot, which is
                    // only written after the call.
                    let prev = cur;
                    cur = cur.wrapping_add(stride);
                    let done: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_CONSUME,
                        u32,
                        ctx,
                        idx,
                        prev,
                        stride
                    );
                    eax = done;
                    if done == 0 {
                        ok = 0;
                    } else {
                        ok = 1;
                        // The original's bit-test sequence: bit `iter` goes
                        // to the low dword below 32, the high dword above.
                        let mut lo = 1u32.wrapping_shl(iter & 31);
                        let mut hi = 0u32;
                        if iter > BIT_HI_HALF {
                            hi = lo;
                            lo ^= hi;
                        }
                        if iter > BIT_PAST_END {
                            hi = lo;
                        }
                        wr32(mask_out, lo);
                        wr32(mask_out.wrapping_add(4), hi);
                        eax = mask_out;
                    }
                }
            }
            iter = iter.wrapping_add(1);
            if iter >= ITERATIONS {
                break;
            }
        }
        (eax & 0xffff_ff00) | ok as u32
    }
});
