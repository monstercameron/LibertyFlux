// original: 0x00b866a0 ATTACH_CAM_TO_PED
/// Script native `ATTACH_CAM_TO_PED` (hash 0x78B00CB2).
///
/// Forwards two script arguments (a camera handle and a ped handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b866a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
