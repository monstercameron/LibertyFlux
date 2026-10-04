// original: 0x00b86640 ADD_PED_TO_CINEMATOGRAPHY_AI
/// Script native `ADD_PED_TO_CINEMATOGRAPHY_AI` (hash 0x62687944).
///
/// Forwards two script arguments (a pedestrian handle and a flag) to the engine. No return slot is written.
export!(cdecl, rw_00b86640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
