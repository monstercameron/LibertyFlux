// original: 0x006A4DA0 rebase_thread_local_pointer_ranges

/// Optionally rebases the pointer-array base with the first helper call, then
/// checks each non-null pointer slot against the thread-local range table.
/// A matching slot is rebased by the same helper and passed to a second helper
/// with the thread-local state pointer. Unmatched or inactive entries are set
/// to null. The object has a pointer at offset 0 and a 16-bit count at offset 4;
/// thiscall takes two stack words and returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_006a4da0(
    this: u32,
    rebase_argument: u32,
    _unused_argument: u32,
) -> u32 {
    const POINTER_ARRAY: u32 = 0x00;
    const ENTRY_COUNT: u32 = 0x04;
    const POINTER_BYTES: u32 = 4;
    const TLS_SLOT: usize = 0;
    const REBASE_ID: u32 = 1;
    const APPLY_ID: u32 = 2;

    let read_u16 = |address: u32| unsafe { (address as *const u16).read_unaligned() };
    let read_u32 = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let write_u32 = |address: u32, value: u32| unsafe { (address as *mut u32).write_unaligned(value) };

    let mut array = read_u32(this.wrapping_add(POINTER_ARRAY));
    if array != 0 {
        let adjustment = lf_checker_rt::callee_thiscall!(REBASE_ID, u32, rebase_argument, array);
        array = array.wrapping_add(adjustment);
        write_u32(this.wrapping_add(POINTER_ARRAY), array);
    }

    let tls_array = lf_checker_rt::tls_slot(TLS_SLOT);
    let thread_state = read_u32(tls_array.wrapping_add(4));
    let range_header = if thread_state == 0 { 0 } else { read_u32(thread_state) };
    let descriptor_count = if range_header == 0 {
        0
    } else {
        read_u16(range_header) as u32 + read_u16(range_header.wrapping_add(2)) as u32
    };
    let entry_count = read_u16(this.wrapping_add(ENTRY_COUNT)) as u32;
    let mut entry_index = 0u32;

    while entry_index < entry_count {
        let slot = array.wrapping_add(entry_index.wrapping_mul(POINTER_BYTES));
        let mut current_pointer = read_u32(slot);
        if thread_state == 0 || current_pointer == 0 || descriptor_count == 0 {
            write_u32(slot, 0);
        } else {
            let mut descriptor_index = 0u32;
            let mut matched = false;
            while descriptor_index < descriptor_count {
                let descriptor = range_header
                    .wrapping_add(8)
                    .wrapping_add(descriptor_index.wrapping_mul(12));
                let range_start = read_u32(descriptor);
                let range_end = range_start.wrapping_add(read_u32(descriptor.wrapping_add(4)));
                if slot >= range_start && slot < range_end {
                    matched = true;
                    break;
                }
                descriptor_index = descriptor_index.wrapping_add(1);
            }
            if matched {
                let adjustment = lf_checker_rt::callee_thiscall!(REBASE_ID, u32, thread_state, current_pointer);
                current_pointer = current_pointer.wrapping_add(adjustment);
                write_u32(slot, current_pointer);
                if current_pointer != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(APPLY_ID, u32, current_pointer, thread_state);
                }
            } else {
                write_u32(slot, 0);
            }
        }
        entry_index = entry_index.wrapping_add(1);
    }

    this
});
