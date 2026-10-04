// original: 0x00b942e0 GENERATE_RANDOM_FLOAT_IN_RANGE
/// GENERATE_RANDOM_FLOAT_IN_RANGE: random float from the engine RNG.
///
/// Native handler. Forwards min, max and an extra seed word to the engine RNG. The engine answer stays in eax; no return slot is written.
export!(cdecl, rw_00b942e0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0); // bit-copied f32
        let a1 = *args.add(1); // bit-copied f32
        let a2 = *args.add(2);
        callee_cdecl!(1, u32, a0, a1, a2)
    }
});
