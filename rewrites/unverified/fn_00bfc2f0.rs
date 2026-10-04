// original: 0x00bfc2f0 rs20f13

// rs20f13 @0xBFC2F0: 14-arg init (thiscall/14). Forwards a1..a11 straight to
// the full init (a-F03 fix: lane version dropped a5; v3 shows it is passed
// through), stamps marker 0x52, runs the optional conversion when a12's low byte is set, stores the a14 float.
export!(thiscall, rw_rs20f13(
    this: *mut u8,
    a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
    a7: u32, a8: u32, a9: u32, a10: u32, a11: u32,
    a12: u32, a13: u32, a14: u32,
) -> u32 {
    unsafe {
        let full: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        full(this as u32, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11);
        *this = 0x52;
        *this.add(2) = 0x0c;
        if a12 as u8 != 0 {
            let conv: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(2) as usize);
            conv(this as u32, a13);
        }
        *(this.add(0x2c) as *mut u32) = a14;
        0
    }
});
