// original: 0x00b64e20 NativeImpl_IS_CHAR_ARMED
// thiscall/0 (IS_CHAR_ARMED native impl). Resolves the current record and
// returns the field at +0x18, or 0 when there is no current record.
export!(thiscall, rw_rs11f14(this: u32) -> u32 {
    unsafe {
        let find: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let record = find(this);
        if record == 0 {
            0
        } else {
            *((record + 0x18) as *const u32)
        }
    }
});
