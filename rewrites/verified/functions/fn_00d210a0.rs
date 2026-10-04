// original: 0x00d210a0 float_band_classifier
// Compares one float against four global thresholds and returns 1 when it
// falls inside either of two bands, else 2. NaN inputs miss every band, so
// they return 2; the negated comparisons below reproduce that exactly.
export!(cdecl, rw_00d210a0(arg_bits: u32) -> u32 {
    unsafe {
        let x = f32::from_bits(arg_bits);
        let hi0 = f32::from_bits(*(global::<u32>(0x00FE8B80)));
        if x > hi0 {
            let lo0 = f32::from_bits(*(global::<u32>(0x00FE8BD0)));
            if lo0 > x {
                return 1;
            }
        }
        let hi1 = f32::from_bits(*(global::<u32>(0x00EE0CC8)));
        if !(x > hi1) {
            return 2;
        }
        let lo1 = f32::from_bits(*(global::<u32>(0x00E993FC)));
        if !(lo1 > x) {
            return 2;
        }
        1
    }
});
