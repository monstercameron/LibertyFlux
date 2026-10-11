// original: 0x0098F0C0 audio_voice_set_f32_8_12

/// Resolves a voice parameter target from the property index, bank, scale,
/// and bank table, then forwards two single precision values to the channel
/// setter. The sentinel property selects a zero target. The second input is
/// forwarded in entry order as the setter's two stack arguments; arithmetic
/// on the target wraps at 32 bits.
lf_checker_rt::export!(thiscall, rw_audio_voice_set_f32_8_12(
    voice: u32,
    first_input: f32,
    second_input: f32
) -> u32 {
    const PROPERTY_INDEX: u32 = 4;
    const BANK_INDEX: u32 = 0x40;
    const SENTINEL: u32 = 0xFF;
    const SCALE_GLOBAL: u32 = 0x115D_968;
    const TABLE_BASE_GLOBAL: u32 = 0x115D_988;
    const BANK_STRIDE: u32 = 0x6F40;
    const TABLE_VALUE: u32 = 0x6F14;

    unsafe {
        let property = u32::from((voice.wrapping_add(PROPERTY_INDEX) as *const u8).read());
        let target = if property == SENTINEL {
            0
        } else {
            let bank = u32::from((voice.wrapping_add(BANK_INDEX) as *const u8).read());
            let scale = lf_checker_rt::global::<u32>(SCALE_GLOBAL).read_unaligned();
            let table_base = lf_checker_rt::global::<u32>(TABLE_BASE_GLOBAL).read_unaligned();
            let value_address = table_base
                .wrapping_add(bank.wrapping_mul(BANK_STRIDE))
                .wrapping_add(TABLE_VALUE);
            let table_value = (value_address as *const u32).read_unaligned();
            property.wrapping_mul(scale).wrapping_add(table_value)
        };

        let set_channel: unsafe extern "thiscall" fn(u32, f32, f32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(1));
        set_channel(target, first_input, second_input)
    }
});
