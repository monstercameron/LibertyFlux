// original: 0x005E0D30 CTaskComplexCopHelicopter::CTaskComplexCopHelicopter_2

/// Allocate a helicopter cop task from the global task pool. If allocation
/// succeeds, run the base complex-task constructor, install the derived
/// vtable and clear the two child-task fields at +0x14 and +0x18. Return the
/// allocated object, or null when the allocator returns null. The incoming
/// source object is unused by this factory-like virtual method.
lf_checker_rt::export!(thiscall, rw_005e0d30(_source: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const VTABLE: u32 = 0x00fe_0f9c;
        const FIRST_TASK: u32 = 0x14;
        const SECOND_TASK: u32 = 0x18;
        const TASK_POOL_ALLOC: u32 = 1;
        const BASE_CONSTRUCTOR: u32 = 2;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let pool_manager = *lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
        let object = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, pool_manager);
        if object == 0 {
            return 0;
        }

        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, object);
        write_u32(object, lf_checker_rt::relocated(VTABLE));
        write_u32(object.wrapping_add(FIRST_TASK), 0);
        write_u32(object.wrapping_add(SECOND_TASK), 0);
        object
    }
});
