// original: 0x00d38c10 selector_code_map
/// Map a small selector to a code: 5 -> 0x1e, 6 -> 0x20, 7 -> 0x1f,
/// anything else -> 0x21.
export!(cdecl, rw_00d38c10(sel: u32) -> u32 {
    match sel {
        5 => 0x1e,
        6 => 0x20,
        7 => 0x1f,
        _ => 0x21,
    }
});
