// original: 0x008972C0 aud_environment_sound_has_table_flag

/// Test whether the selected environment-sound table row has low flag bits
/// equal to one. The row selector is the object's byte at `+0xE8`, its bank is
/// at `+0x40`, and rows are 0x20 bytes apart. Selector 0xFF returns zero.
/// Otherwise EAX preserves the table word's high 24 bits and sets its low byte
/// to one for a match or zero for a non-match.
export!(thiscall, rw_008972c0(audio: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0xE8;
        const BANK: u32 = 0x40;
        const TABLE_POINTER: u32 = 0x0115_D988;
        const TABLE_FLAGS: u32 = 0x54D0;
        const ROW_STRIDE: u32 = 0x20;
        const SELECTOR_NONE: u8 = 0xFF;

        let selector = (audio.wrapping_add(SELECTOR) as *const u8).read();
        if selector == SELECTOR_NONE {
            return 0;
        }

        let bank = (audio.wrapping_add(BANK) as *const u8).read() as u32;
        let row = bank.wrapping_mul(0x37A).wrapping_add(u32::from(selector));
        let table = lf_checker_rt::global::<u32>(TABLE_POINTER).read_unaligned();
        let flags = ((table.wrapping_add(TABLE_FLAGS).wrapping_add(row * ROW_STRIDE)) as *const u32)
            .read_unaligned();
        (flags & 0xFFFF_FF00) | u32::from((flags as u8 & 3) == 1)
    }
});
