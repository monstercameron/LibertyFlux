// original: 0x00953180 find_record
/// Search the 0x400 twenty-byte records for one that is live (first word
/// nonzero) with the wanted id at offset 8 and the wanted sum at offset 12,
/// and return its first word. Returns zero when no record matches.
export!(cdecl, rw_00953180(id: u32, a: u32, b: u32) -> u32 {
    unsafe {
        let want = a.wrapping_add(b);
        let mut i: u32 = 0;
        while i < 0x400 {
            let base = 0x0120f2b8 + i * 20;
            let first = *global::<u32>(base);
            if first != 0
                && *global::<u32>(base + 8) == id
                && *global::<u32>(base + 12) == want
            {
                return first;
            }
            i += 1;
        }
        0
    }
});
