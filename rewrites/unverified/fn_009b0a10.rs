// original: 0x009B0A10 audPoliceScanner::clear_slots_for_entity

/// Clears the active flag and entity identifier for every matching record in
/// the 32-entry table beginning at `this + 0x1f4`. Each entry is `0x1a0`
/// bytes; the identifier is at its start and the active byte is at `+0x0d`.
lf_checker_rt::export!(thiscall, rw_009b0a10(this: u32, entity_id: u32) -> u32 {
    const FIRST_ENTRY: u32 = 0x1f4;
    const ENTRY_STRIDE: u32 = 0x1a0;
    const ENTRY_COUNT: u32 = 32;
    const ACTIVE: u32 = 0x0d;

    unsafe {
        for index in 0..ENTRY_COUNT {
            let entry = this.wrapping_add(FIRST_ENTRY).wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let active = ((entry + ACTIVE) as *const u8).read();
            let stored_id = (entry as *const u32).read_unaligned();
            if active != 0 && stored_id == entity_id {
                ((entry + ACTIVE) as *mut u8).write(0);
                (entry as *mut u32).write_unaligned(0);
            }
        }
        this.wrapping_add(FIRST_ENTRY).wrapping_add(ENTRY_COUNT.wrapping_mul(ENTRY_STRIDE))
    }
});
