// original: 0x009DDEE0 dword_sign_compare (proposed)

/// Compare the words at two pointers: -1 when the first is smaller, 0 when
/// equal, 1 when greater (signed comparison).
///
/// Used as the ordering callback by the registry lookup routines.
///
/// Original: 0x009DDEE0 (cdecl, two stack arguments, no outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DDEE0(a: u32, b: u32) -> u32 {
    unsafe {
        let x = (a as *const u32).read_unaligned() as i32;
        let y = (b as *const u32).read_unaligned() as i32;
        if x < y {
            0xFFFF_FFFF
        } else {
            u32::from(y < x)
        }
    }
});
