// original: 0x00bb2350 IS_IN_LAN_MODE
/// Script native `IS_IN_LAN_MODE` (hash 0x1B8E7EED).
///
/// Takes no script arguments: calls the engine worker and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2350(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
