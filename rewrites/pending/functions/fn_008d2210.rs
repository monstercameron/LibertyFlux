// original: 0x008D2210 Net_PlayerPassesHostCheck
/// Gated check chain over an inner object reference. A zero from the first
/// check passes immediately; otherwise the inner object at offset 0x578 must
/// exist and the second check must pass, and the result is the third check's
/// nonzero outcome. The slot is re-read before the last check.
export!(thiscall, rw_008D2210(this: u32) -> u8 {
    unsafe {
        if callee_thiscall!(0, u32, this) as u8 == 0 {
            return 1;
        }
        if *((this + 0x578) as *const u32) == 0 {
            return 0;
        }
        let inner = *((this + 0x578) as *const u32);
        if callee_thiscall!(1, u32, inner) as u8 == 0 {
            return 0;
        }
        let inner = *((this + 0x578) as *const u32);
        u8::from(callee_thiscall!(2, u32, inner) as u8 != 0)
    }
});
