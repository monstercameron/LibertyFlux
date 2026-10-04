// original: 0x0089dd30 fwd_89d900
/// Forwarding wrapper cdecl/2 -> thiscall/1 (object in first slot).
export!(cdecl, rw_0089dd30(obj: u32, a: u32) -> u32 {
    unsafe {
        
        callee_thiscall!(1, u32, obj, a)
    }
});
