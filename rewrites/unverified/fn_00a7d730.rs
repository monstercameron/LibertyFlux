// original: 0x00A7D730 CPlayRandomAmbientsInfo::CPlayRandomAmbientsInfo
/// Construct CPlayRandomAmbientsInfo through its base constructor, store the low byte of `mode` at +0x18, install the relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d730(this: u32, mode: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA179C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(0x18) as *mut u8).write(mode as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
