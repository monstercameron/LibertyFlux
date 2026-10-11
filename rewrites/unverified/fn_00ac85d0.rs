// original: 0x00AC85D0 compute_stream_mode_signal

/// Select the mode signal from the object's float at `+0x138` using the two
/// ordered comparisons used by the original.
///
/// The first comparison tests `-1.0 <= value` and returns `-1.0` when true,
/// including unordered NaN inputs. Otherwise the second tests `value <= 1.0`
/// and returns `1.0` when true, also including unordered inputs; values above
/// `1.0` are returned unchanged. The function returns the result in ST0.
lf_checker_rt::export!(thiscall, rw_00ac85d0(this: u32) -> f32 {
    const VALUE_OFFSET: u32 = 0x138;
    const LOWER: f32 = -1.0;
    const UPPER: f32 = 1.0;

    let value = unsafe { ((this.wrapping_add(VALUE_OFFSET)) as *const f32).read_unaligned() };
    if !(LOWER > value) {
        LOWER
    } else if !(value > UPPER) {
        UPPER
    } else {
        value
    }
});
