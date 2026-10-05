// original: 0x00bed0a0 piecewise_float_blend
/// Piecewise float function of (x, n1, n2, k) with saturation near +-127.
///
/// With constants C127_5 = 127.5, C255 = 255.0, C127 = 127.0, CM128 = -128.0
/// (file VAs 0x00FE8BC8, 0x00FE8C08, 0x00FE8BC4, 0x00EBA95C) and byte
/// arguments sign-extended to n1, n2: when x > 127.5, t = (255-x)*k + n1 and
/// the result is t, minus 255 when t > 127.0; when x < -127.5, t =
/// n1 - (x+255)*k and the result is t, plus 255 when t < -128.0 (the
/// comparison is -128.0 > t); otherwise the result is n1 + (n2-n1)*k. Every
/// branch is a `comiss`+`jbe`, so NaN takes the not-greater arm, written
/// here as `!(a > b)`. All arithmetic is f32 in the original's operand
/// order with the order pinned. Returns the result in ST0. Cdecl, four
/// stack arguments. Note: the original spills the result into its arg0
/// stack slot, which a Rust rewrite cannot reproduce, so the contract
/// switches the stack check off; the spilled value always equals the
/// returned value and is observed bit-exactly via ST0.
export!(cdecl, rw_00bed0a0(x: f32, n1bits: u32, n2bits: u32, k: f32) -> f32 {
    unsafe {
        const C127_5_ADDR: u32 = 0x00FE8BC8;
        const C255_ADDR: u32 = 0x00FE8C08;
        const C127_ADDR: u32 = 0x00FE8BC4;
        const CM128_ADDR: u32 = 0x00EBA95C;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let c127_5 = f32::from_bits((relocated(C127_5_ADDR) as *const u32).read_unaligned());
        let c255 = f32::from_bits((relocated(C255_ADDR) as *const u32).read_unaligned());
        let c127 = f32::from_bits((relocated(C127_ADDR) as *const u32).read_unaligned());
        let cm128 = f32::from_bits((relocated(CM128_ADDR) as *const u32).read_unaligned());
        let n1 = (n1bits as u8) as i8 as f32;
        if !(x > c127_5) {
            let nx = -x;
            if !(nx > c127_5) {
                let n2 = (n2bits as u8) as i8 as f32;
                add(mul(sub(n2, n1), k), n1)
            } else {
                let t = sub(n1, mul(add(x, c255), k));
                if !(cm128 > t) { t } else { add(t, c255) }
            }
        } else {
            let t = add(mul(sub(c255, x), k), n1);
            if !(t > c127) { t } else { sub(t, c255) }
        }
    }
});
