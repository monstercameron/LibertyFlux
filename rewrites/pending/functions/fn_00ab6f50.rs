// original: 0x00ab6f50 id_or_null_equal
/// Returns 1 when the two ids are equal or either is null, else 0.

export!(cdecl, rw_00ab6f50(a: u32, b: u32) -> u32 {
    if a == b || a == 0 || b == 0 { 1 } else { 0 }
});
