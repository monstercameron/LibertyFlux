// original: 0x00a3ad80 vehicle_dir_filter (proposed)

/// Filter a 3-float direction into `dst` from `src`, scaled by `flag`.
///
/// With `s = inv_len * (1 - src[2]) * 0.5`, where `inv_len` is 0 when
/// `src[0]^2 + src[1]^2` is exactly zero and `1/sqrt` of it otherwise,
/// writes
/// `dst = 0.5 * (1 + [x*s, y*s, 0])`, and when `flag != 0` scales the
/// result by 128. All arithmetic is single precision in the original's
/// operand order. Stdcall/3 (dst, src, flag byte), returns `dst`.
lf_checker_rt::export!(stdcall, rw_00a3ad80(dst: u32, src: u32, flag: u32) -> u32 {
    unsafe {
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        const HALF: f32 = f32::from_bits(0x3F00_0000);
        const SCALE: f32 = f32::from_bits(0x4300_0000); // 128.0
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let x = f32::from_bits(core::ptr::read_unaligned(src as *const u32));
        let y = f32::from_bits(core::ptr::read_unaligned((src + 4) as *const u32));
        let z = f32::from_bits(core::ptr::read_unaligned((src + 8) as *const u32));
        let r2 = add(mul(x, x), mul(y, y));
        let k = mul(sub(ONE, z), HALF);
        core::ptr::write_unaligned(dst as *mut u32, x.to_bits());
        core::ptr::write_unaligned((dst + 4) as *mut u32, y.to_bits());
        core::ptr::write_unaligned((dst + 8) as *mut u32, 0);
        // Zero guard: an exact zero stays zero; anything else, NaN
        // included, takes the inverse square root. (The ucomiss/lahf/test/jp
        // sequence jumps unless ordered-equal to zero.)
        let inv = if r2 == 0.0 { 0.0 } else { div(ONE, r2.sqrt()) };
        let oz = mul(mul(mul(inv, 0.0), k), HALF);
        let oy = mul(add(mul(mul(inv, y), k), ONE), HALF);
        let ox = mul(add(mul(mul(inv, x), k), ONE), HALF);
        let (ox, oy, oz) = if (flag & 0xFF) == 0 {
            (ox, oy, oz)
        } else {
            (mul(ox, SCALE), mul(oy, SCALE), mul(oz, SCALE))
        };
        core::ptr::write_unaligned(dst as *mut u32, ox.to_bits());
        core::ptr::write_unaligned((dst + 4) as *mut u32, oy.to_bits());
        core::ptr::write_unaligned((dst + 8) as *mut u32, oz.to_bits());
        dst
    }
});
