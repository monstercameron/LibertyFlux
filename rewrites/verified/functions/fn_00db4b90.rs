// original: 0x00DB4B90 UIMouseCursor::vf0

/// Slot 0 of the mouse-cursor virtual table: resolve a shared name.
///
/// The function passes one constant (the file address of a name or table
/// entry in the original's data) to a one-argument helper and returns the
/// helper's answer untouched. The helper runs scripted on both sides, so
/// only the call itself is proven here.
///
/// Original: 0x00DB4B90 (cdecl, no stack arguments).
lf_checker_rt::export!(cdecl, rw_00db4b90() -> u32 {
    unsafe {
        const NAME_REF: u32 = 0x00ef2bb0;
        const HELPER: u32 = 1;
        lf_checker_rt::callee_cdecl!(HELPER, u32, lf_checker_rt::relocated(NAME_REF))
    }
});
