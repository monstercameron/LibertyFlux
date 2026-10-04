// original: 0x00bba700 TASK_SHIMMY_LET_GO
/// Script native `TASK_SHIMMY_LET_GO` (hash 0x1AA32729).
///
/// Ped handle; stores low byte of answer.
///
/// Stores the low byte of the engine's answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00bba700(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
