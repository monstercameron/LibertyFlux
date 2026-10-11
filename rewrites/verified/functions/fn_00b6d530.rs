// original: 0x00B6D530 ped_task_ctor_copy_vector_and_options
/// Initializes the vtable, copies three words from arg0 into +0x20..+0x28, stores a dword and
/// low-byte flag, clears fields +0x38, +0x50, +0x54 and +0x58, and returns task.
lf_checker_rt::export!(thiscall, rw_00b6d530(task: u32, arg0_words: u32, arg1_value: u32, arg2_flag: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB1F64;
        const INVALID_WORD: u32 = 0xFFFF_FFFF;
        const BASE_CALLEE: u32 = 1;
        const REFERENCE_CALLEE: u32 = 2;
        const INDIRECT_CALLEE: u32 = 3;
        const FIELD_VTABLE: u32 = 0x0;
        const FIELD_20: u32 = 0x20;
        const FIELD_24: u32 = 0x24;
        const FIELD_28: u32 = 0x28;
        const FIELD_30: u32 = 0x30;
        const FIELD_34: u32 = 0x34;
        const FIELD_38: u32 = 0x38;
        const FIELD_50: u32 = 0x50;
        const FIELD_54: u32 = 0x54;
        const FIELD_58: u32 = 0x58;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task);
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        let copy_20 = unsafe { read_word(arg0_words.wrapping_add(0x0)) };
        unsafe { write_word(task.wrapping_add(FIELD_20), copy_20); }
        let copy_24 = unsafe { read_word(arg0_words.wrapping_add(0x4)) };
        unsafe { write_word(task.wrapping_add(FIELD_24), copy_24); }
        let copy_28 = unsafe { read_word(arg0_words.wrapping_add(0x8)) };
        unsafe { write_word(task.wrapping_add(FIELD_28), copy_28); }
        unsafe { write_word(task.wrapping_add(FIELD_30), arg1_value); }
        unsafe { write_byte(task.wrapping_add(FIELD_34), arg2_flag as u8); }
        unsafe { write_word(task.wrapping_add(FIELD_38), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_50), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_54), 0); }
        unsafe { write_half(task.wrapping_add(FIELD_58), 0); }
        task
    }
});
