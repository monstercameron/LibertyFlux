// original: 0x00a35280 vehicle_banks_rebuild (proposed)

/// Rebuild every live bank of the two bank tables.
///
/// Tables start at `TABLE`, are `0x1090` apart and run to `END`. A table
/// whose head is -1 is cleared (`[0] = [1] = 0`); a zero head is skipped.
/// Any other head is vetted first: through the tag checker (id 1, cdecl/1)
/// when `bl != 0` (answer ignored), or through the dispatch checker
/// (id 2, cdecl/1) when `bl == 0` (a zero answer skips the table).
/// A vetted table is cleared and handed to the rebuilder (id 3, cdecl/1).
/// Cdecl/1 (low byte), no defined return.
lf_checker_rt::export!(cdecl, rw_00a35280(bl: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x012D_FDA0;
        const END: u32 = 0x012E_1EC0;
        const STRIDE: u32 = 0x1090;
        const DEAD: u32 = 0xFFFF_FFFF;
        const TAG_CHECK: u32 = 1;
        const DISPATCH_CHECK: u32 = 2;
        const REBUILD: u32 = 3;
        let mut t = TABLE;
        while t < END {
            let head = core::ptr::read(lf_checker_rt::global::<u32>(t));
            if head == DEAD {
                core::ptr::write(lf_checker_rt::global::<u32>(t + 4), 0);
                core::ptr::write(lf_checker_rt::global::<u32>(t), 0);
            } else if head != 0 {
                let slot = lf_checker_rt::relocated(t);
                let vetted = if (bl & 0xFF) != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(TAG_CHECK, u32, slot);
                    true
                } else {
                    // The original tests only the low byte of the answer.
                    lf_checker_rt::callee_cdecl!(DISPATCH_CHECK, u32, slot) & 0xFF != 0
                };
                if vetted {
                    core::ptr::write(lf_checker_rt::global::<u32>(t + 4), 0);
                    core::ptr::write(lf_checker_rt::global::<u32>(t), 0);
                    let _: u32 = lf_checker_rt::callee_cdecl!(REBUILD, u32, slot);
                }
            }
            t = t.wrapping_add(STRIDE);
        }
        0
    }
});
