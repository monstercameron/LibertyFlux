// original: 0x00A7D3F0 CComplexJumpTaskInfo::CComplexJumpTaskInfo
/// Construct CComplexJumpTaskInfo after its base constructor. The low bytes of `kind`, `variant` and `flags` go to +0x18, +0x19 and +0x1a; the relocated vtable is installed and `this` is returned.
lf_checker_rt::export!(thiscall, rw_00a7d3f0(this: u32, kind: u32, variant: u32, flags: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA1314;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(0x18) as *mut u8).write(kind as u8);
        (this.wrapping_add(0x19) as *mut u8).write(variant as u8);
        (this.wrapping_add(0x1A) as *mut u8).write(flags as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
