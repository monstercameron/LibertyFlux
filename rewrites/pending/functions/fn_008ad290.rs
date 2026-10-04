// original: 0x008ad290 audio_quant_table_lookup
/// Audio scaled table lookup: quantize `arg * 10` and fetch a table entry.
///
/// Scales the input by 10, rounds to an integer with the classic add-and-subtract
/// of 2^23 (with a sign-aware unit adjustment), truncates toward zero exactly
/// like `cvttss2si` (NaN and out-of-range mean the most negative integer),
/// clamps the index into `[0, count - 1]` where `count` is the table's word at
/// `+0x15`, and returns the table float at `+0x17 + index * 4`.
export!(thiscall, rw_008ad290(this_: *const u8, arg: f32) -> f32 {
    unsafe {
        const SCALE: f32 = 10.0; // 0x41200000
        const SIGN_MASK: u32 = 0x8000_0000;
        const ROUND: f32 = 8388608.0; // 2^23
        const ONE: f32 = 1.0;
        let scaled = arg * SCALE;
        let sign_bits = SIGN_MASK & scaled.to_bits();
        let mag = f32::from_bits(scaled.to_bits() ^ sign_bits);
        let round_mask: u32 = if mag < ROUND { 0xFFFF_FFFF } else { 0 };
        let offset = f32::from_bits((ROUND.to_bits() & round_mask) | sign_bits);
        let rounded = (scaled + offset) - offset;
        let frac = rounded - scaled;
        // NLE predicate (imm 6): set when frac > sign, or either is NaN.
        let adjust_mask: u32 = if !(frac <= f32::from_bits(sign_bits)) {
            0xFFFF_FFFF
        } else {
            0
        };
        let index_f = rounded - f32::from_bits(adjust_mask & ONE.to_bits());
        // Truncate toward zero with cvttss2si semantics (saturating `as` differs).
        let mut index: i32 = if index_f.is_nan()
            || index_f >= 2147483648.0f32
            || index_f < -2147483648.0f32
        {
            i32::MIN
        } else {
            index_f as i32
        };
        let table = *(this_ as *const u32) as *const u8;
        let count = u16::from_ne_bytes([*table.add(0x15), *table.add(0x16)]);
        let last = (count as u32).wrapping_sub(1) as i32;
        if index < 0 {
            index = 0;
        } else if index > last {
            index = last;
        }
        let at = table.wrapping_offset(0x17i32.wrapping_add(index.wrapping_mul(4)) as isize);
        core::ptr::read_unaligned(at as *const f32)
    }
});
