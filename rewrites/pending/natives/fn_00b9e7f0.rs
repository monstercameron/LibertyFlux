// original: 0x00b9e7f0 CREATE_RANDOM_CHAR
/// CREATE_RANDOM_CHAR: spawn a random ped at a position.
///
/// Native handler. Forwards 3 float coordinates plus a flags word to the ped-creation engine. Floats are bit-copied, never converted.
export!(cdecl, rw_00b9e7f0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0); // bit-copied f32
        let a1 = *args.add(1); // bit-copied f32
        let a2 = *args.add(2); // bit-copied f32
        let a3 = *args.add(3);
        callee_cdecl!(1, u32, a0, a1, a2, a3)
    }
});
