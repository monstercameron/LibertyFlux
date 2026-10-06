// original: 0x00d6a850 FRONTEND_MENU_MONTAGE_PRESS_MT
/// Handle montage press 5 (original 0x00D6A850, thiscall/0, ends in a tail call).
///
/// Runs the head notification (callee 1) when the global at file VA
/// 0x01593B70 is non-null and its flag byte at `+0x48` is nonzero, then
/// notifies with 5 (callee 2), logs the fixed token (callee 3 on the
/// constant object), pokes the member at `this+4` (callee 4), forwards it (callee 5), advances the selector (callee 6 with 2) and
/// tail-calls the shared step (callee 7). Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a850(this_ptr: u32) -> u32 {
    unsafe {
        const HEAD_GLOBAL: u32 = 0x01593B70;
        const HEAD_FLAG_OFF: u32 = 0x48;
        const MEMBER_OFF: u32 = 4;
        const LOG_OBJ: u32 = 0x01176888;
        const LOG_TOKEN: u32 = 0x00EEAC94;
        let head = (lf_checker_rt::relocated(HEAD_GLOBAL) as *const u32)
            .read_unaligned();
        if head != 0 && ((head + HEAD_FLAG_OFF) as *const u8).read() != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        }
        lf_checker_rt::callee_cdecl!(2, u32, 5);
        lf_checker_rt::callee_thiscall!(3, u32,
            lf_checker_rt::relocated(LOG_OBJ),
            lf_checker_rt::relocated(LOG_TOKEN));
        let member =
            ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(4, u32, member);
        lf_checker_rt::callee_thiscall!(5, u32, member);
        lf_checker_rt::callee_thiscall!(6, u32, this_ptr, 2);
        lf_checker_rt::callee_thiscall!(7, u32, this_ptr);
        0
    }
});
