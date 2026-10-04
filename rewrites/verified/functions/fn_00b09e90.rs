// original: 0x00b09e90 store_scaled_reciprocals
/// Publish two scaled reciprocals and three raw words to shared globals.
///
/// Writes the first, third and fifth words through unchanged, and stores
/// (1 / second) * 30 and (1 / fourth) * 30 in the remaining slots.
export!(cdecl, rw_00b09e90(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const ONE: f32 = f32::from_bits(0x3f80_0000);
        const SCALE: f32 = f32::from_bits(0x41f0_0000);
        *global::<u32>(0x0104_0070) = a0;
        let r0 = ONE / f32::from_bits(a1);
        let r1 = ONE / f32::from_bits(a3);
        *global::<u32>(0x0104_0074) = (r0 * SCALE).to_bits();
        *global::<u32>(0x0104_0078) = a2;
        *global::<u32>(0x0104_007c) = (r1 * SCALE).to_bits();
        *global::<u32>(0x0104_0080) = a4;
        0
    }
});
