// original: 0x00B7E600 CTaskComplexCarDrive::get_follow_on_task

/// Build the follow-on task for one event using the task's fixed event kind.
///
/// The incoming `this` pointer is forwarded in ECX to the event task builder.
/// The one stack argument is forwarded unchanged after the fixed event-kind
/// word. The builder's 32-bit result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00b7e600(this: u32, event: u32) -> u32 {
    const BUILD_CHILD_TASK_FROM_EVENT: u32 = 1;
    const EVENT_KIND: u32 = 0x516;

    unsafe {
        lf_checker_rt::callee_thiscall!(
            BUILD_CHILD_TASK_FROM_EVENT,
            u32,
            this,
            EVENT_KIND,
            event,
        )
    }
});
