// original: 0x00c3b3c0 train_car_matches_and_linked (proposed)
/// Report whether this car is linked to a matching target car.
///
/// Calls helper id 1 (thiscall, this) for a candidate holder. Unlike
/// its siblings it does not null-check: a null answer faults reading
/// `+0x26c`, on both sides identically. Returns 0 in AL when the flag
/// byte at `+0x26c` lacks bit 2 or the target word at `+0xb30` is null.
/// target word at `+0xb30` is null. Loads the target's signed word at
/// `+0x2e` and returns 0 when it differs from the global selector.
/// Otherwise walks forward from this car through the `+0x14d4` links
/// (this car itself is compared first) and returns 1 on a match, else 0.
/// EAX keeps the sign-extended selector word with AL replaced (the walk
/// uses ESI, so EAX still holds it). The chain must be null-terminated.
///
/// Original: 0x00c3b3c0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3b3c0(this: u32) -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x26c;
        const FLAG_BIT: u8 = 4;
        const TARGET: u32 = 0xb30;
        const SEL_WORD: u32 = 0x2e;
        const NEXT: u32 = 0x14d4;
        const SELECTOR: u32 = 0x12fa320;
        const HELPER: u32 = 1;
        let got: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        // No null guard: the original faults on a null answer, and so do we.
        if (((got + FLAG_BYTE) as *const u8).read() & FLAG_BIT) == 0 {
            return got & 0xffff_ff00;
        }
        let want = ((got + TARGET) as *const u32).read_unaligned();
        if want == 0 {
            return got & 0xffff_ff00;
        }
        let sel = ((want + SEL_WORD) as *const u16).read_unaligned() as i16 as i32 as u32;
        let hi = sel & 0xffff_ff00;
        if sel != lf_checker_rt::global::<u32>(SELECTOR).read_unaligned() {
            return hi;
        }
        if this == 0 {
            return hi;
        }
        let mut cur = this;
        loop {
            if cur == want {
                return hi | 1;
            }
            cur = ((cur + NEXT) as *const u32).read_unaligned();
            if cur == 0 {
                return hi;
            }
        }
    }
});
