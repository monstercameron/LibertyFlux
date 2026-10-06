// original: 0x00d6aa00 FRONTEND_MENU_MONTAGE_PRESS_MT
/// Handle a montage press through two readiness gates (original 0x00D6AA00,
/// thiscall/0, one path ends in a tail call).
///
/// Logs the fixed token (callee 1 on the constant object), then checks the
/// first gate (callee 2's low byte): zero tail-calls the fallback (callee 9).
/// Past it, the head notification (callee 3) runs when the global at file VA
/// 0x01593B70 is non-null and its flag byte at `+0x48` is nonzero. The second
/// gate (callee 4's low byte on `this+4`) returns early when zero. Past both,
/// it notifies with 1 (callee 5) unless the mode global at file VA 0x01037720
/// reads 0xf or 8 (both equality checks), runs the shared step (callee 6),
/// forwards the member (callee 7) and advances the selector (callee 8 with
/// 2). Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6aa00(this_ptr: u32) -> u32 {
    unsafe {
        const LOG_OBJ: u32 = 0x01176888;
        const LOG_TOKEN: u32 = 0x00EEAAE0;
        const HEAD_GLOBAL: u32 = 0x01593B70;
        const HEAD_FLAG_OFF: u32 = 0x48;
        const MODE_GLOBAL: u32 = 0x01037720;
        const MEMBER_OFF: u32 = 4;
        lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(LOG_OBJ),
            lf_checker_rt::relocated(LOG_TOKEN));
        let gate1: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        if (gate1 & 0xFF) == 0 {
            lf_checker_rt::callee_thiscall!(9, u32, this_ptr);
            return 0;
        }
        let head = (lf_checker_rt::relocated(HEAD_GLOBAL) as *const u32)
            .read_unaligned();
        if head != 0 && ((head + HEAD_FLAG_OFF) as *const u8).read() != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        }
        let member =
            ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        let gate2: u32 = lf_checker_rt::callee_thiscall!(4, u32, member);
        if (gate2 & 0xFF) == 0 {
            return 0;
        }
        let mode = (lf_checker_rt::relocated(MODE_GLOBAL) as *const u32)
            .read_unaligned();
        if mode != 0xf && mode != 8 {
            lf_checker_rt::callee_cdecl!(5, u32, 1);
        }
        lf_checker_rt::callee_thiscall!(6, u32, this_ptr);
        lf_checker_rt::callee_thiscall!(7, u32, member);
        lf_checker_rt::callee_thiscall!(8, u32, this_ptr, 2);
        0
    }
});
