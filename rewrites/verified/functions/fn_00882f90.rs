// original: 0x00882f90 stream_req_teardown (proposed)
/// Tear down a streaming request: close it, clear two header words, free the buffer.
///
/// Calls the request's close routine (intercepted callee 1, thiscall, no
/// stack arguments) with the object pointer, then reads the buffer pointer
/// at `+0x28`, zeroes the words at `+0x00` and `+0x08`, and, when the buffer
/// is non-null, hands it to the release routine (intercepted callee 2,
/// cdecl, one argument).
///
/// Original: thiscall, no stack arguments, no return value.
lf_checker_rt::export!(thiscall, rw_00882f90(this: u32) -> u32 {
    unsafe {
        const BUFFER: u32 = 0x28;
        const CLOSE_CALLEE: u32 = 1;
        const RELEASE_CALLEE: u32 = 2;
        let _closed: u32 = lf_checker_rt::callee_thiscall!(CLOSE_CALLEE, u32, this);
        let buffer = ((this + BUFFER) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(0);
        ((this + 8) as *mut u32).write_unaligned(0);
        if buffer != 0 {
            let _released: u32 = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, buffer);
        }
        0
    }
});
