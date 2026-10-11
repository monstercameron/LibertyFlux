// original: 0x00885220 streaming_forward_if_non_null

/// Forward a non-null identifier to the streaming registration routine and
/// return its result; a null identifier returns zero without a call. The
/// original is cdecl with one 32-bit stack argument and uses a tail call for
/// the non-null case.
lf_checker_rt::export!(cdecl, rw_00885220(identifier: u32) -> u32 {
    if identifier == 0 {
        0
    } else {
        lf_checker_rt::callee_cdecl!(1, u32, identifier)
    }
});
