// original: 0x00a20c40 cam_blend_clamp_unit (proposed)

/// Blends a worker result into the stored weight and clamps to [0, 1].
///
/// The worker callee runs on the magnitude of the float at `+ABS_OFF`
/// plus four global floats; its result `f` is added to the saved weight
/// at `+W_OFF` (`f + saved`, in that order). The sum is first raised to
/// the global floor, then a negative value becomes 0.0 and a value above
/// 1.0 becomes 1.0 (a NaN sum is stored as is). The outcome lands back in
/// `+W_OFF`. Returns nothing.
///
/// Original: 0x00a20c40 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a20c40(this: u32) -> u32 {
    unsafe {
        const C_WORKER: u32 = 1;
        const W_OFF: u32 = 0x74;
        const ABS_OFF: u32 = 0x30c;
        const G0: u32 = 0x0103_c030;
        const G1: u32 = 0x0103_c034;
        const G2: u32 = 0x0103_c038;
        const G3: u32 = 0x0103_c03c;
        const G_FLOOR: u32 = 0x0103_c048;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const ONE: f32 = 1.0;
        let saved = f32::from_bits(((this + W_OFF) as *const u32).read_unaligned());
        let mag = f32::from_bits(((this + ABS_OFF) as *const u32).read_unaligned() & ABS_MASK);
        let g = lf_checker_rt::global::<u32>;
        let f: f32 = lf_checker_rt::callee_cdecl!(
            C_WORKER, f32, mag.to_bits(), g(G0).read_unaligned(), g(G1).read_unaligned(),
            g(G2).read_unaligned(), g(G3).read_unaligned()
        );
        let mut x0 = core::hint::black_box(f) + core::hint::black_box(saved);
        let floor = f32::from_bits(g(G_FLOOR).read_unaligned());
        if !(floor > x0) {
            x0 = floor;
        }
        if 0.0 > x0 {
            x0 = 0.0;
        } else if x0 > ONE {
            x0 = ONE;
        }
        ((this + W_OFF) as *mut u32).write_unaligned(x0.to_bits());
        0
    }
});
