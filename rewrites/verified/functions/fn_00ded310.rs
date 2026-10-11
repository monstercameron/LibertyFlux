// original: 0x00DED310 UILayoutManager::UILayoutManager

/// Construct a layout manager by forwarding its two 32-bit constructor
/// arguments to the intercepted UI frame constructor, then setting the
/// manager's vtable and three byte or word state locations. The object's
/// inherited frame state belongs to the callee; this routine returns `this`.
lf_checker_rt::export!(thiscall, rw_00ded310(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VTABLE_WORD: u32 = 0x00;
        const FLAG_C7: u32 = 0xc7;
        const FIELD_D0: u32 = 0xd0;
        const FIELD_1E0: u32 = 0x1e0;
        const MANAGER_VTABLE: u32 = 0x00f00064;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, arg0, arg1);
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(MANAGER_VTABLE));
        (this.wrapping_add(FIELD_D0) as *mut u8).write(1);
        (this.wrapping_add(FIELD_1E0) as *mut u8).write(0);
        (this.wrapping_add(FLAG_C7) as *mut u8).write(1);
        this
    }
});
