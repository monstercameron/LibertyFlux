// original: 0x00bc5780 FIND_POSITION_IN_RECORDING
/// Script native `FIND_POSITION_IN_RECORDING` (hash 0x22087F31).
///
/// Forwards one script argument (a vehicle handle) to the engine, which
/// returns a float in ST0, and stores that float bit-for-bit into the
/// return slot.
///
/// Quirk (observed): the handler spills the float through its own incoming
/// stack slot, clobbering the context pointer there. The slot write is the
/// only observable effect and is verified exactly.
export!(cdecl, rw_00bc5780(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});
