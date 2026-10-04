// original: 0x00c6df80 pop_heap_last
/// Move the last element of `[first, last)` to the front and re-heapify the
/// shortened range through the adjust helper.
export!(cdecl, rw_00c6df80(first: u32, last: u32, extra: u32) -> () {
    unsafe {
        let k = *(first as *const u32);
        let v = *((first + 4) as *const u32);
        let lk = *(last.wrapping_sub(8) as *const u32);
        let lv = *(last.wrapping_sub(4) as *const u32);
        *(last.wrapping_sub(8) as *mut u32) = k;
        *(last.wrapping_sub(4) as *mut u32) = v;
        let count = ((last.wrapping_sub(first).wrapping_sub(8) as i32) >> 3) as u32;
        callee_cdecl!(1, u32, first, 0, count, lk, lv, extra);
    }
});
