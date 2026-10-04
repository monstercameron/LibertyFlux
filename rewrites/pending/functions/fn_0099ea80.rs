// original: 0x0099ea80 float_to_level_table
/// Map a float sample to one of twelve table entries.
///
/// Adds a global float bias, truncates toward zero with x87 semantics
/// (out-of-range and NaN inputs behave as the fistp conversion: the low word
/// ends up zero), clamps the index to 1..=12 with zero remapped to 1, and
/// returns the table dword. The explicit range guard reproduces the x87
/// overflow result that a plain Rust float-to-int cast would saturate
/// instead.
export!(stdcall, rw_0099ea80(x: f32) -> u32 {
    unsafe {
        const TWO63: f32 = f32::from_bits(0x5F00_0000); // 2^63, exact in f32
        const TABLE: u32 = 0x12843B4;
        let bias = *global::<f32>(0xFE8830);
        let t = x + bias;
        // Truncate exactly like fistp: values with |t| >= 2^63 (and NaN, via
        // the false comparison) take the overflow path whose low word is 0.
        let wide: i64 = if t.abs() < TWO63 { t as i64 } else { i64::MIN };
        let lo = wide as u32;
        let idx: u32 = if lo == 0 {
            1
        } else if lo > 12 {
            12
        } else {
            lo
        };
        *global::<u32>(TABLE).add(idx as usize)
    }
});
