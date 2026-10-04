// original: 0x00d38f80 three_stage_accept
/// Three-stage acceptance: the fast path accepts, the veto rejects, an
/// optional live check can still reject, else the final poll decides.
export!(cdecl, rw_00d38f80(a1: u32, a2: u32, a3: u32) -> u32 {
    let fast = callee_thiscall!(1, u32, a3, a1);
    if fast & 0xff != 0 {
        return 1;
    }
    let veto = callee_thiscall!(2, u32, a3);
    if veto & 0xff != 0 {
        return 0;
    }
    let live = unsafe { *((a2 + 0x6c) as *const u32) };
    if live != 0 {
        let chk = callee_thiscall!(3, u32, live);
        if chk & 0xff != 0 {
            return 0;
        }
    }
    let fin = callee_thiscall!(4, u32, a3, a1);
    (fin & 0xff != 0) as u32
});
