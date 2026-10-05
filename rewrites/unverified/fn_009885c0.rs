// original: 0x009885C0 audCutscene_slots_clear_check (proposed)

/// Cutscene slot scan: reports whether no slot holds an active entry.
///
/// Reads the entity pointer from the global `ENTITY_SLOT`; a null pointer
/// means clear (returns 1). Otherwise scans the two slot pointers at
/// `+SLOT0_OFF` and `+SLOT0_OFF+4`: a slot counts as active when its pointer
/// is nonzero, its byte at `+STATE_OFF` is zero, and its dword at `+COUNT_OFF`
/// is above zero, compared UNSIGNED (only exactly zero is inactive; a value
/// with the top bit set is active). Returns 0 when any slot is active.
/// Original: stdcall, no stack words, return in `al`.
lf_checker_rt::export!(stdcall, rw_009885C0() -> u32 {
    const ENTITY_SLOT: u32 = 0x1282fb8;
    const SLOT0_OFF: u32 = 0x20;
    const STATE_OFF: u32 = 0x47;
    const COUNT_OFF: u32 = 8;
    unsafe {
        let entity = *lf_checker_rt::global::<u32>(ENTITY_SLOT);
        if entity == 0 {
            return 1;
        }
        for i in 0..2u32 {
            let slot = ((entity + SLOT0_OFF + i * 4) as *const u32).read_unaligned();
            if slot == 0 {
                continue;
            }
            let state = ((slot + STATE_OFF) as *const u8).read();
            if state != 0 {
                continue;
            }
            let count = ((slot + COUNT_OFF) as *const u32).read_unaligned();
            if count > 0 {
                return 0;
            }
        }
        1
    }
});
