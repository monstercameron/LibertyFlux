// original: 0x00bd4d40 NativeImpl_DRAW_MOVIE

/// Prefix the nine native sprite words with a zero flag and blend mode, append the fixed sort and layer values, and submit the sequence to the sprite draw queue. Return the queue helper's result.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd4d40(
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    arg_5: u32,
    arg_6: u32,
    arg_7: u32,
    arg_8: u32,
    arg_9: u32,
) -> u32 {
    const APPEND_DRAW_ENTRY: u32 = 1;
    const LAYER: u32 = 6;
    const SORT_GROUP: u32 = 1;
    const BLEND_MODE: u32 = -2i32 as u32;
    unsafe {
        lf_checker_rt::callee_cdecl!(
            APPEND_DRAW_ENTRY,
            u32,
            0,
            BLEND_MODE,
            arg_1,
            arg_2,
            arg_3,
            arg_4,
            arg_5,
            arg_6,
            arg_7,
            arg_8,
            arg_9,
            SORT_GROUP,
            LAYER,
        )
    }
}
