// original: 0x009df740 tracker_reset_detach
// fn_009df740: tracker reset and list detach (thiscall/0).
//
// Installs the base vtable, runs the reset step, then removes `this` from
// each of the two global lists that still contains it (membership is tested
// by the low byte of the search answer, exactly like the original).
export!(thiscall, rw_009df740(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xE98184);
        let reset: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        reset(this as u32);
        let contains: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let remove: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        if contains(relocated(0x12B41B0), this as u32) & 0xFF != 0 {
            remove(relocated(0x12B41B0), this as u32);
        }
        if contains(relocated(0x12B41B4), this as u32) & 0xFF != 0 {
            remove(relocated(0x12B41B4), this as u32);
        }
        0
    }
});
