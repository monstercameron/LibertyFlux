// original: 0x00b41a90 forward_box_query_plus16
/// Forward the second argument plus 16 to the box query.
export!(stdcall, rw_b41a90(a: u32, b: u32) -> u32 {
    let _ = a;
    callee_cdecl!(2, u32, b.wrapping_add(0x10))
});
