// original: 0x00CBAF30 reverse_task_records

/// Reverse the paired records owned by a task container.
///
/// The thiscall method reads a signed item count at offset zero. Each item has
/// two inline C strings in a 56-byte record, and a parallel four-word vector
/// at offset `VECTOR_TABLE` with 16-byte spacing. For counts greater than one,
/// symmetric strings and vectors are exchanged until the two ends meet. The
/// right vector receives the left vector's first three words and a fourth word
/// read from an uninitialized stack slot; only the zero-filled stack case is
/// represented by this rewrite. Counts whose signed `count - 1` is nonpositive
/// return without touching the records. The method has no stack arguments and
/// returns no value.
lf_checker_rt::export!(thiscall, rw_00cbaf30(this: u32) -> () {
    const COUNT_FIELD: u32 = 0;
    const RECORDS: u32 = 4;
    const FIRST_TEXT: u32 = 0;
    const SECOND_TEXT: u32 = 32;
    const RECORD_STRIDE: u32 = 56;
    const VECTOR_TABLE: u32 = 0x1d0;
    const VECTOR_STRIDE: u32 = 16;
    const FIRST_TEXT_CAPACITY: usize = 32;
    const SECOND_TEXT_CAPACITY: usize = 24;
    const STACK_COMPONENT_BITS: u32 = 0;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe fn capture_c_string(address: u32, capacity: usize) -> ([u8; 32], usize) {
        let mut bytes = [0u8; 32];
        let mut length = 0usize;
        while length < capacity {
            let byte = unsafe { read_u8(address + length as u32) };
            bytes[length] = byte;
            length += 1;
            if byte == 0 {
                break;
            }
        }
        (bytes, length)
    }

    unsafe fn restore_c_string(address: u32, bytes: &[u8; 32], length: usize) {
        for index in 0..length {
            unsafe { write_u8(address + index as u32, bytes[index]) };
        }
    }

    unsafe fn swap_c_strings(left: u32, right: u32, capacity: usize) {
        let (left_bytes, left_length) = unsafe { capture_c_string(left, capacity) };
        let (right_bytes, right_length) = unsafe { capture_c_string(right, capacity) };
        unsafe {
            restore_c_string(left, &right_bytes, right_length);
            restore_c_string(right, &left_bytes, left_length);
        }
    }

    unsafe {
        let final_index = read_u32(this + COUNT_FIELD).wrapping_sub(1);
        if (final_index as i32) <= 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, 0);
            return;
        }

        // This mirrors the original's first unconditional pair exchange and
        // its signed comparison at the end of each loop iteration.
        let mut first_index = 0u32;
        let mut last_index = final_index;
        loop {
            let first_record = this + RECORDS + first_index * RECORD_STRIDE;
            let last_record = this + RECORDS + last_index * RECORD_STRIDE;
            swap_c_strings(
                first_record + FIRST_TEXT,
                last_record + FIRST_TEXT,
                FIRST_TEXT_CAPACITY,
            );
            swap_c_strings(
                first_record + SECOND_TEXT,
                last_record + SECOND_TEXT,
                SECOND_TEXT_CAPACITY,
            );

            let first_vector = this + VECTOR_TABLE + first_index * VECTOR_STRIDE;
            let last_vector = this + VECTOR_TABLE + last_index * VECTOR_STRIDE;
            let last_words = [
                read_u32(last_vector),
                read_u32(last_vector + 4),
                read_u32(last_vector + 8),
                read_u32(last_vector + 12),
            ];
            let first_words = [
                read_u32(first_vector),
                read_u32(first_vector + 4),
                read_u32(first_vector + 8),
            ];
            for (component, word) in last_words.into_iter().enumerate() {
                write_u32(first_vector + component as u32 * 4, word);
            }
            for (component, word) in first_words.into_iter().enumerate() {
                write_u32(last_vector + component as u32 * 4, word);
            }
            write_u32(last_vector + 12, STACK_COMPONENT_BITS);

            first_index = first_index.wrapping_add(1);
            last_index = last_index.wrapping_sub(1);
            if (first_index as i32) >= (last_index as i32) {
                break;
            }
        }

        // The cookie helper's ECX value depends on this implementation's
        // stack frame and is intentionally omitted by the contract.
        let _ = lf_checker_rt::callee_thiscall!(1, u32, 0);
    }
});
