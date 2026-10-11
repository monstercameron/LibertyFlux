// original: 0x00CDA410 script_vm_record_defaults (proposed)

/// Initialize the record's timing and limit defaults and return the record.
///
/// `this` points to the destination object. The nested defaults initializer
/// fills five words at offsets `+0x00..+0x10`; this routine then writes zeroes
/// at `+0x14`, `+0x18`, `+0x1c`, and `+0x24`, the fixed limit `0x36a1` at
/// `+0x20`, and a zero halfword at `+0x28`. Other bytes are left unchanged.
/// The function takes no stack arguments, uses the 32-bit thiscall ABI, and
/// returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_00cda410(this: u32) -> u32 {
    unsafe {
        const STATE_A: u32 = 0x14;
        const STATE_B: u32 = 0x18;
        const STATE_C: u32 = 0x1c;
        const LIMIT: u32 = 0x20;
        const EXTRA_STATE: u32 = 0x24;
        const SHORT_STATE: u32 = 0x28;

        lf_checker_rt::callee_thiscall!(1, u32, this);
        for offset in [STATE_A, STATE_B, STATE_C, EXTRA_STATE] {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(LIMIT) as *mut u32).write_unaligned(0x36a1);
        (this.wrapping_add(SHORT_STATE) as *mut u16).write_unaligned(0);
        this
    }
});
