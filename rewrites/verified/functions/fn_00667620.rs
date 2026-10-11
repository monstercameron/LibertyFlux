// original: 0x00667620 streaming_assign_indices_and_notify

/// For each item in the receiver's pointer array, notify the thread's event
/// object, write the item's zero-based position at byte offset `0x1c`, then
/// call the item's virtual notification slot. The item count is an unsigned
/// 16-bit field at receiver offset `0x14`; the array pointer is at `0x10`.
/// Both virtual calls are intercepted by the checker. The function returns
/// the item count and does no TLS access or calls when that count is zero.
const STREAMING_ITEM_COUNT_OFFSET: u32 = 0x14;
const STREAMING_ITEM_ARRAY_OFFSET: u32 = 0x10;
const STREAMING_ITEM_INDEX_OFFSET: u32 = 0x1c;
const STREAMING_ITEM_POINTER_SIZE: u32 = 4;
const STREAMING_EVENT_CALL_ID: u32 = 1;
const STREAMING_ITEM_CALL_ID: u32 = 2;

#[inline(always)]
unsafe fn streaming_read_u16(address: u32) -> u32 {
    unsafe { (address as *const u16).read_unaligned() as u32 }
}

#[inline(always)]
unsafe fn streaming_read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn streaming_write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

lf_checker_rt::export!(thiscall, rw_00667620(receiver: u32) -> u32 {
    unsafe {
        let item_count = streaming_read_u16(receiver.wrapping_add(STREAMING_ITEM_COUNT_OFFSET));
        for index in 0..item_count {
            let thread_state = lf_checker_rt::tls_slot(0);
            let event_object = streaming_read_u32(thread_state.wrapping_add(8));
            let _ = lf_checker_rt::callee_thiscall!(STREAMING_EVENT_CALL_ID, u32, event_object);

            let item_array = streaming_read_u32(receiver.wrapping_add(STREAMING_ITEM_ARRAY_OFFSET));
            let item = streaming_read_u32(
                item_array.wrapping_add(index.wrapping_mul(STREAMING_ITEM_POINTER_SIZE)),
            );
            streaming_write_u32(item.wrapping_add(STREAMING_ITEM_INDEX_OFFSET), index);
            let _ = lf_checker_rt::callee_thiscall!(STREAMING_ITEM_CALL_ID, u32, item);
        }
        item_count
    }
});
