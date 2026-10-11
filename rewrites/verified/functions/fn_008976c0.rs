// original: 0x008976C0 aud_environment_sound_get_parameter_by_index

/// Return one of the nine parameter words at offset `+0x64`, or zero when the
/// unsigned index is outside the table. The original is thiscall and removes
/// its single 32-bit index argument before returning.
export!(thiscall, rw_008976c0(audio: u32, entry_index: u32) -> u32 {
    unsafe {
        const PARAMETER_TABLE: u32 = 0x64;
        const PARAMETER_COUNT: u32 = 9;

        if entry_index >= PARAMETER_COUNT {
            0
        } else {
            ((audio + PARAMETER_TABLE + entry_index * 4) as *const u32).read_unaligned()
        }
    }
});
