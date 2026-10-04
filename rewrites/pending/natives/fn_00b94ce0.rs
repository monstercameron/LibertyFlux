// original: 0x00b94ce0 IS_PC_VERSION
/// Script native `IS_PC_VERSION` (hash 0x1D9853EA).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
/// The same handler body serves several no-argument boolean natives.
export!(cdecl, rw_00b94ce0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
