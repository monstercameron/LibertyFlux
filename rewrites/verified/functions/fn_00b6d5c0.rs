// original: 0x00B6D5C0 ped_task_ctor_nested_initializer
/// Calls a nested initializer with six fixed words (-1, -1, 8.0f bits, 0, 1.0f bits, 0), installs
/// its own vtable, stores arg0 at +0x34 and calls the reference helper when non-null. Returns task.
lf_checker_rt::export!(thiscall, rw_00b6d5c0(task: u32, arg0_pointer: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB206C;
        const INVALID_WORD: u32 = 0xFFFF_FFFF;
        const BASE_CALLEE: u32 = 1;
        const REFERENCE_CALLEE: u32 = 2;
        const INDIRECT_CALLEE: u32 = 3;
        const FIELD_VTABLE: u32 = 0x0;
        const FIELD_34: u32 = 0x34;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        const INITIAL_EIGHT_F32_BITS: u32 = 0x4100_0000;
        const INITIAL_ONE_F32_BITS: u32 = 0x3F80_0000;
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task, INVALID_WORD, INVALID_WORD, INITIAL_EIGHT_F32_BITS, 0, INITIAL_ONE_F32_BITS, 0);
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        unsafe { write_word(task.wrapping_add(FIELD_34), arg0_pointer); }
        let reference_0 = unsafe { read_word(task.wrapping_add(FIELD_34)) };
        if reference_0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_CALLEE, u32, reference_0, task.wrapping_add(FIELD_34));
        }
        task
    }
});
