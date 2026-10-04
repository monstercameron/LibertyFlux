// original: 0x00698950 bitfield_extract
/// Extract `count` bits at bit offset `index * count` from a word array.
///
/// Reads from the array at offset 0 with the width at offset 4, spanning two
/// words when the field crosses a word boundary. All shifts use x86 masking
/// semantics (a 32-bit width shifts by zero, not out).
export!(thiscall, rw_00698950(this: u32, index: u32) -> u32 {
    unsafe {
        let base = *(this as *const u32) as *const u32;
        let count = *((this as *const u32).add(1));
        let bitpos = count.wrapping_mul(index);
        let remain = 32u32.wrapping_sub(count);
        let word = bitpos >> 5;
        let bit = bitpos & 31;
        let mask = 0xFFFF_FFFFu32.wrapping_shr(remain);
        if bit <= remain {
            let v = *base.add(word as usize);
            v.wrapping_shr(bit) & mask
        } else {
            let lo = *base.add(word as usize);
            let hi = *base.add(word.wrapping_add(1) as usize);
            let combined =
                hi.wrapping_shl(32 - bit) | lo.wrapping_shr(bit);
            combined & mask
        }
    }
});
