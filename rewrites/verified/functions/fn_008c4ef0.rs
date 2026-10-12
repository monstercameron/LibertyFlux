// original: 0x008C4EF0 file_handle_read_line

/// Read up to capacity minus one bytes through the intercepted reader. When no bytes are returned, compare the signed file position with the signed file size and report failure at or beyond end of file. Otherwise stop at NUL or line feed, replace carriage returns with NUL while continuing, terminate the line, and return one in AL.
lf_checker_rt::export!(cdecl, rw_008c4ef0(handle: u32, buffer: u32, capacity: u32) -> u8 {
    const READ_CALLEE: u32 = 87;
    const POSITION_CALLEE: u32 = 88;
    const SIZE_CALLEE: u32 = 89;
    let read_limit = capacity.wrapping_sub(1);
    let bytes_read = unsafe { lf_checker_rt::callee_cdecl!(READ_CALLEE, u32, buffer, read_limit, handle) };
    if bytes_read == 0 {
        let file_position = unsafe { lf_checker_rt::callee_thiscall!(POSITION_CALLEE, u32, handle) };
        let file_size = unsafe { lf_checker_rt::callee_thiscall!(SIZE_CALLEE, u32, handle) };
        if !((file_position as i32) < (file_size as i32)) {
            return 0;
        }
    }
    unsafe {
        let line = buffer as *mut u8;
        let mut index = 0u32;
        while index < bytes_read {
            let character = core::ptr::read_volatile(line.wrapping_add(index as usize));
            if character == 0 {
                return 1;
            }
            if character == 0x0A {
                break;
            }
            if character == 0x0D {
                core::ptr::write_volatile(line.wrapping_add(index as usize), 0);
            }
            index = index.wrapping_add(1);
        }
        core::ptr::write_volatile(line.wrapping_add(index as usize), 0);
    }
    1
});
