// original: 0x00b41a80 forward_box_query
/// Forward one argument to the box query.
export!(stdcall, rw_b41a80(a: u32) -> u32 {
    callee_cdecl!(2, u32, a)
});
