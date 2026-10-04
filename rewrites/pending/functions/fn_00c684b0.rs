// original: 0x00c684b0 scaled_tick_mod_b
// Same shape as rw_00c68480 with a different multiplier, shift and divisor:
// ((0x810e35c1 * ticks) >> 48) % divisor. Returns 0 while the enable byte
// at +0x710 is clear.
export!(thiscall, rw_00c684b0(obj: *const u8) -> u32 {
    unsafe {
        if *obj.add(0x710) == 0 {
            return 0;
        }
        let g = *global::<u32>(0x1173608) as u64;
        let q = (0x810e35c1u64.wrapping_mul(g) >> 48) as u32;
        q % *global::<u32>(0x169e2f4)
    }
});
