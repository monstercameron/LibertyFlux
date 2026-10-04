// original: 0x00bfc0d0 forward_field28
//! rs20f11 @0xBFC0D0: forward field +0x28 with the arg (thiscall/1).
export!(thiscall, rw_rs20f11(this: *const u8, a1: u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, a1, *(this.add(0x28) as *const u32));
        0
    }
});
