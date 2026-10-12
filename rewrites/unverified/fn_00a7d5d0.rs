// original: 0x00A7D5D0 CComplexSlideIntoCoverInfo::CComplexSlideIntoCoverInfo
/// Construct CComplexSlideIntoCoverInfo through its base constructor, store the full state word at +0x18, install its relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d5d0(this: u32, state: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA1AB4;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(state);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
