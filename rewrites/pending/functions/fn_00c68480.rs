// original: 0x00c68480 scaled_tick_mod_a
// Scale the tick global by a fixed multiplier and reduce modulo a global:
// ((0x45e7b273 * ticks) >> 47) % divisor. Returns 0 while the enable byte
// at +0x710 is clear.
export!(thiscall, rw_00c68480(obj: *const u8) -> u32 {
    unsafe {
        if *obj.add(0x710) == 0 {
            return 0;
        }
        let g = *global::<u32>(0x1173608) as u64;
        let q = (0x45e7b273u64.wrapping_mul(g) >> 47) as u32;
        q % *global::<u32>(0x169e3cc)
    }
});
