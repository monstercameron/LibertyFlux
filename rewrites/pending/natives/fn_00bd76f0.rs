// original: 0x00bd76f0 CAN_REGISTER_MISSION_ENTITIES
/// Script native `CAN_REGISTER_MISSION_ENTITIES` (hash 0x66FD3CFC).
///
/// Forwards three script arguments (counts/handles) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd76f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
