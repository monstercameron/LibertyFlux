// original: 0x00bb8f50 SET_NEXT_DESIRED_MOVE_STATE
/// Script native `SET_NEXT_DESIRED_MOVE_STATE` (hash 0x02033258).
///
/// Forwards one script argument (a move-state enum value) to the engine. No
/// return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb8f50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
