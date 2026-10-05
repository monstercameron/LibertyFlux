// original: 0x00A4A9E0 vehicle_slot_f50_equals (proposed)

/// True when the argument is non-zero and equals the word at `this + SLOT`.
///
/// `this` points to a vehicle-ish object; `SLOT` (0x0F50) holds one word
/// (the neighbouring scan routine checks the eight words at 0x0F54..0x0F74).
/// Returns 1 when `val != 0 && val == mem`, else 0. The original leaves the
/// argument's upper bytes in `eax` on the compared path (`sete al`); only the
/// low byte is the result, so the contract compares `al`.
///
/// Original: 0x00A4A9E0 (thiscall, one stack word), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4A9E0(this: u32, val: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x0F50;
        if val == 0 {
            return 0;
        }
        let slot = ((this + SLOT) as *const u32).read_unaligned();
        u32::from(val == slot)
    }
});
