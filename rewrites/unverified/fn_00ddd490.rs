// original: 0x00DDD490 uitextfield_reset_for_close
/// Reset a text-field object to the default style, then run the shared
/// close routine it tail-calls into.
///
/// Writes the default style pointer into the object's head and clears the
/// "has custom style" byte, then forwards `this` to the common teardown
/// helper and returns its result.
lf_checker_rt::export!(thiscall, rw_ddd490(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_checker_rt::relocated(0x00EFD00C);
        *this.add(0xC7) = 0;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32)
    }
});
