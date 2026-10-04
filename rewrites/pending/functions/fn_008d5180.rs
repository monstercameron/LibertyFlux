// original: 0x008D5180 CodeForSelectorValue
/// Maps a selector value to a fixed code: 5 yields the default code, 6, 7
/// and 8 yield their own codes, and every other input yields the default.
export!(cdecl, rw_008D5180(selector: u32) -> u32 {
    match selector.wrapping_sub(5) {
        0 => 0x1e,
        1 => 0x20,
        2 => 0x1f,
        3 => 0x21,
        _ => 0x1e,
    }
});
