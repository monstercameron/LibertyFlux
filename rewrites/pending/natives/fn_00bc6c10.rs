// original: 0x00BC6C10 IS_EMERGENCY_SERVICES_VEHICLE
/// Script native handler `IS_EMERGENCY_SERVICES_VEHICLE`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, calls the engine worker
/// (intercepted by the checker), and writes the zero-extended low byte of the engine answer to the return slot.
export!(cdecl, rw_00bc6c10(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let ans: u32 = callee_cdecl!(1, u32, a0,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
