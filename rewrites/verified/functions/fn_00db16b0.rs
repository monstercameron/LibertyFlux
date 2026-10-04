// original: 0x00db16b0 UILayoutFrame::vf107
/// Run the token follow-up when present, then the two enable steps.
///
/// When the frame holds a non-zero token it is resolved to a target and
/// the two-step follow-up runs as in the sibling handlers. Afterwards two
/// enable steps always run on the frame, each with a one argument, and the
/// second step's result is this function's result.
export!(thiscall, rw_00db16b0(this_ptr: u32) -> u32 {
    unsafe {
        const TOKEN: usize = 0xd4;
        const STEP_SLOT: usize = 0x4c;
        const FINISH_SLOT: usize = 0x16c;
        const FIRST_ENABLE_SLOT: usize = 0x14;
        const SECOND_ENABLE_SLOT: usize = 0x18;

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
        let first_at = *((table + FIRST_ENABLE_SLOT) as *const u32) as usize;
        let first: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(first_at);
        first(this_ptr, 1);

        let table = *(this_ptr as *const u32) as usize;
        let second_at = *((table + SECOND_ENABLE_SLOT) as *const u32) as usize;
        let second: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(second_at);
        second(this_ptr, 1)
    }
});
