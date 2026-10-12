// original: 0x00E38F40 rage::audEntity::audEntity

/// Initialize the base audio entity's vtable and invalid slot identifier.
///
/// The two-byte identifier at offset four is set to 0xFFFF and the relocated
/// base vtable pointer occupies the first word; return the object pointer.
lf_checker_rt::export!(thiscall, rw_e38f40(this: u32) -> u32 {
    unsafe {
        const BASE_VTABLE: u32 = 0xF14260;
        const INVALID_SLOT: u16 = 0xFFFF;
        ((this + 4) as *mut u16).write_unaligned(INVALID_SLOT);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE));
        this
    }
});
