// original: 0x00bfa860 header_init_kind4b_tag0f
//! rs20f3 @0xBFA860: header init, kind 0x4b, tag byte 0x0f (thiscall/2).

use lf_k2_rt::{callee_thiscall, export, global};

/// Tag dword shared by the header-init family, read from the game's data.
#[inline(always)]
unsafe fn header_tag_rs20() -> u32 {
    *global::<u32>(0x11735A4)
}
/// The shared 5-argument header initializer (thiscall/5): installs the kind
/// code, the shared tag, and the two caller values into the object's header.
#[inline(always)]
unsafe fn header_init_rs20(this: *mut u8, kind: u32, a1: u32, a2: u32) {
    
    callee_thiscall!(1, u32, this as u32, kind, header_tag_rs20(), a2, a1, 0);
}

export!(thiscall, rw_rs20f3(this: *mut u8, a1: u32, a2: u32) -> u32 {
    unsafe {
        header_init_rs20(this, 0x4b, a1, a2);
        *this.add(2) = 0x0f;
        0
    }
});
