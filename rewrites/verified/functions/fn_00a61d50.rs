// original: 0x00A61D50 slot_list_remove_at

/// Removes a slot by shifting each later 16-byte record one position toward
/// the front while the signed index is below the signed last index. The
/// record words, including the two float bit fields, are copied in order.
/// The signed count is decremented once even when the index is already at or
/// beyond the last slot. ECX is the list object; the index is one thiscall
/// stack argument. The return register is not a semantic result.
lf_checker_rt::export!(thiscall, rw_00A61D50(this: u32, slot_index: i32) -> u32 {
    const COUNT: u32 = 0;
    const RECORD_AREA: u32 = 0x28;
    const RECORD_BYTES: u32 = 0x10;

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

    let initial_last = unsafe { read_i32(this.wrapping_add(COUNT)) }.wrapping_sub(1);
    let mut current_index = slot_index;
    if current_index < initial_last {
        let mut cursor = this
            .wrapping_add(RECORD_AREA)
            .wrapping_add((current_index as u32).wrapping_mul(RECORD_BYTES));
        loop {
            let source = cursor.wrapping_sub(8);
            let destination = cursor.wrapping_sub(0x18);
            let first_word = unsafe { read_u32(source) };
            let second_word = unsafe { read_u32(source.wrapping_add(4)) };
            let third_word = unsafe { read_u32(source.wrapping_add(8)) };
            unsafe {
                write_u32(destination, first_word);
                write_u32(destination.wrapping_add(4), second_word);
                write_u32(destination.wrapping_add(8), third_word);
            }
            let fourth_word = unsafe { read_u32(source.wrapping_add(12)) };
            unsafe { write_u32(destination.wrapping_add(12), fourth_word) };

            current_index = current_index.wrapping_add(1);
            let current_last = unsafe { read_i32(this.wrapping_add(COUNT)) }.wrapping_sub(1);
            cursor = cursor.wrapping_add(RECORD_BYTES);
            if current_index >= current_last {
                break;
            }
        }
    }

    let count_before_decrement = unsafe { read_i32(this.wrapping_add(COUNT)) };
    unsafe {
        write_u32(this.wrapping_add(COUNT), count_before_decrement.wrapping_sub(1) as u32)
    };
    0
});
