// original: 0x006f5570 pack_bit_fields
/// Packs the caller's values into the bit-field object behind slot 0.
///
/// Forwards the base value (adjusted by the mode flag), the flag itself, and
/// two 16-bit values to four field-writer calls, then flips bit 0 of the
/// status byte at offset 0x32 when it disagrees with the key byte and records
/// the second 16-bit value at offset 0x30.
export!(thiscall, rw_006f5570(
    this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32,
) -> () {
    unsafe {
        let flag = a3 & 1;
        let target = *(this as *const u32);
        let base = if flag != 0 {
            a0.wrapping_add(6)
        } else {
            a0.wrapping_add(4)
        };
        callee_cdecl!(1, u32, target, base, 10, 0);
        callee_cdecl!(1, u32, target, flag, 1, 10);
        callee_cdecl!(1, u32, target, (a2 & 0xFFFF) as u32, 16, 11);
        let keep = (a1 & 0xFFFF) as u16;
        callee_cdecl!(1, u32, target, keep as u32, 16, 27);
        let status = ((this + 0x32) as *mut u8);
        let disagree = (*status ^ (a4 & 0xFF) as u8) & 1;
        *status ^= disagree;
        *((this + 0x30) as *mut u16) = keep;
    }
});
