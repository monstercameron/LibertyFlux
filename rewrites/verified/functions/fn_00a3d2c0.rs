// original: 0x00A3D2C0 vehicle_release_owned_references

/// Release each non-null object pointer stored in the receiver's first two
/// words, in order, then clear the corresponding word. Each release helper is
/// thiscall with the object pointer as `this` and the address of its owner slot
/// as one stack argument. Null slots are left untouched and do not call the
/// helper. The original method is thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3d2c0(this: u32) -> () {
    const FIRST_SLOT: u32 = 0;
    const SECOND_SLOT: u32 = 4;
    const RELEASE_FIRST: u32 = 1;
    const RELEASE_SECOND: u32 = 2;

    #[inline(always)]
    unsafe fn read_pointer(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn clear_slot(address: u32) {
        unsafe { (address as *mut u32).write_unaligned(0) }
    }

    unsafe {
        let first_slot = this.wrapping_add(FIRST_SLOT);
        let first_object = read_pointer(first_slot);
        if first_object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_FIRST, u32, first_object, first_slot);
            clear_slot(first_slot);
        }

        let second_slot = this.wrapping_add(SECOND_SLOT);
        let second_object = read_pointer(second_slot);
        if second_object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_SECOND, u32, second_object, second_slot);
            clear_slot(second_slot);
        }
    }
});
