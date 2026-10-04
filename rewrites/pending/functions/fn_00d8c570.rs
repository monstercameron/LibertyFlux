// original: 0x00d8c570 audio_entity_active_check
/// Activity gate for an audio entity record.
///
/// Requires playback mode (1 or 4). A current-id override of -2 matches
/// anything. Otherwise the entity's id word at 0x2E must equal the current
/// id or one of three alternates; failing that, an override of -3 accepts
/// entities whose flag dword at 0x1304 equals 1.
export!(cdecl, rw_00d8c570(ent: *const u8) -> u32 {
    unsafe {
        let mode = *global::<u32>(0x0179_BF98);
        if mode != 1 && mode != 4 {
            return 0;
        }
        let current = *global::<i32>(0x0179_BFA0);
        if current == -2 {
            return 1;
        }
        let id = *(ent.add(0x2E) as *const i16) as i32;
        if id == current
            || id == *global::<i32>(0x0179_BFA4)
            || id == *global::<i32>(0x0179_BFA8)
            || id == *global::<i32>(0x0179_BFAC)
        {
            return 1;
        }
        if current == -3 && *(ent.add(0x1304) as *const u32) == 1 {
            return 1;
        }
        0
    }
});
