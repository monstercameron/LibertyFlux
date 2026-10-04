// original: 0x00952660 key_table_lookup
/// Look up a key in a table of 0x1000 eight-byte entries (`key`, `value`
/// pairs). Entries whose key is all-ones are empty and skipped. On a match
/// the entry's value is returned. On a miss a helper is asked for a stamp:
/// when the stamp differs from the expected word the stored base is returned,
/// otherwise the base plus one.
export!(cdecl, rw_00952660(key: u32) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        while i < 0x1000 {
            let k = *global::<u32>(0x012008b0 + i * 8);
            if k != 0xffff_ffff && k == key {
                return *global::<u32>(0x012008b4 + i * 8);
            }
            i += 1;
        }
        let stamp: u32 = callee_cdecl!(1, u32,);
        let expected = *global::<u32>(0x011f702c);
        let base = *global::<u32>(0x011f70c4);
        if stamp == expected {
            base.wrapping_add(1)
        } else {
            base
        }
    }
});
