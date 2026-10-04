// original: 0x00b87ec0 M
/// Script native `M` (hash 0x3970702E).
///
/// Forwards two script arguments (a vehicle handle and a float word,
/// bit-for-bit) to the engine. The handler spills the float through a
/// scratch stack slot, fully overwritten, so no quirk bytes survive. No
/// return slot is written.
export!(cdecl, rw_00b87ec0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
