// original: 0x00A7D6F0 CMeleeTaskInfo::CMeleeTaskInfo
/// Construct CMeleeTaskInfo through its linked base constructor with `base_value`, store `field_value` at +0x1c, install the relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d6f0(this: u32, base_value: u32, field_value: u32) -> u32 {
    const LINKED_CTOR: u32 = 7;
    const VTABLE: u32 = 0x00EA123C;
    let _ = lf_checker_rt::callee_thiscall!(LINKED_CTOR, u32, this, base_value);
    unsafe {
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(field_value);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
