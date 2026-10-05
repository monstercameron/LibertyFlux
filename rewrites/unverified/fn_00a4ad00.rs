// original: 0x00A4AD00 CVehicle::vf81

/// Virtual method 81: returns bit 6 of the flag byte at `this + FLAGS`.
///
/// Same shape as `vf74` (bit 4 of the same byte): read one byte, shift right
/// by 6, mask to one bit. Pure view; no writes, no calls.
///
/// Original: 0x00A4AD00 (thiscall, no stack words), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4AD00(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x0F1D;
        const BIT: u32 = 6;
        let byte = ((this + FLAGS) as *const u8).read();
        u32::from((byte >> BIT) & 1)
    }
});
