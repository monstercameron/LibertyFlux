// original: 0x00DDE0E0 UITextField::vf0 notify
/// Notify the UI system with the shared text-field tag object: call the
/// registered handler once, passing the tag's address. Takes no object and
/// no arguments; the handler cleans nothing (caller pops). Returns nothing.
lf_checker_rt::export!(cdecl, rw_00DDE0E0() -> u32 {
    unsafe {
        const TAG_OBJECT: u32 = 0x00EFD1F8;
        const HANDLER: u32 = 1;
        lf_checker_rt::callee_cdecl!(HANDLER, u32, lf_checker_rt::relocated(TAG_OBJECT));
        0
    }
});
