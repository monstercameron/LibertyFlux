// original: 0x00B6D3F0 NativeImpl_REMOVE_CHAR_FROM_CAR_MAINTAIN_POSITION_2
/// Stores a conditional reference pointer and dword with two low-byte flags. The zero dword at
/// +0x1d clears four overlapping bytes before arg3 is stored at +0x21; the reference helper runs
/// for a non-null pointer. The merged symbol maps this to a native name at low confidence, while
/// native analysis proposes an entity-placement helper, also at low confidence. Returns task.
lf_checker_rt::export!(thiscall, rw_00b6d3f0(task: u32, arg0_pointer: u32, arg1_value: u32, arg2_flag: u32, arg3_flag: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB1E04;
        const INVALID_WORD: u32 = 0xFFFF_FFFF;
        const BASE_CALLEE: u32 = 1;
        const REFERENCE_CALLEE: u32 = 2;
        const INDIRECT_CALLEE: u32 = 3;
        const FIELD_VTABLE: u32 = 0x0;
        const FIELD_14: u32 = 0x14;
        const FIELD_18: u32 = 0x18;
        const FIELD_1C: u32 = 0x1C;
        const FIELD_1D: u32 = 0x1D;
        const FIELD_21: u32 = 0x21;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task);
        unsafe { write_word(task.wrapping_add(FIELD_18), arg1_value); }
        unsafe { write_byte(task.wrapping_add(FIELD_1C), arg2_flag as u8); }
        unsafe { write_word(task.wrapping_add(FIELD_1D), 0); }
        unsafe { write_byte(task.wrapping_add(FIELD_21), arg3_flag as u8); }
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        unsafe { write_word(task.wrapping_add(FIELD_14), arg0_pointer); }
        let reference_0 = unsafe { read_word(task.wrapping_add(FIELD_14)) };
        if reference_0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_CALLEE, u32, reference_0, task.wrapping_add(FIELD_14));
        }
        task
    }
});
