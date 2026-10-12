// original: 0x00C84CB0 scrollbar_tables_reset

/// Clear the two global scrollbar-table counters at file virtual addresses
/// `0x016FAC40` and `0x016FAC44`. The routine has no parameters or meaningful
/// return value and writes each counter independently.
lf_checker_rt::export!(cdecl, rw_00c84cb0() -> u32 {
    const SCROLLBAR_COUNT_GLOBAL: u32 = 0x016F_AC40;
    const ENTITY_COUNT_GLOBAL: u32 = 0x016F_AC44;

    unsafe {
        lf_checker_rt::global::<u32>(SCROLLBAR_COUNT_GLOBAL).write_unaligned(0);
        lf_checker_rt::global::<u32>(ENTITY_COUNT_GLOBAL).write_unaligned(0);
    }
    0
});
