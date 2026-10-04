// original: 0x00b64cc0 info_class_in_234
// thiscall/1. Same key lookup as F7; reports whether the info class field is
// 2, 3 or 4. The verdict byte is merged over the class word's high bytes.
export!(thiscall, rw_rs11f8(this: *mut u8, selector: u32) -> u32 {
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
        let class = *((info + 0xc) as *const u32);
        (class & !0xff) | ((class == 2 || class == 3 || class == 4) as u32)
    }
});
