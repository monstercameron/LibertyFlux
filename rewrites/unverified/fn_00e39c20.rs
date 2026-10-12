// original: 0x00E39C20 audio_set_entity_voice_channel_value

/// Forward this entity's voice-channel value to the input store. The channel
/// byte at `this + 4` equal to `0xff` sends zero; otherwise the value is the
/// word at `table + entity_index * ENTITY_STRIDE + channel * channel_scale +
/// VALUE_OFFSET`, plus `channel * channel_scale`. The table base and scale are
/// loaded from the image globals for every call. The input store is reached
/// through a thiscall tail transfer and its return value is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00e39c20(this: u32) -> u32 {
    unsafe {
        const CHANNEL_OFFSET: u32 = 0x04;
        const ENTITY_INDEX_OFFSET: u32 = 0x40;
        const CHANNEL_SCALE_GLOBAL: u32 = 0x0115_D968;
        const TABLE_BASE_GLOBAL: u32 = 0x0115_D988;
        const ENTITY_STRIDE: u32 = 0x6F40;
        const VALUE_OFFSET: u32 = 0x6F14;
        const EMPTY_CHANNEL: u8 = 0xFF;

        let channel = (this.wrapping_add(CHANNEL_OFFSET) as *const u8).read();
        let value = if channel == EMPTY_CHANNEL {
            0
        } else {
            let entity_index = (this.wrapping_add(ENTITY_INDEX_OFFSET) as *const u8).read() as u32;
            let scale = lf_checker_rt::global::<u32>(CHANNEL_SCALE_GLOBAL).read_unaligned();
            let table = lf_checker_rt::global::<u32>(TABLE_BASE_GLOBAL).read_unaligned();
            let channel_offset = (channel as u32).wrapping_mul(scale);
            let value_address = table
                .wrapping_add(entity_index.wrapping_mul(ENTITY_STRIDE))
                .wrapping_add(VALUE_OFFSET)
                .wrapping_add(channel_offset);
            channel_offset.wrapping_add((value_address as *const u32).read_unaligned())
        };

        lf_checker_rt::callee_thiscall!(1, u32, value)
    }
});
