// original: 0x00a7cd00 touch_chain
// Touch every node in the chain hanging off +0x124 (linked via
// +0x11c) with the touch step (thiscall/0). Always returns 1.
export!(thiscall, rw_s13_00a7cd00(this: u32) -> u8 {
    unsafe {
        let touch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut node = *((this + 0x124) as *const u32);
        while node != 0 {
            touch(node);
            node = *((node + 0x11C) as *const u32);
        }
        1
    }
});
