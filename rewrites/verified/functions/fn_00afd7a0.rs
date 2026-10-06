// original: 0x00afd7a0 retry_sample_pair (proposed)

/// Pick a random entry pair from two runtime tables, walk them to a pair of
/// parameter blocks, and blend the blocks' stored vectors, retrying with fresh
/// random draws until every gate passes (at most 41 tries).
///
/// Arguments (cdecl, fifteen stack words; only eight are read): `a0` points to
/// a record whose float at `+8` steers one gate; `a1` is a float forwarded to
/// the acceptance callee; the low bytes of `a8` and `a14` are flag bytes (when
/// set they skip the hidden-flag gate and the record-distance gate); `a10`
/// receives four floats (the blended point plus one word the original reads
/// from its own uninitialised frame, always zero under the checker's defined
/// stack fill); `a11`/`a12` receive the two picked table entries in an order
/// that depends on which return path is taken; `a13` receives the blend
/// factor, or one minus it on the mirrored path. Returns 1 on success, 0 when
/// the tables are empty, a lookup misses, or the tries run out.
///
/// Tables (all in writable game data, zero until the game fills them): a count
/// at `COUNT`, a byte table at `BYTE_TAB`, an entry-pair table at `PAIR_TAB`
/// (two words per index), and two pointer tables `BASE_TAB`/`AUX_TAB` indexed
/// by the low 16 bits of an entry. Each parameter block is 32 bytes; the
/// words at `+0x14/+0x16/+0x18` are signed coordinates scaled by 0.125, 0.125
/// and 0.015625, byte `+0x1b` holds flag bits (0xe0), byte `+0x1c` a level in
/// the low nibble, byte `+0x1e` a hidden bit (0x80) and byte `+0x1f` a near bit
/// (2). The float constants come from the read-only section.
///
/// Per try: draw two masked random words to pick an index and a spare level,
/// resolve both entries through `BASE_TAB` (a null entry returns 0), take the
/// minimum of the two levels, draw a third word for a level gate, then the
/// vector gates (the scaled z-gap must not exceed half the x/y distance; the
/// record float must be within `DIST_LIMIT` of a scaled z unless skipped), a
/// blend factor from a fourth full-range draw, two ordered-pair checks through
/// one callee (each gate calls it with the flagged block first), and the
/// acceptance callee over the blended point. Signed comparisons: the spare
/// level, the level gate and the final draw bound (0x3fff) are signed; the try
/// counter bound (0x28) is unsigned; the float gates are unordered-false
/// above / unordered-true below-or-equal, matching `comiss`+`ja`/`jbe`.
///
/// Original: 0x00afd7a0 (cdecl; reads stack words 0, 1, 8, 10..14).
lf_checker_rt::export!(cdecl, rw_00afd7a0(a0: u32, a1: u32, _a2: u32, _a3: u32, _a4: u32, _a5: u32, _a6: u32, _a7: u32, a8: u32, _a9: u32, a10: u32, a11: u32, a12: u32, a13: u32, a14: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x01600180;
        const BYTE_TAB: u32 = 0x01600188;
        const PAIR_TAB: u32 = 0x01600320;
        const BASE_TAB: u32 = 0x01178284;
        const AUX_TAB: u32 = 0x01178384;
        const DIST_LIMIT: u32 = 0x0103ff9c;
        const FAR_VALUE: u32 = 0x0103ff88;
        const PAIR_OBJ: u32 = 0x01177a80;
        const K_RAND: f32 = f32::from_bits(0x3800_0000); // 2^-15
        const K_LEVEL: f32 = f32::from_bits(0x4080_0000); // 4.0
        const K_MINLV: f32 = f32::from_bits(0x4170_0000); // 15.0
        const K_XY: f32 = f32::from_bits(0x3e00_0000); // 0.125
        const K_Z: f32 = f32::from_bits(0x3c80_0000); // 0.015625
        const K_QTR: f32 = f32::from_bits(0x3e80_0000); // 0.25
        const K_BLEND: f32 = f32::from_bits(0x3800_0100);
        const K_BLEND_MUL: f32 = f32::from_bits(0x3f19_999a); // 0.6
        const K_BLEND_ADD: f32 = f32::from_bits(0x3e4c_cccd); // 0.2
        const K_FAR_MUL: f32 = f32::from_bits(0x3fc0_0000); // 1.5
        const ONE: f32 = 1.0;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const BLOCK_STRIDE: u32 = 0x20;
        const OFF_X: u32 = 0x14;
        const OFF_Y: u32 = 0x16;
        const OFF_Z: u32 = 0x18;
        const OFF_FLAGS: u32 = 0x1b;
        const OFF_LEVEL: u32 = 0x1c;
        const OFF_HIDDEN: u32 = 0x1e;
        const OFF_NEAR: u32 = 0x1f;
        const FLAG_ORDERED: u8 = 0xe0;
        const FLAG_HIDDEN: u8 = 0x80;
        const FLAG_NEAR: u8 = 0x02;
        const MAX_TRIES: u32 = 0x28;
        const FINAL_BOUND: i32 = 0x3fff;
        const RAND_CALLEE: u32 = 1;
        const PAIR_CALLEE: u32 = 2;
        const ACCEPT_CALLEE: u32 = 3;
        const LOOKUP_CALLEE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
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
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & ABS_MASK)
        }
        /// Truncate toward zero exactly like `cvttss2si` (out of range or
        /// NaN gives `i32::MIN`, where a Rust cast would saturate).
        #[inline(always)]
        fn trunc_f32(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }

        let count = g32(COUNT);
        if count == 0 {
            return 0;
        }
        let exhausted: u8 = 0;
        let mut tries: u32 = 0;
        loop {
            // Unsigned bound: tries 1..=41 run, then the zero slot returns.
            if tries > MAX_TRIES {
                return exhausted as u32;
            }
            tries = tries.wrapping_add(1);

            let r1 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,) & 0xffff;
            let pick = trunc_f32(mul(mul(r1 as f32, K_RAND), (count as i32) as f32));
            let r2 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,) & 0xffff;
            let spare = trunc_f32(mul(mul(r2 as f32, K_RAND), K_LEVEL));
            // Signed: a negative spare always passes a non-negative byte.
            if spare >= rd8(lf_checker_rt::relocated(BYTE_TAB).wrapping_add(pick as u32)) as i32 {
                continue;
            }
            let pair = lf_checker_rt::relocated(PAIR_TAB).wrapping_add((pick as u32).wrapping_mul(8));
            let entry_a = rd32(pair);
            let entry_b = rd32(pair.wrapping_add(4));
            let base_a = g32(BASE_TAB.wrapping_add((entry_a & 0xffff).wrapping_mul(4)));
            if base_a == 0 {
                return 0;
            }
            let base_b = g32(BASE_TAB.wrapping_add((entry_b & 0xffff).wrapping_mul(4)));
            if base_b == 0 {
                return 0;
            }
            let obj_a = base_a.wrapping_add((entry_a >> 16).wrapping_mul(BLOCK_STRIDE));
            let obj_b = base_b.wrapping_add((entry_b >> 16).wrapping_mul(BLOCK_STRIDE));
            if (a8 as u8) == 0 {
                if (rd8(obj_a.wrapping_add(OFF_HIDDEN)) & FLAG_HIDDEN) != 0 {
                    continue;
                }
                if (rd8(obj_b.wrapping_add(OFF_HIDDEN)) & FLAG_HIDDEN) != 0 {
                    continue;
                }
            }
            let min_level = (rd8(obj_a.wrapping_add(OFF_LEVEL)) & 0xf)
                .min(rd8(obj_b.wrapping_add(OFF_LEVEL)) & 0xf);
            let r3 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,) & 0xffff;
            let level_roll = trunc_f32(mul(mul(r3 as f32, K_RAND), K_MINLV));
            // Signed against the low byte of the minimum (at most 15).
            if level_roll > min_level as i32 {
                continue;
            }
            let x1 = mul(rd16(obj_a.wrapping_add(OFF_X)) as f32, K_XY);
            let y1 = mul(rd16(obj_a.wrapping_add(OFF_Y)) as f32, K_XY);
            let z1 = mul(rd16(obj_a.wrapping_add(OFF_Z)) as f32, K_Z);
            let x2 = mul(rd16(obj_b.wrapping_add(OFF_X)) as f32, K_XY);
            let y2 = mul(rd16(obj_b.wrapping_add(OFF_Y)) as f32, K_XY);
            let z2 = mul(rd16(obj_b.wrapping_add(OFF_Z)) as f32, K_Z);
            let dx = sub(x1, x2);
            let dy = sub(y1, y2);
            let mut spread = add(mul(dy, dy), mul(dx, dx));
            let dz = absf(sub(z1, z2));
            let dz2 = mul(dz, dz);
            spread = mul(spread, K_QTR);
            // Unordered-false: a NaN gap never retries.
            if dz2 > spread {
                continue;
            }
            if (a14 as u8) == 0 && (rd8(obj_a.wrapping_add(OFF_NEAR)) & FLAG_NEAR) == 0 {
                let limit = gf(DIST_LIMIT);
                let anchor = rdf(a0.wrapping_add(8));
                // Unordered-true: a NaN distance passes this gate.
                if !(absf(sub(anchor, z1)) > limit) {
                    // Falls through to the blend below.
                } else if absf(sub(anchor, z2)) > limit {
                    continue;
                }
            }
            let r4 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
            let t = add(mul(mul((r4 as i32) as f32, K_BLEND), K_BLEND_MUL), K_BLEND_ADD);
            let px = add(mul(sub(x2, x1), t), x1);
            let py = add(mul(sub(y2, y1), t), y1);
            let pz = add(mul(sub(z2, z1), t), z1);
            let obj = lf_checker_rt::relocated(PAIR_OBJ);
            let mut first_ok: u8 = 0;
            if (rd8(obj_b.wrapping_add(OFF_FLAGS)) & FLAG_ORDERED) != 0 {
                first_ok = 1;
                let ans: u32 = lf_checker_rt::callee_thiscall!(PAIR_CALLEE, u32, obj, obj_b, obj_a);
                if (ans as u8) == 0 {
                    first_ok = 0;
                }
            }
            let mut second_ok: u8 = 0;
            if (rd8(obj_a.wrapping_add(OFF_FLAGS)) & FLAG_ORDERED) != 0 {
                let ans: u32 = lf_checker_rt::callee_thiscall!(PAIR_CALLEE, u32, obj, obj_a, obj_b);
                if (ans as u8) != 0 {
                    second_ok = 1;
                }
            }
            if first_ok != 0 && second_ok != 0 {
                continue;
            }
            let gate = if (rd8(obj_a.wrapping_add(OFF_NEAR)) & FLAG_NEAR) != 0 {
                mul(gf(FAR_VALUE), K_FAR_MUL)
            } else {
                0.0
            };
            let point = [px.to_bits(), py.to_bits(), pz.to_bits()];
            let accepted: u32 = lf_checker_rt::callee_cdecl!(
                ACCEPT_CALLEE, u32,
                core::ptr::addr_of!(point) as u32, a0, a1, gate.to_bits()
            );
            if (accepted as u8) == 0 {
                continue;
            }
            wrf(a10, px);
            wrf(a10.wrapping_add(4), py);
            wrf(a10.wrapping_add(8), pz);
            // The original reads one word past its stored point, from frame
            // space it never wrote; the harness defines that fill as zero.
            wrf(a10.wrapping_add(12), f32::from_bits(0));
            if first_ok != 0 {
                wr32(a11, entry_b);
                wr32(a12, entry_a);
                wrf(a13, sub(ONE, t));
                return 1;
            }
            if second_ok != 0 {
                wr32(a11, entry_a);
                wr32(a12, entry_b);
                wrf(a13, t);
                return 1;
            }
            let found: u32 =
                lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, obj, entry_a, entry_b);
            let aux = g32(AUX_TAB.wrapping_add((entry_a & 0xffff).wrapping_mul(4)));
            let tag = rd8(aux.wrapping_add(found.wrapping_mul(8).wrapping_add(5)));
            if (tag & 0x38) == 0 {
                wr32(a11, entry_a);
                wr32(a12, entry_b);
                wrf(a13, t);
                return 1;
            }
            if (tag & 7) == 0 {
                wr32(a11, entry_b);
                wr32(a12, entry_a);
                wrf(a13, t);
                return 1;
            }
            let r5 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
            // Signed: 0x80000000..0xffffffff count as below the bound.
            if (r5 as i32) >= FINAL_BOUND {
                wr32(a11, entry_b);
                wr32(a12, entry_a);
                wrf(a13, sub(ONE, t));
                return 1;
            }
            wr32(a11, entry_a);
            wr32(a12, entry_b);
            wrf(a13, t);
            return 1;
        }
    }
});
