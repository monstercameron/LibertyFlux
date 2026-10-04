// original: 0x00e63a10 notify_block_b
/// Notify with block 0xE71570.
///
/// Trivial forwarder to the shared notifier (cdecl/1); returns its answer.
export!(cdecl, rw_00e63a10() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE71570)) }
});
