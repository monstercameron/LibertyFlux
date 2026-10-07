// original: 0x00b64c80 info_has_status_bits
// thiscall/1. Resolves a key (current entry when the selector is nonzero, else
// the indexed slot) to an info record; reports whether both status bits
// (5 and 12) are set. The low result byte is merged over the shifted flags,
// matching the original's partial-register return.
///
/// Proven scope: one this/key/info chain with selector values 0, 1, 2, and
/// 255; only one key and info record are supplied. Flags cycle through
/// 0, 32, 4096, 4128, 8224, 0xffffffff, and 0x12345678. Both callees are
/// fixed-pointer stubs.
export!(thiscall, rw_rs11f7(this: *mut u8, selector: u32) -> u32 {
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
        let flags = *((info + 0x20) as *const u32);
        let merged = (flags >> 5) & !0xff;
        if flags & 0x20 != 0 && flags & 0x1000 != 0 {
            merged | 1
        } else {
            merged
        }
    }
});
