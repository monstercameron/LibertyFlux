// original: 0x00bd3950 DELETE_CHECKPOINT
/// Script native `DELETE_CHECKPOINT` (hash 0x1293731D).
///
/// Forwards one script argument (a checkpoint handle) to the engine. No
/// return slot is written. (The original cleans its pushed argument with
/// `(an instruction of the original)`; the effect on the stack pointer is identical to the plain
/// cdecl return here.)
export!(cdecl, rw_00bd3950(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
