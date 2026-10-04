// original: 0x00bfc0b0 forward_field20
//! rs20f10 @0xBFC0B0: forward field +0x20 with the arg (thiscall/1).
export!(thiscall, rw_rs20f10(this: *const u8, a1: u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, a1, *(this.add(0x20) as *const u32));
        0
    }
});
