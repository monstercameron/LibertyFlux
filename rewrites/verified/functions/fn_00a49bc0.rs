// original: 0x00a49bc0 vehicle_clamped_delta
/// Input minus a mode-selected threshold, or -1.0 when under it.
///
/// The threshold is 350.0 while the mode word is below 2, else 770.0.
/// Returns `input - threshold` unless the threshold exceeds the input
/// (NaN input takes the subtraction path), in which case -1.0 comes back
/// (stdcall, two stack arguments; the second is unread). Result in ST0.
export!(stdcall, rw_00a49bc0(value: u32, _filler: u32) -> f32 {
    unsafe {
        let mode = (relocated(0x011d6fd4) as *const i32).read_unaligned();
        let thresh = if mode < 2 { 0x00e9d128 } else { 0x00e9d134 };
        let limit = f32::from_bits((relocated(thresh) as *const u32).read_unaligned());
        let input = f32::from_bits(value);
        if core::hint::black_box(limit) > core::hint::black_box(input) {
            f32::from_bits((relocated(0x00fe8d94) as *const u32).read_unaligned())
        } else {
            core::hint::black_box(input) - core::hint::black_box(limit)
        }
    }
});
