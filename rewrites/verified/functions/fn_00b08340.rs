// original: 0x00b08340 refresh_then_clear_flag
/// Run the shared refresh step, then clear the pending flag.
///
/// Calls the module refresh helper and clears the same pending byte its
/// sibling clears, returning the refresh result.
export!(cdecl, rw_00b08340() -> u32 {
    unsafe {
        const PENDING_FLAG: u32 = 0x012B_D193;
        let out: u32 = callee_cdecl!(1, u32,);
        *global::<u8>(PENDING_FLAG) = 0;
        out
    }
});
