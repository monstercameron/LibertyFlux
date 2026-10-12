// original: 0x00B22AA0 portal_tracker_segment_test
/// Send the segment's XYZ delta, the object pointer and four caller words to
/// the scripted segment helper. The helper receives pointers to local delta
/// words and writes a candidate segment parameter through the second pointer.
/// Return true only when the helper accepts and its parameter is strictly
/// between zero and one. Thiscall with four stack arguments; the two local
/// pointer addresses vary by frame, so the contract checks their contents.
lf_checker_rt::export!(thiscall, rw_00b22aa0(this: u32, query_a: u32, query_b: u32, query_c: u32, query_d: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

        #[inline(always)]
        fn sub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        unsafe fn float_at(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }

        const ENDPOINT: u32 = 0x10;
        const THRESHOLD: u32 = 0x00FE_88E8;

        let start_x = float_at(this);
        let start_y = float_at(this.wrapping_add(4));
        let start_z = float_at(this.wrapping_add(8));
        let end_x = float_at(this.wrapping_add(ENDPOINT));
        let end_y = float_at(this.wrapping_add(ENDPOINT + 4));
        let end_z = float_at(this.wrapping_add(ENDPOINT + 8));
        let delta_x = sub(end_x, start_x);
        let delta_y = sub(end_y, start_y);
        let delta_z = sub(end_z, start_z);
        let mut local_words = [0, delta_x.to_bits(), delta_y.to_bits(), delta_z.to_bits()];
        let delta_pointer = local_words.as_mut_ptr().wrapping_add(1) as u32;
        let candidate_pointer = local_words.as_mut_ptr() as u32;

        let helper_succeeded = lf_checker_rt::callee_cdecl!(
            1, u32, query_a, query_b, query_c, this, delta_pointer, candidate_pointer, query_d
        ) & 0xff != 0;
        if !helper_succeeded {
            return 0;
        }

        let candidate = f32::from_bits(local_words[0]);
        let limit_bits = read_u32(lf_checker_rt::relocated(THRESHOLD) as u32);
        let limit = f32::from_bits(limit_bits);
        let below_limit = candidate < limit;
        let positive = candidate > 0.0;
        u32::from(positive && below_limit)
    }
});
