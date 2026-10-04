// original: 0x00b93e40 ARE_CREDITS_FINISHED
/// Script native `ARE_CREDITS_FINISHED` (hash 0x63A669B6).
///
/// Takes no script arguments: calls the engine worker with no
/// arguments and stores the low byte of its answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00b93e40(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
