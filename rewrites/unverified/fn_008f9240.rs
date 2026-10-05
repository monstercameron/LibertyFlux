// original: 0x008F9240 stream_remove_by_id_both
/// Remove one id from both streaming pools.
///
/// Forwards the argument to the same removal routine on each of the
/// two global pool objects in turn. Cdecl, one stack argument;
/// returns the second call's result.
export!(cdecl, rw_008f9240(arg: u32) -> u32 {
    unsafe {
        const POOL_A: u32 = 0x118e7c0;
        const POOL_B: u32 = 0x118e7c8;
        callee_thiscall!(1, u32, relocated(POOL_A), arg);
        callee_thiscall!(2, u32, relocated(POOL_B), arg)
    }
});
