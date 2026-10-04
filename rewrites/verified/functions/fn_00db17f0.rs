// original: 0x00db17f0 UILayoutFrame::vf113
/// Run the token follow-up when present, then clear the pending flag.
///
/// When the frame holds a non-zero token it is resolved to a target and
/// the two-step follow-up runs as in the sibling handlers; if the global
/// pending flag is clear at that point, the shared notifier runs first.
/// The pending flag is always cleared before returning. The result is the
/// last handler that ran, or zero when there was no token.
export!(thiscall, rw_00db17f0(this_ptr: u32) -> u32 {
    unsafe {
        const TOKEN: usize = 0xe0;
        const STEP_SLOT: usize = 0x4c;
        const FINISH_SLOT: usize = 0x190;
        const PENDING_FLAG: u32 = 0x017a65a6;
        const NOTIFIER: u32 = 0x019d2e08;

        let token = *((this_ptr as *const u8).add(TOKEN) as *const u32);
        let result = if token == 0 {
            0
        } else {
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
            let stepped = finish(target, arg);

            if *global::<u8>(PENDING_FLAG) == 0 {
                callee_thiscall!(4, u32, relocated(NOTIFIER), 1)
            } else {
                stepped
            }
        };
        *global::<u8>(PENDING_FLAG) = 0;
        result
    }
});
