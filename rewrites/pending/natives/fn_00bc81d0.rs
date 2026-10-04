// original: 0x00bc81d0 SWITCH_CAR_GENERATOR
/// Script native `SWITCH_CAR_GENERATOR` (hash 0x7CE83A30).
///
/// Forwards two script arguments (a car-generator handle and a switch value) to the engine. No return slot is written.
export!(cdecl, rw_00bc81d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
