// original: 0x00CF1930 StatGroup_GetList

/// Return the address of the selected statistics group's data array.
///
/// The cdecl argument is an unsigned group index. Values zero through seven
/// select one of eight image data arrays; any other index returns null. The
/// returned addresses are relocated from the original image so this rewrite
/// remains correct when the checker maps the image away from its preferred
/// base. The function does not read array contents.
lf_checker_rt::export!(cdecl, rw_fn_00cf1930(group_index: u32) -> u32 {
    let array_va = match group_index {
        0 => 0x0105_35e0,
        1 => 0x0105_2730,
        2 => 0x0105_27c8,
        3 => 0x0105_2ca8,
        4 => 0x0105_2d98,
        5 => 0x0105_2508,
        6 => 0x0105_28e8,
        7 => 0x0105_34b8,
        _ => return 0,
    };
    lf_checker_rt::relocated(array_va)
});
