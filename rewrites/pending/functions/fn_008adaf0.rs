// original: 0x008adaf0 audio_table_round_index
/// Round-and-index table lookup.
///
/// Rounds the argument to an integer with the add-magic-and-subtract trick,
/// clamps the index into the table at `[this]` (sized by its count field) and
/// returns the selected entry.
export!(thiscall, rw_008adaf0(this: u32, arg: f32) -> f64 {
    unsafe {
        let x3 = arg;
        let mag = *global::<f32>(0xFE8CF8);
        let one = *global::<f32>(0xFE88E8);
        let sign = f32::from_bits(0x80000000u32 & x3.to_bits());
        let ax = f32::from_bits(x3.to_bits() ^ sign.to_bits());
        let mut x2 = if ax < mag { mag } else { 0.0 };
        x2 = f32::from_bits(x2.to_bits() | sign.to_bits());
        let mut x1 = x3 + x2;
        let table = ld32(this);
        let limit = (((table.wrapping_add(0x15)) as *const u16).read_unaligned() as u32)
            .wrapping_sub(1);
        x1 = x1 - x2;
        let diff = x1 - x3;
        // Predicate byte is 0x06 = NLE (capstone prints "cmpnless"): unordered counts as true.
        let adj = if !(diff <= sign) { one } else { 0.0 };
        x1 = x1 - adj;
        // cvttss2si: out-of-range and NaN inputs yield 0x80000000.
        let eax: i32 = if x1.is_nan() || x1 >= 2147483648.0 || x1 < -2147483648.0 {
            i32::MIN
        } else {
            x1 as i32
        };
        let idx: u32 = if eax < 0 {
            0
        } else if eax > limit as i32 {
            limit
        } else {
            eax as u32
        };
        let addr = table.wrapping_add(idx.wrapping_mul(4)).wrapping_add(0x17);
        f32::from_bits(ld32(addr)) as f64
    }
});
