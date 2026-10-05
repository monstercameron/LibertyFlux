// original: 0x00c24d50 cam_lerp_f54 (proposed)
/// Interpolate the float at +0x54; otherwise identical to the +0x50 shape.
///
/// Original: 0x00c24d50 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c24d50(obj: u32, tv: u32, out: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0x54;
        let mut first: u32 = 0;
        let mut second: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            1, u32, obj,
            &mut first as *mut u32 as u32, &mut second as *mut u32 as u32
        );
        let a = (first.wrapping_add(FIELD_OFF) as *const f32).read_unaligned();
        let b = (second.wrapping_add(FIELD_OFF) as *const f32).read_unaligned();
        let t = f32::from_bits(tv);
        let d = core::hint::black_box(b) - core::hint::black_box(a);
        let p = core::hint::black_box(d) * core::hint::black_box(t);
        let y = core::hint::black_box(p) + core::hint::black_box(a);
        (out as *mut f32).write_unaligned(y);
        (out & 0xffff_ff00) | 1
    }
});
