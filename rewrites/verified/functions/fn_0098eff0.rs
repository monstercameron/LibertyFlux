// original: 0x0098EFF0 audio_voice_set_float_200

/// Computes the selected voice property address from the byte fields at
/// offsets 4 and 0x40. A sentinel property index selects address zero;
/// otherwise the global scale and per-bank table word form the address.
/// The supplied f32 is forwarded unchanged to the intercepted thiscall
/// setter, whose return value remains the wrapper's EAX result.
lf_checker_rt::export!(thiscall, rw_audio_voice_set_float_200(this: u32, value: f32) -> u32 {
    const PROPERTY_INDEX: u32 = 4;
    const BANK_INDEX: u32 = 0x40;
    const SENTINEL: u32 = 0xFF;
    const SCALE_GLOBAL: u32 = 0x115D_968;
    const TABLE_BASE_GLOBAL: u32 = 0x115D_988;
    const BANK_STRIDE: u32 = 0x6F40;
    const TABLE_VALUE: u32 = 0x6F14;

    unsafe {
        let property = u32::from((this.wrapping_add(PROPERTY_INDEX) as *const u8).read());
        let target = if property == SENTINEL {
            0
        } else {
            let bank = u32::from((this.wrapping_add(BANK_INDEX) as *const u8).read());
            let scale = lf_checker_rt::global::<u32>(SCALE_GLOBAL).read_unaligned();
            let table_base = lf_checker_rt::global::<u32>(TABLE_BASE_GLOBAL).read_unaligned();
            let bank_offset = bank.wrapping_mul(BANK_STRIDE);
            let table_address = table_base.wrapping_add(bank_offset).wrapping_add(TABLE_VALUE);
            let base_value = (table_address as *const u32).read_unaligned();
            property.wrapping_mul(scale).wrapping_add(base_value)
        };

        let setter: unsafe extern "thiscall" fn(u32, f32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(1));
        setter(target, value)
    }
});
