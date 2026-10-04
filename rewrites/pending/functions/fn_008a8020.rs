// original: 0x008a8020 audio_scaled_ratio_response
/// Blend two audio levels from a scaled product-over-count ratio.
///
/// Returns 1.0 for a zero count. Otherwise it widens the signed count
/// through the double domain with the original's sign-dependent bias (0 or
/// 2^32), scales it by 0.001, and divides the product of the two inputs by
/// it. The smaller of that ratio and the ceiling stored at +0x1710
/// wins (a NaN ratio resolves to the ratio itself); the winner is then
/// combined with the base at +0x170C: a
/// non-negative ratio yields `base / (base + peak)` while a negative or NaN
/// ratio yields `base / (base + floor)`, where the floor is the negated
/// ceiling unless the ceiling exceeds the ratio.
export!(thiscall, rw_008a8020(this: *const u8, a: f32, n: i32, b: f32) -> f64 {
    unsafe {
        if n == 0 {
            return 1.0;
        }
        let cap = *(this.add(0x1710) as *const f32);
        let base = *(this.add(0x170C) as *const f32);
        let bias = if n < 0 { 4_294_967_296.0f64 } else { 0.0 };
        let scaled = ((n as f64 + bias) as f32) * 0.001;
        let ratio = (a * b) / scaled;
        let mut peak = if ratio > cap { cap } else { ratio };
        let neg_cap = f32::from_bits(cap.to_bits() ^ 0x8000_0000);
        let sum = base + peak;
        peak = base / sum;
        let mut floor = neg_cap;
        if !(neg_cap > ratio) {
            floor = ratio;
        }
        let out = if !(ratio >= 0.0) {
            base / (base + floor)
        } else {
            peak
        };
        out as f64
    }
});

