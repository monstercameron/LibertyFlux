// original: 0x00b8bd40 CHANGE_BLIP_PRIORITY
/// Script native `CHANGE_BLIP_PRIORITY` (hash 0x69EC0E70).
///
/// Forwards two script arguments (a blip handle and the new priority) to the engine. No return slot is written.
export!(cdecl, rw_00b8bd40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
