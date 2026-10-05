// original: 0x00CD7990 CEntitySeekPosCalculatorRadiusAngleOffset::vf1

/// Original: 0x00CD7990 (thiscall, three stack words, callee pops 12).
#[allow(clippy::too_many_lines)]
export!(thiscall, rw_00cd7990(this: u32, tag: u32, seek: u32, target: u32) -> u32 {
    unsafe {
        const SEEK_ANGLE: u32 = 0x1c;
        const SEEK_LINK: u32 = 0x20;
        const LINK_DIR_X: u32 = 0x10;
        const LINK_DIR_Y: u32 = 0x14;
        const LINK_BASE: u32 = 0x30;
        const DESC_BASE: u32 = 0x10;
        const CALC_SCALE: u32 = 0x04;
        const CALC_BASE_ANGLE: u32 = 0x08;
        const SIGN_FLIP: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let neg_const: u32 = *global::<u32>(0xFE8FA0);
        debug_assert_eq!(neg_const, SIGN_FLIP);

        let link = rd32(seek.wrapping_add(SEEK_LINK));
        let (dir_x, dir_y): (f32, f32) = if link == 0 {
            let angle_bits = rd32(seek.wrapping_add(SEEK_ANGLE));
            let cos_bits: u32 = callee_cdecl!(1, u32, angle_bits);
            let sin_bits: u32 = callee_cdecl!(2, u32, angle_bits);
            (
                f32::from_bits(cos_bits ^ neg_const),
                f32::from_bits(sin_bits),
            )
        } else {
            (rdf(link.wrapping_add(LINK_DIR_X)), rdf(link.wrapping_add(LINK_DIR_Y)))
        };

        let raw: f32 = callee_cdecl!(3, f32, dir_x.to_bits(), dir_y.to_bits(), 0, 0);
        let wrapped_once: f32 = callee_cdecl!(4, f32, raw.to_bits());
        let base_angle = rdf(this.wrapping_add(CALC_BASE_ANGLE));
        let wrapped: f32 = callee_cdecl!(4, f32, add(base_angle, wrapped_once).to_bits());

        let cos2_bits: u32 = callee_cdecl!(1, u32, wrapped.to_bits());
        let sin2_bits: u32 = callee_cdecl!(2, u32, wrapped.to_bits());
        let scale = rdf(this.wrapping_add(CALC_SCALE));
        let term_x = mul(scale, f32::from_bits(cos2_bits ^ neg_const));
        let term_y = mul(scale, f32::from_bits(sin2_bits));
        let term_z = mul(scale, 0.0f32);

        let link2 = rd32(seek.wrapping_add(SEEK_LINK));
        let base = if link2 == 0 {
            seek.wrapping_add(DESC_BASE)
        } else {
            link2.wrapping_add(LINK_BASE)
        };
        let acc_z = add(rdf(base.wrapping_add(8)), term_z);
        let acc_x = add(rdf(base), term_x);
        let acc_y = add(rdf(base.wrapping_add(4)), term_y);

        // Fourth word: the original reads its own uninitialised frame slot
        // here, which the checker fills with the defined stack fill (0).
        let frame = [acc_x.to_bits(), acc_y.to_bits(), acc_z.to_bits(), 0u32];
        let frame_ptr = frame.as_ptr() as u32;
        callee_cdecl!(5, u32, tag, frame_ptr, target)
    }
});
