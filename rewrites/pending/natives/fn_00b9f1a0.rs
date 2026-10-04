// original: 0x00b9f1a0 GET_CREATE_RANDOM_COPS
/// Script native `GET_CREATE_RANDOM_COPS` (hash 0x4F9342F3).
///
/// Takes no script arguments: calls the engine worker with no
/// arguments and stores the low byte of its answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00b9f1a0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
