// original: 0x00a09f60 cleanup_record_matches (proposed)
/// Test whether a cleanup record matches a wanted handle and mode.
///
/// When `want` is clear the answer is simply whether the record's handle
/// at +8 equals `handle`. Otherwise the record kind at +0xc decides: kind
/// 1 matches outright, any other nonzero kind matches only on the handle,
/// and kind 0 resolves the handle and matches when the target is flagged
/// or the cleanup gate is armed. Returns 1 or 0 (low byte only). Thiscall
/// with four stack words; the third is unread.
lf_checker_rt::export!(thiscall, rw_00a09f60(this: u32, rec: u32, handle: u32,
                                             _x: u32, want: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 0;
        const GATE: u32 = 1;
        const FLAG_OFF: u32 = 0x94;
        if (want & 0xff) == 0 {
            let h = ((rec + 8) as *const u32).read_unaligned();
            return (h == handle) as u32;
        }
        let kind = ((rec + 0x0c) as *const u32).read_unaligned();
        let mut live = false;
        if kind == 1 {
            live = true;
        } else if kind == 0 {
            let target = lf_checker_rt::callee_cdecl!(
                RESOLVE, u32, ((rec + 8) as *const u32).read_unaligned());
            if target != 0 {
                if ((target + FLAG_OFF) as *const u8).read() != 0 {
                    live = true;
                } else {
                    let armed: u32 = lf_checker_rt::callee_cdecl!(GATE, u32,);
                    if (armed & 0xff) != 0 {
                        live = true;
                    }
                }
            }
        }
        let h = ((rec + 8) as *const u32).read_unaligned();
        if h == handle {
            return 1;
        }
        live as u32
    }
});
