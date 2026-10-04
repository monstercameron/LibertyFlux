// original: 0x00bd8940 NETWORK_IS_SESSION_INVITABLE
/// Script native handler `NETWORK_IS_SESSION_INVITABLE`.
///
/// Reads the argument array at ctx+8, no script arguments, calls the engine worker
/// (intercepted by the checker), and writes the zero-extended low byte of the engine answer to the return slot.
export!(cdecl, rw_00bd8940(ctx: *const u8) -> u32 {
    unsafe {
        let ans: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
