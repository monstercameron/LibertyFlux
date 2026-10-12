// original: 0x0086E560 Script_InsnLength
/// Return the encoded byte length for the instruction at the supplied byte
/// pointer. Most marker bytes select a fixed length; the two variable forms
/// use the following byte as either a scaled count or an unscaled count. The
/// selector range is unsigned and all other first-byte values return one.
lf_checker_rt::export!(thiscall, rw_0086E560(instruction: u32) -> u32 {
    const FIXED_FIVE_FIRST_OPCODE: u8 = 0x22;
    const FIXED_FIVE_LAST_OPCODE: u8 = 0x24;
    const FIXED_FIVE_MIDDLE_FIRST_OPCODE: u8 = 0x29;
    const FIXED_FIVE_MIDDLE_LAST_OPCODE: u8 = 0x2A;
    const FIXED_FIVE_FINAL_OPCODE: u8 = 0x2E;
    const FIXED_THREE_OPCODE_A: u8 = 0x28;
    const FIXED_THREE_OPCODE_B: u8 = 0x30;
    const FIXED_SEVEN_OPCODE: u8 = 0x2D;
    const FIXED_FOUR_OPCODE: u8 = 0x2F;
    const FIRST_VARIABLE_OPCODE: u8 = 0x42;
    const SECOND_VARIABLE_OPCODE: u8 = 0x43;
    const FIXED_TWO_FIRST_OPCODE: u8 = 0x45;
    const FIXED_TWO_LAST_OPCODE: u8 = 0x48;
    const FIXED_FIVE_BYTES: u32 = 5;
    const FIXED_THREE_BYTES: u32 = 3;
    const FIXED_SEVEN_BYTES: u32 = 7;
    const FIXED_FOUR_BYTES: u32 = 4;
    const FIXED_TWO_BYTES: u32 = 2;
    const UNKNOWN_OPCODE_BYTES: u32 = 1;
    const SCALE_SHIFT: u32 = 3;
    const VARIABLE_BASE_BYTES: u32 = 2;

    unsafe {
        let encoded = instruction as *const u8;
        let opcode = encoded.read_unaligned();

        match opcode {
            FIXED_FIVE_FIRST_OPCODE..=FIXED_FIVE_LAST_OPCODE
            | FIXED_FIVE_MIDDLE_FIRST_OPCODE..=FIXED_FIVE_MIDDLE_LAST_OPCODE
            | FIXED_FIVE_FINAL_OPCODE => FIXED_FIVE_BYTES,
            FIXED_THREE_OPCODE_A | FIXED_THREE_OPCODE_B => FIXED_THREE_BYTES,
            FIXED_SEVEN_OPCODE => FIXED_SEVEN_BYTES,
            FIXED_FOUR_OPCODE => FIXED_FOUR_BYTES,
            FIRST_VARIABLE_OPCODE => (encoded.add(1).read_unaligned() as u32)
                .wrapping_shl(SCALE_SHIFT)
                .wrapping_add(VARIABLE_BASE_BYTES),
            SECOND_VARIABLE_OPCODE => (encoded.add(1).read_unaligned() as u32)
                .wrapping_add(VARIABLE_BASE_BYTES),
            FIXED_TWO_FIRST_OPCODE..=FIXED_TWO_LAST_OPCODE => FIXED_TWO_BYTES,
            _ => UNKNOWN_OPCODE_BYTES,
        }
    }
});
