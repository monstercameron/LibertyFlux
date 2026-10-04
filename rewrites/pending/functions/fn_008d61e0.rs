// original: 0x008d61e0 handle_matches_selected
/// Compares the +0xc handle of the dereferenced first pointer against the
/// selected id of the second object: its +0xb30 field when the +0x26c flags
/// have bit 2 set, otherwise zero. Returns 1 on equality, else 0.
export!(cdecl, rw_008d61e0(p1: u32, p2: u32) -> u32 {
    unsafe {
        let obj = *(p1 as *const u32);
        let want = *((obj + 0xc) as *const u32);
        let got = if *((p2 + 0x26c) as *const u8) & 4 != 0 {
            *((p2 + 0xb30) as *const u32)
        } else {
            0
        };
        (got == want) as u32
    }
});
