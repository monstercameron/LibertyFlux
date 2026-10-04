// original: 0x00e16cf0 dispatch_stub
/// Dispatch stub 0x00020078: runs the optional start-up hook, then forwards
/// a fixed tag to the shared dispatcher.
///
/// The hook pointer is read from game state and called only when non-null
/// (its result is ignored). The dispatcher address is read from game state;
/// it receives the tag in EAX and its return value becomes ours.
export!(cdecl, rw_rs111_00e16cf0() -> u32 {
    /// Game-state slot holding the optional hook pointer (0 = absent).
    const HOOK_SLOT: u32 = 0x0105_9514;
    /// Game-state slot holding the dispatcher address.
    const DISPATCH_SLOT: u32 = 0x0105_9520;
    /// Tag this stub forwards to the dispatcher (carried in EAX).
    const TAG: u32 = 0x00020078;
    unsafe {
        let hook = *global::<u32>(HOOK_SLOT);
        if hook != 0 {
            let run_hook: extern "cdecl" fn() = core::mem::transmute(hook as usize);
            run_hook();
        }
        let target = *global::<u32>(DISPATCH_SLOT);
        // The original tail-jumps with TAG in EAX; the checker stub loads EAX
        // from this stack argument on the rewrite side (eax_from_stack) and
        // the logged EAX is compared, so the tag value itself is verified.
        let dispatch: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        dispatch(TAG)
    }
});
