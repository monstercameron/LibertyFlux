// original: 0x00bb8b60 GET_MOBILE_PHONE_TASK_SUB_TASK
/// Script native `GET_MOBILE_PHONE_TASK_SUB_TASK` (hash 0x517B226E).
///
/// Forwards two script arguments (a character handle and a task index) to
/// the engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00bb8b60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
