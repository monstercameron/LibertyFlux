// original: 0x00BE58A0 task_pose_transform_out (proposed)

/// Copy the task's pose to an output record, transforming it when a
/// transform object is present.
///
/// `this` points to the task (pose words at `+0x20`..`+0x2c`, an optional
/// transform-object pointer at `+0x30`, an enable byte at `+0x80`); `out`
/// receives four words. When the enable byte is clear nothing is written
/// and the result is 0 (the original also leaks the incoming accumulator
/// in the upper return bytes; only the low byte is meaningful).
///
/// Otherwise the four pose words are copied to `out`. With no transform
/// object the result is 1. With one, and when its slot at `+0x20` is null,
/// callee 0 refreshes the object (writing the slot) and callee 1
/// re-resolves it from `+0x10`; the slot then points at a 4x4 matrix whose
/// first three rows transform the copied triple, in the original's
/// operation order, back into `out[0..3]`.
///
/// The fourth output word is never computed on the transform path: the
/// original stores an uninitialized stack slot there. Under the checker the
/// defined stack fill pins it to 0, which is what this rewrite stores.
///
/// Original: 0x00BE58A0 (thiscall, one stack word, low byte of eax is the
/// boolean result).
lf_checker_rt::export!(thiscall, rw_00BE58A0(this: u32, out: u32) -> u32 {
    unsafe {
        const POSE: u32 = 0x20;
        const XFORM_OBJ: u32 = 0x30;
        const ENABLE: u32 = 0x80;
        const OBJ_MATRIX: u32 = 0x20;
        const OBJ_RESOLVE: u32 = 0x10;
        const REFRESH_CALLEE: u32 = 0;
        const RESOLVE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        if (this + ENABLE) as *const u8 == core::ptr::null() {
            return 0;
        }
        let enabled: u8 = unsafe { ((this + ENABLE) as *const u8).read() };
        if enabled == 0 {
            return 0;
        }
        let w0 = rd32(this + POSE);
        let w1 = rd32(this + POSE + 4);
        let w2 = rd32(this + POSE + 8);
        let w3 = rd32(this + POSE + 12);
        wr32(out, w0);
        wr32(out + 4, w1);
        wr32(out + 8, w2);
        wr32(out + 12, w3);
        let obj = rd32(this + XFORM_OBJ);
        if obj == 0 {
            return 1;
        }
        if rd32(obj + OBJ_MATRIX) == 0 {
            lf_checker_rt::callee_thiscall!(REFRESH_CALLEE, u32, obj);
            let slot = rd32(obj + OBJ_MATRIX);
            lf_checker_rt::callee_thiscall!(RESOLVE_CALLEE, u32, obj + OBJ_RESOLVE, slot);
        }
        let m = rd32(obj + OBJ_MATRIX);
        let s0 = rdf(out);
        let s1 = rdf(out + 4);
        let s2 = rdf(out + 8);
        let p0 = mul(rdf(m), s0);
        let p1 = mul(rdf(m + 0x10), s1);
        let p2 = mul(rdf(m + 0x14), s1);
        let mut r0 = add(p1, p0);
        let p3 = mul(rdf(m + 0x20), s2);
        let p4 = mul(rdf(m + 0x18), s1);
        r0 = add(r0, p3);
        let p5 = mul(rdf(m + 4), s0);
        r0 = add(r0, rdf(m + 0x30));
        let mut r1 = add(p2, p5);
        let p6 = mul(rdf(m + 0x24), s2);
        r1 = add(r1, p6);
        let p7 = mul(rdf(m + 8), s0);
        r1 = add(r1, rdf(m + 0x34));
        let mut r2 = add(p4, p7);
        let p8 = mul(rdf(m + 0x28), s2);
        r2 = add(r2, p8);
        r2 = add(r2, rdf(m + 0x38));
        wrf(out, r0);
        wrf(out + 4, r1);
        // Uninitialized stack slot in the original; the defined fill is 0.
        wr32(out + 12, 0);
        wrf(out + 8, r2);
        1
    }
});
