// original: 0x009CBC90 GET_SPEECH_FOR_EMERGENCY_SERVICE_CALL
// GET_SPEECH_FOR_EMERGENCY_SERVICE_CALL: script native handler returning an integer to the script.
// Takes no script arguments, stores the engine answer (full EAX)
// in the return slot, and returns it in EAX like the original.
export!(cdecl, rw_009cbc90(ctx: u32) -> u32 {
    let answer: u32 = callee_cdecl!(1, u32,);
    let slot = unsafe { *(ctx as *const u32) as *mut u32 };
    unsafe { *slot = answer };
    answer
});
