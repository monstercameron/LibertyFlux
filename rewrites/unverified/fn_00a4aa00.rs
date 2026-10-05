// original: 0x00A4AA00 CVehicle::vf74

/// Virtual method 74: returns bit 4 of the flag byte at `this + FLAGS`.
///
/// Reads one byte, shifts right by 4 and masks to one bit. Pure view over the
/// object; no writes, no calls.
///
/// Original: 0x00A4AA00 (thiscall, no stack words), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4AA00(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x0F1D;
        const BIT: u32 = 4;
        let byte = ((this + FLAGS) as *const u8).read();
        u32::from((byte >> BIT) & 1)
    }
});
