// original: 0x00e16b50 dispatch_stub
/// Gated dispatch 0x0032028a: runs the optional start-up hook, asks the query
/// routine whether the tag is live, then either forwards the tag to the
/// shared dispatcher or returns the default value from game state.
///
/// The hook pointer is called only when non-null (its result is ignored).
/// The query routine takes the tag and its zero/non-zero answer selects the
/// path; only the taken path's behaviour is observable per call.
export!(cdecl, rw_rs111_00e16b50() -> u32 {
    /// Game-state slot holding the optional hook pointer (0 = absent).
    const HOOK_SLOT: u32 = 0x0105_9514;
    /// Game-state slot holding the query routine address.
    const QUERY_SLOT: u32 = 0x0105_9518;
    /// Game-state slot holding the default return value.
    const DEFAULT_SLOT: u32 = 0x0105_951c;
    /// Game-state slot holding the dispatcher address.
    const DISPATCH_SLOT: u32 = 0x0105_9520;
    /// Tag this stub queries and forwards (carried in EAX at dispatch).
    const TAG: u32 = 0x0032028a;
    unsafe {
        let hook = *global::<u32>(HOOK_SLOT);
        if hook != 0 {
            let run_hook: extern "cdecl" fn() = core::mem::transmute(hook as usize);
            run_hook();
        }
        let query_at = *global::<u32>(QUERY_SLOT);
        let query: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(query_at as usize);
        if query(TAG) != 0 {
            let target = *global::<u32>(DISPATCH_SLOT);
            // As in the plain dispatch stubs, the tag travels to the checker
            // stub as a stack argument and is loaded into EAX there, so the
            // logged EAX comparison verifies the tag value itself.
            let dispatch: extern "cdecl" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            dispatch(TAG)
        } else {
            *global::<u32>(DEFAULT_SLOT)
        }
    }
});
