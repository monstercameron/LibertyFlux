// original: 0x00bf1140 blend_table_triple
/// Blend a table triple with a source triple by weight `t`.
///
/// Computes `dst[i] = tab[i] * (1 - t) + src[i] * t` for the triple stored at
/// `this + idx*12 + 0x4c/0x50/0x54` (`idx` is the low word of the last
/// argument) into `dst+0x30/0x34/0x38`, with `1.0` read from the image's
/// float-constant table. The original also stores one word of uninitialized
/// stack scratch to `dst+0x3c`; the contract pins scratch to zero, so this
/// rewrite stores zero there. Returns `src`.
export!(thiscall, rw_bf1140(this_obj: u32, dst: u32, src: u32, t_bits: u32, idx_arg: u32) -> u32 {
    unsafe {
        let one = f32::from_bits(*global::<u32>(0x00fe88e8));
        let t = f32::from_bits(t_bits);
        let s = one - t;
        let idx = (idx_arg & 0xffff) as u32;
        let base = this_obj + idx * 12;
        let tab0 = f32::from_bits(*((base + 0x4c) as *const u32));
        let tab1 = f32::from_bits(*((base + 0x50) as *const u32));
        let tab2 = f32::from_bits(*((base + 0x54) as *const u32));
        let src0 = f32::from_bits(*(src as *const u32));
        let src1 = f32::from_bits(*((src + 4) as *const u32));
        let src2 = f32::from_bits(*((src + 8) as *const u32));
        let out0 = s * tab0 + src0 * t;
        let out1 = s * tab1 + src1 * t;
        let out2 = s * tab2 + src2 * t;
        *((dst + 0x30) as *mut u32) = out0.to_bits();
        *((dst + 0x34) as *mut u32) = out1.to_bits();
        *((dst + 0x38) as *mut u32) = out2.to_bits();
        *((dst + 0x3c) as *mut u32) = 0;
        src
    }
});
