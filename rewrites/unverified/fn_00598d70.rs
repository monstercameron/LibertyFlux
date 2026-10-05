// original: 0x00598D70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_4, player_schema::LeaderboardInfo, 10>::vf14 (symbols)

/// Read one row of a concrete leaderboard into the caller's buffers.
///
/// `this` is the leaderboard-info object. The six stack arguments are an
/// accumulator base (`base`), three out-pointers (an 8-byte value slot
/// `value_out`, an 8-byte column mask `mask_out`, a 1-byte flag `flag_out`),
/// an opaque context (`ctx`) handed to every callee, and a range extension
/// (`extra`). Thiscall: `this` in ECX, callee cleans six words.
///
/// The out Buffers are zeroed first. Virtual slot `VF_PRESENT` (+`0x2c`) on
/// `this` answers the "present" column `sel`; then the id lookup
/// (`LOOKUP_CALLEE`, fastcall of `LEADERBOARD_ID` and a 12-byte frame struct)
/// fills the frame struct whose last word is a pointer to the column table.
/// A zero low byte from the lookup returns that value at once.
///
/// Otherwise each column index `i` below `COLUMN_COUNT` is visited while the
/// running flag is set: virtual slot `VF_COLUMN` (+`0x30`) maps `i` to a
/// column id, `PROBE_CALLEE` may skip the column, and otherwise the column's
/// table cell selects a width through a five-entry jump table on
/// `kind - 1`: widths 8 for kinds 1, 2, 3 and 5, else 0. When `sel == i` the
/// fetch pair (`FETCH_CALLEE`, `SIZE_CALLEE`) copies 8 bytes from the fetched
/// object at `+4` into `value_out` if the reported size fits, and the flag
/// records the outcome; any other column grows the accumulator by the width
/// and, while it stays within `extra + base` (unsigned, wrapping),
/// `COMMIT_CALLEE` writes bit `i` into the 64-bit mask at `mask_out` (low
/// word first, bit-test semantics: bit `i mod 32`, folded across the words
/// for `i >= 32`). A failed step clears the flag and ends the walk.
///
/// The return value keeps the original's low byte (the final flag) over the
/// upper bytes of the last value the original held in EAX, mirrored exactly:
/// the lookup answer on the early path, else the last callee answer, table
/// index arithmetic (`kind - 1`), or out-pointer (`flag_out`, `mask_out`).
///
/// Original: 0x00598D70 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00598d70(this: u32, base: u32, value_out: u32, mask_out: u32, flag_out: u32, ctx: u32, extra: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x156;
        const COLUMN_COUNT: u32 = 0x1a;
        const VF_PRESENT: u32 = 0x2c;
        const VF_COLUMN: u32 = 0x30;
        // Contract callee ids: 1 = [this + VF_PRESENT], 2 = lookup,
        // 3 = [this + VF_COLUMN], 4..8 = probe, kind, fetch, size, commit.
        const CAL_LOOKUP: u32 = 2;
        const CAL_VF_COLUMN: u32 = 3;
        const CAL_PROBE: u32 = 4;
        const CAL_KIND: u32 = 5;
        const CAL_FETCH: u32 = 6;
        const CAL_SIZE: u32 = 7;
        const CAL_COMMIT: u32 = 8;
        const FULL_WIDTH: u32 = 8;
        const COPY_LIMIT: u32 = 8;

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
        let cap = extra.wrapping_add(base);

        let present: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VF_PRESENT)) as usize);
        let sel = present(this);

        let mut frame = [0u32; 3];
        frame[2] = 0;
        let found: u32 = lf_checker_rt::callee_fastcall!(
            CAL_LOOKUP, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if (found as u8) == 0 {
            return found;
        }
        let table = frame[2];
        let mut eax = found;
        let mut ok: u8 = 1;
        let mut acc = base;
        let mut i = 0u32;
        while i < COLUMN_COUNT {
            if ok == 0 {
                break;
            }
            let column: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(this).wrapping_add(VF_COLUMN)) as usize);
            let col = column(this, i);
            eax = col;
            let skipped: u32 = lf_checker_rt::callee_thiscall!(CAL_PROBE, u32, ctx, col);
            eax = skipped;
            if (skipped as u8) == 0 {
                let cell = rd32(table.wrapping_add(col.wrapping_mul(4)));
                let kind: u32 = lf_checker_rt::callee_thiscall!(CAL_KIND, u32, cell);
                eax = kind.wrapping_sub(1);
                // Jump table over kind - 1, entries [T, T, T, D, T]: T sets
                // the full width, D and any out-of-range value keep zero.
                let adj = eax;
                let width = if adj <= 4 && adj != 3 { FULL_WIDTH } else { 0 };
                if sel == i {
                    let obj: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_FETCH, u32, ctx, col);
                    ok = 0;
                    eax = obj;
                    if obj != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(CAL_SIZE, u32, obj);
                        eax = size;
                        if size <= COPY_LIMIT {
                            wr32(value_out, rd32(obj.wrapping_add(4)));
                            wr32(value_out.wrapping_add(4), rd32(obj.wrapping_add(8)));
                            ok = 1;
                        }
                    }
                    eax = flag_out;
                    wr8(flag_out, ok);
                } else {
                    let prev = acc;
                    acc = acc.wrapping_add(width);
                    if acc > cap {
                        ok = 0;
                    } else {
                        let done: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_COMMIT, u32, ctx, col, prev, width);
                        eax = done;
                        if (done as u8) == 0 {
                            ok = 0;
                        } else {
                            // Bit-test of i into a zero word, folded across
                            // the 64-bit mask exactly as the original's
                            // compare-and-move pair does.
                            let mut lo = 1u32 << (i & 31);
                            let mut hi = 0u32;
                            if i >= 0x20 {
                                hi = lo;
                            }
                            lo ^= hi;
                            if i >= 0x40 {
                                hi = lo;
                            }
                            wr32(mask_out, lo);
                            wr32(mask_out.wrapping_add(4), hi);
                            ok = 1;
                            eax = mask_out;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        (eax & 0xffff_ff00) | (ok as u32)
    }
});
