// original: 0x008ef280 equal_32_bytes
/// Compare two 32-byte blocks for equality.
///
/// Returns 1 when all 32 bytes match, else 0, stopping at the first
/// difference.
export!(cdecl, rw_008ef280(a: u32, b: u32) -> u8 {
    unsafe {
        let x = a as *const u8;
        let y = b as *const u8;
        for i in 0..32 {
            if *x.add(i) != *y.add(i) {
                return 0;
            }
        }
        1
    }
});
