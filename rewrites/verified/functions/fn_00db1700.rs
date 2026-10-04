// original: 0x00db1700 UILayoutFrame::vf108
/// Run the token follow-up when present, then always run the reset step.
///
/// When the frame holds a non-zero token it is resolved to a target and
/// the two-step follow-up runs as in the sibling handlers. Afterwards the
/// frame's reset step always runs with a zero argument, and its result is
/// this function's result.
export!(thiscall, rw_00db1700(this_ptr: u32) -> u32 {
    unsafe {
        const TOKEN: usize = 0xd4;
        const STEP_SLOT: usize = 0x4c;
        const FINISH_SLOT: usize = 0x16c;
        const RESET_SLOT: usize = 0x18;

        let token = *((this_ptr as *const u8).add(TOKEN) as *const u32);
        if token != 0 {
            let target = callee_cdecl!(1, u32, token);

            let table = *(this_ptr as *const u32) as usize;
            let step_at = *((table + STEP_SLOT) as *const u32) as usize;
            let step: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(step_at);
            let arg = step(this_ptr);

            let target_table = *(target as *const u32) as usize;
            let finish_at = *((target_table + FINISH_SLOT) as *const u32) as usize;
            let finish: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(finish_at);
            finish(target, arg);
        }

        let table = *(this_ptr as *const u32) as usize;
        let reset_at = *((table + RESET_SLOT) as *const u32) as usize;
        let reset: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(reset_at);
        reset(this_ptr, 0)
    }
});
