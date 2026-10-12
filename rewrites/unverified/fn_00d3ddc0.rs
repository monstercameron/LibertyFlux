// original: 0x00D3DDC0 CAnimRequestHelper::reset

/// Reset an animation request helper's linkage and request state. The object
/// arrives in ECX; flag bit 0 at +0x0a is cleared, the state word at +8 and
/// dword at +4 become all ones, and the relocated helper vtable is stored at
/// +0. EAX returns the object pointer; no stack words are consumed.
lf_checker_rt::export!(thiscall, rw_D3DDC0(this: u32) -> u32 {
    const VTABLE_SLOT: u32 = 0;
    const REQUEST_ID: u32 = 4;
    const REQUEST_STATE: u32 = 8;
    const REQUEST_FLAGS: u32 = 0x0a;
    const ACTIVE_BIT: u16 = 1;
    const HELPER_VTABLE: u32 = 0x00EE3BCC;
    unsafe {
        let flags = (this + REQUEST_FLAGS) as *mut u16;
        let value = flags.read_unaligned();
        flags.write_unaligned(value & !ACTIVE_BIT);
        core::ptr::write_unaligned((this + REQUEST_STATE) as *mut u16, u16::MAX);
        core::ptr::write_unaligned((this + REQUEST_ID) as *mut u32, u32::MAX);
        core::ptr::write_unaligned((this + VTABLE_SLOT) as *mut u32, lf_checker_rt::relocated(HELPER_VTABLE));
    }
    this
});
