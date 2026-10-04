// original: 0x00985360 audio_set_emitter_flag_bit
/// Set bit 0 of the flag byte selected by the emitter lookup tables.
///
/// Resolves the emitter record from the bytes at `this+4` (selector `a`)
/// and `this+0x40` (bank `b`): `rec = G1*a + T[G2 + b*0x6f40 + 0x6f14]`
/// with all arithmetic wrapping. When `a` is 0xFF the record address is
/// null (the original then faults reading address 0xe8; the rewrite faults
/// the same way). Otherwise sets bit 0 of the byte at `rec+0xe8` to bit 0
/// of `arg`, leaving the other bits alone. Returns the untouched high bytes
/// of G2 with the flipped low bit.
///
/// Bit-twiddling note: `mem ^= (mem ^ arg) & 1` flips the low bit exactly
/// when it differs from the argument's low bit.
export!(thiscall, rw_00985360(this: u32, arg: u32) -> u32 {
    unsafe {
        const G1: u32 = 0x115d968;
        const G2: u32 = 0x115d988;
        const BANK_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f14;
        const FLAG_OFF: u32 = 0xe8;
        let a = *((this.wrapping_add(4)) as *const u8) as u32;
        // Null path (a == 0xFF): the record address is null and EAX keeps
        // only the selector byte in its high part (zero); the load below
        // then faults exactly like the original.
        let (rec, hi) = if a == 0xFF {
            (0u32, 0u32)
        } else {
            let b = *((this.wrapping_add(0x40)) as *const u8) as u32;
            let g1 = *global::<u32>(G1);
            let g2 = *global::<u32>(G2);
            let slot = g2
                .wrapping_add(b.wrapping_mul(BANK_STRIDE))
                .wrapping_add(TABLE_BIAS);
            let rec = g1
                .wrapping_mul(a)
                .wrapping_add(*(slot as *const u32));
            (rec, g2 & 0xFFFFFF00)
        };
        let p = rec.wrapping_add(FLAG_OFF) as *mut u8;
        let old = *p;
        let bit = (old ^ (arg as u8)) & 1;
        *p = old ^ bit;
        hi | (bit as u32)
    }
});
