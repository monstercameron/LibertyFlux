// original: 0x00c24930 cam_ease_b (proposed)
/// Ease `t`: feed `(t*c0)*c1` in that SSE order to the shared float worker
/// (float in, float out, both through XMM0) and return its result.
///
/// Original: 0x00c24930 (stdcall, one stack word; float result).
lf_checker_rt::export!(stdcall, rw_00c24930(tv: u32) -> f32 {
    unsafe {
        const C0: u32 = 0x00fe8ba8;
        const C1: u32 = 0x00fe8728;
        let r = lf_checker_rt::relocated;
        let t = f32::from_bits(tv);
        let c0 = (r(C0) as *const f32).read_unaligned();
        let c1 = (r(C1) as *const f32).read_unaligned();
        let pre = core::hint::black_box(core::hint::black_box(t) * core::hint::black_box(c0)) * core::hint::black_box(c1);
        // Bits from EAX: Rust would read an f32 result from ST0, but the stub
        // returns it in XMM0/EAX.
        let mid_bits: u32 = lf_checker_rt::callee_cdecl!(1, u32, pre.to_bits());
        f32::from_bits(mid_bits)
    }
});
