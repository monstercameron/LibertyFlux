// original: 0x00be4ba0 task_note_nearest_candidate (proposed)

/// Consider one object as the nearest candidate, keeping the best so far.
///
/// The shared best pair (object at `BEST_OBJ`, distance-squared at
/// `BEST_DIST`) and the query point (`QUERY + 0/4/8`) live in globals. Most
/// objects are measured: the squared distance from the query point to the
/// object's position (its link at `+ LINK_OFF` (0x20) plus `POS_BIAS`
/// (0x30), or `this + POS_INLINE` (0x10) when the link is null) is formed as
/// `(dy*y + dx*x) + dz*z` in that order, where the deltas subtract the
/// query point. An object whose area tag (low word at `+ AREA_OFF` (0x2e),
/// sign-extended) disagrees with the context's (`context + CTX_AREA`
/// (0x6c)) is skipped unless the context carries -1, and a null context
/// skips the check. The candidate wins when no best exists yet or its
/// distance is strictly below the best; winning stores the object and its
/// distance.
///
/// A few objects skip measuring: when the class field
/// (`(+ CLASS_OFF (0x28) >> 6) & 0xf`) is 2, 3 or 4, the state nibble
/// (`+ STATE_OFF (0x1e2) & 0xf`) is at least 2, a secondary object exists at
/// `+ AUX_OFF` (0x1bc) and its masked kind (`(+ KIND_OFF (0x28)) & 0x3c0`)
/// is `AUX_KIND` (0xc0), the object is accepted without touching the best
/// pair. Returns 1 (an 8-bit bool) on every path.
///
/// Float order is the original's; the below-best test is `!(best > dist)`
/// so NaN keeps the old best exactly like the original's compare-and-branch.
///
/// Original: 0x00be4ba0 (cdecl, two stack words: object, context-or-null).
lf_checker_rt::export!(cdecl, rw_00be4ba0(obj: u32, context: u32) -> u32 {
    unsafe {
        const CLASS_OFF: u32 = 0x28;
        const STATE_OFF: u32 = 0x1e2;
        const AUX_OFF: u32 = 0x1bc;
        const KIND_OFF: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const AUX_KIND: u32 = 0xc0;
        const LINK_OFF: u32 = 0x20;
        const POS_BIAS: u32 = 0x30;
        const POS_INLINE: u32 = 0x10;
        const AREA_OFF: u32 = 0x2e;
        const CTX_AREA: u32 = 0x6c;
        const ANY_AREA: u32 = 0xffffffff;
        const QUERY: u32 = 0x01682ec0;
        const BEST_OBJ: u32 = 0x0167f5a4;
        const BEST_DIST: u32 = 0x0167f630;
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
        let class = ((obj.wrapping_add(CLASS_OFF) as *const u32).read_unaligned() >> 6) & 0xf;
        if class > 1 && class < 5 {
            let state = (obj.wrapping_add(STATE_OFF) as *const u8).read() & 0xf;
            if state >= 2 {
                let aux = (obj.wrapping_add(AUX_OFF) as *const u32).read_unaligned();
                if aux != 0 {
                    let kind = (aux.wrapping_add(KIND_OFF) as *const u32).read_unaligned()
                        & KIND_MASK;
                    if kind == AUX_KIND {
                        return 1;
                    }
                }
            }
        }
        let link = (obj.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let pos = if link == 0 {
            obj.wrapping_add(POS_INLINE)
        } else {
            link.wrapping_add(POS_BIAS)
        };
        let qx = f32::from_bits(
            (lf_checker_rt::global::<u32>(QUERY) as *const u32).read_unaligned(),
        );
        let qy = f32::from_bits(
            (lf_checker_rt::global::<u32>(QUERY.wrapping_add(4)) as *const u32).read_unaligned(),
        );
        let qz = f32::from_bits(
            (lf_checker_rt::global::<u32>(QUERY.wrapping_add(8)) as *const u32).read_unaligned(),
        );
        let dx = sub(
            f32::from_bits((pos as *const u32).read_unaligned()),
            qx,
        );
        let dy = sub(
            f32::from_bits((pos.wrapping_add(4) as *const u32).read_unaligned()),
            qy,
        );
        let dz = sub(
            f32::from_bits((pos.wrapping_add(8) as *const u32).read_unaligned()),
            qz,
        );
        let dist = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        if context != 0 {
            let want = (obj.wrapping_add(AREA_OFF) as *const i16).read_unaligned() as i32 as u32;
            let have = (context.wrapping_add(CTX_AREA) as *const u32).read_unaligned();
            if want != have && have != ANY_AREA {
                return 1;
            }
        }
        let best_obj = lf_checker_rt::global::<u32>(BEST_OBJ);
        if (best_obj as *const u32).read_unaligned() != 0 {
            let best = f32::from_bits(
                (lf_checker_rt::global::<u32>(BEST_DIST) as *const u32).read_unaligned(),
            );
            if !(best > dist) {
                return 1;
            }
        }
        best_obj.write_unaligned(obj);
        (lf_checker_rt::global::<u32>(BEST_DIST) as *mut u32).write_unaligned(dist.to_bits());
        1
    }
});
