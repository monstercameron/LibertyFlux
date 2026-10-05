// original: 0x00b8bb60 ADD_POINT_TO_GPS_RACE_TRACK
/// Script-native handler that appends one point, then hands off.
///
/// Takes one word, the script context pointer. Reads the argument block
/// through two pointer levels (the table word past the context header,
/// then the block the table points at) and takes the block's first word
/// plus the two words after it (the point's coordinates, copied as raw
/// bits). Appends to the context's own list: stores the block pointer at
/// the slot selected by the stored count, copies the three words to the
/// freshly grown record past the list, and bumps the stored count by one.
/// Then overwrites the incoming argument slot with the fresh record's
/// address and transfers to the shared implementation, returning whatever
/// that call answers. The slot overwrite turns the stack comparison off
/// for this contract; the forwarded address is compared via the call log.
export!(cdecl, rw_00b8bb60(ctx: u32) -> u32 {
    unsafe {
        let table = *(ctx.wrapping_add(8) as *const u32);
        let block = *(table as *const u32);
        let first = *(block as *const u32);
        let coord0 = *(block.wrapping_add(4) as *const u32);
        let coord1 = *(block.wrapping_add(8) as *const u32);
        let count = *(ctx.wrapping_add(12) as *const u32);
        *(ctx.wrapping_add(0x10).wrapping_add(count.wrapping_mul(4)) as *mut u32) = block;
        let record = ctx.wrapping_add(count.wrapping_add(2).wrapping_mul(16));
        *(record as *mut u32) = first;
        *(record.wrapping_add(4) as *mut u32) = coord0;
        *(record.wrapping_add(8) as *mut u32) = coord1;
        *(ctx.wrapping_add(12) as *mut u32) = count.wrapping_add(1);
        callee_cdecl!(1, u32, record)
    }
});
