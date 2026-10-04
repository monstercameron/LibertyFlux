// original: 0x00bfc1c0 full_init_11arg_tag0b
//! rs20f12 @0xBFC1C0: the 11-argument full init (thiscall/11). Header init with
//! kind 0x4c, two sub-inits, then the tail fields: a word, two packed flag
//! bits, a float and three bytes; tag byte 0x0b. Returns the last byte.
/// Tag dword shared by the header-init family, read from the game's data.
#[inline(always)]
unsafe fn header_tag_rs20() -> u32 {
    *global::<u32>(0x11735A4)
}
/// The shared 5-argument header initializer (thiscall/5): installs the kind
/// code, the shared tag, and the two caller values into the object's header.
#[inline(always)]
unsafe fn header_init_rs20(this: *mut u8, kind: u32, a1: u32, a2: u32) {
    let init: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(callee_addr(1) as usize);
    init(this as u32, kind, header_tag_rs20(), a2, a1, 0);
}

export!(thiscall, rw_rs20f12(
    this: *mut u8,
    a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
    a7: u32, a8: u32, a9: u32, a10: u32, a11: u32,
) -> u32 {
    unsafe {
        header_init_rs20(this, 0x4c, a1, a2);
        let sub2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        sub2(this as u32, a3);
        let sub3: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        sub3(this as u32, a4);
        *(this.add(0x24) as *mut u16) = a6 as u16;
        // The original merges the old flag bits then masks them away; the
        // final value is exactly the two incoming bits.
        *this.add(0x2b) = (((a8 & 1) << 1) | (a7 & 1)) as u8;
        *(this.add(0x1c) as *mut u32) = a5;
        *this.add(0x28) = a9 as u8;
        *this.add(0x29) = a10 as u8;
        *this.add(0x2a) = a11 as u8;
        *(this.add(0x26) as *mut u16) = 0;
        *this.add(2) = 0x0b;
        a11 & 0xFF
    }
});
