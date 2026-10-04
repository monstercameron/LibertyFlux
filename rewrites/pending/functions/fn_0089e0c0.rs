// original: 0x0089e0c0 fwd_89dfa0
/// Forwarding wrapper cdecl/2 -> thiscall/1 (object in first slot).
export!(cdecl, rw_0089e0c0(obj: u32, a: u32) -> u32 {
    unsafe {
        
        callee_thiscall!(1, u32, obj, a)
    }
});
