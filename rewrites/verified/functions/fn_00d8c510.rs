// original: 0x00d8c510 audio_entity_audible_check
/// Audibility gate for an audio entity record.
///
/// Requires playback mode (1 or 4). Unless the current-entity override is
/// -1 (match anything), the entity's id word at 0x2E must equal the current
/// id or one of three alternates. Two mute bytes at 0x218/0x219 must be
/// clear. Returns 1 when the entity may sound, else 0.
export!(cdecl, rw_00d8c510(ent: *const u8) -> u32 {
    unsafe {
        let mode = *global::<u32>(0x0179_BF98);
        if mode != 1 && mode != 4 {
            return 0;
        }
        let current = *global::<i32>(0x0179_BFA0);
        if current != -1 {
            let id = *(ent.add(0x2E) as *const i16) as i32;
            if id != current
                && id != *global::<i32>(0x0179_BFA4)
                && id != *global::<i32>(0x0179_BFA8)
                && id != *global::<i32>(0x0179_BFAC)
            {
                return 0;
            }
        }
        if *ent.add(0x218) != 0 {
            return 0;
        }
        if *ent.add(0x219) != 0 {
            return 0;
        }
        1
    }
});
