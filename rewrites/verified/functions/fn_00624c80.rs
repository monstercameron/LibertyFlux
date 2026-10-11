// original: 0x00624C80 pending_00624c80

/// Construct the receiver's helper state. The first base helper receives the
/// receiver; the two state words at offsets 0x48 and 0x4c are then cleared.
/// Three member helpers run in order for subobjects at 0x50, 0x40 and 0x88.
/// The thiscall returns the original receiver pointer.
lf_checker_rt::export!(thiscall, rw_00624c80(this: u32) -> u32 {
    const STATE_COOKIE: u32 = 0x48;
    const FIRST_MEMBER: u32 = 0x50;
    const SECOND_MEMBER: u32 = 0x40;
    const THIRD_MEMBER: u32 = 0x88;
    let _base = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        (this.wrapping_add(STATE_COOKIE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_COOKIE + 4) as *mut u32).write_unaligned(0);
    }
    let _first = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(FIRST_MEMBER));
    let _second = lf_checker_rt::callee_thiscall!(3, u32, this.wrapping_add(SECOND_MEMBER));
    let _third = lf_checker_rt::callee_thiscall!(4, u32, this.wrapping_add(THIRD_MEMBER));
    this
});
