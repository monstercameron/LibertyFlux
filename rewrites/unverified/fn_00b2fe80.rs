// original: 0x00b2fe80 ped_task_range_check (proposed)

/// Decide whether a point is inside either of two scaled ranges.
///
/// `obj` points at a record whose dword at `LINK_SLOT` selects the reference
/// point: when zero, the three floats at `obj + LOCAL_PT` are used and two
/// initialisation callees run (the second fills `LINK_SLOT`); otherwise the
/// floats at `link + FAR_PT` are used. Row `index` of the global int table at
/// `ROW_BASE` (stride `ROW_STRIDE`, signed dwords at `+ROW_X`/`+ROW_Y`,
/// converted exactly to float and multiplied by `scale`) gives two radii.
/// Let `dd` be the squared distance from `vec` to the reference point. The
/// first test passes when the scaled X radius is positive, `flag_a` is set,
/// its square covers `dd`, and either `flag_c` is set or the dot product of
/// the offset with the direction at `link + DIR_{X,Y,Z}` is positive. The
/// fallback test passes when the scaled Y radius is positive, `flag_b` is
/// set, and its square covers `dd`. Returns 1 when either test passes, else
/// 0. All float comparisons keep the original's unordered (NaN) behaviour.
///
/// Original: cdecl, eight stack words (object, one unread word, table index,
/// point, two flag bytes, scale bits, one flag byte); only the low byte of
/// each flag word is read.
lf_checker_rt::export!(cdecl, rw_00b2fe80(obj: u32, _pad: u32, index: u32, vec: u32, flag_b: u32, flag_a: u32, scale_bits: u32, flag_c: u32) -> u32 {
    unsafe {
        const LINK_SLOT: u32 = 0x20;
        const LOCAL_PT: u32 = 0x10;
        const FAR_PT: u32 = 0x30;
        const ROW_BASE: u32 = 0x0166_1410;
        const ROW_STRIDE: u32 = 0x34;
        const ROW_X: u32 = 0x0C;
        const ROW_Y: u32 = 0x10;
        const DIR_X: u32 = 0x10;
        const DIR_Y: u32 = 0x14;
        const DIR_Z: u32 = 0x18;
        const CALLEE_INIT: u32 = 1;
        const CALLEE_FILL_LINK: u32 = 2;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let link = (obj.wrapping_add(LINK_SLOT) as *const u32).read_unaligned();
        let src = if link == 0 {
            obj.wrapping_add(LOCAL_PT)
        } else {
            link.wrapping_add(FAR_PT)
        };
        let s0 = (src as *const f32).read_unaligned();
        let s1 = (src.wrapping_add(4) as *const f32).read_unaligned();
        let s2 = (src.wrapping_add(8) as *const f32).read_unaligned();
        if link == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_INIT, u32, obj);
            let old =
                (obj.wrapping_add(LINK_SLOT) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_FILL_LINK,
                u32,
                obj.wrapping_add(LOCAL_PT),
                old
            );
        }
        let row = ROW_BASE.wrapping_add(index.wrapping_mul(ROW_STRIDE));
        let dir = (obj.wrapping_add(LINK_SLOT) as *const u32).read_unaligned();
        let base = lf_checker_rt::relocated(row);
        let xi = (base.wrapping_add(ROW_X) as *const i32).read_unaligned();
        let yi = (base.wrapping_add(ROW_Y) as *const i32).read_unaligned();
        let scale = f32::from_bits(scale_bits);
        let dx = sub((vec as *const f32).read_unaligned(), s0);
        let dy = sub((vec.wrapping_add(4) as *const f32).read_unaligned(), s1);
        let dz = sub((vec.wrapping_add(8) as *const f32).read_unaligned(), s2);
        let mut xs = mul(xi as f32, scale);
        let mut ys = mul(yi as f32, scale);
        let mut dist = add(mul(dy, dy), mul(dx, dx));
        dist = add(dist, mul(dz, dz));
        // First test: comiss+jbe/jb keep NaN on the failing side.
        let mut first_ok = false;
        if xs > 0.0 && (flag_a as u8) != 0 {
            xs = mul(xs, xs);
            if xs >= dist {
                if (flag_c as u8) != 0 {
                    first_ok = true;
                } else {
                    let cx = (dir.wrapping_add(DIR_X) as *const f32)
                        .read_unaligned();
                    let cy = (dir.wrapping_add(DIR_Y) as *const f32)
                        .read_unaligned();
                    let cz = (dir.wrapping_add(DIR_Z) as *const f32)
                        .read_unaligned();
                    let dot = add(add(mul(dy, cy), mul(dx, cx)), mul(dz, cz));
                    if dot > 0.0 {
                        first_ok = true;
                    }
                }
            }
        }
        if first_ok {
            return 1;
        }
        // Fallback test: comiss+jbe/jae likewise fail on NaN.
        if !(ys > 0.0) {
            return 0;
        }
        if (flag_b as u8) == 0 {
            return 0;
        }
        ys = mul(ys, ys);
        if ys >= dist { 1 } else { 0 }
    }
});
