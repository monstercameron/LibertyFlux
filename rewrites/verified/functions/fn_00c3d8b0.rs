// original: 0x00c3d8b0 train_set_flag_bit4_chain (proposed)
/// Set or clear flag bit 4 of this car and every following car.
///
/// `this` (ECX) points to a train car, `flag` carries the new bit value
/// in its low bit. Bit 4 (mask 0x10) of the byte at `+0x14e5` is replaced
/// by that bit on this car and then on each car reached through the
/// `+0x14d4` next-link chain, which must be null-terminated. All other
/// bits are kept. Always returns 0 (EAX holds the terminating null).
/// No calls.
///
/// Original: 0x00c3d8b0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3d8b0(this: u32, flag: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x14d4;
        const FLAGS: u32 = 0x14e5;
        const BIT: u8 = 0x10;
        let bit = (((flag & 1) as u8) << 4) & BIT;
        let b = ((this + FLAGS) as *const u8).read();
        ((this + FLAGS) as *mut u8).write((b & !BIT) | bit);
        let mut cur = ((this + NEXT) as *const u32).read_unaligned();
        while cur != 0 {
            let b = ((cur + FLAGS) as *const u8).read();
            ((cur + FLAGS) as *mut u8).write((b & !BIT) | bit);
            cur = ((cur + NEXT) as *const u32).read_unaligned();
        }
        0
    }
});
