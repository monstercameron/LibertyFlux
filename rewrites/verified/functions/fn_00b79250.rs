// original: 0x00b79250 index_to_float_0_0_1_2_3
/// Map a small index to a float constant.
///
/// Returns 1.0 for index 2, 2.0 for index 3, 3.0 for index 4, and 0.0
/// for any other index.
export!(cdecl, rw_00b79250(idx: u32) -> f32 {
    match idx {
        2 => 1.0,
        3 => 2.0,
        4 => 3.0,
        _ => 0.0,
    }
});
