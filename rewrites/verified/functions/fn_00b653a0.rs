// original: 0x00b653a0 step_with_override_3
// thiscall/3. Holds the override byte at +0x109 while running the shared
// three-argument step, then releases it. Returns the step's answer.
export!(thiscall, rw_rs11f15(this: *mut u8, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        *this.add(0x109) = 1;
        let step: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let out = step(this as u32, a, b, c);
        *this.add(0x109) = 0;
        out
    }
});
