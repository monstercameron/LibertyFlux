// original: 0x00e63c00 notify_block_c
/// Notify with block 0xE715D0.
///
/// Trivial forwarder to the shared notifier (cdecl/1); returns its answer.
export!(cdecl, rw_00e63c00() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE715D0)) }
});
