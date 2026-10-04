// original: 0x0089d510 fwd_89c3e0
/// Forwarding wrapper cdecl/3 -> thiscall/2 (object in first slot).
export!(cdecl, rw_0089d510(obj: u32, a: u32, b: u32) -> u32 {
    unsafe {
        
        callee_thiscall!(1, u32, obj, a, b)
    }
});
