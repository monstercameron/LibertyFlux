// original: 0x00D72D70 replay_marker_mode (proposed)

/// Draw one replay marker in the given mode (0 or 1), positioning it from
/// the object's coordinates.
///
/// `this` points at the marker object, whose words at `POS_A` and `POS_B`
/// hold two coordinates. Both are biased by the constant at file address
/// `BIAS` (about 0.005): the draw position is `(coord_a + K, (coord_b +
/// coord_a) - K)`, computed in that order. Modes other than 0 and 1 draw
/// nothing and return the mode unchanged.
///
/// Mode 0 fills an 8-byte scratch block through the fill callee and emits
/// the pair, then emits the biased position; mode 1 does the same with a
/// 44-byte fill. Both then pick a style (7 only when the flag byte at
/// `FLAG_A` is not 106 while the flag byte at `FLAG_B` is zero, else 2),
/// commit, scale by the constant 1.1, and resolve a slot for the mode code
/// (62 for mode 0, 59 for mode 1) through the lookup callee, whose answer
/// selects the ping target. The lookup callee receives a pointer to the
/// mode's own stack slot.
///
/// Floats are compared bit-exact; the arithmetic keeps the original's
/// operand order with both operands pinned. The fill callee's out-words are
/// scripted by the contract and observed as the pair callee's arguments.
///
/// Original: 0x00D72D70 (thiscall, one stack argument, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00d72d70(this: u32, mode: u32) -> u32 {
    unsafe {
        const POS_A: u32 = 0x08;
        const POS_B: u32 = 0x10;
        const BIAS: u32 = 0x00FE86EC;
        const FLAG_A: u32 = 0x0116C250;
        const FLAG_B: u32 = 0x0116C253;
        const FLAG_SET: u8 = 0x6a;
        const STYLE_ALT: u32 = 7;
        const STYLE_MAIN: u32 = 2;
        const SCALE_1_1: u32 = 0x3f8ccccd;
        const MODE0_CODE: u32 = 0x3e;
        const MODE1_CODE: u32 = 0x3b;

        const C_FILL: u32 = 1;
        const C_PAIR: u32 = 2;
        const C_POS: u32 = 3;
        const C_STYLE: u32 = 4;
        const C_COMMIT: u32 = 5;
        const C_SCALE: u32 = 6;
        const C_LOOKUP: u32 = 7;
        const C_PING: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let coord_a = f32::from_bits(rd32(this + POS_A));
        let coord_b = f32::from_bits(rd32(this + POS_B));
        let k = f32::from_bits(rd32(lf_checker_rt::relocated(BIAS)));
        let pos_x = add(coord_a, k);
        let pos_y = sub(add(coord_b, coord_a), k);

        let code = if mode == 0 {
            let mut buf = [0u32; 2];
            lf_checker_rt::callee_cdecl!(C_FILL, u32, buf.as_mut_ptr() as u32, 8);
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, buf[0], buf[1]);
            lf_checker_rt::callee_cdecl!(C_POS, u32, pos_x.to_bits(), pos_y.to_bits());
            MODE0_CODE
        } else if mode == 1 {
            let mut buf = [0u32; 2];
            lf_checker_rt::callee_cdecl!(C_FILL, u32, buf.as_mut_ptr() as u32, 0x2c);
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, buf[0], buf[1]);
            lf_checker_rt::callee_cdecl!(C_POS, u32, pos_x.to_bits(), pos_y.to_bits());
            MODE1_CODE
        } else {
            return mode;
        };

        let flag_a = rd8(lf_checker_rt::relocated(FLAG_A));
        let flag_b = rd8(lf_checker_rt::relocated(FLAG_B));
        let style = if flag_a == FLAG_SET || flag_b != 0 { STYLE_MAIN } else { STYLE_ALT };
        lf_checker_rt::callee_cdecl!(C_STYLE, u32, style);
        lf_checker_rt::callee_cdecl!(C_COMMIT, u32, 1);
        lf_checker_rt::callee_cdecl!(C_SCALE, u32, SCALE_1_1);
        let slot: u32 = lf_checker_rt::callee_cdecl!(C_LOOKUP, u32, &mode as *const u32 as u32, code);
        lf_checker_rt::callee_cdecl!(C_PING, u32, rd32(slot));
        0
    }
});
