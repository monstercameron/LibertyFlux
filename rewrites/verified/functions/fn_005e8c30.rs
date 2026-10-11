// original: 0x005E8C30 CTaskComplexWanderCop::vf19

/// Create the first wander-cop task from the current event context.
///
/// `this` is the wander-cop task and `event` supplies a navigation owner at
/// `+0x21C` and a route state at `+0x22C`. If the owner's task kind is not 2,
/// the base wander task creates the first subtask. For kind 2, an active route
/// asks the shared timing pool for an object and, when allocation succeeds,
/// initializes it with the fixed audio parameters. An inactive route invokes
/// virtual slot 1 on the current subtask when present, otherwise it delegates
/// to the base task. Each selected callee's return value is returned unchanged;
/// the function pops one incoming stack word.
lf_checker_rt::export!(thiscall, rw_005e8c30(this: u32, event: u32) -> u32 {
    const EVENT_OWNER_OFFSET: u32 = 0x21c;
    const OWNER_TASK_KIND_OFFSET: u32 = 0x12c;
    const EVENT_ROUTE_OFFSET: u32 = 0x22c;
    const ROUTE_ACTIVE_BYTE_OFFSET: u32 = 0x08;
    const CURRENT_SUBTASK_OFFSET: u32 = 0x80;
    const SPECIAL_TASK_KIND: u32 = 2;
    const TIMING_POOL_GLOBAL_VA: u32 = 0x0167E2A0;
    const AUDIO_DURATION: u32 = 0x186a0;
    const AUDIO_KIND: u32 = 1;
    const AUDIO_FLAGS: u32 = 0;
    const AUDIO_GAIN_BITS: u32 = 0x41000000;

    let owner = unsafe {
        (event.wrapping_add(EVENT_OWNER_OFFSET) as *const u32).read_unaligned()
    };
    let task_kind = unsafe {
        (owner.wrapping_add(OWNER_TASK_KIND_OFFSET) as *const u32).read_unaligned()
    };
    if task_kind != SPECIAL_TASK_KIND {
        return lf_checker_rt::callee_thiscall!(2, u32, this, event);
    }

    let route = unsafe {
        (event.wrapping_add(EVENT_ROUTE_OFFSET) as *const u32).read_unaligned()
    };
    let route_active = unsafe {
        (route.wrapping_add(ROUTE_ACTIVE_BYTE_OFFSET) as *const u8).read()
    };
    if route_active != 0 {
        let pool_manager = unsafe {
            lf_checker_rt::global::<u32>(TIMING_POOL_GLOBAL_VA).read_unaligned()
        };
        let allocated = lf_checker_rt::callee_thiscall!(1, u32, pool_manager);
        if allocated == 0 {
            return 0;
        }
        return lf_checker_rt::callee_thiscall!(
            4,
            u32,
            allocated,
            AUDIO_DURATION,
            AUDIO_KIND,
            AUDIO_FLAGS,
            AUDIO_GAIN_BITS
        );
    }

    let subtask = unsafe {
        (this.wrapping_add(CURRENT_SUBTASK_OFFSET) as *const u32).read_unaligned()
    };
    if subtask == 0 {
        return lf_checker_rt::callee_thiscall!(2, u32, this, event);
    }
    lf_checker_rt::callee_thiscall!(3, u32, subtask)
});
