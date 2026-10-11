// original: 0x00D177A0 OFFICER_DOWN
/// Process up to sixteen active task entries owned by a `CPed`'s intelligence
/// object. The intelligence pointer is at `root + 0x224`; its task pointer
/// slots begin at `+0x168`. Each task is ignored when absent, inactive at
/// `+0x211`, or already marked at `+0x270` with bit `0x1000`.
///
/// An eligibility callback chooses the processing path. On its affirmative
/// path, state at `task + 0x21c`, then `+0x12c`, selects effects; a signed
/// timer result is compared with `0x3fff` when that state is 2 and byte
/// `task + 0x210` is zero. Otherwise, state at `root + 0x21c`, then `+0x12c`,
/// can select a recovery effect when the task state is 0 or 1. A successful
/// effect starts a follow-up task and sets the task's completion bit. A
/// nonzero `task + 0x210` also marks it complete on the affirmative path
/// after its effect checks.
///
/// The root is a 32-bit pointer and is the sole stdcall argument; the callee
/// removes four stack bytes. The return value is not meaningful. Effect IDs
/// are image references and are translated through `relocated` for the
/// worker's mapped image base.
#[unsafe(no_mangle)]
pub extern "stdcall" fn rw_00d177a0(root: u32) {
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe fn mark_complete(task: u32) {
        let flags = unsafe { read_u32(task + TASK_FLAGS) };
        unsafe { write_u32(task + TASK_FLAGS, flags | COMPLETE_BIT) };
    }

    fn start_effect(root: u32, effect_id: u32) -> bool {
        let returned = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            root + EFFECT_CONTEXT,
            lf_checker_rt::relocated(effect_id),
            0,
            0,
            0,
            u32::MAX,
            0,
            0,
            1.0f32.to_bits(),
            0,
            0,
        );
        returned & 0xff != 0
    }

    fn queue_follow_up(root: u32, task: u32, event_id: u32) {
        let _ = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            root + FOLLOW_UP_CONTEXT,
            lf_checker_rt::relocated(event_id),
            0,
            task,
            0x2ee,
            0x4b5,
            0,
            0,
            0x1f4,
            0x1f4,
            1,
        );
    }

    const PED_INTELLIGENCE: u32 = 0x224;
    const TASK_SLOTS: u32 = 0x168;
    const TASK_SLOT_COUNT: u32 = 16;
    const TASK_ACTIVE: u32 = 0x211;
    const TASK_EVENT_STATE: u32 = 0x210;
    const TASK_OWNER: u32 = 0x21c;
    const TASK_FLAGS: u32 = 0x270;
    const OWNER_STATE: u32 = 0x12c;
    const ROOT_OWNER: u32 = 0x21c;
    const COMPLETE_BIT: u32 = 0x1000;
    const EFFECT_CONTEXT: u32 = 0x570;
    const FOLLOW_UP_CONTEXT: u32 = 0xbb0;
    const ALERT_EFFECT: u32 = 0x00ed_f594;
    const FOLLOW_UP_EVENT: u32 = 0x00ed_f5d4;
    const MARKED_EFFECT: u32 = 0x00ed_f5a8;
    const FALLBACK_EFFECT: u32 = 0x00ed_f5b8;
    const RECOVERY_EFFECT: u32 = 0x00ed_f5e0;
    const RECOVERY_EVENT: u32 = 0x00ed_f5ec;
    const TIMER_LIMIT: i32 = 0x3fff;

    for slot in 0..TASK_SLOT_COUNT {
        let manager = unsafe { read_u32(root + PED_INTELLIGENCE) };
        let task = unsafe { read_u32(manager + TASK_SLOTS + slot * 4) };
        if task == 0
            || unsafe { read_u8(task + TASK_ACTIVE) } == 0
            || unsafe { read_u32(task + TASK_FLAGS) } & COMPLETE_BIT != 0
        {
            continue;
        }

        let eligible = lf_checker_rt::callee_thiscall!(1, u32, manager, task) & 0xff != 0;
        if !eligible {
            let root_owner = unsafe { read_u32(root + ROOT_OWNER) };
            let root_state = unsafe { read_u32(root_owner + OWNER_STATE) };
            let task_owner = unsafe { read_u32(task + TASK_OWNER) };
            let task_state = unsafe { read_u32(task_owner + OWNER_STATE) };
            if root_state == 2 && (task_state == 0 || task_state == 1) {
                if start_effect(root, RECOVERY_EFFECT) {
                    queue_follow_up(root, task, RECOVERY_EVENT);
                }
                unsafe { mark_complete(task) };
            }
            continue;
        }

        let task_owner = unsafe { read_u32(task + TASK_OWNER) };
        let task_state = unsafe { read_u32(task_owner + OWNER_STATE) };
        let event_state = unsafe { read_u8(task + TASK_EVENT_STATE) };
        if task_state == 2 {
            if event_state == 0 {
                let timer = lf_checker_rt::callee_cdecl!(2, u32,);
                if (timer as i32) < TIMER_LIMIT {
                    if start_effect(root, ALERT_EFFECT) {
                        queue_follow_up(root, task, FOLLOW_UP_EVENT);
                        unsafe { mark_complete(task) };
                        continue;
                    }
                }
            }
            if start_effect(root, MARKED_EFFECT) {
                queue_follow_up(root, task, FOLLOW_UP_EVENT);
            }
            unsafe { mark_complete(task) };
            continue;
        }

        if event_state != 0 {
            unsafe { mark_complete(task) };
            continue;
        }
        if start_effect(root, FALLBACK_EFFECT) {
            queue_follow_up(root, task, FOLLOW_UP_EVENT);
        }
        unsafe { mark_complete(task) };
    }
}
