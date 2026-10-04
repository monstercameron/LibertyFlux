// original: 0x00bb76d0 UPDATE_LOAD_SCENE
/// Script native `UPDATE_LOAD_SCENE` (hash 0x513D68DB).
///
/// Takes no script arguments: calls the engine worker with no
/// /// arguments and stores the low byte of its answer (zero-extended)
/// /// into the return slot.
export!(cdecl, rw_00bb76d0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
