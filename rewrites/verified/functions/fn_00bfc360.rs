// original: 0x00bfc360 rs20f14

// rs20f14 @0xBFC360: 11-arg init (thiscall/11). Forwards to the full init
// with a zero in the a7 slot (the incoming a7 selects flag bit 2 instead),
// stamps marker 0x4e. EAX keeps the callee's high bytes over the flag byte.
export!(thiscall, rw_rs20f14(
    this: *mut u8,
    a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
    a7: u32, a8: u32, a9: u32, a10: u32, a11: u32,
) -> u32 {
    unsafe {
        let full: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let r = full(this as u32, a1, a2, a3, a4, a5, a6, 0, a8, a9, a10, a11);
        *this = 0x4e;
        *this.add(2) = 0x0c;
        let cell = this.add(0x2b);
        let flag = (((a7 as u8).wrapping_shl(2)) ^ *cell) & 4;
        *cell ^= flag;
        (r & 0xFFFFFF00) | flag as u32
    }
});
