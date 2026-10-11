// original: 0x00A3D500 vehicle_store_and_advance_slot

/// Store `value` through the slot selected by the receiver's counter at
/// `+0xfa8`. Slot records have a `0x14` byte stride; the external update helper
/// receives the record pointer as `this` and performs the field write. Then
/// increment the counter and return the next record pointer using the
/// original layout formula. Counter and pointer arithmetic intentionally wrap
/// at 32 bits because they describe the game's 32-bit object layout. The
/// original method is thiscall with one stack argument, popped by the method.
lf_checker_rt::export!(thiscall, rw_00a3d500(this: u32, value: u32) -> u32 {
    const SLOT_COUNTER: u32 = 0x0fa8;
    const SLOT_STRIDE: u32 = 0x14;
    const CURRENT_SLOT_BASE: u32 = 0x0fac;
    const NEXT_SLOT_BASE: u32 = 0x0f98;
    const UPDATE_HELPER: u32 = 1;

    unsafe {
        let counter_address = this.wrapping_add(SLOT_COUNTER);
        let current_index = (counter_address as *const u32).read_unaligned();
        let current_slot = this
            .wrapping_add(CURRENT_SLOT_BASE)
            .wrapping_add(current_index.wrapping_mul(SLOT_STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(UPDATE_HELPER, u32, current_slot, value);
        let next_index = current_index.wrapping_add(1);
        (counter_address as *mut u32).write_unaligned(next_index);
        this.wrapping_add(NEXT_SLOT_BASE)
            .wrapping_add(next_index.wrapping_mul(SLOT_STRIDE))
    }
});
