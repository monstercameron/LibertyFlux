// original: 0x008d7140 combine_frames_and_dispatch
/// Combine two setup vectors, optionally negate one, dispatch the result.
///
/// Asks the setup helper twice for 4-float vectors, takes their dot
/// product, and when it is strictly negative
/// negates the second vector's copy (otherwise uses it as is). Passes the
/// chosen vector with the first vector and the incoming level to the
/// combine helper, which fills a 16-byte frame answer, then hands that
/// answer to the sink helper with the incoming handle.
export!(cdecl, rw_008d7140(first_arg: u32, second_arg: u32, level: f32, handle: u32) -> u32 {
    unsafe {
        use core::hint::black_box;
        let mut first = [0u32; 4];
        let mut second = [0u32; 4];
        callee_thiscall!(1, u32, first.as_mut_ptr() as u32, first_arg);
        callee_thiscall!(2, u32, second.as_mut_ptr() as u32, second_arg);
        let f = |w: u32| f32::from_bits(w);
        let mut blend = black_box(black_box(f(first[0])) * black_box(f(second[0])));
        blend = black_box(blend + black_box(black_box(f(first[1])) * black_box(f(second[1]))));
        blend = black_box(blend + black_box(black_box(f(first[2])) * black_box(f(second[2]))));
        blend = black_box(blend + black_box(black_box(f(first[3])) * black_box(f(second[3]))));
        let mut picked = second;
        if blend < 0.0 {
            let mask = *(global::<u32>(0x00fe8fa0) as *const u32);
            picked[0] ^= mask;
            picked[1] ^= mask;
            picked[2] ^= mask;
            picked[3] ^= mask;
        }
        let mut answer = [0u32; 4];
        callee_thiscall!(
            3,
            u32,
            answer.as_mut_ptr() as u32,
            level.to_bits(),
            first.as_ptr() as u32,
            picked.as_ptr() as u32
        );
        callee_thiscall!(4, u32, handle, answer.as_mut_ptr() as u32);
        0
    }
});
