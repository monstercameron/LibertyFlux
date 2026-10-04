// original: 0x009538f0 shared_handle_dispatch
/// Forward the shared handle to the registry dispatcher (tail call).
export!(cdecl, rw_009538f0() -> u32 {
    unsafe {
        let this = *global::<u32>(0x11F6954);
        callee_thiscall!(0, u32, this)
    }
});
