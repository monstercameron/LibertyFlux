// original: 0x009CB6C0 clock_field_store_normalize (proposed)

/// Store four clock fields into the timing globals, then normalize overflow.
///
/// `a0` is stored to the hour field, `a1` to the minute field, `a2` to the
/// second field, and the current tick read from the tick source global goes
/// to the accumulator field. All range checks are signed: a negative field
/// keeps its value and contributes no carry. When `a3` is non-negative it is
/// treated as a new
/// day counter sample: the difference from the previous sample plus seven,
/// divided by seven, updates the previous sample, and the remainder is added
/// to the day accumulator.
///
/// The shared tail (entered by tail jump in the original) then normalizes:
/// seconds carry into minutes at 60, minutes carry into hours at 60, hours
/// carry into the day accumulator at 24. Finally the month slot is advanced
/// through the days-per-month table while the day accumulator exceeds the
/// current month's length, and the slot is clamped into range.
///
/// All multiplications/divisions below are the original's exact
/// multiply-shift sequences, so quotients, remainders and the return value
/// match for every input, including negative dividends.
///
/// Globals (file VAs): previous sample `0x1295834`, accumulator `0x129583C`,
/// month slot `0x1295840`, day accumulator `0x1295844`, hour `0x1295848`,
/// minute `0x129584C`, second `0x1295850`, tick source `0x1173608`, and the
/// read-only month-length table at `0x103AC97`.
///
/// Original: 0x009CB6C0 (cdecl, four stack words). Returns the month length
/// byte when the slot needed no advance, otherwise the last slot-step delta.
lf_checker_rt::export!(cdecl, rw_009CB6C0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const G_PREV: u32 = 0x1295834;
        const G_ACCUM: u32 = 0x129583C;
        const G_SLOT: u32 = 0x1295840;
        const G_DAY: u32 = 0x1295844;
        const G_HOUR: u32 = 0x1295848;
        const G_MIN: u32 = 0x129584C;
        const G_SEC: u32 = 0x1295850;
        const G_TICK: u32 = 0x1173608;
        const MONTH_TABLE: u32 = 0x103AC97;
        const SECONDS_PER_MINUTE: u32 = 60;
        const MINUTES_PER_HOUR: u32 = 60;
        const HOURS_PER_DAY: u32 = 24;
        const MONTHS_PER_YEAR: u32 = 12;
        const DIV60_MAGIC: u64 = 0x88888889;
        const DIV24_MAGIC: u64 = 0xAAAAAAAB;
        const DIV12_MAGIC: i64 = 0x2AAAAAAB;

        #[inline(always)]
        unsafe fn rd(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read() }
        }
        #[inline(always)]
        unsafe fn wr(file_va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(file_va).write(v) }
        }
        /// Unsigned divide by 60 as the original's `mul` + shift sequence.
        /// Returns (quotient, remainder).
        #[inline(always)]
        fn udiv60(n: u32) -> (u32, u32) {
            let hi = ((DIV60_MAGIC * n as u64) >> 32) as u32;
            let q = hi >> 5;
            let r = n.wrapping_sub(q.wrapping_mul(SECONDS_PER_MINUTE));
            (q, r)
        }
        /// Unsigned divide by 24 as the original's `mul` + shift sequence.
        /// Returns (quotient, remainder).
        #[inline(always)]
        fn udiv24(n: u32) -> (u32, u32) {
            let hi = ((DIV24_MAGIC * n as u64) >> 32) as u32;
            let q = hi >> 4;
            let r = n.wrapping_sub(q.wrapping_mul(HOURS_PER_DAY));
            (q, r)
        }

        // Prologue: store the four fields and the current tick.
        wr(G_ACCUM, rd(G_TICK));
        wr(G_HOUR, a0);
        wr(G_MIN, a1);
        if (a3 as i32) >= 0 {
            // Signed divide of (sample - previous + 7) by 7: Rust `/` and `%`
            // on i32 truncate toward zero exactly like `idiv`. The divisor is
            // the constant 7, so no divide error is possible.
            let diff = (a3 as i32).wrapping_sub(rd(G_PREV) as i32).wrapping_add(7);
            let rem = diff % 7;
            wr(G_PREV, a3);
            wr(G_DAY, (rd(G_DAY) as i32).wrapping_add(rem) as u32);
        }
        wr(G_SEC, a2);

        // Shared tail: carry seconds into minutes, minutes into hours,
        // hours into the day accumulator. The range checks are SIGNED
        // (`jl`): a negative field keeps its value and contributes no carry.
        let mut field = rd(G_SEC);
        if (field as i32) >= SECONDS_PER_MINUTE as i32 {
            let (q, r) = udiv60(field);
            wr(G_SEC, r);
            field = rd(G_MIN).wrapping_add(q);
            wr(G_MIN, field);
        } else {
            field = rd(G_MIN);
        }
        if (field as i32) >= MINUTES_PER_HOUR as i32 {
            let (q, r) = udiv60(field);
            wr(G_MIN, r);
            field = rd(G_HOUR).wrapping_add(q);
            wr(G_HOUR, field);
        } else {
            field = rd(G_HOUR);
        }
        if (field as i32) >= HOURS_PER_DAY as i32 {
            let (q, r) = udiv24(field);
            wr(G_HOUR, r);
            field = rd(G_DAY).wrapping_add(q);
            wr(G_DAY, field);
        } else {
            field = rd(G_DAY);
        }

        // Advance the month slot while the day accumulator exceeds the
        // current month's length. Each step adds one slot minus twelve
        // times the slot divided by twelve (signed, original's sequence).
        let mut slot = rd(G_SLOT);
        let month_len = (lf_checker_rt::relocated(MONTH_TABLE).wrapping_add(slot) as *const u8).read();
        if (field as i32) > month_len as i32 {
            wr(G_DAY, 1);
            let step_result = loop {
                let prod = DIV12_MAGIC.wrapping_mul(slot as i32 as i64);
                let mut edt = (prod >> 32) as i32;
                edt >>= 1;
                let steps = ((((edt as u32) >> 31) as i32).wrapping_add(edt))
                    .wrapping_mul(3)
                    .wrapping_shl(2);
                let delta = 1i32.wrapping_sub(steps);
                slot = (slot as i32).wrapping_add(delta) as u32;
                let cur =
                    (lf_checker_rt::relocated(MONTH_TABLE + 1).wrapping_add(slot).wrapping_sub(1)
                        as *const u8)
                        .read();
                if cur >= 1 {
                    break delta as u32;
                }
            };
            if (slot as i32) > MONTHS_PER_YEAR as i32 {
                slot = 1;
            }
            wr(G_SLOT, slot);
            step_result
        } else {
            if (slot as i32) > MONTHS_PER_YEAR as i32 {
                slot = 1;
            }
            wr(G_SLOT, slot);
            month_len as u32
        }
    }
});
