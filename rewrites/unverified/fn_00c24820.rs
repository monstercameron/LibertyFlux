// original: 0x00c24820 cam_ease_a (proposed)
/// Ease `t`: `post(scale(t))` where `scale` is `(t*c0 + c1)*c2` in that
/// SSE order and `post` adds `c3` to the worker's float result. The worker
/// takes its float in XMM0 with no stack arguments and returns its float in
/// XMM0; the contract transports the value through a hidden stack slot on
/// the rewrite side only.
///
/// Original: 0x00c24820 (stdcall, one stack word; float result).
lf_checker_rt::export!(stdcall, rw_00c24820(tv: u32) -> f32 {
    unsafe {
        const C0: u32 = 0x00fe8ba8;
        const C1: u32 = 0x00e8121c;
        const C2: u32 = 0x00fe8728;
        const C3: u32 = 0x00fe88e8;
        let r = lf_checker_rt::relocated;
        let t = f32::from_bits(tv);
        let c0 = (r(C0) as *const f32).read_unaligned();
        let c1 = (r(C1) as *const f32).read_unaligned();
        let c2 = (r(C2) as *const f32).read_unaligned();
        let pre = core::hint::black_box(core::hint::black_box(core::hint::black_box(t) * core::hint::black_box(c0)) + core::hint::black_box(c1)) * core::hint::black_box(c2);
        // The stub returns the float in XMM0 and its bits in EAX; Rust would
        // read an f32 result from ST0, so take the bits from EAX instead.
        let mid_bits: u32 = lf_checker_rt::callee_cdecl!(1, u32, pre.to_bits());
        let mid = f32::from_bits(mid_bits);
        let c3 = (r(C3) as *const f32).read_unaligned();
        core::hint::black_box(mid) + core::hint::black_box(c3)
    }
});
