// original: 0x00dc85a0 task_ctor_empty_flags
/// Initializes the empty task shell after invoking the base constructor.
/// The object receives its class vtable and zeroed state fields at offsets
/// `+0x14` (one byte) and `+0x18` (one word). The thiscall routine takes no
/// stack arguments and returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00dc85a0(this_ptr: u32) -> u32 {
    unsafe {
        const CLASS_VTABLE_VA: u32 = 0x00ef4d14;
        const BASE_CTOR: u32 = 1;
        let _base_result = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this_ptr);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(CLASS_VTABLE_VA));
        (this_ptr.wrapping_add(0x14) as *mut u8).write_unaligned(0);
        (this_ptr.wrapping_add(0x18) as *mut u32).write_unaligned(0);
        this_ptr
    }
});
