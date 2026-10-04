// original: 0x009535d0 band_index_or_zero
/// Map a band code to its slot index, or 0 when the code is unknown.
export!(cdecl, rw_009535d0(code: u32) -> u32 {
    match code {
        0x19 => 3,
        0x32 => 2,
        0x4B => 1,
        0x64 => 0,
        0x7D => 5,
        0x96 => 6,
        0xAF => 7,
        0xC8 => 8,
        0xE1 => 4,
        _ => 0,
    }
});
