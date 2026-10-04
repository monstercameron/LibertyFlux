// original: 0x0089dd10 fwd_89d7e0
/// Forwarding wrapper cdecl/3 -> thiscall/2 (object in first slot).
export!(cdecl, rw_0089dd10(obj: u32, a: u32, b: u32) -> u32 {
    unsafe {
        let target: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        target(obj, a, b)
    }
});
