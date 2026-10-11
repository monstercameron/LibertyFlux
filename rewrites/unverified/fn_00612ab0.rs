// original: 0x00612AB0 physics_scale_track_weights

/// Divide the supplied scalar weight by the running track-weight total from
/// the scripted helper call, multiply each selected track's +4 float by
/// that ratio, and store the original scalar at `this + 0x104`. The selected
/// range begins at byte `+0x10e` and has byte count `+0x10f`; the track array
/// is reached through `track_container + 0xd4`. The helper is called through
/// the checker runtime so its return and arguments are compared on both
/// sides. This is a 32-bit thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00612AB0(this: u32, scalar_bits: u32, track_container: u32) -> u32 {
    #[inline(always)]
    fn div_ordered(numerator: f32, denominator: f32) -> f32 {
        core::hint::black_box(numerator) / core::hint::black_box(denominator)
    }
    #[inline(always)]
    fn mul_ordered(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    unsafe {
        const START_INDEX: u32 = 0x10e;
        const TRACK_COUNT: u32 = 0x10f;
        const TRACKS_FIELD: u32 = 0xd4;
        const TRACK_WEIGHT: u32 = 4;
        const STORED_SCALAR: u32 = 0x104;
        const TOTAL_HELPER: u32 = 1;
        let total = lf_checker_rt::callee_thiscall!(TOTAL_HELPER, f32, this, track_container);
        let scalar = f32::from_bits(scalar_bits);
        let factor = div_ordered(scalar, total);
        let start = ((this + START_INDEX) as *const u8).read() as u32;
        let count = ((this + TRACK_COUNT) as *const u8).read() as u32;
        if count != 0 {
            let tracks = (((track_container + TRACKS_FIELD) as *const u32).read_unaligned());
            for step in 0..count {
                let track = ((tracks + (start + step) * 4) as *const u32).read_unaligned();
                let old_weight = ((track + TRACK_WEIGHT) as *const f32).read_unaligned();
                ((track + TRACK_WEIGHT) as *mut f32).write_unaligned(mul_ordered(factor, old_weight));
            }
        }
        ((this + STORED_SCALAR) as *mut u32).write_unaligned(scalar_bits);
        0
    }
});
