// original: 0x00a2b720 ped_task_gate

/// Gate a ped task request on state flags and a status check.
/// Returns 1 when the request may proceed: the vtable flag at `+0x264`
/// of `[this]` must be clear or the whole status check is skipped; then,
/// unless the third argument is zero and the global at `G_HAS` is zero,
/// the status worker (callee 1, thiscall on this) must answer nonzero and
/// the global byte at `G_BYTE` must be zero. Afterwards the flag word
/// `args[0] & 0x1000` rejects, and the candidate at the first stack
/// argument proceeds unless its state field (`+0x28` masked with 0x3C0)
/// equals 0xC0 while its sub-flag (`+0x270` shifted right 4) has bit 0
/// set.
/// Original: 0x00a2b720 (thiscall, three stack words, byte result).
lf_checker_rt::export!(thiscall, rw_00a2b720(this: u32, cand: u32, flags: u32, mode: u32) -> u8 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
        const VT_FLAG_OFF: u32 = 0x264;
        const VT_FLAG_BIT: u32 = 0x800000;
        const G_HAS: u32 = 0x1160c68;
        const G_BYTE_BASE: u32 = 0x105c644;
        const G_BYTE_SHIFT: u32 = 16;
        const REJECT_BIT: u32 = 0x1000;
        const STATE_OFF: u32 = 0x28;
        const STATE_MASK: u32 = 0x3c0;
        const STATE_BUSY: u32 = 0xc0;
        const SUB_OFF: u32 = 0x270;
        const STATUS: u32 = 1;
        let vt = rd32(this);
        if (rd32(vt.wrapping_add(VT_FLAG_OFF)) & VT_FLAG_BIT) == 0
            && ((mode as u8) != 0 || rd32(lf_checker_rt::relocated(G_HAS)) != 0)
        {
            if lf_checker_rt::callee_thiscall!(STATUS, u8, this) == 0 {
                return 0;
            }
            let gb = (rd32(lf_checker_rt::relocated(G_BYTE_BASE)) >> G_BYTE_SHIFT) as u8;
            if gb != 0 {
                return 0;
            }
        }
        if (flags & REJECT_BIT) != 0 {
            return 0;
        }
        if (rd32(cand.wrapping_add(STATE_OFF)) & STATE_MASK) != STATE_BUSY {
            return 1;
        }
        if ((rd32(cand.wrapping_add(SUB_OFF)) >> 4) & 1) != 0 {
            0
        } else {
            1
        }
    }
});
