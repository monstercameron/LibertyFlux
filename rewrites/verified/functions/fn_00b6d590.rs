// original: 0x00B6D590 ped_task_ctor_owned_reference
/// Initializes the vtable, stores arg0 at +0x14 and calls the reference helper when it is non-null.
/// Returns task.
lf_checker_rt::export!(thiscall, rw_00b6d590(task: u32, arg0_pointer: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB1F0C;
        const INVALID_WORD: u32 = 0xFFFF_FFFF;
        const BASE_CALLEE: u32 = 1;
        const REFERENCE_CALLEE: u32 = 2;
        const INDIRECT_CALLEE: u32 = 3;
        const FIELD_VTABLE: u32 = 0x0;
        const FIELD_14: u32 = 0x14;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task);
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        unsafe { write_word(task.wrapping_add(FIELD_14), arg0_pointer); }
        let reference_0 = unsafe { read_word(task.wrapping_add(FIELD_14)) };
        if reference_0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_CALLEE, u32, reference_0, task.wrapping_add(FIELD_14));
        }
        task
    }
});
