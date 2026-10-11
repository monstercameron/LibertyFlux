// original: 0x00626DD0 rage::rlFireAndForgetTask<rage::snJoinCompleteTask>::vf0

/// Replace the first word of the task with its relocated vtable, release the optional
/// resource at +0x18, then invoke the current thread's TLS callback when flags bit 0 is set.
/// The callback chain begins at TLS slot 0, follows the object pointer at +8 and its vtable
/// at +0, and calls the entry at vtable +0x0C. Both calls receive the original task as their
/// stack argument; the resource pointer or TLS object is passed in ECX. The method returns
/// the task pointer in EAX. The proof fabricates the TLS object and callback table.
lf_checker_rt::export!(thiscall, rw_00626dd0(this: u32, flags: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00FE_4D60;
        const RESOURCE_POINTER: u32 = 0x18;
        const TLS_SLOT: usize = 0;
        const TLS_OBJECT_LINK: u32 = 8;
        const VTABLE_TARGET_SLOT: u32 = 12;
        const OPTIONAL_RELEASE_FLAG: u32 = 1;
        const RELEASE_RESOURCE: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        write_u32(this, lf_checker_rt::relocated(TASK_VTABLE));
        if resource != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE, u32, resource, this);
        }

        if flags & OPTIONAL_RELEASE_FLAG != 0 {
            let tls_object = lf_checker_rt::tls_slot(TLS_SLOT);
            let callback_object = read_u32(tls_object.wrapping_add(TLS_OBJECT_LINK));
            let callback_vtable = read_u32(callback_object);
            let callback_target = read_u32(callback_vtable.wrapping_add(VTABLE_TARGET_SLOT));
            let callback: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callback_target as usize);
            let _ = callback(callback_object, this);
        }
        this
    }
});
