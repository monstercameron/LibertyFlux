// original: 0x00d6a6e0 FRONTEND_MENU_MONTAGE_PRESS_MT
/// Dispatch a montage press on the mode (original 0x00D6A6E0,
/// thiscall/0, three arms end in a tail call).
///
/// runs the head notification (callee 1) when the global at file VA 0x01593B70 is non-null and its flag byte at `+0x48` is nonzero, then switches on the mode dword at file VA 0x01037720 (all equality checks): mode 3 notifies with 9, mode 9 notifies with 10,
/// any other mode except 10 notifies with 3 and, unless the mode is 1,
/// advances the selector (callee 3 with 2) and clears the slot byte; each of
/// those three arms then logs its token (callee 4), runs the shared step
/// (callee 5) and tail-calls the member forwarder (callee 6). Mode 10
/// returns after the head step. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a6e0(this_ptr: u32) -> u32 {
    unsafe {
        const HEAD_GLOBAL: u32 = 0x01593B70;
        const HEAD_FLAG_OFF: u32 = 0x48;
        const MODE_GLOBAL: u32 = 0x01037720;
        const LOG_OBJ: u32 = 0x01176888;
        const MEMBER_OFF: u32 = 4;
        const LINK0_OFF: u32 = 0x1c;
        const LINK1_OFF: u32 = 8;
        const FLAG_OFF: u32 = 0x20;
        let head = (lf_checker_rt::relocated(HEAD_GLOBAL) as *const u32)
            .read_unaligned();
        if head != 0 && ((head + HEAD_FLAG_OFF) as *const u8).read() != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        }
        let mode = (lf_checker_rt::relocated(MODE_GLOBAL) as *const u32)
            .read_unaligned();
        // The notify call (callee 2) runs before the selector step on every
        // arm, so the arm is selected first and the calls follow in order.
        let (notify, token, advance): (u32, u32, bool);
        if mode == 3 {
            notify = 9;
            token = 0x00EEAC34;
            advance = false;
        } else if mode == 9 {
            notify = 10;
            token = 0x00EEAC54;
            advance = false;
        } else if mode == 10 {
            return 0;
        } else {
            notify = 3;
            token = 0x00EEAC74;
            advance = mode != 1;
        }
        lf_checker_rt::callee_cdecl!(2, u32, notify);
        if advance {
            lf_checker_rt::callee_thiscall!(3, u32, this_ptr, 2);
            let mid =
                ((this_ptr + LINK0_OFF) as *const u32).read_unaligned();
            let slot =
                ((mid + LINK1_OFF) as *const u32).read_unaligned();
            ((slot + FLAG_OFF) as *mut u8).write(0);
        }
        lf_checker_rt::callee_thiscall!(4, u32,
            lf_checker_rt::relocated(LOG_OBJ), lf_checker_rt::relocated(token));
        lf_checker_rt::callee_thiscall!(5, u32, this_ptr);
        let member =
            ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(6, u32, member);
        0
    }
});
