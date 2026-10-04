// original: 0x00b9e920 DAMAGE_PED_BODY_PART
/// Script native `DAMAGE_PED_BODY_PART` (hash 0x0744307B).
///
/// Forwards three script arguments (a character handle, a body-part id and a damage amount) to the engine. No return slot is written.
export!(cdecl, rw_00b9e920(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
