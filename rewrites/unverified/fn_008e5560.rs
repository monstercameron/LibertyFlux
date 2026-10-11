// original: 0x008E5560 audio_entity_is_idle

/// Report whether the entity's two audio-state bytes are jointly clear.
///
/// The byte at `0x4B0` is the primary state. A clear primary byte returns one
/// immediately; otherwise the byte at `0x4B1` must also be zero. The method
/// therefore returns `primary == 0 || secondary == 0` in AL. It is a
/// 32-bit thiscall predicate with no stack arguments.
lf_checker_rt::export!(thiscall, rw_008e5560(this: u32) -> u32 {
    unsafe {
        const PRIMARY_STATE: usize = 0x4b0;
        const SECONDARY_STATE: usize = 0x4b1;
        let primary = (this as usize as *const u8).add(PRIMARY_STATE).read();
        if primary == 0 {
            return 1;
        }
        let secondary = (this as usize as *const u8).add(SECONDARY_STATE).read();
        (secondary == 0) as u32
    }
});
