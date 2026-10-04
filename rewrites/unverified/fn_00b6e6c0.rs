// original: 0x00b6e6c0 select_anim_and_invoke (proposed)
/// Resolve an animation id from a seat index, then invoke the dispatcher.
///
/// `this` points to the task and one stack word (`arg`) is passed through to
/// the dispatcher. When the id dword at `+0x1c` is not -1 the function returns
/// at once (EAX keeps its entry value, so no return channel is compared).
/// Otherwise the seat dword at `+0x28` selects the id: 5/6/7/8 give
/// 0x145/0x147/0x146/0x148, any other value leaves -1 in place.
///
/// The dispatcher (callee 1, cdecl/8) is then called with the record pointer
/// `p` at `+0x24` as: ([t+0xc4], this+0x1c, arg, p, 0, 0, 0, 1), where
/// `t` is the dword at table_base + sign_extend(word at p+0x2e)*4.
/// Its answer is stored at `+0x20`.
///
/// Original: 0x00b6e6c0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00b6e6c0(this: u32, arg: u32) -> u32 {
    unsafe {
        const ID_OFF: u32 = 0x1c;
        const OUT_OFF: u32 = 0x20;
        const REC_OFF: u32 = 0x24;
        const SEAT_OFF: u32 = 0x28;
        const KEY_OFF: u32 = 0x2e;
        const SLOT_OFF: u32 = 0xc4;
        const TABLE: u32 = 0x01295cd8;
        const UNSET: u32 = 0xffffffff;
        let idp = (this + ID_OFF) as *mut u32;
        if idp.read_unaligned() != UNSET {
            return 0;
        }
        let seat = ((this + SEAT_OFF) as *const u32).read_unaligned();
        match seat.wrapping_sub(5) {
            0 => idp.write_unaligned(0x145),
            1 => idp.write_unaligned(0x147),
            2 => idp.write_unaligned(0x146),
            3 => idp.write_unaligned(0x148),
            _ => {}
        }
        let p = ((this + REC_OFF) as *const u32).read_unaligned();
        let w = ((p + KEY_OFF) as *const i16).read_unaligned() as i32 as u32;
        let t = (lf_checker_rt::relocated(TABLE).wrapping_add(w.wrapping_mul(4)) as *const u32).read_unaligned();
        let slot = ((t + SLOT_OFF) as *const u32).read_unaligned();
        let ans: u32 = lf_checker_rt::callee_cdecl!(1, u32, slot, this + ID_OFF, arg, p, 0, 0, 0, 1);
        ((this + OUT_OFF) as *mut u32).write_unaligned(ans);
        0
    }
});

