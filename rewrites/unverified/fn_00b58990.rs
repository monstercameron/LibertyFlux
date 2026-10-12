// original: 0x00b58990 flag_bit5_to_byte (proposed)

/// Copy bit 5 of the flag word at `+0x24` into the byte at `+0x74`.
///
/// `this` points to the object. The return value is the shifted flag word
/// with only its lowest bit kept: the shift is a full 32-bit shift and the
/// mask applies to the low byte alone, so the upper 24 bits of the return
/// value are bits 13..35 of the flag word (i.e. flag bits 13..31 shifted).
lf_checker_rt::export!(thiscall, rw_00b58990(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x24;
        const OUT_BYTE: u32 = 0x74;
        const BIT: u32 = 5;
        let v = (this.wrapping_add(FLAGS) as *const u32).read_unaligned();
        let shifted = v >> BIT;
        let bit = (shifted & 1) as u8;
        (this.wrapping_add(OUT_BYTE) as *mut u8).write(bit);
        (shifted & 0xffff_ff00) | (bit as u32)
    }
});
