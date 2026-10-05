// original: 0x00c24ab0 cam_lerp_f50 (proposed)
/// Interpolate the float at +0x50 between the two blend sources the worker
/// resolves into frame slots: `*out = a + (b - a) * t` with `a` from the
/// first slot, `b` from the second, in the original's SSE order. The slot
/// addresses are frame pointers (compared by snapshot effect, not value).
/// Returns `out` with its low byte forced to 1 (the original sets only
/// `al` after loading `out` into eax).
///
/// One of six identical shapes differing only in the field offset.
///
/// Original: 0x00c24ab0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c24ab0(obj: u32, tv: u32, out: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0x50;
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
