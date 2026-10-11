// original: 0x00664B30 network_task_state_initialize (proposed)

/// Initializes the state fields of a task-like object after a helper call.
///
/// The helper receives the object in ECX and no stack arguments. The method
/// then stores its relocated vtable pointer at `+0`, clears dwords at `+0x60`,
/// `+0x64`, `+0x68`, and `+0x90` through `+0x9c`, sets bit zero in the byte at
/// `+0x6c` while preserving its other bits, and clears the byte at `+0x6d`.
/// It returns the original object pointer in EAX. The calling convention is
/// thiscall with no explicit arguments.
lf_checker_rt::export!(thiscall, rw_00664b30(this: u32) -> u32 {
    unsafe {
        const SUBOBJECT_CALL: u32 = 1;
        const VTABLE_FILE_VA: u32 = 0x00fe_33e0;

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(SUBOBJECT_CALL, u32, this);
        unsafe { write_word(this, lf_checker_rt::relocated(VTABLE_FILE_VA)) };
        unsafe { write_word(this.wrapping_add(0x60), 0) };
        unsafe { write_word(this.wrapping_add(0x64), 0) };
        unsafe { write_word(this.wrapping_add(0x68), 0) };
        unsafe { *((this.wrapping_add(0x6c)) as *mut u8) |= 1 };
        unsafe { *((this.wrapping_add(0x6d)) as *mut u8) = 0 };
        unsafe { write_word(this.wrapping_add(0x90), 0) };
        unsafe { write_word(this.wrapping_add(0x94), 0) };
        unsafe { write_word(this.wrapping_add(0x98), 0) };
        unsafe { write_word(this.wrapping_add(0x9c), 0) };
        this
    }
});
