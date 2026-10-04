// original: 0x00e63910 notify_block_a
/// Notify with block 0xE71510.
///
/// Trivial forwarder: pushes one constant block pointer and calls the shared
/// notifier (cdecl/1, stubbed by the checker), returning its answer.
export!(cdecl, rw_00e63910() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE71510)) }
});
