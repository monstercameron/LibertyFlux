// original: 0x00B9F220 GET_GROUP_CHAR_DUCKS_WHEN_AIMED_AT
// GET_GROUP_CHAR_DUCKS_WHEN_AIMED_AT: script native handler returning a boolean to the script.
// Forwards args[0], keeps only the low byte of the engine
// answer (movzx) and stores it as a dword in the return slot.
// Returns the return-slot pointer in EAX like the original.
export!(cdecl, rw_00b9f220(ctx: u32) -> u32 {
    let base = ctx as *const u32;
    let args = unsafe { *base.add(2) as *const u32 };
    let a0 = unsafe { *args };
    let answer: u32 = callee_cdecl!(1, u32, a0);
    let slot = unsafe { *base as *mut u32 };
    unsafe { *slot = answer & 0xFF };
    slot as u32
});
