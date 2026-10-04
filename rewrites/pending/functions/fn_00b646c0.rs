// original: 0x00b646c0 resolve_with_override
// thiscall/2. Holds the override byte at +0x109 while running the shared
// resolve step, then releases it. Returns the step's answer.
export!(thiscall, rw_rs11f4(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        *this.add(0x109) = 1;
        let resolve: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let out = resolve(this as u32, a, b);
        *this.add(0x109) = 0;
        out
    }
});
