// original: 0x00A7D510 CComplexNewUseCoverInfo::CComplexNewUseCoverInfo
/// Construct CComplexNewUseCoverInfo after its base constructor. Copy `state` to +0x18, copy the low bytes of the six flags to +0x1c through +0x21, install the relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d510(this: u32, state: u32, flag_a: u32, flag_b: u32, flag_c: u32, flag_d: u32, flag_e: u32, flag_f: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA1AFC;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(state);
        (this.wrapping_add(0x1C) as *mut u8).write(flag_a as u8);
        (this.wrapping_add(0x1D) as *mut u8).write(flag_b as u8);
        (this.wrapping_add(0x1E) as *mut u8).write(flag_c as u8);
        (this.wrapping_add(0x1F) as *mut u8).write(flag_d as u8);
        (this.wrapping_add(0x20) as *mut u8).write(flag_e as u8);
        (this.wrapping_add(0x21) as *mut u8).write(flag_f as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
