// original: 0x00D92F00 mainloop_timing_cleanup_twenty_two_byte_items
lf_checker_rt::export!(thiscall, rw_00d92f00(array_owner: u32, count: u32) -> u32 {
    const ELEMENT_STRIDE: u32 = 0x16;
    let range = unsafe { request_timing_array_item(array_owner, count, ELEMENT_STRIDE) };
    unsafe { invoke_timing_item_helper(range, count, ELEMENT_STRIDE, 2) }
});
