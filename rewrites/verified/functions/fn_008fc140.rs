// original: 0x008FC140 stream_pools_teardown
/// Tear down both streaming pools.
///
/// Calls the pool teardown routine on the first global pool, then
/// tail-calls it on the second pool. Cdecl, no arguments; returns the
/// tail call's result.
export!(cdecl, rw_008fc140() -> u32 {
    unsafe {
        const POOL_A: u32 = 0x118e7c0;
        const POOL_B: u32 = 0x118e7c8;
        callee_thiscall!(1, u32, relocated(POOL_A));
        callee_thiscall!(2, u32, relocated(POOL_B))
    }
});
