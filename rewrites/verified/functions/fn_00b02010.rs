// original: 0x00b02010 heapify_defaults
/// Forward three arguments plus two zero words to the heapify routine;
/// return its result.
export!(cdecl, rw_00b02010(a: u32, b: u32, c: u32) -> u32 {
    callee_cdecl!(1, u32, a, b, c, 0, 0)
});
