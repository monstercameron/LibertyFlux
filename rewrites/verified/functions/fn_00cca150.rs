// original: 0x00cca150 task_ctor_rng_floats
/// Task constructor with game-RNG-derived float fields: base-construct
/// (id 1), store arg 0 at +0x14, fold the low bits of args 1..2 into the
/// flag byte at +0x18 (keeping old bits 7 and 0, setting bit 0), stamp the
/// vtable, then run the shared multiply-add RNG twice, scaling each 23-bit
/// draw into the float fields at +0x1C/+0x20. Returns `this`.
/// (No merged symbol; name proposed.)
export!(thiscall, rw_00cca150(this: *mut u8, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xED9F7C;
        const RNG_MULT: u64 = 0x5CDC_FAA7;
        const DRAW_MASK: u32 = 0x7F_FFFF;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *((this.add(0x14)) as *mut u32) = a0;
        let flag_bits: u8 = (((a2 as u8 & 1) << 5) | (a1 as u8 & 1)) << 1;
        let kept: u8 = *this.add(0x18) & 0x81;
        *this.add(0x18) = flag_bits | kept | 1;
        *(this as *mut u32) = relocated(VTABLE);
        let g0 = global::<u32>(0x11101A0);
        let g1 = global::<u32>(0x11101A4);
        let draw = |lo: u32, hi: u32| {
            let prod = (lo as u64).wrapping_mul(RNG_MULT).wrapping_add(hi as u64);
            *g0 = prod as u32;
            *g1 = (prod >> 32) as u32;
            (prod as u32) & DRAW_MASK
        };
        let m1: f32 = *global::<f32>(0xFE864C);
        let x1 = ((draw(*g0, *g1) as f32) * m1 * *global::<f32>(0xFE8830))
            + *global::<f32>(0xFE88E8);
        *((this.add(0x1C)) as *mut f32) = x1;
        let x2 = ((draw(*g0, *g1) as f32) * m1 * *global::<f32>(0xFE8B08))
            + *global::<f32>(0xFE8B20);
        *((this.add(0x20)) as *mut f32) = x2;
        this as u32
    }
});
