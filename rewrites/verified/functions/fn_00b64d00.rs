// original: 0x00b64d00 info_has_status_bit5
// thiscall/1. Same key lookup; returns status bit 5 of the info record.
export!(thiscall, rw_rs11f9(this: *mut u8, selector: u32) -> u32 {
    unsafe {
        let key = if (selector as u8) != 0 {
            let current: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            current(this as u32)
        } else {
            let base = *(this as *const u32);
            *((this as u32).wrapping_add(base.wrapping_add(3).wrapping_mul(12)) as *const u32)
        };
        let info = callee_cdecl!(2, u32, key);
        (*((info + 0x20) as *const u32) >> 5) & 1
    }
});
