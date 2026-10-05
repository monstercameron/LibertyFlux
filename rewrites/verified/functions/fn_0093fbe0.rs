// original: 0x0093fbe0 Player_GetByIndex

/// Return the streaming-slot pointer at `index`, or null.
///
/// `index` must be below `SLOT_COUNT` (32); anything larger returns null
/// without touching the table. Otherwise returns word `index` of the
/// slot-pointer table at `SLOT_TABLE`, which may itself be null.
///
/// Original: 0x0093fbe0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_0093fbe0(index: u32) -> u32 {
    const SLOT_TABLE: u32 = 0x11A8808;
    const SLOT_COUNT: u32 = 32;
    if index >= SLOT_COUNT {
        return 0;
    }
    unsafe {
        let table = lf_checker_rt::global::<u32>(SLOT_TABLE) as *const u32;
        table.add(index as usize).read_unaligned()
    }
});
