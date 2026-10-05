// original: 0x008C6190 stream_rows_build
/// Build `count`-selected streaming rows through helpers, then finalise.
///
/// Runs `(count - 1) / 24 + 1` rounds (none when `count` is 0; the divisor
/// is a multiply-by-magic shift sequence): each round asks the emit helper
/// for a scratch row, the pair helper for a decoded key stored into the
/// row table at `[this]`, and the emit helper again for the row's second
/// half, stores `extra` beside the key, advances the cursor at `cursor`,
/// and from the second round on writes each row's delta word. Afterwards
/// adds `rounds * 12` to the total at `done` and, when at least one round
/// ran, runs the finalise helper and writes the trailing delta. Reports
/// the trailing delta's low byte when any round ran, else 0.
/// Original: thiscall, five stack words.
lf_checker_rt::export!(thiscall, rw_008c6190(this: u32, token: u32,
                                              count: u32, done: u32,
                                              extra: u32, cursor: u32) -> u32 {
    unsafe {
        const EMIT_CALLEE: u32 = 1;
        const EMIT2_CALLEE: u32 = 2;
        const PAIR_CALLEE: u32 = 3;
        const FINAL_CALLEE: u32 = 4;
        const COOKIE_CALLEE: u32 = 5;
        const ROW_STRIDE: u32 = 16;
        if count == 0 {
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0;
        }
        let rounds = (((count.wrapping_sub(1) as u64)
            .wrapping_mul(0xAAAAAAAB) >> 32) as u32 >> 3)
            .wrapping_add(1);
        let table = (this as *const u32).read_unaligned();
        let mut first = true;
        let mut r: u32 = 0;
        while r < rounds {
            let mut scratch: u32 = 0;
            lf_checker_rt::callee_thiscall!(
                EMIT_CALLEE, u32, token,
                &mut scratch as *mut u32 as u32, 8);
            let mut scratch2: u32 = 0;
            let key: u32 = lf_checker_rt::callee_cdecl!(
                PAIR_CALLEE, u32,
                &mut scratch2 as *mut u32 as u32, 0);
            let idx = (cursor as *const u32).read_unaligned();
            let row = table.wrapping_add(idx.wrapping_mul(ROW_STRIDE));
            (row as *mut u32).write_unaligned(key);
            lf_checker_rt::callee_thiscall!(
                EMIT2_CALLEE, u32, token, row.wrapping_add(8), 4);
            ((row + 4) as *mut u32).write_unaligned(extra);
            if !first {
                let hi = ((row + 8) as *const u32).read_unaligned();
                let lo = ((row - 8) as *const u32).read_unaligned();
                ((row - 4) as *mut u32)
                    .write_unaligned(hi.wrapping_sub(lo));
            }
            (cursor as *mut u32).write_unaligned(idx.wrapping_add(1));
            first = false;
            r += 1;
        }
        (done as *mut u32).write_unaligned(
            (done as *const u32).read_unaligned()
                .wrapping_add(rounds.wrapping_mul(12)));
        let idx = (cursor as *const u32).read_unaligned();
        let row = table.wrapping_add(idx.wrapping_mul(ROW_STRIDE));
        let ans: u32 =
            lf_checker_rt::callee_thiscall!(FINAL_CALLEE, u32, token);
        let lo = ((row - 8) as *const u32).read_unaligned();
        let delta = ans.wrapping_sub(lo);
        ((row - 4) as *mut u32).write_unaligned(delta);
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        // The finalise answer replaces al: only the delta's low byte stays.
        delta & 0xFF
    }
});
