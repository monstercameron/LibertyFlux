// original: 0x0089e090 fwd_89de40
/// Forwarding wrapper cdecl/2 -> thiscall/1 (object in first slot).
export!(cdecl, rw_0089e090(obj: u32, a: u32) -> u32 {
    unsafe {
        
        callee_thiscall!(1, u32, obj, a)
    }
});
