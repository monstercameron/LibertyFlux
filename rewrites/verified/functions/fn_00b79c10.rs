// original: 0x00B79C10 CTaskComplexScreamInCarThenLeave::CTaskComplexScreamInCarThenLeave
/// Construct a scream-in-car task and initialize its nested-task state.
///
/// `this` is the task object; the two stack words supply a nullable nested
/// task and a payload value. The constructor writes the relocated task
/// vtable, copies those words to offsets `0x14` and `0x18`, clears the state
/// fields at `0x1c`, `0x20`, and `0x24`, and calls the attach helper only for
/// a non-null nested task. It returns the original `this` pointer.
lf_checker_rt::export!(thiscall, rw_00b79c10(this: u32, nested_task: u32, task_payload: u32) -> u32 {
    const TASK_VTABLE: u32 = 0x00EB32DC;
    const NESTED_TASK: u32 = 0x14;
    const TASK_PAYLOAD: u32 = 0x18;
    const SEQUENCE_STATE: u32 = 0x1C;
    const BLEND_STATE: u32 = 0x20;
    const TASK_FLAGS: u32 = 0x24;
    const BASE_CONSTRUCTOR: u32 = 1;
    const ATTACH_NESTED_TASK: u32 = 2;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
        write32(this, lf_checker_rt::relocated(TASK_VTABLE));
        write32(this.wrapping_add(NESTED_TASK), nested_task);
        write32(this.wrapping_add(TASK_PAYLOAD), task_payload);
        write32(this.wrapping_add(SEQUENCE_STATE), 0);
        write32(this.wrapping_add(BLEND_STATE), 0);
        write16(this.wrapping_add(TASK_FLAGS), 0);
        let nested = read_u32(this.wrapping_add(NESTED_TASK));
        if nested != 0 {
            let slot = this.wrapping_add(NESTED_TASK);
            let _ = lf_checker_rt::callee_thiscall!(ATTACH_NESTED_TASK, u32, nested, slot);
        }
        this
    }
});
