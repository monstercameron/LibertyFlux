// original: 0x00B126A0 CDynamicEntity::vf54

/// For dynamic states 2, 3, 4 and 6, choose the entity's external data plus
/// 0x30 when present, or its inline storage at +0x10 otherwise. Invoke the
/// callback stored at the object table's +4 slot with `this + 0x80`, the chosen
/// data pointer and a zero mode. Other state values return without a call.
/// This is a no-argument thiscall method.
lf_checker_rt::export!(thiscall, rw_00b126a0(this: u32) -> () {
    const STATE: u32 = 0x28;
    const EXTERNAL_DATA: u32 = 0x20;
    const INLINE_DATA: u32 = 0x10;
    const CALLBACK_TABLE: u32 = 0x80;
    const CALLBACK_SLOT: u32 = 4;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let state = (read_u32(this.wrapping_add(STATE)) >> 6) & 0xF;
        if !matches!(state, 2 | 3 | 4 | 6) {
            return;
        }

        let external = read_u32(this.wrapping_add(EXTERNAL_DATA));
        let source = if external != 0 {
            external.wrapping_add(0x30)
        } else {
            this.wrapping_add(INLINE_DATA)
        };
        let object_table = read_u32(this.wrapping_add(CALLBACK_TABLE));
        let callback_address = read_u32(object_table.wrapping_add(CALLBACK_SLOT));
        let callback: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callback_address as usize);
        let _ = callback(this.wrapping_add(CALLBACK_TABLE), source, 0);
    }
});
