// original: 0x00B9FE70 IS_CHAR_TOUCHING_OBJECT
// IS_CHAR_TOUCHING_OBJECT: script native handler returning a boolean to the script.
// Forwards args[0], args[1], keeps only the low byte of the engine
// answer (movzx) and stores it as a dword in the return slot.
// Returns the return-slot pointer in EAX like the original.
export!(cdecl, rw_00b9fe70(ctx: u32) -> u32 {
    let base = ctx as *const u32;
    let args = unsafe { *base.add(2) as *const u32 };
    let a0 = unsafe { *args };
    let a1 = unsafe { *args.add(1) };
    let answer: u32 = callee_cdecl!(1, u32, a0, a1);
    let slot = unsafe { *base as *mut u32 };
    unsafe { *slot = answer & 0xFF };
    slot as u32
});
