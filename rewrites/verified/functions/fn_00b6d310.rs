// original: 0x00B6D310 ped_task_ctor_reference_values_and_flag_alt
/// Initializes base fields, stores a conditional reference pointer, a dword, low-byte flag, cleared
/// byte and two further dwords. The reference helper runs for a non-null pointer. Returns task.
lf_checker_rt::export!(thiscall, rw_00b6d310(task: u32, arg0_pointer: u32, arg1_value: u32, arg2_flag: u32, arg3_value: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB1AEC;
        const INVALID_WORD: u32 = 0xFFFF_FFFF;
        const BASE_CALLEE: u32 = 1;
        const REFERENCE_CALLEE: u32 = 2;
        const INDIRECT_CALLEE: u32 = 3;
        const FIELD_VTABLE: u32 = 0x0;
        const FIELD_14: u32 = 0x14;
        const FIELD_18: u32 = 0x18;
        const FIELD_1C: u32 = 0x1C;
        const FIELD_20: u32 = 0x20;
        const FIELD_24: u32 = 0x24;
        const FIELD_28: u32 = 0x28;
        const FIELD_2C: u32 = 0x2C;
        const FIELD_2D: u32 = 0x2D;
        const FIELD_30: u32 = 0x30;
        const FIELD_34: u32 = 0x34;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task);
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        unsafe { write_byte(task.wrapping_add(FIELD_14), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_18), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_1C), INVALID_WORD); }
        unsafe { write_word(task.wrapping_add(FIELD_20), INVALID_WORD); }
        unsafe { write_word(task.wrapping_add(FIELD_24), arg0_pointer); }
        unsafe { write_word(task.wrapping_add(FIELD_28), arg1_value); }
        unsafe { write_byte(task.wrapping_add(FIELD_2C), arg2_flag as u8); }
        unsafe { write_byte(task.wrapping_add(FIELD_2D), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_30), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_34), arg3_value); }
        let reference_0 = unsafe { read_word(task.wrapping_add(FIELD_24)) };
        if reference_0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_CALLEE, u32, reference_0, task.wrapping_add(FIELD_24));
        }
        task
    }
});
