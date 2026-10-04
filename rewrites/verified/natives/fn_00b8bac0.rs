// original: 0x00b8bac0 ADD_BLIP_FOR_OBJECT
/// Script native `ADD_BLIP_FOR_OBJECT` (hash 0x70CC1487).
///
/// Attaches a radar blip to an object. Forwards two script arguments (the
/// object handle and an out-pointer through which the engine returns the new
/// blip handle). No return slot is written by the handler itself.
export!(cdecl, rw_00b8bac0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
