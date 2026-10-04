// original: 0x00a1df30 cam_blend_add_both (proposed)

/// Adds one blended value into two target fields.
///
/// `this` is passed to the worker callee together with `a0` and a flag
/// that is 1 when the mode word at `+MODE_OFF` is zero. The callee's float
/// result `f` is added into the float at `+FIELD_OFF` of each of `a1` and
/// `a2` (`f + field`, in that operand order, evaluated separately from
/// the same `f`). Returns nothing.
///
/// Original: 0x00a1df30 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00a1df30(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        const MODE_OFF: u32 = 0x130;
        const FIELD_OFF: u32 = 8;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        unsafe fn rdf(x: u32) -> f32 {
            unsafe { f32::from_bits((x as *const u32).read_unaligned()) }
        }
        let b = u32::from(((this + MODE_OFF) as *const u32).read_unaligned() == 0);
        let f: f32 = lf_checker_rt::callee_thiscall!(CALLEE, f32, this, a0, b);
        let r1 = add(f, rdf(a1 + FIELD_OFF));
        ((a1 + FIELD_OFF) as *mut u32).write_unaligned(r1.to_bits());
        let r2 = add(f, rdf(a2 + FIELD_OFF));
        ((a2 + FIELD_OFF) as *mut u32).write_unaligned(r2.to_bits());
        0
    }
});
