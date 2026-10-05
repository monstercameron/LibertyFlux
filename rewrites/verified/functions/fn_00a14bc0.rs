// original: 0x00a14bc0 gated_clamp_apply (proposed)
/// Clamp two slots toward zero when every gate passes.
///
/// Returns without doing anything unless the latched global still equals
/// its idle constant, `ent` is non-null, `ent + 0x118` carries bit 0x80000
/// and `ent + 0x24` carries bit 0x20000000. Then resolves the triple at
/// `this + 0x40` through the ten-argument callee, adds 2.5 to the returned
/// word, and returns unless that sum is ordered-greater-or-equal to the
/// float at `this + 0x48`. Otherwise each of `this + 0x19c` and `out` keeps
/// its value when negative and becomes +0.0 otherwise (an unordered NaN
/// comparison also yields +0.0). No return value is set. Thiscall.
export!(thiscall, rw_00a14bc0(this: u32, ent: u32, out: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const LATCH: u32 = 0x0103b760;
        const IDLE: u32 = 0x00e9afb4;
        const BIT_A_OFF: u32 = 0x118;
        const BIT_A: u32 = 0x80000;
        const BIT_B_OFF: u32 = 0x24;
        const BIT_B: u32 = 0x20000000;
        const V0_OFF: u32 = 0x40;
        const V1_OFF: u32 = 0x44;
        const V2_OFF: u32 = 0x48;
        const K_A: u32 = 0x42c80000;
        const K_B: u32 = 0x41a00000;
        const ADD_ADDR: u32 = 0x00fe8a60;
        const CLAMP_OFF: u32 = 0x19c;
        let g = f32::from_bits(*global::<u32>(LATCH));
        if g != f32::from_bits(*global::<u32>(IDLE)) {
            return 0;
        }
        if ent == 0 {
            return 0;
        }
        if ((ent + BIT_A_OFF) as *const u32).read_unaligned() & BIT_A == 0 {
            return 0;
        }
        if ((ent + BIT_B_OFF) as *const u32).read_unaligned() & BIT_B == 0 {
            return 0;
        }
        let f0 = ((this + V0_OFF) as *const u32).read_unaligned();
        let f1 = ((this + V1_OFF) as *const u32).read_unaligned();
        let f2 = ((this + V2_OFF) as *const u32).read_unaligned();
        let mut slot = [0u32; 12];
        callee_cdecl!(
            RESOLVE,
            u32,
            f0,
            f1,
            f2,
            &mut slot as *mut u32 as u32,
            1,
            0,
            0,
            K_A,
            K_B,
            0
        );
        let got = f32::from_bits(slot[0]);
        let k = f32::from_bits(*global::<u32>(ADD_ADDR));
        let r = core::hint::black_box(got) + core::hint::black_box(k);
        let lim = f32::from_bits(f2);
        if !(r >= lim) {
            return 0;
        }
        let m = f32::from_bits(((this + CLAMP_OFF) as *const u32).read_unaligned());
        let m = if 0.0 > m { m } else { 0.0 };
        ((this + CLAMP_OFF) as *mut u32).write_unaligned(m.to_bits());
        let o = f32::from_bits((out as *const u32).read_unaligned());
        let o = if 0.0 > o { o } else { 0.0 };
        (out as *mut u32).write_unaligned(o.to_bits());
        0
    }
});
