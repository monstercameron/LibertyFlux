// original: 0x00a21b10 cam_peak_track_max (proposed)

/// Tracks the peak of a value, publishing rises through an out-pointer.
///
/// `a0` is a float argument (bits), `p` an out-pointer, and `this` points
/// to a record holding the peak float at `+PEAK_OFF`. When `a0` is
/// strictly above the stored peak, the out-pointer is updated to `a0`.
/// Either way the stored peak becomes `a0`. A NaN argument updates the
/// peak but not the out-pointer. Returns nothing.
///
/// Original: 0x00a21b10 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a21b10(this: u32, a0: u32, p: u32) -> u32 {
    unsafe {
        const PEAK_OFF: u32 = 0x2d0;
        let v = f32::from_bits(a0);
        let cur = f32::from_bits(((this + PEAK_OFF) as *const u32).read_unaligned());
        if v > cur {
            (p as *mut u32).write_unaligned(a0);
        }
        ((this + PEAK_OFF) as *mut u32).write_unaligned(a0);
        0
    }
});
