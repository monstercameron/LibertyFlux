// original: 0x00a49700 vehicle_get_subpart_via_model
/// Resolve a subpart through a chain of three intercepted calls.
///
/// Calls the model-row helper (callee 1) with (`this`, `a`), then the
/// joiner (callee 2) with object `r1` and (`this`, `b`, 0), then the subpart
/// search (callee 3) with (`this`, `r2`, `this`): the original pushes only
/// `r2`, so the search's second stack word is whatever sits below it, which
/// is the caller's own saved `this` (thiscall, two stack words). Returns the
/// search's answer.
export!(thiscall, rw_00a49700(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        let r1: u32 = callee_thiscall!(1, u32, this, a);
        let r2: u32 = callee_thiscall!(2, u32, r1, this, b);
        callee_thiscall!(3, u32, this, r2, 0)
    }
});
