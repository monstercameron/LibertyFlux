// original: 0x00A71910 CTaskComplexPlayerSettingsTask::vf18
/// Update the player's settings task from its active event. Active-phase
/// completion may set the interface flag; otherwise the event and current
/// state select a replacement task, a timer task, or an early return. The
/// rewrite preserves exact event and state tests, helper order, state writes,
/// and task-construction arguments.
#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }
#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

const SETTINGS_INTERFACE: u32 = 0x08;
const SETTINGS_PHASE: u32 = 0x14;
const SETTINGS_VALUE: u32 = 0x18;
const SETTINGS_STATE: u32 = 0x1c;
const EVENT_CONTEXT_COMPONENT: u32 = 0x2b0;
const TASK_FLAGS: u32 = 0x0c;
const POOL_POINTER: u32 = 0x0167e2a0;

const S_EVENT_TYPE: u32 = 1;
const S_PHASE_RESULT: u32 = 2;
const S_READY_CHECK: u32 = 3;
const S_FIRST_GATE: u32 = 4;
const S_FIRST_STATE: u32 = 5;
const S_SECOND_GATE: u32 = 6;
const S_ALLOC_TIMER: u32 = 7;
const S_SET_TIMER: u32 = 8;
const S_STATE_CHECK: u32 = 9;
const S_STATE_GATE: u32 = 10;
const S_STATE_READY: u32 = 11;
const S_ALLOC_TASK: u32 = 12;
const S_BUILD_TASK: u32 = 13;

/// Update the player's settings task from its active event. A false phase
/// flag selects a state-dependent task from event 0xca or 0x190, or polls
/// the event context and two readiness helpers to choose states 0xd7, 0xda,
/// 0xdd or -1. States 0xd8 and 0xdb return no task. A true phase flag calls
/// the active task's completion interface and may set its bit 1. New tasks
/// use the existing state and value fields and are initialized with the
/// fixed timer arguments. Event type and state helpers return unsigned words;
/// state membership and event equality are exact integer comparisons. This
/// is a thiscall with one stack pointer argument; all callees are scripted.
unsafe fn player_settings_event(this: u32, event_record: u32) -> u32 {
    unsafe {
        let interface = read_u32(this + SETTINGS_INTERFACE);
        if read_u8(this + SETTINGS_PHASE) != 0 {
            let _ = lf_checker_rt::callee_thiscall!(S_EVENT_TYPE, u32, interface);
            if read_u32(interface + TASK_FLAGS) & 1 == 0 {
                let completed = lf_checker_rt::callee_thiscall!(
                    S_PHASE_RESULT,
                    u8,
                    interface,
                    event_record,
                    1,
                    0
                );
                if completed != 0 {
                    write_u32(interface + TASK_FLAGS, read_u32(interface + TASK_FLAGS) | 2);
                }
            }
            return 0;
        }

        let event_type = lf_checker_rt::callee_thiscall!(S_EVENT_TYPE, u32, interface);
        let original_state = read_u32(this + SETTINGS_STATE);
        if event_type == 0xca || event_type == 0x190 {
            let next_state = match original_state {
                0xd7 | 0xda => {
                    let ready = lf_checker_rt::callee_thiscall!(S_STATE_CHECK, u8, this);
                    if ready != 0 { 0xdb } else { 0xd8 }
                }
                0xd8 | 0xdb => return 0,
                0xd9 | 0xdc => 0xdd,
                _ => {
                    let allowed = lf_checker_rt::callee_thiscall!(
                        S_STATE_GATE,
                        u8,
                        event_record.wrapping_add(EVENT_CONTEXT_COMPONENT),
                        1
                    );
                    if allowed != 0 {
                        let ready = lf_checker_rt::callee_thiscall!(S_STATE_READY, u8, this);
                        if ready != 0 { 0xda } else { 0xd7 }
                    } else {
                        0xdd
                    }
                }
            };
            write_u32(this + SETTINGS_STATE, next_state);
            let stored_state = read_u32(this + SETTINGS_STATE);
            return make_settings_task(this, stored_state);
        }

        let context_ready = lf_checker_rt::callee_thiscall!(
            S_READY_CHECK,
            u8,
            event_record.wrapping_add(EVENT_CONTEXT_COMPONENT),
            1
        );
        if context_ready != 0 {
            let first_ready = lf_checker_rt::callee_cdecl!(S_FIRST_GATE, u8,);
            if first_ready != 0 {
                let state_ready = lf_checker_rt::callee_thiscall!(S_FIRST_STATE, u8, this);
                let next_state = if state_ready != 0 { 0xd7 } else { 0xda };
                write_u32(this + SETTINGS_STATE, next_state);
                let stored_state = read_u32(this + SETTINGS_STATE);
                return make_settings_task(this, stored_state);
            }
        }

        let second_ready = lf_checker_rt::callee_cdecl!(S_SECOND_GATE, u8,);
        if second_ready != 0 {
            write_u32(this + SETTINGS_STATE, 0xdd);
            let stored_state = read_u32(this + SETTINGS_STATE);
            return make_settings_task(this, stored_state);
        }

        write_u32(this + SETTINGS_STATE, u32::MAX);
        let pool = lf_checker_rt::global::<u32>(POOL_POINTER).read_unaligned();
        let timer_task = lf_checker_rt::callee_thiscall!(S_ALLOC_TIMER, u32, pool);
        if timer_task == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(S_SET_TIMER, u32, timer_task, 5000)
    }
}

#[inline(always)]
unsafe fn make_settings_task(this: u32, state: u32) -> u32 {
    unsafe {
        let pool = lf_checker_rt::global::<u32>(POOL_POINTER).read_unaligned();
        let task = lf_checker_rt::callee_thiscall!(S_ALLOC_TASK, u32, pool);
        if task == 0 {
            return 0;
        }
        let value = read_u32(this + SETTINGS_VALUE);
        lf_checker_rt::callee_thiscall!(
            S_BUILD_TASK,
            u32,
            task,
            value,
            state,
            0x41000000,
            0,
            0x3f800000,
            0
        )
    }
}

lf_checker_rt::export!(thiscall, rw_00a71910(this: u32, event_record: u32) -> u32 {
    unsafe { player_settings_event(this, event_record) }
});

