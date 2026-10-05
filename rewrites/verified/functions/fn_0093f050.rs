// original: 0x0093f050 LocalPlayerPed

/// Return the id word of the currently selected streaming slot's object.
///
/// Reads the selected index from `SELECTED_INDEX`. An index of -1, a null
/// table entry, or (by falling through) a live entry's id word at offset
/// `OBJ_ID`. Note the table index is not range-checked: only -1 is
/// excluded before the table read.
///
/// Original: 0x0093f050 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0093f050() -> u32 {
    const SELECTED_INDEX: u32 = 0x1036F14;
    const SLOT_TABLE: u32 = 0x11A8808;
    const OBJ_ID: u32 = 0x598;
    unsafe {
        let idx = (lf_checker_rt::global::<u32>(SELECTED_INDEX) as *const u32).read_unaligned();
        if idx == u32::MAX {
            return 0;
        }
        let entry = lf_checker_rt::relocated(SLOT_TABLE).wrapping_add(idx.wrapping_mul(4));
        let slot = (entry as *const u32).read_unaligned();
        if slot == 0 {
            return 0;
        }
        ((slot + OBJ_ID) as *const u32).read_unaligned()
    }
});
