// original: 0x00e6cd80 veh_table_fill_80 (proposed)

/// Fill a fixed vehicle table with -1: `1820 dwords at BASE = 0xFFFFFFFF`.
///
/// The original sets EAX to -1 and runs `rep stosd` over the table. Takes no
/// arguments and returns nothing meaningful; the table contents are the whole
/// behaviour.
///
/// Original: 0x00e6cd80 (cdecl, no arguments, no return channel).
lf_checker_rt::export!(cdecl, rw_00e6cd80() -> u32 {
    unsafe {
        const BASE: u32 = 0x0178e7c8;
        const COUNT: u32 = 0x0000071c;
        let table = core::slice::from_raw_parts_mut(
            lf_checker_rt::global::<u32>(BASE), COUNT as usize);
        table.fill(0xFFFF_FFFF);
        0
    }
});
