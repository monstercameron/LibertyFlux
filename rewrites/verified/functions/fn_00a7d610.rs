// original: 0x00A7D610 CDieInfo::CDieInfo
/// Default-construct CDieInfo after its base constructor. Install the relocated vtable, set the four words at +0x18 through +0x24 to `u32::MAX`, clear +0x28, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d610(this: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA17E4;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(0);
    }
    this
});
