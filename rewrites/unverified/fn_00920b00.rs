// original: 0x00920B00 uppercase_latin_byte_string

/// Convert ASCII lowercase bytes in a NUL-terminated mutable string.
///
/// The single stack argument is the string pointer; the function uses no
/// object state and returns the address of the terminating NUL. Bytes in
/// `0x61..=0x7A` are reduced by `0x20`; every other byte is left unchanged,
/// including bytes at or above `0x80` and the terminator. The original checks
/// the locale selector at `0x116C250`, but its signed-byte comparison leaves
/// the high-byte range unchanged. The input must remain readable through its
/// NUL.
///
/// Calling convention: cdecl with one pointer argument; no outgoing calls.
lf_checker_rt::export!(cdecl, rw_00920B00(buffer: u32) -> u32 {
    unsafe {
        const ASCII_LOWER_FIRST: u8 = b'a';
        const ASCII_LOWER_LAST: u8 = b'z';
        const UPPERCASE_DELTA: u8 = 0x20;

        let mut cursor = buffer;
        loop {
            let byte = (cursor as *const u8).read();
            if byte == 0 {
                return cursor;
            }

            if (ASCII_LOWER_FIRST..=ASCII_LOWER_LAST).contains(&byte) {
                (cursor as *mut u8).write(byte.wrapping_sub(UPPERCASE_DELTA));
            }
            cursor = cursor.wrapping_add(1);
        }
    }
});
