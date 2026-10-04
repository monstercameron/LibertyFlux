// original: 0x00b9ae00 SWITCH_PED_PATHS_ON
/// Script native `SWITCH_PED_PATHS_ON` (hash 0x67D908DF).
///
/// Forwards six script arguments (a volume's corner coordinates) to the
/// engine as raw bit-patterns, so float arguments survive bit-exactly. No
/// return slot is written.
/// (The original stages the six words through local stack slots below its
/// frame; the checker does not observe that scratch, only the outgoing
/// call's arguments, which match exactly.)
export!(cdecl, rw_00b9ae00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
        )
    }
});
