// original: 0x006727A0 append_format_to_buffer
//! Append formatted text to a bounded 2 KiB buffer. A null buffer is returned
//! immediately. Otherwise, scan to the current terminator, place a terminator
//! at byte `BUFFER_CAPACITY`, and forward the remaining capacity, format
//! pointer, and caller-owned variadic words to the scripted formatter helper.
//! The first variadic word anchors the stack area passed as `va_list`; the
//! helper snapshots it instead of reading it. The formatter's return count is
//! ignored and the original buffer pointer is returned.
const BUFFER_CAPACITY: u32 = 0x800;
const TERMINATOR_OFFSET: usize = BUFFER_CAPACITY as usize;

#[no_mangle]
pub unsafe extern "cdecl" fn fn_006727A0(
    buffer: *mut u8,
    format: *const u8,
    first_variadic_word: u32,
) -> *mut u8 {
    if buffer.is_null() {
        return buffer;
    }

    let mut existing_length = 0usize;
    while unsafe { core::ptr::read_volatile(buffer.add(existing_length)) } != 0 {
        existing_length = existing_length.wrapping_add(1);
    }

    let append_buffer = unsafe { buffer.add(existing_length) };
    unsafe {
        core::ptr::write_volatile(buffer.add(TERMINATOR_OFFSET), 0);
    }
    let remaining_capacity = BUFFER_CAPACITY.wrapping_sub(existing_length as u32);
    let variadic_words = core::ptr::addr_of!(first_variadic_word);
    let _formatted_length = lf_checker_rt::callee_cdecl!(
        1,
        u32,
        append_buffer as u32,
        remaining_capacity,
        format as u32,
        variadic_words as u32,
    );
    buffer
}
