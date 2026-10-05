// original: 0x00c24970 cam_ease_c (proposed)
/// Ease `t`: the input to the shared float worker is `(c1 - t*c0)*c2`
/// in that SSE order; the worker's result then has `c3` added and is
/// multiplied by `c4`. Float order is the original's.
///
/// Original: 0x00c24970 (stdcall, one stack word; float result).
lf_checker_rt::export!(stdcall, rw_00c24970(tv: u32) -> f32 {
    unsafe {
        const C0: u32 = 0x00e81218;
        const C1: u32 = 0x00e8121c;
        const C2: u32 = 0x00fe8728;
        const C3: u32 = 0x00fe88e8;
        const C4: u32 = 0x00fe8830;
        let r = lf_checker_rt::relocated;
        let t = f32::from_bits(tv);
        let c0 = (r(C0) as *const f32).read_unaligned();
        let p = core::hint::black_box(t) * core::hint::black_box(c0);
        let c1 = (r(C1) as *const f32).read_unaligned();
        let d = core::hint::black_box(c1) - core::hint::black_box(p);
        let c2 = (r(C2) as *const f32).read_unaligned();
        let pre = core::hint::black_box(d) * core::hint::black_box(c2);
        // Bits from EAX: Rust would read an f32 result from ST0, but the stub
        // returns it in XMM0/EAX.
        let mid_bits: u32 = lf_checker_rt::callee_cdecl!(1, u32, pre.to_bits());
        let mid = f32::from_bits(mid_bits);
        let c3 = (r(C3) as *const f32).read_unaligned();
        let post = core::hint::black_box(mid) + core::hint::black_box(c3);
        let c4 = (r(C4) as *const f32).read_unaligned();
        core::hint::black_box(post) * core::hint::black_box(c4)
    }
});
