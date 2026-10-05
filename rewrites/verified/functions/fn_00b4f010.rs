// original: 0x00b4f010 read_timer_value (proposed)

/// Read the task timer and store it when non-negative.
///
/// The task at `this + 0x128` is queried through its virtual slot at
/// `+0x1C` (passed `this`), returning a float. A negative result is
/// discarded (returns 0); otherwise the bits are stored to `out` and 1 is
/// returned. A NaN result counts as non-negative: it is stored and 1 is
/// returned, matching the original's jump-on-unordered compare.
///
/// Original: 0x00b4f010 (thiscall, one stack word; boolean in AL).
lf_checker_rt::export!(thiscall, rw_00b4f010(this: u32, out: u32) -> u32 {
    unsafe {
        const TASK: u32 = 0x128;
        const QUERY_SLOT: u32 = 0x1c;
        let task = ((this + TASK) as *const u32).read_unaligned();
        let vt = (task as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32, u32) -> f32 =
            core::mem::transmute(((vt + QUERY_SLOT) as *const u32).read_unaligned() as usize);
        let v = query(task, this);
        if !(0.0 > v) {
            (out as *mut u32).write_unaligned(v.to_bits());
            1
        } else {
            0
        }
    }
});
