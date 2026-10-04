// original: 0x00bd79f0 GET_HOST_ID
/// Script native `GET_HOST_ID` (hash 0x79C84DBC).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer (the host id) into the return slot. Unlike
/// the boolean natives, this handler keeps the whole answer.
export!(cdecl, rw_00bd79f0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
