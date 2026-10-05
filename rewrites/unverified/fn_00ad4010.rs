// original: 0x00AD4010 audio_clamp_mix_entry (proposed)

/// Add a value into one mix-table entry and clamp it to its bounds.
///
/// Unless the gate byte (from four gate globals) equals the expected flag,
/// returns the gate at once. Otherwise indexes the mix table by two
/// remainders (`(arg / 2 + 100000) mod 100`, truncating division), adds the
/// third argument as a float onto the entry, clamps the sum into
/// [upper, lower] (an unordered comparison keeps the bound, so NaN settles
/// on the lower bound) and stores it back. Cdecl/4 (row, unread, value,
/// flag); the value word doubles as the column index source and the float
/// addend. Returns the column quotient.
lf_checker_rt::export!(cdecl, rw_00ad4010(a0: u32, _a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const GATE_A: u32 = 0x011F7060;
        const GATE_B: u32 = 0x012088B4;
        const GATE_C: u32 = 0x00F1C040;
        const GATE_D: u32 = 0x01037720;
        const GATE_D_VALUE: u32 = 0x12;
        const TABLE: u32 = 0x0155C8B8;
        const UPPER: u32 = 0x00E7E870;
        const LOWER: u32 = 0x00FE8B38;
        const BIAS: i32 = 100_000;
        const DIVISOR: i32 = 100;
        const COLUMNS: i32 = 100;
        let gate = if lf_checker_rt::global::<u32>(GATE_A).read() == 1 {
            1u8
        } else if lf_checker_rt::global::<u32>(GATE_B).read()
            != lf_checker_rt::global::<u32>(GATE_C).read()
        {
            1u8
        } else if lf_checker_rt::global::<u32>(GATE_D).read() == GATE_D_VALUE {
            1u8
        } else {
            0u8
        };
        if gate != a3 as u8 {
            return gate as u32;
        }
        let half0 = (a0 as i32 / 2).wrapping_add(BIAS);
        let r0 = half0 % DIVISOR;
        let half1 = (a2 as i32 / 2).wrapping_add(BIAS);
        let q1 = half1 / DIVISOR;
        let r1 = half1 % DIVISOR;
        let idx = r0.wrapping_mul(COLUMNS).wrapping_add(r1);
        let slot = lf_checker_rt::relocated(TABLE)
            .wrapping_add((idx as u32).wrapping_mul(4)) as *mut f32;
        let cur = core::hint::black_box(slot.read());
        let add = core::hint::black_box(f32::from_bits(a2));
        let mut v = cur + add;
        let upper = lf_checker_rt::global::<f32>(UPPER).read();
        if !(v > upper) {
            v = upper;
        }
        let lower = lf_checker_rt::global::<f32>(LOWER).read();
        if !(lower > v) {
            v = lower;
        }
        slot.write(v);
        q1 as u32
    }
});
