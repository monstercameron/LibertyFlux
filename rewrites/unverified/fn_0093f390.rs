// original: 0x0093f390 stream_slot_find_by_id (proposed)

/// Find the first streaming slot whose object id equals `key`.
///
/// Scans the 32-word slot-pointer table at `SLOT_TABLE` from index 0.
/// Null entries are skipped; for a live entry the id word at offset
/// `OBJ_ID` is compared against `key`. Returns the index of the first
/// match, or `u32::MAX` when no entry matches.
///
/// Original: 0x0093f390 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_0093f390(key: u32) -> u32 {
    const SLOT_TABLE: u32 = 0x11A8808;
    const SLOT_COUNT: u32 = 32;
    const OBJ_ID: u32 = 0x598;
    unsafe {
        let table = lf_checker_rt::global::<u32>(SLOT_TABLE) as *const u32;
        let mut i: u32 = 0;
        while i < SLOT_COUNT {
            let slot = table.add(i as usize).read_unaligned();
            if slot != 0 {
                let id = ((slot + OBJ_ID) as *const u32).read_unaligned();
                if id == key {
                    return i;
                }
            }
            i += 1;
        }
        u32::MAX
    }
});
