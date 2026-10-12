// original: 0x00A5A480 slot_list_append

/// Appends one 16-byte slot record while the signed count is below eight.
/// The record is copied as four words without changing any float bits. Its
/// destination uses the incremented count as a 16-byte index. After the copy,
/// the word at object offset zero is incremented, matching the original even
/// when a negative count makes the destination overlap that word. ECX holds
/// the object; one record pointer is passed on the stack. AL reports success.
lf_checker_rt::export!(thiscall, rw_00A5A480(this: u32, source_record: u32) -> u32 {
    const COUNT: u32 = 0x00;
    const SLOT_STRIDE: u32 = 0x10;
    const SLOT_LIMIT: i32 = 8;

    #[inline(always)]
    unsafe fn read_i32(address: u32) -> i32 {
        unsafe { (address as *const i32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let object_count = unsafe { read_i32(this.wrapping_add(COUNT)) };
    if object_count >= SLOT_LIMIT {
        return 0;
    }

    let slot_index = object_count.wrapping_add(1) as u32;
    let slot_address = this.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
    let first_word = unsafe { read_u32(source_record) };
    let second_word = unsafe { read_u32(source_record.wrapping_add(4)) };
    let third_word = unsafe { read_u32(source_record.wrapping_add(8)) };
    unsafe {
        write_u32(slot_address, first_word);
        write_u32(slot_address.wrapping_add(4), second_word);
        write_u32(slot_address.wrapping_add(8), third_word);
    }
    let fourth_word = unsafe { read_u32(source_record.wrapping_add(12)) };
    unsafe { write_u32(slot_address.wrapping_add(12), fourth_word) };

    let count_after_copy = unsafe { read_i32(this.wrapping_add(COUNT)) };
    unsafe { write_u32(this.wrapping_add(COUNT), count_after_copy.wrapping_add(1) as u32) };
    1
});
