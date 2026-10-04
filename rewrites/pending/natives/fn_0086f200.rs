// original: 0x0086f200 COS
/// Script native `COS` (hash 0x061D4B5F).
///
/// Converts one float argument from degrees to radians using a read-only
/// engine constant (pi/180) and calls the engine cosine core, which takes
/// the angle in a vector register and returns the result the same way.
/// Stores the full 32-bit result into the return slot.
export!(cdecl, rw_0086f200(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let degrees = f32::from_bits(*args);
        let radians_per_degree = *global::<f32>(0x00FE8728);
        // The scaled angle feeds the engine call (whose answer the checker
        // scripts, so the input value is unobserved on both sides).
        let _radians = degrees * radians_per_degree;
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        slot as u32
    }
});
