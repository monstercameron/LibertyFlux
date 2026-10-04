// original: 0x009CBE80 IS_RADIO_HUD_ON
// IS_RADIO_HUD_ON: script native handler returning a boolean to the script.
// Takes no script arguments, keeps only the low byte of the engine
// answer (movzx) and stores it as a dword in the return slot.
// Returns the return-slot pointer in EAX like the original.
export!(cdecl, rw_009cbe80(ctx: u32) -> u32 {
    let base = ctx as *const u32;
    let answer: u32 = callee_cdecl!(1, u32,);
    let slot = unsafe { *base as *mut u32 };
    unsafe { *slot = answer & 0xFF };
    slot as u32
});
