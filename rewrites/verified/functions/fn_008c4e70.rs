// original: 0x008C4E70 config_read_clean_line

/// Read a configuration line into the shared 1024-byte buffer. A false read returns null. When the mode byte is nonzero, replace bytes below ASCII space and commas with spaces; then return a pointer past all leading spaces. High-bit bytes are preserved.
lf_checker_rt::export!(cdecl, rw_008c4e70(handle: u32, mode: u32) -> u32 {
    const BUFFER_VA: u32 = 0x0116BB68;
    const BUFFER_CAPACITY: u32 = 0x400;
    const READ_LINE_CALLEE: u32 = 90;
    const SPACE: u8 = 0x20;
    const COMMA: u8 = 0x2C;

    let read_succeeded = lf_checker_rt::callee_cdecl!(
        READ_LINE_CALLEE,
        u32,
        handle,
        lf_checker_rt::relocated(BUFFER_VA),
        BUFFER_CAPACITY,
    );
    if read_succeeded == 0 {
        return 0;
    }
    unsafe {
        let buffer = lf_checker_rt::global::<u8>(BUFFER_VA);

        if mode as u8 != 0 {
            let mut index = 0u32;
            loop {
                let byte = core::ptr::read_volatile(buffer.wrapping_add(index as usize));
                if byte == 0 {
                    break;
                }
                if byte < SPACE || byte == COMMA {
                    core::ptr::write_volatile(buffer.wrapping_add(index as usize), SPACE);
                }
                index = index.wrapping_add(1);
            }
        }

        let mut leading_spaces = 0u32;
        loop {
            let byte = core::ptr::read_volatile(buffer.wrapping_add(leading_spaces as usize));
            if byte == 0 || byte != SPACE {
                break;
            }
            leading_spaces = leading_spaces.wrapping_add(1);
        }
        lf_checker_rt::relocated(BUFFER_VA.wrapping_add(leading_spaces))
    }
});
