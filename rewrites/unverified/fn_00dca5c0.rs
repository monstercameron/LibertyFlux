// original: 0x00DCA5C0 CTaskComplexCarReactToVehicleCollisionGetOut::~CTaskComplexCarReactToVehicleCollisionGetOut

/// Destroys the get-out collision-reaction task.
///
/// `this` is supplied in ECX and there are no stack arguments. The destructor
/// changes the object's vtable to the get-out collision-reaction vtable, then
/// visits the pointer slots at `+0x18`, `+0x1c`, and `+0x20` in that order.
/// Each non-null slot is passed with its current value to the timing-node
/// detach callee. It then tail-calls the common task destructor and returns
/// that callee's EAX value unchanged.
lf_checker_rt::export!(thiscall, rw_00dca5c0(this: u32) -> u32 {
    unsafe {
        const DETACH_TIMING_NODE: u32 = 1;
        const COMMON_TASK_DESTRUCTOR: u32 = 2;
        const COLLISION_TASK_VTABLE_FILE_VA: u32 = 0x00EF5194;
        const REFERENCE_SLOTS: [u32; 3] = [0x18, 0x1c, 0x20];

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        write_u32(this, lf_checker_rt::relocated(COLLISION_TASK_VTABLE_FILE_VA));
        for offset in REFERENCE_SLOTS {
            let slot = this.wrapping_add(offset);
            let reference = read_u32(slot);
            if reference != 0 {
                let _ = lf_checker_rt::callee_thiscall!(DETACH_TIMING_NODE, u32, reference, slot);
            }
        }

        lf_checker_rt::callee_thiscall!(COMMON_TASK_DESTRUCTOR, u32, this)
    }
});
