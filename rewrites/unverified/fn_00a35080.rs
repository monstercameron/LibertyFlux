// original: 0x00a35080 vehicle_handle_lookup (proposed)

/// Linear search of a 90-entry handle table for one value.
///
/// Entries are dwords spaced `0x2c` apart from `TABLE` to `END`; answers 1
/// on the first entry equal to `needle`, else 0. Cdecl/1, returns AL.
lf_checker_rt::export!(cdecl, rw_00a35080(needle: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x012D_E150;
        const END: u32 = 0x012D_F0C8;
        const STRIDE: u32 = 0x2C;
        let mut p = TABLE;
        while p < END {
            if core::ptr::read(lf_checker_rt::global::<u32>(p)) == needle {
                return 1;
            }
            p = p.wrapping_add(STRIDE);
        }
        0
    }
});
