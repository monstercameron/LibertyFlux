// original: 0x008C4C70 file_read_fully

/// Open the requested file with the fixed read mode, allocate a buffer through the intercepted handle helper, read the requested byte count, append a NUL byte, close the handle, and return the buffer. A failed open returns all bits set.
lf_checker_rt::export!(cdecl, rw_008c4c70(path: u32, requested_length: u32) -> u32 {
    const FILE_OBJECT_VA: u32 = 0x0110C0A0;
    const MODE_IDENTIFIER_VA: u32 = 0x00E7F09A;
    const OPEN_CALLEE: u32 = 70;
    const BUFFER_CALLEE: u32 = 71;
    const READ_CALLEE: u32 = 72;
    const CLOSE_CALLEE: u32 = 73;
    let handle = unsafe { lf_checker_rt::callee_thiscall!(OPEN_CALLEE, u32, lf_checker_rt::relocated(FILE_OBJECT_VA), path, lf_checker_rt::relocated(MODE_IDENTIFIER_VA), 0, 1) };
    if handle == 0 {
        return u32::MAX;
    }
    let buffer = unsafe { lf_checker_rt::callee_thiscall!(BUFFER_CALLEE, u32, handle) };
    let length_for_read = requested_length;
    let _ = unsafe { lf_checker_rt::callee_thiscall!(READ_CALLEE, u32, handle, length_for_read, buffer) };
    unsafe {
        core::ptr::write_unaligned((buffer as *mut u8).wrapping_add(requested_length as usize), 0);
        let _ = lf_checker_rt::callee_thiscall!(CLOSE_CALLEE, u32, handle);
    }
    buffer
});
