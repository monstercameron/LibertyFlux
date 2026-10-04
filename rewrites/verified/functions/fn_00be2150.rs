// original: 0x00be2150 CTaskComplexUseAttractor::vf20 (symbols)

/// Notify the attractor's owner when the active effect matches, then report it.
///
/// Walks two links from this task (`this + LINK0`, then `+ LINK1`, both
/// 0x14): a null at either stop ends the walk. The target must carry effect
/// kind `WANT_KIND` (0x100) in the masked bits (`& KIND_MASK`, 0x3c0) of its
/// word at `+ KIND_OFF` (0x28) and have that kind's bit set in its flag word
/// at `+ FLAG_OFF` (0x210); otherwise nothing happens. On a full match the
/// notify callee runs with (`arg`, 0, 0) and this task's owner (`this +
/// OWNER_OFF`, 0x8) as object; its answer is discarded. Returns the owner in
/// all cases.
///
/// Original: 0x00be2150 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be2150(this: u32, arg: u32) -> u32 {
    unsafe {
        const LINK0: u32 = 0x14;
        const LINK1: u32 = 0x14;
        const KIND_OFF: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const WANT_KIND: u32 = 0x100;
        const FLAG_OFF: u32 = 0x210;
        const OWNER_OFF: u32 = 0x08;
        const NOTIFY: u32 = 1;
        let owner = (this.wrapping_add(OWNER_OFF) as *const u32).read_unaligned();
        let a = (this.wrapping_add(LINK0) as *const u32).read_unaligned();
        if a != 0 {
            let b = (a.wrapping_add(LINK1) as *const u32).read_unaligned();
            if b != 0 {
                let kind = (b.wrapping_add(KIND_OFF) as *const u32).read_unaligned() & KIND_MASK;
                if kind == WANT_KIND {
                    let flags = (b.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
                    if flags & kind != 0 {
                        lf_checker_rt::callee_thiscall!(NOTIFY, u32, owner, arg, 0, 0);
                    }
                }
            }
        }
        owner
    }
});
