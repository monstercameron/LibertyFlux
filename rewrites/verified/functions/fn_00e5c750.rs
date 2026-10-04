// original: 0x00e5c750 timing_notify_e6e420
/// Forwards one registered handler address to the central dispatcher.
///
/// Pushes the handler slot address and calls the dispatcher (cdecl/1),
/// returning the dispatcher's answer unchanged.
export!(cdecl, rw_00e5c750() -> u32 {
    unsafe {
        const HANDLER: u32 = 0x00E6E420;
        callee_cdecl!(1, u32, relocated(HANDLER))
    }
});

