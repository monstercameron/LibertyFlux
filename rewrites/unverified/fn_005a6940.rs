// original: 0x005A6940 com_variant_state_init

/// Initialize the COM variant state at this object and return its address.
///
/// The thiscall stack argument is stored at `this + 0x20`; dwords at `+0x24`
/// through `+0x34` and the byte at `+0x38` are cleared. The object pointer is
/// then passed to OLE Automation `VariantInit`, and the method returns `this`.
lf_checker_rt::export!(thiscall, rw_005a6940(this: u32, initial_type: u32) -> u32 {
    const TYPE_FIELD: u32 = 0x20;
    const ZERO_DWORD_FIELDS: [u32; 5] = [0x24, 0x28, 0x2C, 0x30, 0x34];
    const READY_BYTE: u32 = 0x38;
    const ZERO: u32 = 0;

    unsafe {
        (this.wrapping_add(TYPE_FIELD) as *mut u32).write_unaligned(initial_type);
        for offset in ZERO_DWORD_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(ZERO);
        }
        (this.wrapping_add(READY_BYTE) as *mut u8).write_unaligned(0);
    }
    let _ = lf_checker_rt::callee_stdcall!(1, u32, this);
    this
});
