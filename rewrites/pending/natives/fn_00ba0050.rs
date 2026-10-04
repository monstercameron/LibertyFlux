// original: 0x00ba0050 IS_PED_ATTACHED_TO_OBJECT
/// Script native `IS_PED_ATTACHED_TO_OBJECT` (hash 0x0BCE3423).
///
/// Forwards two script arguments (a ped and an object handle) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00ba0050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
