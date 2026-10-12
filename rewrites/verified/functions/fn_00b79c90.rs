// original: 0x00B79C90 CTaskComplexStealCar::CTaskComplexStealCar
/// Construct a steal-car task and reset its timer and state fields.
///
/// `this` is the task object and the single stack word is an optional nested
/// task pointer stored at offset `0x14`. The constructor writes the relocated
/// task vtable, clears the timer, state, flag, and sequence fields, attaches
/// the nested task only when non-null, and returns the original `this`.
lf_checker_rt::export!(thiscall, rw_00b79c90(this: u32, nested_task: u32) -> u32 {
    const TASK_VTABLE: u32 = 0x00EB31D4;
    const NESTED_TASK: u32 = 0x14;
    const TASK_TIMER: u32 = 0x18;
    const TASK_STATE: u32 = 0x1C;
    const TASK_FLAGS: u32 = 0x20;
    const SEQUENCE_STATE: u32 = 0x24;
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
        write32(this.wrapping_add(TASK_TIMER), 0);
        write32(this.wrapping_add(TASK_STATE), 0);
        write16(this.wrapping_add(TASK_FLAGS), 0);
        write32(this.wrapping_add(SEQUENCE_STATE), 0);
        let nested = read_u32(this.wrapping_add(NESTED_TASK));
        if nested != 0 {
            let slot = this.wrapping_add(NESTED_TASK);
            let _ = lf_checker_rt::callee_thiscall!(ATTACH_NESTED_TASK, u32, nested, slot);
        }
        this
    }
});
