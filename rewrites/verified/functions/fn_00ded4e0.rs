// original: 0x00DED4E0 UIButton::UIButton

/// Construct a button by forwarding its two constructor arguments to the
/// intercepted UI frame constructor. Then install the button vtable, clear
/// the two 32-bit state words at offsets `0x1e0` and `0x1e4`, and initialise
/// the structure's pointer field at `0x1e8` and following word to all ones.
/// The constructor returns its object pointer.
lf_checker_rt::export!(thiscall, rw_00ded4e0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VTABLE_WORD: u32 = 0x00;
        const FIELD_1E0: u32 = 0x1e0;
        const FIELD_1E4: u32 = 0x1e4;
        const FIELD_1E8: u32 = 0x1e8;
        const FIELD_1EC: u32 = 0x1ec;
        const BUTTON_VTABLE: u32 = 0x00f003cc;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, arg0, arg1);
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(BUTTON_VTABLE));
        (this.wrapping_add(FIELD_1E8) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(FIELD_1EC) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(FIELD_1E0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(FIELD_1E4) as *mut u32).write_unaligned(0);
        this
    }
});
