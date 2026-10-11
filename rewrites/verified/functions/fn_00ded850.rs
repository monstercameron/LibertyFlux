// original: 0x00DED850 UIMontageEditorControls::UIMontageEditorControls

/// Construct montage editor controls by forwarding both 32-bit constructor
/// arguments to the intercepted UI frame constructor. Install the controls
/// vtable, clear the eight consecutive 32-bit state words beginning at
/// offset `0x1e4`, and return the object pointer.
lf_checker_rt::export!(thiscall, rw_00ded850(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VTABLE_WORD: u32 = 0x00;
        const FIRST_STATE_WORD: u32 = 0x1e4;
        const STATE_WORDS: u32 = 8;
        const CONTROLS_VTABLE: u32 = 0x00f00884;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, arg0, arg1);
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(CONTROLS_VTABLE));
        for word in 0..STATE_WORDS {
            (this.wrapping_add(FIRST_STATE_WORD + word * 4) as *mut u32)
                .write_unaligned(0);
        }
        this
    }
});
