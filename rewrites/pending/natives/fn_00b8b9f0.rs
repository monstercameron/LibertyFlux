// original: 0x00b8b9f0 ADD_BLIP_FOR_CONTACT
/// Script native `ADD_BLIP_FOR_CONTACT` (hash 0x7C671162).
///
/// Forwards four script arguments to the engine: three float bit-patterns
/// (a position) and an integer. No return slot is written.
export!(cdecl, rw_00b8b9f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
