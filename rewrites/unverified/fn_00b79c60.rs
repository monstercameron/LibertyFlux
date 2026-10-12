// original: 0x00B79C60 CTaskComplexShuffleBetweenSeats::CTaskComplexShuffleBetweenSeats
/// Construct a shuffle-between-seats task around an optional nested task.
///
/// `this` is the task object and the single stack word is its nullable nested
/// task pointer. The constructor writes the relocated task vtable and stores
/// the pointer at offset `0x14`. A non-null task is attached through the
/// address of that slot; null skips the helper. The original `this` value is
/// returned.
lf_checker_rt::export!(thiscall, rw_00b79c60(this: u32, nested_task: u32) -> u32 {
    const TASK_VTABLE: u32 = 0x00EB3764;
    const NESTED_TASK: u32 = 0x14;
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
        let nested = read_u32(this.wrapping_add(NESTED_TASK));
        if nested != 0 {
            let slot = this.wrapping_add(NESTED_TASK);
            let _ = lf_checker_rt::callee_thiscall!(ATTACH_NESTED_TASK, u32, nested, slot);
        }
        this
    }
});
