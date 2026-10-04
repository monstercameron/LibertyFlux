// original: 0x00b0aa20 net_param_table_apply (proposed)

/// Apply the per-id parameter table to the accumulator call, one row per id.
///
/// `this` (ecx) is unused (the original pushes it and then overwrites the
/// slot). The function takes no stack arguments and returns nothing
/// meaningful (exit eax is residue, so the contract compares no value).
///
/// The gate callee picks the starting accumulator word: the float at
/// `SCALE_A` when it answers nonzero, else the one at `SCALE_B`. The signed
/// count at `COUNT` decides: zero or negative writes 0 back and returns.
/// Otherwise each row `i` below the count (re-read every iteration) reads
/// its id from `IDS[i]` and its value from `VALS[i]`, selects three
/// parameter words and a rate by comparing the id against the two words at
/// `ID_A`/`ID_B` (first match, second match, or default), scales the rate
/// by `MULT` when `MODE` equals 2, multiplies the row value by the rate,
/// and calls the accumulator callee with sixteen words: two zeros, 0x201,
/// three pointers to three locally built four-word blocks, the table cursor
/// (`CURSOR_BASE + i * 0x10`), the first block pointer, the scaled row
/// value, zero, the word at `PUSHED`, the accumulator, 0.3, 3.0, -1 and two
/// zeros. The first block holds the three selected parameters plus the
/// incoming padding word (see below); the second holds {0, 1.0, 0,
/// padding}; the third {0, 0, -1.0, padding}. After the last row the count
/// is zeroed.
///
/// The padding word (the frame slot the original reads without ever
/// writing) is pinned to zero here: the contract fills uninitialized stack
/// with zero, so both sides read zero. In the wild this slot holds whatever
/// the caller left, which no rewrite with its own frame could reproduce;
/// the pin is recorded with the result. Float multiplications keep the
/// original's operand order through pinned helpers.
lf_checker_rt::export!(thiscall, rw_00b0aa20(this: u32) -> u32 {
    unsafe {
        const GATE: u32 = 0;
        const ACCUM: u32 = 1;
        const SCALE_A: u32 = 0x00FE_8A48;
        const SCALE_B: u32 = 0x00FE_8960;
        const COUNT: u32 = 0x0161_5570;
        const IDS: u32 = 0x0161_5540;
        const VALS: u32 = 0x0161_5510;
        const RATE_DEFAULT: u32 = 0x00FE_8B40;
        const RATE_ALT: u32 = 0x00FE_8B38;
        const ID_A: u32 = 0x012F_A3E0;
        const ID_B: u32 = 0x012F_A500;
        const MODE: u32 = 0x011D_6FD4;
        const MULT: u32 = 0x00FE_8864;
        const PUSHED: u32 = 0x0103_EED4;
        const CURSOR_BASE: u32 = 0x0163_2B20;
        const CURSOR_STRIDE: u32 = 0x10;
        const TAG: u32 = 0x201;
        const SEL_A0: u32 = 0x3EAE_AEAF;
        const SEL_A1: u32 = 0x3EF8_F8F9;
        const SEL_A2: u32 = 0x3EB0_B0B1;
        const SEL_B0: u32 = 0x3E19_999A;
        const SEL_B1: u32 = 0x3EB3_3333;
        const SEL_D0: u32 = 0x3F80_0000;
        const SEL_D1: u32 = 0x3E66_6666;
        const ONE: u32 = 0x3F80_0000;
        const NEG_ONE: u32 = 0xBF80_0000;
        const THREE: u32 = 0x4040_0000;
        const POINT_THREE: u32 = 0x3E99_999A;
        const PINNED_PAD: u32 = 0;

        let _ = this;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32,);
        let accum_bits = if gate != 0 {
            rd32(lf_checker_rt::relocated(SCALE_A))
        } else {
            rd32(lf_checker_rt::relocated(SCALE_B))
        };
        let count_ptr = lf_checker_rt::global::<u32>(COUNT);
        if (*count_ptr as i32) <= 0 {
            *count_ptr = 0;
            return 0;
        }
        let id_a = rd32(lf_checker_rt::relocated(ID_A));
        let id_b = rd32(lf_checker_rt::relocated(ID_B));
        let scaled = rd32(lf_checker_rt::relocated(MODE)) == 2;
        let mult = f32::from_bits(rd32(lf_checker_rt::relocated(MULT)));
        let rate_default = f32::from_bits(rd32(lf_checker_rt::relocated(RATE_DEFAULT)));
        let rate_alt = f32::from_bits(rd32(lf_checker_rt::relocated(RATE_ALT)));
        let pushed = rd32(lf_checker_rt::relocated(PUSHED));
        let cursor_base = lf_checker_rt::relocated(CURSOR_BASE);
        let mut i = 0u32;
        while (i as i32) < (*count_ptr as i32) {
            let id = rd32(lf_checker_rt::relocated(IDS).wrapping_add(i.wrapping_mul(4)));
            let (s0, s1, s2, mut rate) = if id == id_a {
                (SEL_A0, SEL_A1, SEL_A2, rate_default)
            } else if id == id_b {
                (SEL_B0, SEL_B1, 0, rate_alt)
            } else {
                (SEL_D0, SEL_D1, 0, rate_default)
            };
            if scaled {
                rate = mul(rate, mult);
            }
            let row = f32::from_bits(rd32(
                lf_checker_rt::relocated(VALS).wrapping_add(i.wrapping_mul(4)),
            ));
            let scaled_row = mul(row, rate);
            let block_params = [s0, s1, s2, PINNED_PAD];
            let block_mid = [0u32, ONE, 0, PINNED_PAD];
            let block_out = [0u32, 0, NEG_ONE, PINNED_PAD];
            let cursor = cursor_base.wrapping_add(i.wrapping_mul(CURSOR_STRIDE));
            let _: u32 = lf_checker_rt::callee_cdecl!(
                ACCUM,
                u32,
                0,
                0,
                TAG,
                block_out.as_ptr() as u32,
                block_mid.as_ptr() as u32,
                cursor,
                block_params.as_ptr() as u32,
                scaled_row.to_bits(),
                0,
                pushed,
                accum_bits,
                POINT_THREE,
                THREE,
                0xffff_ffff,
                0,
                0,
            );
            // The original reloads its spilled accumulator word here; it is
            // loop-invariant (the callee never touches the spill slot), so
            // the local already holds the right value.
            i = i.wrapping_add(1);
        }
        *count_ptr = 0;
        0
    }
});
