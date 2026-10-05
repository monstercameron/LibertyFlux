// original: 0x00c25010 cam_interp_teardown (proposed)
/// Tear down both interpolators when present (guarding each release on its
/// field being non-null), then tail-call the shared teardown worker with
/// the object. Returns the worker's result.
///
/// Original: 0x00c25010 (thiscall, no stack words; tail call).
lf_checker_rt::export!(thiscall, rw_00c25010(obj: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x14c;
        const SLOT_B: u32 = 0x150;
        if (obj.wrapping_add(SLOT_A) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, obj);
        }
        if (obj.wrapping_add(SLOT_B) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, obj);
        }
        lf_checker_rt::callee_thiscall!(3, u32, obj)
    }
});
