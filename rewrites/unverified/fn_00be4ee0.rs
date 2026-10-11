// original: 0x00BE4EE0 CTaskComplexControlMovement::vf21

/// Reuse an existing control-movement task when the context mode bit is set
/// and its suppression bit is clear. Otherwise allocate a task, run its base
/// initializer, install the task vtable, and clear its timer, flag, and state.
/// A failed allocation returns zero; the reuse path returns the delegated
/// helper result. The context pointer is the method's stack argument.
lf_checker_rt::export!(thiscall, rw_00be4ee0(_this: u32, context: u32) -> u32 {
    unsafe {
        const MODE_FLAGS: u32 = 0x29c;
        const CREATE_SUPPRESSION_FLAGS: u32 = 0x26c;
        const TASK_VTABLE: u32 = 0x00eb391c;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let reuse_existing = read_u8(context + MODE_FLAGS) & 4 != 0
            && read_u8(context + CREATE_SUPPRESSION_FLAGS) & 1 == 0;
        let manager_slot = lf_checker_rt::global::<u32>(0x0167e2a0);
        let manager = read_u32(manager_slot as u32);

        if reuse_existing {
            let existing_task = lf_checker_rt::callee_thiscall!(1, u32, manager);
            if existing_task == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(2, u32, existing_task);
        }

        let task = lf_checker_rt::callee_thiscall!(1, u32, manager);
        if task == 0 {
            return 0;
        }
        let _base_init = lf_checker_rt::callee_thiscall!(3, u32, task);
        write_u32(task, lf_checker_rt::relocated(TASK_VTABLE));
        write_u32(task + 0x14, 0);
        write_u8(task + 0x18, 0);
        write_u32(task + 0x1c, 0);
        task
    }
});
