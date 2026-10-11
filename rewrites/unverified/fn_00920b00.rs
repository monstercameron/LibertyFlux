// original: 0x00920B00 uppercase_latin_byte_string

/// Convert selected lowercase bytes in a NUL-terminated mutable string.
///
/// The single stack argument is the string pointer; the function uses no
/// object state and returns the address of the terminating NUL. ASCII
/// lowercase bytes become uppercase. When the locale selector at `0x116C250`
/// contains `r`, bytes in the additional inclusive range `0xAE..=0xCD` are
/// also reduced by `0x20`. Bytes outside those ranges are copied unchanged,
/// including the terminator. The input must remain readable through its NUL.
///
/// Calling convention: cdecl with one pointer argument; no outgoing calls.
lf_checker_rt::export!(cdecl, rw_00920B00(buffer: u32) -> u32 {
    unsafe {
        const LOCALE_SELECTOR: u32 = 0x116C250;
        const ASCII_LOWER_FIRST: u8 = b'a';
        const ASCII_LOWER_LAST: u8 = b'z';
        const LOCALE_LOWER_FIRST: u8 = 0xAE;
        const LOCALE_LOWER_LAST: u8 = 0xCD;
        const UPPERCASE_DELTA: u8 = 0x20;

        let locale_r = lf_checker_rt::global::<u8>(LOCALE_SELECTOR).read() == b'r';
        let mut cursor = buffer;
        loop {
            let byte = (cursor as *const u8).read();
            if byte == 0 {
                return cursor;
            }

            let ascii_lower = (ASCII_LOWER_FIRST..=ASCII_LOWER_LAST).contains(&byte);
            let locale_lower = locale_r
                && (LOCALE_LOWER_FIRST..=LOCALE_LOWER_LAST).contains(&byte);
            if ascii_lower || locale_lower {
                (cursor as *mut u8).write(byte.wrapping_sub(UPPERCASE_DELTA));
            }
            cursor = cursor.wrapping_add(1);
        }
    }
});
