// original: 0x0089e090 fwd_89de40
/// Forwarding wrapper cdecl/2 -> thiscall/1 (object in first slot).
export!(cdecl, rw_0089e090(obj: u32, a: u32) -> u32 {
    unsafe {
        let target: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        target(obj, a)
    }
});
