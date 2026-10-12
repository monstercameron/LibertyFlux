// original: 0x00A2FFE0 CEntity::vf7

/// Records an entity and attaches its pool link to the current interior.
///
/// The thiscall entry receives the entity in ECX. It stores the entity in a
/// global tracking word, then reads the signed handle at byte offset 0x48.
/// Handles at or below signed -1 return unchanged. Other handles are looked
/// up through the current pool global. A null lookup returns zero; a live
/// entry receives the entity plus its sign-extended signed byte at 0x40 and
/// signed word at 0x44. The last helper's return value is preserved in EAX.
lf_checker_rt::export!(thiscall, rw_00a2ffe0(entity: u32) -> u32 {
    const TRACKED_ENTITY: u32 = 0x012D_DF10;
    const CURRENT_POOL: u32 = 0x012F_B214;
    const HANDLE_WORD: usize = 18;
    const INTERIOR_BYTE_OFFSET: usize = 0x40;
    const INTERIOR_WORD_OFFSET: usize = 0x44;
    const POOL_LOOKUP: u32 = 1;
    const TRACK_ENTITY: u32 = 2;

    unsafe {
        lf_checker_rt::global::<u32>(TRACKED_ENTITY)
            .write_unaligned(entity);
        let entity_bytes = entity as *const u8;
        let handle = (entity as *const u32)
            .add(HANDLE_WORD)
            .read_unaligned();
        if (handle as i32) <= -1 {
            return handle;
        }

        let pool = lf_checker_rt::global::<u32>(CURRENT_POOL).read_unaligned();
        let pool_entry =
            lf_checker_rt::callee_thiscall!(POOL_LOOKUP, u32, pool, handle);
        if pool_entry == 0 {
            return 0;
        }

        let interior_byte = entity_bytes
            .add(INTERIOR_BYTE_OFFSET)
            .cast::<i8>()
            .read_unaligned() as i32 as u32;
        let interior_word = entity_bytes
            .add(INTERIOR_WORD_OFFSET)
            .cast::<i16>()
            .read_unaligned() as i32 as u32;
        lf_checker_rt::callee_thiscall!(
            TRACK_ENTITY,
            u32,
            pool_entry,
            entity,
            interior_byte,
            interior_word
        )
    }
});
