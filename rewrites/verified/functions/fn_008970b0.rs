// original: 0x008970B0 aud_environment_sound_get_table_value

/// Look up an environment-sound table word using the object's selector byte
/// and bank byte. The table has 0x20-byte rows; the selected dword is reduced
/// to a signed 29-bit value. Selector 0xFF returns -1. This thiscall routine
/// also discards one unused stack word.
export!(thiscall, rw_008970b0(audio: u32, _unused: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0xE8;
        const BANK: u32 = 0x40;
        const TABLE_POINTER: u32 = 0x0115_D988;
        const TABLE_VALUE: u32 = 0x54C8;
        const ROW_STRIDE: u32 = 0x20;
        const SELECTOR_NONE: u8 = 0xFF;

        let selector = (audio.wrapping_add(SELECTOR) as *const u8).read();
        if selector == SELECTOR_NONE {
            return u32::MAX;
        }

        let bank = (audio.wrapping_add(BANK) as *const u8).read() as u32;
        let row = bank.wrapping_mul(0x37A).wrapping_add(u32::from(selector));
        let table = lf_checker_rt::global::<u32>(TABLE_POINTER).read_unaligned();
        let value = ((table.wrapping_add(TABLE_VALUE).wrapping_add(row * ROW_STRIDE)) as *const u32)
            .read_unaligned();
        ((value.wrapping_shl(3)) as i32 >> 3) as u32
    }
});
