// original: 0x00E46590 UIScreen::UIScreen_2

/// Install the screen vtable, clear its one-byte initialization flag, and
/// transfer the same object to the shared UI child-cleanup helper. The method
/// is a thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00e46590(this: u32) -> u32 {
    const SCREEN_VTABLE: u32 = 0x00F1_5ECC;
    const INITIALIZED_FLAG_OFFSET: u32 = 0xC7;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(SCREEN_VTABLE));
        (this.wrapping_add(INITIALIZED_FLAG_OFFSET) as *mut u8).write(0);
    }

    lf_checker_rt::callee_thiscall!(1, u32, this)
});
