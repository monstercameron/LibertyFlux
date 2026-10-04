// original: 0x00bc6ce0 IS_PAY_N_SPRAY_ACTIVE
/// Script native `IS_PAY_N_SPRAY_ACTIVE` (hash 0x1EE70376).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6ce0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
