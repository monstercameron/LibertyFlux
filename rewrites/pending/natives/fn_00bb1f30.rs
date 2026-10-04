// original: 0x00BB1F30 GET_PLAYER_ID
// GET_PLAYER_ID: script native handler returning an integer to the script.
// Takes no script arguments, stores the engine answer (full EAX)
// in the return slot, and returns it in EAX like the original.
export!(cdecl, rw_00bb1f30(ctx: u32) -> u32 {
    let answer: u32 = callee_cdecl!(1, u32,);
    let slot = unsafe { *(ctx as *const u32) as *mut u32 };
    unsafe { *slot = answer };
    answer
});
