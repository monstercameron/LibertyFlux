// original: 0x00bb2230 HAS_DEATHARREST_EXECUTED
/// Script native handler `HAS_DEATHARREST_EXECUTED` (hash 0x3B0C6738).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2230(ctx: u32) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
