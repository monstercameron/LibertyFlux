// original: 0x008C6D70 cmp_field4_3way
/// Compare two records by the unsigned dword at offset 4, three ways.
///
/// Returns 0 when the keys are equal, 1 when the second record's key is
/// smaller, and -1 (as u32) when it is larger. Used as an ordering callback.
/// Reads only, makes no calls. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c6d70(left: u32, right: u32) -> u32 {
    unsafe {
        const KEY: u32 = 4;
        let a = ((left + KEY) as *const u32).read_unaligned();
        let b = ((right + KEY) as *const u32).read_unaligned();
        if a == b {
            0
        } else if b < a {
            1
        } else {
            0xFFFF_FFFF
        }
    }
});
