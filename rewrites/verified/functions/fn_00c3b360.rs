// original: 0x00c3b360 train_car_in_consist (proposed)
/// Report whether a target car is linked to this car in either direction.
///
/// Calls helper id 1 (thiscall, this) which returns a candidate holder or
/// null. Returns 0 in AL when the candidate is null, its flag byte at
/// `+0x26c` lacks bit 2, or its target word at `+0xb30` is null. Otherwise
/// searches for the target: first backwards from this car through the
/// `+0x14d0` links (this car itself is compared first), then forwards from
/// this car through the `+0x14d4` links. Returns 1 in AL on a match, else
/// 0. The upper 24 bits of EAX are the found node's high bits on a match
/// (the search walks EAX itself), the helper answer's high bits when the
/// candidate is rejected, and zero when a chain runs out. Both chains
/// must be null-terminated.
///
/// Original: 0x00c3b360 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3b360(this: u32) -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x26c;
        const FLAG_BIT: u8 = 4;
        const TARGET: u32 = 0xb30;
        const PREV: u32 = 0x14d0;
        const NEXT: u32 = 0x14d4;
        const HELPER: u32 = 1;
        let got: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        let hi = got & 0xffff_ff00;
        if got == 0 {
            return hi;
        }
        if (((got + FLAG_BYTE) as *const u8).read() & FLAG_BIT) == 0 {
            return hi;
        }
        let want = ((got + TARGET) as *const u32).read_unaligned();
        if want == 0 {
            return hi;
        }
        // Backwards: this car first, then its +0x14d0 predecessors. EAX
        // walks the chain, so a match returns over the node's high bits.
        let mut cur = this;
        if cur != 0 {
            loop {
                if cur == want {
                    return (cur & 0xffff_ff00) | 1;
                }
                cur = ((cur + PREV) as *const u32).read_unaligned();
                if cur == 0 {
                    break;
                }
            }
        }
        // Forwards through the +0x14d4 successors (this car not rechecked).
        // Running out leaves null in EAX, hence a plain 0.
        let mut fwd = ((this + NEXT) as *const u32).read_unaligned();
        loop {
            if fwd == 0 {
                return 0;
            }
            if fwd == want {
                return (fwd & 0xffff_ff00) | 1;
            }
            fwd = ((fwd + NEXT) as *const u32).read_unaligned();
        }
    }
});
