// original: 0x00c6df40 make_heap_entry
/// Make-heap over `[first, last)`, forwarding two trailing zeroes to the
/// worker (its extra slots).
export!(cdecl, rw_00c6df40(first: u32, last: u32, extra: u32) -> () {
    callee_cdecl!(1, u32, first, last, extra, 0, 0);
});
