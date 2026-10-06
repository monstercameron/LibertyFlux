// original: 0x009013f0 input_project_point (proposed)
/// Project a 2D point through the calibrated matrix into the output pair.
///
/// `a0` points at the input floats `(x0, x1)`, `a1` at two output floats,
/// `a2` is a base whose `+0x200` word is handed to the setup callee (a
/// stdcall of one word). The fill callee (invoked with a 16-word buffer)
/// provides the matrix words `W0..W15`; with the constant `C` the outputs
/// are `out0 = (C / x1acc) * x5acc` and `out1 = (C / x1acc) * x3acc` where
/// `x5acc = W4*x1 + W0*x0 + W8*0 + W12`,
/// `x3acc = W5*x1 + W1*x0 + W9*0 + W13` and
/// `x1acc = W7*x1 + W3*x0 + W11*0 + W15`, every operation in the original's
/// order (the `*0` terms are real multiplications: NaN and infinite inputs
/// propagate through them). Returns the output pointer with its low byte
/// set. Cdecl with three stack words.
export!(cdecl, rw_009013f0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Setup callee id (stdcall, one word: base + 0x200).
        const SETUP_ID: u32 = 1;
        /// Fill callee id (writes the 16 matrix words at its buffer).
        const FILL_ID: u32 = 2;
        /// Setup argument offset and calibration constant (file VA).
        const SETUP_OFF: u32 = 0x200;
        const CAL: u32 = 0x00FE88E8;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let x0 = f32::from_bits((a0 as *const u32).read_unaligned());
        let x1 = f32::from_bits(((a0.wrapping_add(4)) as *const u32).read_unaligned());
        let _: u32 = callee_stdcall!(SETUP_ID, u32, a2.wrapping_add(SETUP_OFF));
        let mut w = [0u32; 16];
        let _: u32 = callee_thiscall!(FILL_ID, u32, w.as_mut_ptr() as u32);
        let f = |i: usize| f32::from_bits(w[i]);
        let zero = 0.0f32;
        let mut x5 = add(mul(f(4), x1), mul(f(0), x0));
        x5 = add(x5, mul(f(8), zero));
        let mut x3 = add(mul(f(5), x1), mul(f(1), x0));
        x3 = add(x3, mul(f(9), zero));
        x5 = add(x5, f(12));
        let mut x1a = add(mul(f(7), x1), mul(f(3), x0));
        x3 = add(x3, f(13));
        x1a = add(x1a, mul(f(11), zero));
        let c = f32::from_bits((global::<u32>(CAL)).read_unaligned());
        x1a = add(x1a, f(15));
        let inv = core::hint::black_box(c) / core::hint::black_box(x1a);
        let o0 = mul(inv, x5);
        let o1 = mul(inv, x3);
        ((a1.wrapping_add(0)) as *mut u32).write_unaligned(o0.to_bits());
        ((a1.wrapping_add(4)) as *mut u32).write_unaligned(o1.to_bits());
        (a1 & 0xFFFFFF00) | 1
    }
});
