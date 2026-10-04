// original: 0x00db1740 UILayoutFrame::vf112
/// Run the three gated follow-ups, then clear the pending flag.
///
/// The token section resolves the frame's token and runs the two-step
/// follow-up; the middle section runs only when the frame's marker byte
/// is set, forwarding a fresh stamp to the shared sink; the tail section
/// only re-notifies. Each section notifies the shared notifier unless the
/// global pending flag is set. The flag is always cleared before
/// returning, and the result is the last handler that ran, or zero when
/// no section fired.
export!(thiscall, rw_00db1740(this_ptr: u32) -> u32 {
    unsafe {
        const TOKEN: usize = 0xdc;
        const MARKER: usize = 0xeb;
        const TAIL_TOKEN: usize = 0xe0;
        const STEP_SLOT: usize = 0x4c;
        const FINISH_SLOT: usize = 0x184;
        const PENDING_FLAG: u32 = 0x017a65a5;
        const SINK_OBJECT: u32 = 0x018b6c8c;
        const NOTIFIER: u32 = 0x019d2e08;

        let obj = this_ptr as *const u8;
        let mut result: u32 = 0;

        let token = *(obj.add(TOKEN) as *const u32);
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
            result = finish(target, arg);

            if *global::<u8>(PENDING_FLAG) == 0 {
                result = callee_thiscall!(4, u32, relocated(NOTIFIER), 1);
            }
        }

        let marker = *(obj.add(MARKER) as *const u8);
        if marker != 0 && *global::<u8>(PENDING_FLAG) == 0 {
            let table = *(this_ptr as *const u32) as usize;
            let step_at = *((table + STEP_SLOT) as *const u32) as usize;
            let step: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(step_at);
            let stamp = step(this_ptr);
            result = callee_thiscall!(5, u32, *global::<u32>(SINK_OBJECT), stamp);
            if *global::<u8>(PENDING_FLAG) == 0 {
                result = callee_thiscall!(4, u32, relocated(NOTIFIER), 1);
            }
        }

        let tail = *(obj.add(TAIL_TOKEN) as *const u32);
        if tail != 0 && *global::<u8>(PENDING_FLAG) == 0 {
            result = callee_thiscall!(4, u32, relocated(NOTIFIER), 1);
        }

        *global::<u8>(PENDING_FLAG) = 0;
        result
    }
});
