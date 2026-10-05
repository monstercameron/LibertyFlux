// original: 0x00a8a290 pool_dispatch_by_index (proposed)

/// Map a small index to a slot argument through a table, tail-calling the
/// slot setter on the live paths.
///
/// `this` is the pool object and `idx` the index: 0 and 12 forward 0, 3
/// forwards 1, 6 forwards 2, 9 forwards 3. Any other `idx` (including above
/// 12) returns `idx` unchanged and makes no call.
///
/// Original: 0x00A8A290 (thiscall, one stack word, tail-calls its callee).
lf_checker_rt::export!(thiscall, rw_00a8a290(this: u32, idx: u32) -> u32 {
    unsafe {
        const CALLEE_SET_SLOT: u32 = 1;
        let arg = match idx {
            0 | 12 => 0,
            3 => 1,
            6 => 2,
            9 => 3,
            _ => return idx,
        };
        lf_checker_rt::callee_thiscall!(CALLEE_SET_SLOT, u32, this, arg)
    }
});
