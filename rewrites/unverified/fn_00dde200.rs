// original: 0x00DDE200 set_cursor_color_flag
/// Set or clear the cursor-colour override on the render state at +0x1E4.
/// A nonzero low byte forces the colour's top byte to 0xFF (opaque);
/// zero clears the override byte. Returns the render state pointer.
lf_checker_rt::export!(thiscall, rw_dde200(this: u32, flag: u32) -> u32 {
    unsafe {
        let state = *((this + 0x1E4) as *const u32);
        if flag & 0xFF != 0 {
            let colour = (state + 0x1E0) as *mut u32;
            *colour |= 0xFF000000;
        } else {
            *((state + 0x1E3) as *mut u8) = 0;
        }
        state
    }
});
