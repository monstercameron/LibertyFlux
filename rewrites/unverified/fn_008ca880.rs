// original: 0x008CA880 stream_load_maybe_chain (proposed)

/// Loads the streaming lists for `mode` through the list callee (callee 0,
/// cdecl with the mode), then, when `mode` is 1, tail-jumps to a second
/// loader routine instead of returning.
///
/// NOTE (unverified): the mode==1 path is a conditional tail jump to another
/// game function, which the checker cannot intercept (only E8 call sites and
/// unconditional E9 tail jumps are patchable), so no contract can prove this
/// rewrite. The mode!=1 fallthrough is a plain call plus return.
///
/// One stack argument (cdecl); no result on the fallthrough path.
lf_checker_rt::export!(cdecl, rw_008CA880(mode: u32) -> u32 {
    unsafe {
        /// List callee id.
        const LOAD: u32 = 0;
        let _l: u32 = lf_checker_rt::callee_cdecl!(LOAD, u32, mode);
        if mode == 1 {
            // Original tail-jumps to the chained loader here; there is no
            // checker transport for a conditional tail jump, so this path
            // is deliberately left unimplemented.
            unimplemented!();
        }
        0
    }
});
