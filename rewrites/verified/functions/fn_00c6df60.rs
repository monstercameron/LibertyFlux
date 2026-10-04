// original: 0x00c6df60 heap_sort_range
/// Heap-sort entry: forwards to partial-sort with a zero fourth argument.
export!(cdecl, rw_00c6df60(a0: u32, a1: u32, a2: u32, a3: u32) -> () {
    callee_cdecl!(1, u32, a0, a1, a2, 0, a3);
});
