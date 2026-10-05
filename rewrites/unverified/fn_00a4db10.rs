// original: 0x00A4DB10 CHeli::vf56

/// Virtual method 56: true when neither of two flag bits is set.
///
/// Returns 0 when bit 3 of the byte at `this + FLAG_A` (0x118) is set,
/// otherwise 0 when bit 4 of the byte at `this + FLAG_B` (0x0F1D) is set,
/// else 1. Pure view; no writes, no calls.
///
/// Original: 0x00A4DB10 (thiscall, no stack words), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4DB10(this: u32) -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x0118;
        const FLAG_B: u32 = 0x0F1D;
        const MASK_A: u8 = 0x08;
        const MASK_B: u8 = 0x10;
        if ((this + FLAG_A) as *const u8).read() & MASK_A != 0 {
            return 0;
        }
        if ((this + FLAG_B) as *const u8).read() & MASK_B != 0 {
            return 0;
        }
        1
    }
});
