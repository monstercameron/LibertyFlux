// original: 0x00cf75d0 climb_task_simple_ctor (proposed)

/// Constructs a simple climb task: runs the shared base constructor, stores
/// the task parameter at `+0x14`, stamps the vtable and returns the object.
///
/// Original: 0x00cf75d0 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf75d0(this: u32, param: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00edf38c;
        const PARAM_SLOT: u32 = 0x14;
        const BASE_CTOR: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this + PARAM_SLOT) as *mut u32).write_unaligned(param);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
