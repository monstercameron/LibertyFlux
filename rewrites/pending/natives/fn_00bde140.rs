// original: 0x00bde140 GET_TASK_PLACE_CAR_BOMB_UNSUCCESSFUL
/// Script native `GET_TASK_PLACE_CAR_BOMB_UNSUCCESSFUL` (hash 0x0A4608E9).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bde140(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        slot as u32
    }
});
