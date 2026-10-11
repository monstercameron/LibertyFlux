// original: 0x00DED530 UIButton::UIButton_2

/// Reset the button vtable, run its intercepted component teardown helper,
/// then transfer to the intercepted base cleanup routine. This body has no
/// stack arguments and no declared return value.
lf_checker_rt::export!(thiscall, rw_00ded530(this: u32) -> u32 {
    unsafe {
        const VTABLE_WORD: u32 = 0x00;
        const BUTTON_VTABLE: u32 = 0x00f003cc;
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(BUTTON_VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
        0
    }
});
