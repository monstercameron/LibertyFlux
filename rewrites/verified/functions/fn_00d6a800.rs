// original: 0x00d6a800 replay_montage_handler_b
/// Handle montage input 11 (original 0x00D6A800, thiscall/0, ends in a tail call).
///
/// Runs the head notification (callee 1) when the global at file VA
/// 0x01593B70 is non-null and its flag byte at `+0x48` is nonzero, then
/// notifies with 11 (callee 2), pokes the member at `this+4` (callee 4),
/// forwards it (callee 5), advances the selector (callee 6 with 2) and
/// tail-calls the shared step (callee 7). Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a800(this_ptr: u32) -> u32 {
    unsafe {
        const HEAD_GLOBAL: u32 = 0x01593B70;
        const HEAD_FLAG_OFF: u32 = 0x48;
        const MEMBER_OFF: u32 = 4;
        const LOG_OBJ: u32 = 0x01176888;
        let head = (lf_checker_rt::relocated(HEAD_GLOBAL) as *const u32)
            .read_unaligned();
        if head != 0 && ((head + HEAD_FLAG_OFF) as *const u8).read() != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        }
        lf_checker_rt::callee_cdecl!(2, u32, 11);
        let member =
            ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(4, u32, member);
        lf_checker_rt::callee_thiscall!(5, u32, member);
        lf_checker_rt::callee_thiscall!(6, u32, this_ptr, 2);
        lf_checker_rt::callee_thiscall!(7, u32, this_ptr);
        0
    }
});
