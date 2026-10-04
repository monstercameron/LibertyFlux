// original: 0x00bfc3c0 rs20f15

// rs20f15 @0xBFC3C0: 12-arg init (thiscall/12). Forwards to the full init
// with zeros in the a6/a7 slots, stamps marker 0x51, chains the 1-arg init.
export!(thiscall, rw_rs20f15(
    this: *mut u8,
    a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, _a6: u32, _a7: u32,
    a8: u32, a9: u32, a10: u32, a11: u32, a12: u32,
) -> u32 {
    unsafe {
        let full: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        full(this as u32, a1, a2, a3, a4, a5, 0, 0, a8, a9, a10, a11);
        *this = 0x51;
        *this.add(2) = 0x0c;
        let chain: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        chain(this as u32, a12);
        0
    }
});
