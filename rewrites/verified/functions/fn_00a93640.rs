// original: 0x00a93640 streaming_forward_to_handler (proposed)

/// Forward a value to the stream handler with a fixed callback address.
///
/// Loads the handler object from its global, then invokes the handler
/// callee with `(value, 0x00a935c0)` and returns its answer. Thin
/// forwarder; the callback address is an immediate in the original.
///
/// Original: cdecl, one stack argument. One callee (thiscall, 2 args).
lf_checker_rt::export!(cdecl, rw_00a93640(value: u32) -> u32 {
    unsafe {
        const HANDLER_G: u32 = 0x012fb260;
        const CALLBACK: u32 = 0x00a935c0;
        const HANDLE: u32 = 0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let handler = rd32(lf_checker_rt::relocated(HANDLER_G));
        // The callback is an absolute immediate with a relocation entry, so
        // the original pushes the relocated address, not the file address.
        let callback = lf_checker_rt::relocated(CALLBACK);
        lf_checker_rt::callee_thiscall!(HANDLE, u32, handler, value, callback)
    }
});
