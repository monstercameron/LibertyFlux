// original: 0x00bc6700 IS_CAR_IN_AIR_PROPER
/// Report whether a car is properly airborne: call the engine with the
/// script argument and store the low byte of its answer (zero-extended)
/// in the return slot.
export!(cdecl, rw_00bc6700(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let ans: u32 = callee_cdecl!(1, u32, *args);
        *(*ctx as *mut u32) = ans & 0xFF;
        0
    }
});
