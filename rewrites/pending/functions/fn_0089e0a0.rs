// original: 0x0089e0a0 fwd_89dee0
/// Forwarding wrapper cdecl/3 -> thiscall/2 (object in first slot).
export!(cdecl, rw_0089e0a0(obj: u32, a: u32, b: u32) -> u32 {
    unsafe {
        
        callee_thiscall!(1, u32, obj, a, b)
    }
});
