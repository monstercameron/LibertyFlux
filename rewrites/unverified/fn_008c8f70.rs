// original: 0x008C8F70 ptrbox_free
/// Release the buffer owned by a one-word handle box, then clear the box.
///
/// If the pointer at `box_` is non-null it is handed to the freeing callee
/// and the box is cleared. Returns nothing. Original: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_008c8f70(box_: u32) -> u32 {
    unsafe {
        const FREE_CALLEE: u32 = 1;
        let slot = box_ as *mut u32;
        let owned = slot.read_unaligned();
        if owned != 0 {
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, owned);
            slot.write_unaligned(0);
        }
        0
    }
});
