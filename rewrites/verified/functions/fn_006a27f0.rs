// original: 0x006A27F0 reserve_stream_table_slot
/// Reserves one eight-byte stream-table slot and returns its address.
///
/// The function reads a base pointer and two adjacent 16-bit values from
/// global state: the current entry count and allocated capacity. When the
/// values are equal, it grows capacity by sixteen, asks the allocator for a
/// new table, copies the existing entries, releases the old table through a
/// callback in the current thread's context, and installs the new base. It
/// then increments the count with 16-bit wrapping and returns the address of
/// the slot at the old count. The function uses stdcall with one ignored
/// stack word. The proof covers count/capacity pairs for zero to three entries,
/// including both the growth and no-growth paths, and a successful allocator.
const STREAM_TABLE_BASE: u32 = 0x019F_2488;
const STREAM_ENTRY_COUNT: u32 = 0x019F_248C;
const STREAM_CAPACITY: u32 = 0x019F_248E;
const STREAM_ENTRY_BYTES: u32 = 8;
const STREAM_CAPACITY_GROWTH: u16 = 16;
const TLS_CONTEXT_SLOT: usize = 0;
const TLS_CONTEXT_OFFSET_WORDS: usize = 2;
const VTABLE_RELEASE_SLOT_WORDS: usize = 3;

lf_checker_rt::export!(stdcall, rw_006a27f0(_reserved: u32) -> u32 {
    let table_base_slot = lf_checker_rt::global::<u32>(STREAM_TABLE_BASE);
    let count_slot = lf_checker_rt::global::<u16>(STREAM_ENTRY_COUNT);
    let capacity_slot = lf_checker_rt::global::<u16>(STREAM_CAPACITY);
    let old_table = unsafe { core::ptr::read_volatile(table_base_slot) };
    let count = unsafe { core::ptr::read_volatile(count_slot) };
    let capacity = unsafe { core::ptr::read_volatile(capacity_slot) };

    if count == capacity {
        let grown_capacity = capacity.wrapping_add(STREAM_CAPACITY_GROWTH);
        unsafe { core::ptr::write_volatile(capacity_slot, grown_capacity) };
        let new_table = lf_checker_rt::callee_stdcall!(1, u32, u32::from(grown_capacity));

        let mut index = 0u16;
        while index < count {
            let byte_offset = u32::from(index) * STREAM_ENTRY_BYTES;
            let source = old_table.wrapping_add(byte_offset) as *const u32;
            let destination = new_table.wrapping_add(byte_offset) as *mut u32;
            let first_word = unsafe { core::ptr::read_volatile(source) };
            let second_word = unsafe { core::ptr::read_volatile(source.add(1)) };
            unsafe {
                core::ptr::write_volatile(destination, first_word);
                core::ptr::write_volatile(destination.add(1), second_word);
            }
            index = index.wrapping_add(1);
        }

        if old_table != 0 {
            let thread_context = lf_checker_rt::tls_slot(TLS_CONTEXT_SLOT);
            let callback_owner = unsafe {
                core::ptr::read_volatile(
                    (thread_context as *const u32).add(TLS_CONTEXT_OFFSET_WORDS),
                )
            };
            let callback_vtable = unsafe {
                core::ptr::read_volatile(callback_owner as *const u32)
            };
            let release_callback = unsafe {
                core::ptr::read_volatile(
                    (callback_vtable as *const u32).add(VTABLE_RELEASE_SLOT_WORDS),
                )
            };
            let release: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                core::mem::transmute(release_callback as usize)
            };
            let _ = unsafe { release(callback_owner, old_table) };
        }

        unsafe { core::ptr::write_volatile(table_base_slot, new_table) };
    }

    unsafe {
        core::ptr::write_volatile(count_slot, count.wrapping_add(1));
    }
    let current_table = unsafe { core::ptr::read_volatile(table_base_slot) };
    current_table.wrapping_add(u32::from(count) * STREAM_ENTRY_BYTES)
});
