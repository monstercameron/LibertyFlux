// original: 0x00bd42c0 SET_MOVIE_VOLUME
// Rewrite of the SET_MOVIE_VOLUME native handler.

/// Script native `SET_MOVIE_VOLUME(volume)`.
///
/// Forwards the single float argument to the engine movie-volume routine. No
/// return slot is written; the engine's answer is left in the return register.
export!(cdecl, rw_bd42c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args)
    }
});
