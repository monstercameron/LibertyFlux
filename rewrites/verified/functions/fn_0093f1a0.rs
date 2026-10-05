// original: 0x0093f1a0 stream_selected_if_state (proposed)

/// Return the selected object only when its state word equals 3.
///
/// Calls the flagged-word getter with a zero argument (ignored by the
/// callee); a null result returns null. Otherwise reads the state word at
/// offset `OBJ_STATE`: returns the object when it equals `WANTED_STATE`,
/// null otherwise.
///
/// Original: 0x0093f1a0 (cdecl, no arguments; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093f1a0() -> u32 {
    const FLAGGED_GETTER: u32 = 1;
    const OBJ_STATE: u32 = 0x1300;
    const WANTED_STATE: u32 = 3;
    unsafe {
        let obj: u32 = lf_checker_rt::callee_cdecl!(FLAGGED_GETTER, u32, 0u32);
        if obj == 0 {
            return 0;
        }
        let state = ((obj + OBJ_STATE) as *const u32).read_unaligned();
        if state == WANTED_STATE {
            obj
        } else {
            0
        }
    }
});
