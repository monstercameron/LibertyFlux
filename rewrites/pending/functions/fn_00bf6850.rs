// original: 0x00bf6850 blend_keys
/// Blend four key values between this and the target by the factor `a2` and
/// emit each through the keyed setter. (The factor instruction reads
/// [esp+0x1c], which static accounting places in the middle of (a0, a1), but
/// a fixed-input probe proves the value used is exactly a2; the addressing is
/// recorded as an anomaly in the lane report.)
export!(thiscall, rw_bf6850(this: *const u8, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let t = f32::from_bits(a2);
        let dst = a1 as *const u8;
        let lerp = |off: usize| -> u32 {
            let d = f32::from_bits(*((dst.add(off)) as *const u32));
            let s = f32::from_bits(*((this.add(off)) as *const u32));
            ((d - s) * t + s).to_bits()
        };
        let _: u32 = callee_thiscall!(1, u32, a0, relocated(0xEBBF18), lerp(0x18));
        let tag = *((this.add(8)) as *const u32);
        let h1: u32 = callee_cdecl!(2, u32, relocated(0xEBBF20), 0);
        if tag != h1 {
            let _: u32 = callee_thiscall!(1, u32, a0, relocated(0xEBBF34), lerp(0x1C));
        }
        let h2: u32 = callee_cdecl!(3, u32, relocated(0xEBBF3C), 0);
        let mut skip = tag == h2;
        if !skip {
            let h3: u32 = callee_cdecl!(4, u32, relocated(0xEBBF50), 0);
            skip = tag == h3;
        }
        if !skip {
            let _: u32 = callee_thiscall!(1, u32, a0, relocated(0xEBBF64), lerp(0x20));
        }
        callee_thiscall!(1, u32, a0, relocated(0xEBBF6C), lerp(0x24))
    }
});
