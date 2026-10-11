// original: 0x009C2A30 copy_line_after_newlines

/// Skip leading carriage-return and line-feed bytes in `input`, then copy the
/// next line to `output`, stopping before CR, LF, or NUL and writing a NUL
/// terminator. If the input is empty after the leading delimiters, return
/// zero without touching `output`; otherwise return the address of the byte
/// where the copy stopped. The function uses byte pointers and cdecl args.
lf_checker_rt::export!(cdecl, rw_009c2a30(input: u32, output: u32) -> u32 {
    unsafe {
        let mut cursor = input as usize as *const u8;
        while matches!(cursor.read(), b'\n' | b'\r') {
            cursor = cursor.add(1);
        }
        let mut byte = cursor.read();
        if byte == 0 {
            return 0;
        }
        let mut destination = output as usize as *mut u8;
        loop {
            if matches!(byte, 0 | b'\n' | b'\r') {
                destination.write(0);
                return cursor as usize as u32;
            }
            cursor = cursor.add(1);
            destination.write(byte);
            destination = destination.add(1);
            byte = cursor.read();
        }
    }
});
