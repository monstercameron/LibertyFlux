// original: 0x00BD84A0 NETWORK_GET_NUMBER_OF_GAMES
// Rewrite of native handler NETWORK_GET_NUMBER_OF_GAMES.
//
// Passes no arguments; stores the engine's full-word answer.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bd84a0(ctx: u32) -> u32 {
    unsafe {
        let ret = *(ctx as *const u32) as *mut u32;
        let engine: extern "cdecl" fn() -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ans = engine();
        *ret = ans;
        ans
    }
});
