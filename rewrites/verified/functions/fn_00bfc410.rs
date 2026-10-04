// original: 0x00bfc410 rs20f16

// rs20f16 @0xBFC410: 14-arg init (thiscall/14). Forwards to the full init,
// stamps marker 0x4f, runs the optional conversion, chains the 1-arg init,
// then sets flag bit 3 from a12's low bit.
export!(thiscall, rw_rs20f16(
    this: *mut u8,
    a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
    a7: u32, a8: u32, a9: u32, a10: u32, a11: u32,
    a12: u32, a13: u32, a14: u32,
) -> u32 {
    unsafe {
        let full: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        full(this as u32, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11);
        *this = 0x4f;
        *this.add(2) = 0x0a;
        let sel = a12 as u8;
        if sel != 0 {
            let conv: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(2) as usize);
            conv(this as u32, a13);
        }
        let chain: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        chain(this as u32, a14);
        let cell = this.add(0x2b);
        *cell ^= (sel.wrapping_shl(3) ^ *cell) & 8;
        0
    }
});
