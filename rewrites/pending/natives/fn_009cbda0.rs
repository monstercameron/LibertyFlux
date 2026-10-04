// original: 0x009cbda0 IS_ANY_SPEECH_PLAYING
/// Report whether any speech is playing: call the engine with the
/// script argument and store the low byte of its answer (zero-extended)
/// in the return slot.
export!(cdecl, rw_009cbda0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let ans: u32 = callee_cdecl!(1, u32, *args);
        *(*ctx as *mut u32) = ans & 0xFF;
        0
    }
});
