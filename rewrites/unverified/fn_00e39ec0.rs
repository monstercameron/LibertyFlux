// original: 0x00E39EC0 UIMontageMoviePlayer::configure_texture_source_and_hud_colour

/// Update the explicitly supplied montage player and new texture source. If
/// the current texture has a live old source, save that source on the player
/// and increment its 16-bit reference count. Increment the new source count,
/// ask the texture helper to release and refresh, install the two exact
/// identity-coordinate pairs, then dispatch the HUD-colour lookup and store
/// the first word of its returned data at texture offset `0x1e0`. The function
/// is cdecl with two pointer arguments; its first argument is also passed by
/// address to the native dispatcher.
lf_checker_rt::export!(cdecl, rw_00e39ec0(movie: u32, new_source: u32) -> u32 {
    const MOVIE_TEXTURE_OFFSET: u32 = 0x1E0;
    const TEXTURE_SOURCE_SLOT_OFFSET: u32 = 0x1D4;
    const MOVIE_OLD_SOURCE_OFFSET: u32 = 0x230;
    const SOURCE_REFERENCE_COUNT_OFFSET: u32 = 0x0A;
    const TEXTURE_COORDINATES_OFFSET: u32 = 0x204;
    const LOOKUP_RESULT_OFFSET: u32 = 0x1E0;
    const FLOAT_ZERO_BITS: u32 = 0x0000_0000;
    const FLOAT_ONE_BITS: u32 = 0x3F80_0000;

    unsafe {
        let texture = (movie.wrapping_add(MOVIE_TEXTURE_OFFSET) as *const u32)
            .read_unaligned();
        let old_source_slot = (texture.wrapping_add(TEXTURE_SOURCE_SLOT_OFFSET) as *const u32)
            .read_unaligned();
        if old_source_slot != 0 {
            let old_source = (old_source_slot as *const u32).read_unaligned();
            if old_source != 0 {
                (movie.wrapping_add(MOVIE_OLD_SOURCE_OFFSET) as *mut u32)
                    .write_unaligned(old_source);
                let count = (old_source.wrapping_add(SOURCE_REFERENCE_COUNT_OFFSET) as *const u16)
                    .read_unaligned();
                (old_source.wrapping_add(SOURCE_REFERENCE_COUNT_OFFSET) as *mut u16)
                    .write_unaligned(count.wrapping_add(1));
            }
        }

        let new_count = (new_source.wrapping_add(SOURCE_REFERENCE_COUNT_OFFSET) as *const u16)
            .read_unaligned();
        (new_source.wrapping_add(SOURCE_REFERENCE_COUNT_OFFSET) as *mut u16)
            .write_unaligned(new_count.wrapping_add(1));
        let _release_result = lf_checker_rt::callee_thiscall!(1, u32, texture, new_source);

        (texture.wrapping_add(TEXTURE_COORDINATES_OFFSET) as *mut u32)
            .write_unaligned(FLOAT_ZERO_BITS);
        (texture.wrapping_add(TEXTURE_COORDINATES_OFFSET + 4) as *mut u32)
            .write_unaligned(FLOAT_ONE_BITS);
        (texture.wrapping_add(TEXTURE_COORDINATES_OFFSET + 8) as *mut u32)
            .write_unaligned(FLOAT_ONE_BITS);
        (texture.wrapping_add(TEXTURE_COORDINATES_OFFSET + 12) as *mut u32)
            .write_unaligned(FLOAT_ZERO_BITS);

        let mut movie_argument = movie;
        let argument_snapshot = &mut movie_argument as *mut u32 as u32;
        let lookup_result = lf_checker_rt::callee_cdecl!(2, u32, argument_snapshot, 0);
        let replacement_source = (lookup_result as *const u32).read_unaligned();
        (texture.wrapping_add(LOOKUP_RESULT_OFFSET) as *mut u32)
            .write_unaligned(replacement_source);
        replacement_source
    }
});
