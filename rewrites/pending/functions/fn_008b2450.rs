// original: 0x008b2450 audio_param_block_update
/// Fold one block of four audio parameters into this object's state.
///
/// The four input floats are clamped to [0, 1] (NaN counts as 0), scaled by
/// per-parameter gains, and fanned out: a scaled copy lands in the field at
/// +0x6c, two ladders of repeated products fill the coefficient tables at
/// +0x10..+0x60, and the scaled block plus its integer tag is appended to a
/// three-slot ring starting at +0x78 whose index lives at +0xb8.
export!(thiscall, rw_008b2450(this: *mut u8, params: *const u8) -> () {
    // Gains and biases from the original's read-only float table.
    const ONE: f32 = f32::from_bits(0x3F80_0000); // 1.0
    const D_SCALE: f32 = f32::from_bits(0x3ECC_CCCD); // 0.4
    const A_SCALE: f32 = f32::from_bits(0x3E8F_5C29); // 0.28
    const A_BIAS: f32 = f32::from_bits(0x3F33_3333); // 0.7
    const B_SCALE: f32 = f32::from_bits(0x4040_0000); // 3.0
    const C_SCALE: f32 = f32::from_bits(0x4000_0000); // 2.0

    /// Clamp to [0, 1] the way the original's compare/branch ladder does:
    /// NaN and values <= 0 become +0.0, values >= 1 become 1.0.
    #[inline(always)]
    fn clamp01(v: f32) -> f32 {
        if v > 0.0 {
            if 1.0 > v {
                v
            } else {
                1.0
            }
        } else {
            0.0
        }
    }
    #[inline(always)]
    unsafe fn rf32(base: *const u8, off: usize) -> f32 {
        *(base.add(off) as *const f32)
    }
    #[inline(always)]
    unsafe fn wf32(base: *mut u8, off: usize, v: f32) {
        *(base.add(off) as *mut f32) = v;
    }
    #[inline(always)]
    unsafe fn r32(base: *const u8, off: usize) -> u32 {
        *(base.add(off) as *const u32)
    }
    #[inline(always)]
    unsafe fn w32(base: *mut u8, off: usize, v: u32) {
        *(base.add(off) as *mut u32) = v;
    }

    unsafe {
        let a = clamp01(rf32(params, 0x00));
        let b = clamp01(rf32(params, 0x04));
        let c = clamp01(rf32(params, 0x08));
        let d = clamp01(rf32(params, 0x0C));
        let tag = r32(params, 0x10);

        let d4 = d * D_SCALE;
        let a1 = a * A_SCALE + A_BIAS;
        let b3 = b * B_SCALE;
        let c2 = c * C_SCALE;

        wf32(this, 0x6C, a1);

        // First ladder: descending powers of d4 mixed with a1.
        let t2 = d4 * a1;
        let s5 = ONE - d4;
        let t1 = t2 * d4;
        let t0 = t1 * d4;
        let t3 = t0 * d4;
        wf32(this, 0x10, t2);
        wf32(this, 0x14, t1);
        wf32(this, 0x18, t0);
        wf32(this, 0x1C, t3);

        // Second ladder: powers of d4 against (1 - d4), plus its rotations.
        let p = s5 * d4;
        let q = p * d4;
        let r = q * d4;
        wf32(this, 0x50, r);
        wf32(this, 0x54, q);
        wf32(this, 0x58, p);
        wf32(this, 0x5C, s5);
        wf32(this, 0x40, q);
        wf32(this, 0x44, p);
        wf32(this, 0x48, s5);
        wf32(this, 0x4C, 0.0);
        wf32(this, 0x30, p);
        wf32(this, 0x34, s5);
        wf32(this, 0x38, 0.0);
        wf32(this, 0x3C, 0.0);
        wf32(this, 0x20, s5);
        wf32(this, 0x24, 0.0);
        wf32(this, 0x28, 0.0);
        wf32(this, 0x2C, 0.0);

        // Append the scaled block to the three-slot ring.
        let old = r32(this, 0xB8);
        let slot = (old.wrapping_add(6).wrapping_mul(5).wrapping_mul(4)) as usize;
        wf32(this, slot, a1);
        wf32(this, slot + 4, b3);
        wf32(this, slot + 8, c2);
        wf32(this, slot + 12, d4);
        w32(this, slot + 16, tag);
        w32(this, 0xB8, old.wrapping_add(1) % 3);
    }
});
