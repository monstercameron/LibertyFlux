// original: 0x00a35030 vehicle_pair_lookup (proposed)

/// Search two tables for one value, one candidate word per table.
///
/// Tables start at `TABLE`, are `0x1090` apart and run to `END`. A table
/// whose head word is zero is skipped; otherwise the word at `+0x10` is
/// compared against `needle`. Answers 1 on a match, else 0.
///
/// Note the inner loop counter starts at 0 and exits when it reaches 1, so
/// it makes a single pass: only `+0x10` is ever compared. The `+0x610` step
/// to a second slot at `+0x620` executes but its result is never used.
/// Cdecl/1, returns AL.
lf_checker_rt::export!(cdecl, rw_00a35030(needle: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x012D_FDA0;
        const END: u32 = 0x012E_1EC0;
        const TSTRIDE: u32 = 0x1090;
        const ENTRY: u32 = 0x10;
        let mut t = TABLE;
        while t < END {
            if core::ptr::read(lf_checker_rt::global::<u32>(t)) != 0 {
                if core::ptr::read(lf_checker_rt::global::<u32>(t + ENTRY)) == needle {
                    return 1;
                }
            }
            t = t.wrapping_add(TSTRIDE);
        }
        0
    }
});
