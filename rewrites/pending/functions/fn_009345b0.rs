// original: 0x009345b0 refresh_and_clear
/// 0x009345B0: if the dirty flag at +0x164 is set, run one refresh pass,
/// then clear the flag. No defined return value.
export!(thiscall, rw_009345b0(this: *mut u8) -> u32 {
    unsafe {
        if core::ptr::read(this.add(0x164)) != 0 {
            callee_thiscall!(1, u32, this as u32, 1u32);
        }
        core::ptr::write(this.add(0x164), 0u8);
    }
    0
});
