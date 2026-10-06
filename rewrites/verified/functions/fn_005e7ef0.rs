// original: 0x005e7ef0 DOES_WEB_PAGE_EXIST
/// Test whether a web page exists.
///
/// Passes a 512-byte scratch buffer in ECX with the page argument (script
/// argument 0) in EDX to the engine implementation, stores the low byte of
/// the answer in the return slot, then runs the stack-cookie check.
///
/// The contract skips the ECX address: the buffer is uninitialised scratch
/// that the function never reads back. EDX is compared by value.
export!(cdecl, rw_005e7ef0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let buf = [0u32; 128];
        let answer = callee_fastcall!(1, u32, buf.as_ptr() as u32, *args);
        let slot = *(ctx as *const u32);
        *(slot as *mut u32) = answer & 0xff;
        callee_cdecl!(2, u32,);
        slot
    }
});
