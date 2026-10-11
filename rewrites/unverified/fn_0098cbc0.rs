// original: 0x0098CBC0 audVehicleAudioEntity::get_slot_by_handle

/// Reads the unsigned handle at `this + 0xA3C`. The invalid sentinel returns
/// zero; otherwise the handle selects the first word of a 16-byte table row.
/// The lookup table is game-owned data.
lf_checker_rt::export!(thiscall, rw_audvehicleaudioentity_get_slot_by_handle(this: u32) -> u32 {
    const HANDLE: u32 = 0xA3C;
    const SLOT_TABLE: u32 = 0x1283_2B8;
    const ROW_WORDS: usize = 4;

    unsafe {
        let handle = (this.wrapping_add(HANDLE) as *const u32).read_unaligned();
        if handle == u32::MAX {
            return 0;
        }
        lf_checker_rt::global::<u32>(SLOT_TABLE)
            .add(handle as usize * ROW_WORDS)
            .read_unaligned()
    }
});
