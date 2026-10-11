// original: 0x00B80890 CEventNewTask::CEventNewTask

/// Construct a new-task event. First invoke the `CEvent` base constructor,
/// then install this event's vtable and store the three stack arguments at
/// offsets `+0x10`, `+0x14`, and the low byte at `+0x0c`. Return `this`.
lf_checker_rt::export!(thiscall, rw_b80890(this: u32, task_type: u32, task_data: u32, flags: u32) -> u32 {
    unsafe {
        const EVENT_VTABLE: u32 = 0x00EB3ACC;
        const FIELD_TYPE: u32 = 0x10;
        const FIELD_DATA: u32 = 0x14;
        const FIELD_FLAGS: u32 = 0x0c;
        const BASE_CTOR: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this + FIELD_FLAGS) as *mut u8).write(flags as u8);
        ((this + FIELD_TYPE) as *mut u32).write_unaligned(task_type);
        ((this + FIELD_DATA) as *mut u32).write_unaligned(task_data);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        this
    }
});
