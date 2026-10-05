// original: 0x00c3b5d0 train_round_and_bump_counter (proposed)
/// Round a shifted sum, then bump a global word counter with wraparound.
///
/// `obj` points at the car, `fbits` is a float bit pattern. Forms
/// `t = [obj+4] + f`, scales by 0.02 and adds 60.0, rounds to an integer
/// through a mask-based round (absolute value via the sign bit, compare
/// against 2^23, round-half-up through add/subtract of the mask, then a
/// correction of -1.0 when the rounded value is not less than the
/// absolute input), and truncates toward zero exactly like `cvttss2si`
/// (out-of-range and NaN yield 0x80000000). The low 16 bits of that are
/// then replaced by the global counter word: when it is below 0xffff it
/// is incremented and stored back, else helper id 1 runs (cdecl, no
/// arguments) and the counter resets to 1 with EAX set to 1. Returns the
/// combined EAX. (The original also spills an intermediate to its
/// incoming stack slot; the rewrite cannot address that slot, so the
/// contract compares everything except the stack — see narrowed.)
///
/// Original: 0x00c3b5d0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00c3b5d0(obj: u32, fbits: u32) -> u32 {
    unsafe {
        const C02: u32 = 0x3ca3_d70a; // 0.02
        const C60: u32 = 0x4270_0000; // 60.0
        const SIGN: u32 = 0x8000_0000;
        const BIG: u32 = 0x4b00_0000; // 2^23
        const ONE: u32 = 0x3f80_0000; // 1.0
        const COUNTER: u32 = 0x11a8908;
        const LIMIT: u16 = 0xffff;
        const RESET: u32 = 1;
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        let t = add(
            f32::from_bits(((obj + 4) as *const u32).read_unaligned()),
            f32::from_bits(fbits),
        );
        let scaled = add(mul(t, f32::from_bits(C02)), f32::from_bits(C60));
        let sb = scaled.to_bits();
        let sign = sb & SIGN;
        // cmpltss(abs, 2^23): mask is all-ones when abs < 2^23.
        let a = f32::from_bits(sb & !SIGN);
        let m = if a < f32::from_bits(BIG) { 0xffff_ffff } else { 0 };
        // addend is 2^23 with scaled's sign, or signed zero.
        let adj = (BIG & m) | sign;
        let r = sub(add(scaled, f32::from_bits(adj)), f32::from_bits(adj));
        // cmpnless(r - scaled, signed zero): all-ones unless below.
        let d = sub(r, scaled);
        let s = f32::from_bits(sign);
        let corr = (if d < s { 0u32 } else { 0xffff_ffff }) & ONE;
        let rounded = sub(r, f32::from_bits(corr));
        // cvttss2si: invalid -> 0x80000000.
        let v: i32 = if rounded.is_nan() || rounded >= 2147483648.0 || rounded < -2147483648.0 {
            core::hint::black_box(0x80000000u32) as i32
        } else {
            core::hint::black_box(rounded) as i32
        };
        let mut eax = ((v as u32) & 0xffff_0000) | (lf_checker_rt::global::<u16>(COUNTER).read() as u32);
        if (eax & 0xffff) as u16 >= LIMIT {
            let _: u32 = lf_checker_rt::callee_cdecl!(RESET, u32,);
            lf_checker_rt::global::<u16>(COUNTER).write(1);
            eax = 1;
        } else {
            let nx = ((eax & 0xffff) as u16).wrapping_add(1);
            lf_checker_rt::global::<u16>(COUNTER).write(nx);
            eax = (eax & 0xffff_0000) | (nx as u32);
        }
        eax
    }
});
