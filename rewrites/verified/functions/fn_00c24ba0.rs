// original: 0x00c24ba0 cam_blend_dispatch (proposed)
/// Resolve the two blend sources into frame slots, then dispatch on the
/// flag byte at +0x1e8: set calls the direct blend worker, clear the
/// indirect one, both with `(out, first, second, t)` where the two source
/// pointers come from the resolved slots. Returns the worker's answer with
/// its low byte forced to 1.
///
/// Original: 0x00c24ba0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c24ba0(obj: u32, tv: u32, out: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x1e8;
        let mut first: u32 = 0;
        let mut second: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            1, u32, obj,
            &mut first as *mut u32 as u32, &mut second as *mut u32 as u32
        );
        let r: u32;
        if (obj.wrapping_add(FLAG_OFF) as *const u8).read() != 0 {
            r = lf_checker_rt::callee_thiscall!(2, u32, obj, tv, first, second, out);
        } else {
            r = lf_checker_rt::callee_thiscall!(3, u32, obj, tv, first, second, out);
        }
        (r & 0xffff_ff00) | 1
    }
});
