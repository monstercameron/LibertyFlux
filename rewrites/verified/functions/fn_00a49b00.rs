// original: 0x00a49b00 vehicle_has_occupant_or_flag
/// True when the vehicle has any occupant or its override flag is set.
///
/// Returns 1 when bit 0x40 of the byte at `this+0xF1D` is set, when the
/// dword at `this+0xF50` is nonzero, or when any of the eight dwords at
/// `this+0xF54..` is nonzero; else 0 (thiscall, no stack arguments). Only
/// AL is compared: the upper bytes keep the entry ECX bytes.
export!(thiscall, rw_00a49b00(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0xf1d;
        const FLAG_BIT: u8 = 0x40;
        const SLOTS_OFF: u32 = 0xf50;
        const TRAIL_OFF: u32 = 0xf54;
        const TRAIL_N: u32 = 8;
        if ((this.wrapping_add(FLAG_OFF)) as *const u8).read() & FLAG_BIT != 0 {
            return 1;
        }
        if (this.wrapping_add(SLOTS_OFF) as *const u32).read_unaligned() != 0 {
            return 1;
        }
        let mut p = this.wrapping_add(TRAIL_OFF);
        let mut i = 0u32;
        while i < TRAIL_N {
            if (p as *const u32).read_unaligned() != 0 {
                return 1;
            }
            p = p.wrapping_add(4);
            i += 1;
        }
        0
    }
});
