// original: 0x00A4B5D0 vehicle_decay_u16_12be (proposed)

/// Decays the u16 at `this + LEVEL` by the truncated global rate, notifying
/// through the vtable when it reaches zero.
///
/// `w = u16[this + LEVEL]` (0x12BE). A zero level returns at once. Otherwise
/// `step = trunc(global_float * 50.0 * 16.666666)` as a 64-bit x87-style
/// truncation (out-of-range and NaN yield the indefinite value; the multiply
/// order is pinned with `black_box`), taken unsigned as `s`: when `w > s`
/// the level drops to `w - s` (note: strictly greater, unlike the sibling
/// decay which subtracts on equality too), else the level is zeroed. A still
/// non-zero level returns; a zero level notifies through virtual slot
/// `VTABLE_SLOT` (0x180) with `this` in `ecx` and five stack words (the word
/// at `this + PAYLOAD` (0x12C0), 0, 1, 0, 0x33), loading the target through
/// the object's vtable exactly like the original.
///
/// Original: 0x00A4B5D0 (thiscall, no stack words), one indirect callee.
lf_checker_rt::export!(thiscall, rw_00A4B5D0(this: u32) -> u32 {
    unsafe {
        const LEVEL: u32 = 0x12BE;
        const RATE: u32 = 0x011735BC;
        const PAYLOAD: u32 = 0x12C0;
        const VTABLE_SLOT: u32 = 0x180;
        const K1: f32 = f32::from_bits(0x4248_0000); // 50.0
        const K2: f32 = f32::from_bits(0x4185_5555); // 16.666666
        const TWO63: f32 = f32::from_bits(0x5F00_0000); // 2^63
        const ZERO: u32 = 0;
        const ONE: u32 = 1;
        const TAG: u32 = 0x33;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let w = ((this + LEVEL) as *const u16).read_unaligned() as u32;
        if w == 0 {
            return 0;
        }
        let g = f32::from_bits(
            lf_checker_rt::global::<u32>(RATE).read_unaligned(),
        );
        let x = mul(mul(g, K1), K2);
        let t: i64 = if x.is_nan() || x >= TWO63 || x < -TWO63 {
            i64::MIN
        } else {
            x as i64
        };
        let s = t as u32;
        let level = (this + LEVEL) as *mut u16;
        if w > s {
            level.write_unaligned((w - s) as u16);
        } else {
            level.write_unaligned(0);
        }
        if level.read_unaligned() != 0 {
            return 0;
        }
        let vtable = ((this as *const u32).read_unaligned() + VTABLE_SLOT)
            as *const u32;
        let target: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(vtable.read_unaligned() as usize);
        let payload = ((this + PAYLOAD) as *const u32).read_unaligned();
        target(this, payload, ZERO, ONE, ZERO, TAG);
        0
    }
});
