// original: 0x00db1840 UILayoutFrame::vf114
/// Run the token follow-up when present, then hand off or clear the flag.
///
/// When the frame holds a non-zero token it is resolved to a target and
/// the two-step follow-up runs as in the sibling handlers. If the global
/// handoff flag is clear, control passes to the shared handoff routine
/// with the global handoff object (the original jumps; this rewrite calls
/// the same routine and returns its result). Otherwise the flag is
/// cleared. The result is zero when there was no token.
export!(thiscall, rw_00db1840(this_ptr: u32) -> u32 {
    unsafe {
        const TOKEN: usize = 0xe4;
        const STEP_SLOT: usize = 0x4c;
        const FINISH_SLOT: usize = 0x19c;
        const HANDOFF_FLAG: u32 = 0x017a65a4;
        const HANDOFF_OBJECT: u32 = 0x018b6c8c;

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
        let stepped = finish(target, arg);

        if *global::<u8>(HANDOFF_FLAG) == 0 {
            callee_thiscall!(5, u32, *global::<u32>(HANDOFF_OBJECT))
        } else {
            *global::<u8>(HANDOFF_FLAG) = 0;
            stepped
        }
    }
});
