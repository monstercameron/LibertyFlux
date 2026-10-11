// original: 0x005FE080 streaming_find_item_matches

/// Find the final item index matching each of two scripted helper queries.
///
/// The receiver has a byte item count at `+0x1f2`, an item-pointer array at
/// `+0xd0`, and two byte offsets at `+0xfc` and `+0x100`. For every item, the
/// helper is called with the item's payload pointer at `+0x11c` and the first
/// query. A zero result records that index and skips the second query for that
/// item; otherwise the helper receives the second query and a zero result
/// records that index. If both sentinels were replaced, the two indices are
/// written into the caller's buffer at the receiver's byte offsets. The
/// function uses thiscall and takes a buffer pointer and two query words.
lf_checker_rt::export!(thiscall, rw_005fe080(
    this: u32,
    output_buffer: u32,
    first_query: u32,
    second_query: u32,
) -> () {
    unsafe {
        const ITEM_COUNT: u32 = 0x1f2;
        const ITEM_POINTERS: u32 = 0xd0;
        const FIRST_OUTPUT_OFFSET: u32 = 0xfc;
        const SECOND_OUTPUT_OFFSET: u32 = 0x100;
        const PAYLOAD_OFFSET: u32 = 0x11c;
        const MATCHER_ID: u32 = 1;

        let count = (this.wrapping_add(ITEM_COUNT) as *const u8).read() as u32;
        let items = (this.wrapping_add(ITEM_POINTERS) as *const u32).read_unaligned();
        let mut first_match = u32::MAX;
        let mut second_match = u32::MAX;

        for item_index in 0..count {
            let item = (items.wrapping_add(item_index.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            let payload = item.wrapping_add(PAYLOAD_OFFSET);
            let first_result =
                lf_checker_rt::callee_cdecl!(MATCHER_ID, u32, payload, first_query);
            if first_result == 0 {
                first_match = item_index;
                continue;
            }

            let second_result =
                lf_checker_rt::callee_cdecl!(MATCHER_ID, u32, payload, second_query);
            if second_result == 0 {
                second_match = item_index;
            }
        }

        if first_match != u32::MAX && second_match != u32::MAX {
            let first_offset =
                (this.wrapping_add(FIRST_OUTPUT_OFFSET) as *const u32).read_unaligned();
            let second_offset =
                (this.wrapping_add(SECOND_OUTPUT_OFFSET) as *const u32).read_unaligned();
            (output_buffer.wrapping_add(first_offset) as *mut u8).write(first_match as u8);
            (output_buffer.wrapping_add(second_offset) as *mut u8).write(second_match as u8);
        }
    }
});
