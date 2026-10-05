// original: 0x00698b90 unary_bitstream_decode
/// Decode a unary-coded value from a bit stream, then scale and maybe negate.
///
/// Consumes bits from the array at offset 0, advancing the position at
/// `*pos`, until the first set bit; the zero count feeds a decode call along
/// with the flag byte at offset 8. The answer OR the count shifted by the
/// flag is negated when a further stream bit is set. Returns the result.
export!(thiscall, rw_00698b90(this: u32, pos_ptr: u32) -> u32 {
    unsafe {
        let base = *(this as *const u32) as *const u32;
        let posp = pos_ptr as *mut u32;
        let mut pos = *posp;
        let mut bit =
            (*base.add((pos >> 5) as usize)).wrapping_shr(pos & 31) & 1;
        pos = pos.wrapping_add(1);
        *posp = pos;
        let mut zeros = 0u32;
        while bit == 0 {
            pos = *posp;
            bit = (*base.add((pos >> 5) as usize)).wrapping_shr(pos & 31) & 1;
            pos = pos.wrapping_add(1);
            *posp = pos;
            zeros = zeros.wrapping_add(1);
        }
        let flag = *((this as *const u8).add(8));
        
        let answer = callee_thiscall!(1, u32, this, pos_ptr, flag as u32);
        let mut value = answer | zeros.wrapping_shl((flag & 31) as u32);
        if value != 0 {
            pos = *posp;
            bit = (*base.add((pos >> 5) as usize)).wrapping_shr(pos & 31) & 1;
            *posp = pos.wrapping_add(1);
            if bit == 1 {
                value = value.wrapping_neg();
            }
        }
        value
    }
});
