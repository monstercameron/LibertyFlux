// original: 0x00B6D080 ped_task_ctor_timer_and_vector
/// Initializes a task object, copies three dwords from arg1, reads one relocated global dword into
/// +0x30, stores arg0 at +0x1c and arg2 at +0x34, and clears four fields. The shared reference
/// helper runs after initialization when arg0 is non-null. Returns task.
lf_checker_rt::export!(thiscall, rw_00b6d080(task: u32, arg0_pointer: u32, arg1_words: u32, arg2_value: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB1A94;
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
        const FIELD_30: u32 = 0x30;
        const FIELD_34: u32 = 0x34;
        const FIELD_40: u32 = 0x40;
        const FIELD_44: u32 = 0x44;
        const FIELD_48: u32 = 0x48;
        const FIELD_50: u32 = 0x50;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task);
        unsafe { write_word(task.wrapping_add(FIELD_1C), arg0_pointer); }
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        unsafe { write_byte(task.wrapping_add(FIELD_14), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_18), 0); }
        let copy_20 = unsafe { read_word(arg1_words.wrapping_add(0x0)) };
        unsafe { write_word(task.wrapping_add(FIELD_20), copy_20); }
        let copy_24 = unsafe { read_word(arg1_words.wrapping_add(0x4)) };
        unsafe { write_word(task.wrapping_add(FIELD_24), copy_24); }
        let copy_28 = unsafe { read_word(arg1_words.wrapping_add(0x8)) };
        unsafe { write_word(task.wrapping_add(FIELD_28), copy_28); }
        const GLOBAL_VALUE_VA: u32 = 0x011735B4;
        let global_value = unsafe { read_word(lf_checker_rt::global::<u32>(GLOBAL_VALUE_VA) as u32) };
        unsafe { write_word(task.wrapping_add(FIELD_30), global_value); }
        unsafe { write_word(task.wrapping_add(FIELD_34), arg2_value); }
        unsafe { write_word(task.wrapping_add(FIELD_40), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_44), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_48), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_50), 0); }
        let reference_0 = unsafe { read_word(task.wrapping_add(FIELD_1C)) };
        if reference_0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_CALLEE, u32, reference_0, task.wrapping_add(FIELD_1C));
        }
        task
    }
});
