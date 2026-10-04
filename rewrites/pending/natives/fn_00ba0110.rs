// original: 0x00ba0110 IS_PED_HOLDING_AN_OBJECT
/// Report whether a ped holds an object: call the engine with the
/// script argument and store the low byte of its answer (zero-extended)
/// in the return slot.
export!(cdecl, rw_00ba0110(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let ans: u32 = callee_cdecl!(1, u32, *args);
        *(*ctx as *mut u32) = ans & 0xFF;
        0
    }
});
