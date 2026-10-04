// original: 0x00e04726 register_predicate_callback
/// Registers the 0x00E046E5 predicate as a callback and returns 0.
///
/// Pushes the (relocated) predicate address to the registrar callee
/// (stubbed by the checker); the registrar's answer is discarded.
export!(cdecl, rw_00e04726() -> u32 {
    unsafe {
        callee_cdecl!(1, u32, relocated(0x00E046E5));
        0
    }
});
