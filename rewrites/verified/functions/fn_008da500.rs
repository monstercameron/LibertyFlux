// original: 0x008DA500 loading_tick_dispatch

/// Dispatch one loading tick using five global gates, one context byte, a dialog query, and two state queries. Each early active condition tail-calls the loading-cycle updater; otherwise the final two state bits select between the updater and the alternate loading routine. All callees are intercepted by the checker contract.
lf_checker_rt::export!(thiscall, rw_008da500(context: u32) -> u32 {
    const GLOBAL_ACTIVE_VA: u32 = 0x01037650;
    const GLOBAL_DIALOG_VA: u32 = 0x011609F6;
    const GLOBAL_FADE_VA: u32 = 0x0150C171;
    const STATE_OBJECT_VA: u32 = 0x0118D804;
    const STATE_FLAGS_VA: u32 = 0x018B6EA4;
    const LOADING_CONTEXT_VA: u32 = 0x011737D0;
    const CONTEXT_STATE_OFFSET: usize = 0x0FCB;
    const STATE_PRIMARY_OFFSET: u32 = 0x424;
    const STATE_SECONDARY_OFFSET: u32 = 0x448;
    const FLAG_BIT_6: u32 = 1 << 6;
    const FLAG_BIT_9: u32 = 1 << 9;
    const DIALOG_CALLEE: u32 = 91;
    const STATE_QUERY_CALLEE: u32 = 92;
    const CYCLE_UPDATE_CALLEE: u32 = 93;
    const ALTERNATE_UPDATE_CALLEE: u32 = 94;

    unsafe {
        let tail_cycle = || lf_checker_rt::callee_thiscall!(
            CYCLE_UPDATE_CALLEE, u32, lf_checker_rt::relocated(LOADING_CONTEXT_VA)
        );
        let tail_alternate = || lf_checker_rt::callee_thiscall!(
            ALTERNATE_UPDATE_CALLEE, u32, lf_checker_rt::relocated(LOADING_CONTEXT_VA)
        );
        if core::ptr::read_volatile(lf_checker_rt::global::<u8>(GLOBAL_ACTIVE_VA).cast_const()) == 0 {
            return tail_alternate();
        }
        if core::ptr::read_volatile((context as *const u8).add(CONTEXT_STATE_OFFSET)) != 0 {
            return tail_cycle();
        }
        if core::ptr::read_volatile(lf_checker_rt::global::<u8>(GLOBAL_DIALOG_VA).cast_const()) == 0
            && core::ptr::read_volatile(lf_checker_rt::global::<u8>(GLOBAL_FADE_VA).cast_const()) == 0
            && lf_checker_rt::callee_cdecl!(DIALOG_CALLEE, u32,) == 0
        {
            let state_object = core::ptr::read_volatile(lf_checker_rt::global::<u32>(STATE_OBJECT_VA).cast_const());
            if lf_checker_rt::callee_thiscall!(STATE_QUERY_CALLEE, u32, state_object.wrapping_add(STATE_PRIMARY_OFFSET)) == 2 {
                return tail_cycle();
            }
            if lf_checker_rt::callee_thiscall!(STATE_QUERY_CALLEE, u32, state_object.wrapping_add(STATE_SECONDARY_OFFSET)) == 2 {
                return tail_cycle();
            }
        }
        let flags = core::ptr::read_volatile(lf_checker_rt::global::<u32>(STATE_FLAGS_VA).cast_const());
        if flags & (FLAG_BIT_6 | FLAG_BIT_9) != 0 {
            tail_cycle()
        } else {
            tail_alternate()
        }
    }
});
