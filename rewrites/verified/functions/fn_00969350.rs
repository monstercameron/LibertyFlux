// original: 0x00969350 clear_timing_flag_bit

/// Clears one bit from the object’s word array. The signed index is divided
/// by 32 arithmetically, while its low five bits select the bit within that
/// word; negative indices therefore address words before the array base.
lf_checker_rt::export!(thiscall, rw_00969350(this: u32, index: u32) -> u32 {
    unsafe {
        const WORD_ARRAY_PTR: u32 = 0x31d0;
        let array = (this.wrapping_add(WORD_ARRAY_PTR) as *const u32).read_unaligned();
        let signed_index = index as i32;
        let word_index = signed_index >> 5;
        let bit_index = index & 31;
        let clear_mask = !(1u32.wrapping_shl(bit_index));
        let word_address = array.wrapping_add(word_index.wrapping_mul(4) as u32);
        let slot = word_address as *mut u32;
        slot.write_unaligned(slot.read_unaligned() & clear_mask);
        clear_mask
    }
});
