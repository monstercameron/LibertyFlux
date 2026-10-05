// original: 0x00c8d240 facing_check_by_type (proposed)

/// Check that an entity faces a direction, by entity kind and subtype.
///
/// `ent` points to the entity, `v` to 3 floats giving the tested
/// direction. The check applies only when the entity's low 3 bits (its
/// kind) are 2, 3, 4 or 5; any other kind accepts at once (returns 1).
/// Entities with a zero pointer, zero kind bits, a non-zero liveness
/// probe (callee 1, cdecl), or a subtype (bits 9..7 of the first word)
/// above 6 are rejected (return 0).
///
/// Each of the 7 subtypes fetches a direction from the entity (callee 3,
/// thiscall, returning the direction pointer) and dots it against the
/// tested direction; subtype 0 uses the direction as given, the others
/// first run it through a normaliser (callee 2, thiscall, in place;
/// subtype 1 flattens it to the horizontal plane first). The dot product
/// must not be below a per-subtype limit. Subtypes 3-6 then fetch a
/// second direction and run a second test of the same shape against a
/// second limit: subtypes 3 and 5 require `!(LIMIT_B > value)`-style
/// acceptance from above (their comparison reads `limit < value` and
/// rejects), subtypes 4 and 6 require the value not below their limit.
/// The second value mixes the direction with signed zeros in the exact
/// operation order below (a NaN input propagates through the multiplies,
/// so the order is part of the behaviour).
///
/// All float gates are ordered `>=` comparisons matching the original's
/// `comiss`+`jb` (below-or-unordered rejects: a NaN operand is rejected,
/// never accepted).
///
/// Original: 0x00c8d240 (cdecl, two stack words). Returns 1/0 in `al`.
lf_checker_rt::export!(cdecl, rw_00c8d240(ent: u32, v: u32) -> u32 {
    unsafe {
        const LIM_SUB0: u32 = 0x00fe8628;
        const LIM_SUB1: u32 = 0x00ed6d28;
        const LIM_SUB2: u32 = 0x00fe8878;
        const LIM_SUB34: u32 = 0x00fe8844;
        const LIM_SUB35B: u32 = 0x00fe87e4;
        const LIM_SUB46B: u32 = 0x00fe8d70;

        #[inline(always)]
        unsafe fn rd(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn fetch(ent: u32) -> (f32, f32, f32) {
            unsafe {
                let mut buf = [0u32; 3];
                let out = &mut buf as *mut [u32; 3] as u32;
                let p = lf_checker_rt::callee_thiscall!(3u32, u32, ent, out, 0u32);
                (
                    f32::from_bits((p as *const u32).read_unaligned()),
                    f32::from_bits(((p + 4) as *const u32).read_unaligned()),
                    f32::from_bits(((p + 8) as *const u32).read_unaligned()),
                )
            }
        }
        #[inline(always)]
        unsafe fn normalised(vx: f32, vy: f32, vz: f32) -> (f32, f32, f32) {
            unsafe {
                let mut buf = [vx.to_bits(), vy.to_bits(), vz.to_bits()];
                let b = &mut buf as *mut [u32; 3] as u32;
                lf_checker_rt::callee_thiscall!(2u32, u32, b);
                (
                    f32::from_bits(buf[0]),
                    f32::from_bits(buf[1]),
                    f32::from_bits(buf[2]),
                )
            }
        }
        /// Second test, subtypes 3 and 5: reject when `limit < value`.
        #[inline(always)]
        fn second_a(x: f32, y: f32, z: f32, bx: f32, by: f32, bz: f32, limit: f32) -> bool {
            let zero = 0.0f32;
            let z0 = mul(z, zero);
            let t3 = sub(z0, y);
            let t4 = sub(x, z0);
            let y0 = mul(y, zero);
            let x0 = mul(x, zero);
            let t5 = sub(y0, x0);
            let p3 = mul(t3, bx);
            let p4 = mul(t4, by);
            let s = add(p4, p3);
            let p5 = mul(t5, bz);
            let val = add(s, p5);
            limit >= val
        }
        /// Second test, subtypes 4 and 6: reject when `value < limit`.
        #[inline(always)]
        fn second_b(x: f32, y: f32, z: f32, bx: f32, by: f32, bz: f32, limit: f32) -> bool {
            let zero = 0.0f32;
            let z0 = mul(z, zero);
            let t3 = sub(x, z0);
            let t4 = sub(z0, y);
            let y0 = mul(y, zero);
            let x0 = mul(x, zero);
            let t5 = sub(y0, x0);
            let p4 = mul(t4, bx);
            let p3 = mul(t3, by);
            let s = add(p3, p4);
            let p5 = mul(t5, bz);
            let val = add(s, p5);
            val >= limit
        }

        if ent == 0 {
            return 0;
        }
        let head = rd32(ent);
        let kind = head & 7;
        if kind == 0 {
            return 0;
        }
        if (lf_checker_rt::callee_cdecl!(1u32, u32, ent) & 0xff) != 0 {
            return 0;
        }
        if kind != 2 && kind != 3 && kind != 4 && kind != 5 {
            return 1;
        }
        let sub_ = (head >> 7) & 7;
        if sub_ > 6 {
            return 0;
        }
        let (vx, vy, vz) = (rd(v), rd(v + 4), rd(v + 8));
        match sub_ {
            0 => {
                let (ox, oy, oz) = fetch(ent);
                let dot = add(add(mul(oy, vy), mul(vx, ox)), mul(oz, vz));
                (dot >= rd(lf_checker_rt::relocated(LIM_SUB0))) as u32
            }
            1 => {
                let (bx, by, bz) = normalised(vx, vy, 0.0);
                let (ox, oy, oz) = fetch(ent);
                let dot = add(add(mul(oy, by), mul(ox, bx)), mul(oz, bz));
                (dot >= rd(lf_checker_rt::relocated(LIM_SUB1))) as u32
            }
            2 => {
                let (bx, by, bz) = normalised(vx, vy, vz);
                let (ox, oy, oz) = fetch(ent);
                let dot = add(add(mul(oy, by), mul(ox, bx)), mul(oz, bz));
                (dot >= rd(lf_checker_rt::relocated(LIM_SUB2))) as u32
            }
            3 => {
                let (bx, by, bz) = normalised(vx, vy, vz);
                let (ox, oy, oz) = fetch(ent);
                let dot = add(add(mul(oy, by), mul(bx, ox)), mul(oz, bz));
                if !(dot >= rd(lf_checker_rt::relocated(LIM_SUB34))) {
                    return 0;
                }
                let (x, y, z) = fetch(ent);
                second_a(x, y, z, bx, by, bz, rd(lf_checker_rt::relocated(LIM_SUB35B))) as u32
            }
            4 => {
                let (bx, by, bz) = normalised(vx, vy, vz);
                let (ox, oy, oz) = fetch(ent);
                let dot = add(add(mul(oy, by), mul(bx, ox)), mul(oz, bz));
                if !(dot >= rd(lf_checker_rt::relocated(LIM_SUB34))) {
                    return 0;
                }
                let (x, y, z) = fetch(ent);
                second_b(x, y, z, bx, by, bz, rd(lf_checker_rt::relocated(LIM_SUB46B))) as u32
            }
            5 => {
                let (bx, by, bz) = normalised(vx, vy, vz);
                let (ox, oy, oz) = fetch(ent);
                let dot = add(add(mul(oy, by), mul(bx, ox)), mul(oz, bz));
                if !(dot >= rd(lf_checker_rt::relocated(LIM_SUB2))) {
                    return 0;
                }
                let (x, y, z) = fetch(ent);
                second_a(x, y, z, bx, by, bz, rd(lf_checker_rt::relocated(LIM_SUB35B))) as u32
            }
            _ => {
                // Subtype 6 (the `sub_ > 6` guard leaves only 6 here).
                let (bx, by, bz) = normalised(vx, vy, vz);
                let (ox, oy, oz) = fetch(ent);
                let dot = add(add(mul(oy, by), mul(ox, bx)), mul(oz, bz));
                if !(dot >= rd(lf_checker_rt::relocated(LIM_SUB2))) {
                    return 0;
                }
                let (x, y, z) = fetch(ent);
                second_b(x, y, z, bx, by, bz, rd(lf_checker_rt::relocated(LIM_SUB46B))) as u32
            }
        }
    }
});
