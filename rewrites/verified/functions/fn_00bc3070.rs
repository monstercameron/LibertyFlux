// original: 0x00BC3070 task_flags_to_mask (proposed)

/// Translate the low byte of a task flag word into its task option mask.
/// Bit 0 selects the base value; bits 1, 2, 3, 5, 6 and 7 add separate option
/// bits. Input bit 4 and all higher bits are ignored. The original accepts
/// one cdecl stack word and returns the mask in EAX.
lf_checker_rt::export!(cdecl, rw_00BC3070(flags: u32) -> u32 {
    const BASE_CLEAR: u32 = 0x20;
    const BASE_SET: u32 = 0x21;
    const OPTION_BIT_1: u32 = 0x02;
    const OPTION_BIT_2: u32 = 0x100;
    const OPTION_BIT_3: u32 = 0x04;
    const OPTION_BIT_5: u32 = 0x10;
    const OPTION_BIT_6: u32 = 0x200;
    const OPTION_BIT_7: u32 = 0x400;

    let low_flags = flags as u8;
    let mut task_mask = if low_flags & 1 == 0 { BASE_CLEAR } else { BASE_SET };
    if low_flags & 0x02 != 0 { task_mask |= OPTION_BIT_1; }
    if low_flags & 0x04 != 0 { task_mask |= OPTION_BIT_2; }
    if low_flags & 0x08 != 0 { task_mask |= OPTION_BIT_3; }
    if low_flags & 0x20 != 0 { task_mask |= OPTION_BIT_5; }
    if low_flags & 0x40 != 0 { task_mask |= OPTION_BIT_6; }
    if low_flags & 0x80 != 0 { task_mask |= OPTION_BIT_7; }
    task_mask
});
