// original: 0x00A7D5F0 CDeadInfo::CDeadInfo
/// Construct CDeadInfo through its base constructor, copy the low byte of `mode` to +0x18, install its relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d5f0(this: u32, mode: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA182C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(0x18) as *mut u8).write(mode as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
