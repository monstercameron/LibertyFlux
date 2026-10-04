// original: 0x00bb8af0 END_PED_QUEUE_MEMBERSHIP_LIST
/// Script native `END_PED_QUEUE_MEMBERSHIP_LIST` (hash 0x4449534F).
///
/// Calls the engine worker with no arguments and stores its full 32-bit
/// answer into the return slot. Unlike most handlers it issues the call
/// before reading the context pointer.
export!(cdecl, rw_00bb8af0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
