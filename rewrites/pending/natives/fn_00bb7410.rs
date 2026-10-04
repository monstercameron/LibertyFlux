// original: 0x00bb7410 IS_STREAMING_PRIORITY_REQUESTS
/// Script native `IS_STREAMING_PRIORITY_REQUESTS` (hash 0x64342B55).
///
/// Calls the engine with no arguments and stores the low byte of its
/// answer (zero-extended) into the return slot. No script arguments are
/// read.
export!(cdecl, rw_00bb7410(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
