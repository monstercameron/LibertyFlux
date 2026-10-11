// original: 0x00B6D210 ped_task_ctor_reference_and_flags
/// Initializes the base word and adjacent flag bytes, stores a pointer at +0x20 and a dword at
/// +0x24, then calls the reference helper for a non-null pointer. The flag arguments are consumed
/// as low bytes. Returns task.
lf_checker_rt::export!(thiscall, rw_00b6d210(task: u32, arg0_pointer: u32, arg1_value: u32, arg2_flag: u32, arg3_flag: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB1D54;
        const INVALID_WORD: u32 = 0xFFFF_FFFF;
        const BASE_CALLEE: u32 = 1;
        const REFERENCE_CALLEE: u32 = 2;
        const INDIRECT_CALLEE: u32 = 3;
        const FIELD_VTABLE: u32 = 0x0;
        const FIELD_14: u32 = 0x14;
        const FIELD_18: u32 = 0x18;
        const FIELD_1C: u32 = 0x1C;
        const FIELD_1D: u32 = 0x1D;
        const FIELD_1E: u32 = 0x1E;
        const FIELD_1F: u32 = 0x1F;
        const FIELD_20: u32 = 0x20;
        const FIELD_24: u32 = 0x24;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task);
        unsafe { write_byte(task.wrapping_add(FIELD_1D), arg2_flag as u8); }
        unsafe { write_byte(task.wrapping_add(FIELD_1E), arg3_flag as u8); }
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        unsafe { write_half(task.wrapping_add(FIELD_14), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_18), 0); }
        unsafe { write_byte(task.wrapping_add(FIELD_1C), 0); }
        unsafe { write_byte(task.wrapping_add(FIELD_1F), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_20), arg0_pointer); }
        unsafe { write_word(task.wrapping_add(FIELD_24), arg1_value); }
        let reference_0 = unsafe { read_word(task.wrapping_add(FIELD_20)) };
        if reference_0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_CALLEE, u32, reference_0, task.wrapping_add(FIELD_20));
        }
        task
    }
});
