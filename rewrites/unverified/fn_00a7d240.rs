// original: 0x00A7D240 CComplexCombatRetreatInfo::CComplexCombatRetreatInfo_2
/// Construct the second CComplexCombatRetreatInfo form. The base constructor runs first, then the low byte of `flag` is stored at +0x18 and this class's relocated vtable replaces the base vtable. Returns the same object.
lf_checker_rt::export!(thiscall, rw_00a7d240(this: u32, flag: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA1EA4;
    const FLAG_OFFSET: u32 = 0x18;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(FLAG_OFFSET) as *mut u8).write(flag as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
