// original: 0x00bb9260 TASK_CAR_MISSION_NOT_AGAINST_TRAFFIC
/// Script native `TASK_CAR_MISSION_NOT_AGAINST_TRAFFIC` (hash 0x3BE7444A).
///
/// Forwards eight script arguments to the engine, one of them a float
/// word (passed as raw bits). The original stages that word through a
/// scratch stack slot, fully overwritten before the call, so only the
/// argument bits are observable. No return slot is written.
export!(cdecl, rw_00bb9260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7))
    }
});
