// original: 0x00c3b330 train_is_helper_owned (proposed)
/// Report whether the helper object belongs to this car and is active.
///
/// Calls helper id 1 (thiscall, this) which returns a candidate object or
/// null. Returns 1 in AL when the candidate is non-null, its flag byte at
/// `+0x26c` has bit 2 set, and its owner word at `+0xb30` equals this car;
/// else 0 in AL. The upper 24 bits of EAX keep the callee answer's high
/// bits on every path (the original only writes AL).
///
/// Original: 0x00c3b330 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3b330(this: u32) -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x26c;
        const FLAG_BIT: u8 = 4;
        const OWNER: u32 = 0xb30;
        const HELPER: u32 = 1;
        let got: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        let hi = got & 0xffff_ff00;
        if got == 0 {
            return hi;
        }
        if (((got + FLAG_BYTE) as *const u8).read() & FLAG_BIT) == 0 {
            return hi;
        }
        if (((got + OWNER) as *const u32).read_unaligned() != this) {
            return hi;
        }
        hi | 1
    }
});
