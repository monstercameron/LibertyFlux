// original: 0x00DF3380 UITransitionViewer::~UITransitionViewer__deleting

/// Run the transition viewer destructor, then conditionally request object
/// deletion when the low bit of the deleting-destructor flag is set. The
/// destructor always runs first; the one-argument deletion call is cdecl and
/// occurs only on the set-bit path. The wrapper returns the original object
/// pointer and lets the caller clean up its single stack argument.
lf_checker_rt::export!(thiscall, rw_00df3380(this: u32, deleting_flag: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if deleting_flag & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
