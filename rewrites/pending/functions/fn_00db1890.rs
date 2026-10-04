// original: 0x00db1890 UILayoutFrame::vf111
/// If the frame holds a token, resolve it and run the two-step follow-up.
///
/// A zero token means there is nothing to do and the result is zero.
/// Otherwise the token is resolved to a target object, the frame computes
/// a follow-up argument through its own step handler, and the target's
/// finish handler runs with that argument; its result is returned.
export!(thiscall, rw_00db1890(this_ptr: u32) -> u32 {
    unsafe {
        const TOKEN: usize = 0xd8;
        const STEP_SLOT: usize = 0x4c;
        const FINISH_SLOT: usize = 0x178;

        let token = *((this_ptr as *const u8).add(TOKEN) as *const u32);
        if token == 0 {
            return 0;
        }
        let target = callee_cdecl!(1, u32, token);

        let table = *(this_ptr as *const u32) as usize;
        let step_at = *((table + STEP_SLOT) as *const u32) as usize;
        let step: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(step_at);
        let arg = step(this_ptr);

        let target_table = *(target as *const u32) as usize;
        let finish_at = *((target_table + FINISH_SLOT) as *const u32) as usize;
        let finish: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(finish_at);
        finish(target, arg)
    }
});
