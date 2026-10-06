// original: 0x00d6a620 FRONTEND_MENU_MONTAGE_PRESS_MT
/// Poke the owned member, then log a fixed token (original 0x00D6A620,
/// thiscall/0).
///
/// Calls the no-argument method (callee 1) on the member at `this+4`, then
/// calls the one-argument method (callee 2) on the constant object at file VA
/// 0x01176888 with the constant token at file VA 0x00EEABD4. The two constants
/// are image addresses, passed relocated. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a620(this_ptr: u32) -> u32 {
    unsafe {
        const MEMBER_OFF: u32 = 4;
        const LOG_OBJ: u32 = 0x01176888;
        const LOG_TOKEN: u32 = 0x00EEABD4;
        let member = ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, member);
        lf_checker_rt::callee_thiscall!(2, u32,
            lf_checker_rt::relocated(LOG_OBJ),
            lf_checker_rt::relocated(LOG_TOKEN));
        0
    }
});
