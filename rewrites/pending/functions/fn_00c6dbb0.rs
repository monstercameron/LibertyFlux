// original: 0x00c6dbb0 final_insertion_sort
/// Finish a sort with insertion sort: ranges longer than 16 elements get a
/// guarded pass over the first 16 and an unguarded pass over the rest.
export!(cdecl, rw_00c6dbb0(first: u32, last: u32, extra: u32) -> () {
    let span = last.wrapping_sub(first) & 0xFFFF_FFF8;
    if (span as i32) > 0x80 {
        let mid = first.wrapping_add(0x80);
        callee_cdecl!(1, u32, first, mid, 0, extra);
        callee_cdecl!(2, u32, mid, last, 0, extra);
    } else {
        callee_cdecl!(1, u32, first, last, 0, extra);
    }
});
