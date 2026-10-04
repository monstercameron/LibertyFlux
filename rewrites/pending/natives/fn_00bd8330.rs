// original: 0x00BD8330 NETWORK_GET_FRIEND_COUNT
// NETWORK_GET_FRIEND_COUNT: script native handler returning an integer to the script.
// Takes no script arguments, stores the engine answer (full EAX)
// in the return slot, and returns it in EAX like the original.
lf_rn04_rt::export!(cdecl, rw_00bd8330(ctx: u32) -> u32 {
    let answer: u32 = lf_rn04_rt::callee_cdecl!(1, u32,);
    let slot = unsafe { *(ctx as *const u32) as *mut u32 };
    unsafe { *slot = answer };
    answer
});
