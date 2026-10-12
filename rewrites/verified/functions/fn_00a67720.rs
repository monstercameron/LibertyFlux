// original: 0x00A67720 teardown_all_active_explosions

/// Scan the sixteen active explosion slots and pass each occupied slot's index
/// to the intercepted teardown helper. The slot table is read from the mapped
/// image through the runtime's relocation helper; the function is cdecl with
/// no arguments or meaningful return value.
lf_checker_rt::export!(cdecl, rw_00a67720() -> u32 {
    const ACTIVE_SLOTS_FILE_VA: u32 = 0x012F8380;
    const SLOT_COUNT: u32 = 16;
    const TEARDOWN_CALLEE: u32 = 1;

    unsafe {
        let active_slots = lf_checker_rt::global::<u32>(ACTIVE_SLOTS_FILE_VA);
        for slot in 0..SLOT_COUNT {
            if active_slots.add(slot as usize).read_unaligned() != 0 {
                let _ignored_result = lf_checker_rt::callee_cdecl!(
                    TEARDOWN_CALLEE,
                    u32,
                    slot,
                );
            }
        }
        0
    }
});
