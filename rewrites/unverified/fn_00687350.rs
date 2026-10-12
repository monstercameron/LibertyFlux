// original: 0x00687350 locked_ref_release (proposed)

/// Release a lock object whose first word points to a vtable, whose second
/// word points to an optional semaphore record, and whose vtable stores the
/// mutex handle at `+0x10`. When the semaphore record is present, wait on the
/// mutex if its handle is nonzero, decrement the record count at `+8`, clear
/// the semaphore field, and release the mutex if its handle is nonzero. The
/// vtable field is cleared on every completing path. Import calls are
/// represented by scripted stand-ins; the function has no semantic return.
lf_checker_rt::export!(thiscall, rw_00687350(lock_object: u32) -> u32 {
    unsafe {
        const OBJECT_VTABLE: u32 = 0x00;
        const OBJECT_SEMAPHORE: u32 = 0x04;
        const VTABLE_MUTEX_HANDLE: u32 = 0x10;
        const SEMAPHORE_COUNT: u32 = 0x08;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let initial_semaphore = unsafe { read_u32(lock_object.wrapping_add(OBJECT_SEMAPHORE)) };
        if initial_semaphore == 0 {
            unsafe { write_u32(lock_object.wrapping_add(OBJECT_VTABLE), 0) };
            return 0;
        }

        let vtable = unsafe { read_u32(lock_object.wrapping_add(OBJECT_VTABLE)) };
        let wait_handle = unsafe { read_u32(vtable.wrapping_add(VTABLE_MUTEX_HANDLE)) };
        if wait_handle != 0 {
            let _ = unsafe { lf_checker_rt::callee_stdcall!(1, u32, wait_handle, u32::MAX) };
        }

        let semaphore = unsafe { read_u32(lock_object.wrapping_add(OBJECT_SEMAPHORE)) };
        if semaphore != 0 {
            let count_address = semaphore.wrapping_add(SEMAPHORE_COUNT);
            let count = unsafe { read_u32(count_address) };
            unsafe {
                write_u32(count_address, count.wrapping_sub(1));
                write_u32(lock_object.wrapping_add(OBJECT_SEMAPHORE), 0);
            }
        }

        let current_vtable = unsafe { read_u32(lock_object.wrapping_add(OBJECT_VTABLE)) };
        let release_handle = unsafe { read_u32(current_vtable.wrapping_add(VTABLE_MUTEX_HANDLE)) };
        let result = if release_handle != 0 {
            unsafe { lf_checker_rt::callee_stdcall!(2, u32, release_handle) }
        } else {
            0
        };
        unsafe { write_u32(lock_object.wrapping_add(OBJECT_VTABLE), 0) };
        result
    }
});
