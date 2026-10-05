// original: 0x00bd8ee0 NETWORK_VERIFY_USER_STRING
/// Script native `NETWORK_VERIFY_USER_STRING` (hash 0x59884407).
///
/// Forwards one script argument to the engine worker. (The original drops
/// its pushed word with `(an instruction of the original)`; the effect on the stack pointer is
/// identical to the plain cdecl return here.) No return slot is written.
export!(cdecl, rw_00bd8ee0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
