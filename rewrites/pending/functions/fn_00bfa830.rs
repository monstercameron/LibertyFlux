// original: 0x00bfa830 header_init_kind48_tag0f
//! rs20f2 @0xBFA830: header init, kind 0x48, tag byte 0x0f (thiscall/2).
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

export!(thiscall, rw_rs20f2(this: *mut u8, a1: u32, a2: u32) -> u32 {
    unsafe {
        header_init_rs20(this, 0x48, a1, a2);
        *this.add(2) = 0x0f;
        0
    }
});
