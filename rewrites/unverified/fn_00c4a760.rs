// original: 0x00C4A760 timing_node_remove_from_owner

/// Remove the current timing node, clear its state, and copy a four-word
/// record into the receiver.
///
/// The receiver is in ECX and the stack argument points to a non-null record.
/// A non-null node pointer at receiver offset 0x168 is detached through the
/// timing helper. A nonzero status dword at 0x164 is cleared. The routine
/// clears bytes 0x181 and 0x183, writes whether the record pointer is non-null
/// to byte 0x182, and copies four words from the record to offsets 0x190,
/// 0x194, 0x198, and 0x19c. The fourth copied word remains in EAX as the routine returns; the two
/// float words are copied bit-for-bit without arithmetic.
lf_checker_rt::export!(thiscall, rw_c4a760(this: u32, record: u32) -> u32 {
    const STATUS_FIELD: u32 = 0x164;
    const NODE_POINTER_FIELD: u32 = 0x168;
    const CLEAR_FLAG_A: u32 = 0x181;
    const RECORD_PRESENT_FLAG: u32 = 0x182;
    const CLEAR_FLAG_B: u32 = 0x183;
    const OUTPUT_WORDS: [u32; 4] = [0x190, 0x194, 0x198, 0x19c];
    const DETACH_HELPER: u32 = 1;

    let slot = this.wrapping_add(NODE_POINTER_FIELD);
    let old_node = unsafe { (slot as *const u32).read_unaligned() };
    if old_node != 0 {
        let _ = lf_checker_rt::callee_thiscall!(DETACH_HELPER, u32, old_node, slot);
    }

    let status = unsafe { (this.wrapping_add(STATUS_FIELD) as *const u32).read_unaligned() };
    if status != 0 {
        unsafe { (this.wrapping_add(STATUS_FIELD) as *mut u32).write_unaligned(0) };
    }

    unsafe {
        (this.wrapping_add(CLEAR_FLAG_A) as *mut u8).write_unaligned(0);
        (this.wrapping_add(CLEAR_FLAG_B) as *mut u8).write_unaligned(0);
        (this.wrapping_add(RECORD_PRESENT_FLAG) as *mut u8)
            .write_unaligned(u8::from(record != 0));
    }

    let mut return_word = 0;
    for (index, output_offset) in OUTPUT_WORDS.into_iter().enumerate() {
        let value = unsafe {
            (record.wrapping_add((index as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned()
        };
        if index == 3 {
            return_word = value;
        }
        unsafe {
            (this.wrapping_add(output_offset) as *mut u32).write_unaligned(value);
        }
    }
    return_word
});
