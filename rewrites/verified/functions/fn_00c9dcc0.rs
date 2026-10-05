// original: 0x00C9DCC0 guarded_task_op_dispatch (proposed)

/// Run a chain of state guards, then dispatch one of three operation calls.
///
/// `this` points to the task object; ten stack words follow, of which `a3`
/// is an optional context pointer, `a5` a selector, `a7` a flag word and
/// `a10` a signed bound, the rest opaque words forwarded to the operation.
/// Each guard returns as-is when it fails: the state query must report zero
/// (or the context must be absent with a zero marker), `a5` must be -1 when
/// the context is absent, the flag byte at the controller's `+0x210` must be
/// clear, bits `0x300000` of its `+0x29c` word must be clear (and when bits
/// `0xc00000` are set, `a7` must carry `0x20`); those two flag guards return
/// the flags word itself. The range probe must return zero, and unless bit 5
/// of `a7` is set, a proximity check follows: the distance from the anchor
/// triple to the three global floats must not strictly exceed 400, else the
/// anchor pointer is returned. The worker lookup then
/// decides: a worker whose signed level byte does not exceed `a10` gets the
/// eleven-word operation; no worker allocates a fresh one (returning the
/// two-word fallback on failure) and runs the twelve-word operation followed
/// by the two-word completion.
///
/// Original: 0x00C9DCC0 (thiscall, ten stack words).
lf_checker_rt::export!(
    thiscall,
    rw_00C9DCC0(
        this: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: u32,
        a9: u32,
        a10: u32,
    ) -> u32 {
        unsafe {
            const CTL_OFF: u32 = 0x40;
            const CTX_MARKER: u32 = 0x6C;
            const G_ANCHOR: u32 = 0x128E340;
            const RANGE_LIMIT: f32 = 400.0;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn rd8(a: u32) -> u8 {
                unsafe { (a as *const u8).read() }
            }
            #[inline(always)]
            unsafe fn rdf(a: u32) -> f32 {
                unsafe { f32::from_bits(rd32(a)) }
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

            let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
            if (r1 as u8) != 0 && a3 != 0 && rd32(a3 + CTX_MARKER) == 0 {
                return r1;
            }
            if a5 != 0xFFFF_FFFF && a3 == 0 {
                return r1;
            }
            let ctl = rd32(this + CTL_OFF);
            if rd8(ctl + 0x210) != 0 {
                return r1;
            }
            let flags = rd32(ctl + 0x29C);
            if flags & 0x300000 != 0 {
                return flags;
            }
            if flags & 0xC00000 != 0 && a7 & 0x20 == 0 {
                return flags;
            }
            let r2: u32 =
                lf_checker_rt::callee_thiscall!(2, u32, rd32(ctl + 0x78), 0x14000000, 1);
            if r2 != 0 {
                return r2;
            }
            if (a7 >> 5) & 1 == 0 {
                let r3: u32 = lf_checker_rt::callee_thiscall!(3, u32, ctl, 0);
                if (r3 as u8) == 0 {
                    return r3;
                }
                let anchor = rd32(ctl + 0x20);
                let dx = sub(rdf(anchor + 0x30), rdf(lf_checker_rt::relocated(G_ANCHOR)));
                let dy = sub(
                    rdf(anchor + 0x34),
                    rdf(lf_checker_rt::relocated(G_ANCHOR) + 4),
                );
                let dz = sub(
                    rdf(anchor + 0x38),
                    rdf(lf_checker_rt::relocated(G_ANCHOR) + 8),
                );
                let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
                if dist2 > RANGE_LIMIT {
                    return anchor;
                }
            }
            let r4: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 0);
            if r4 != 0 {
                let level = rd8(r4 + 0x60) as i8 as i32;
                if (a10 as i32) < level {
                    return r4;
                }
                return lf_checker_rt::callee_thiscall!(
                    5, u32, r4, a1, a2, 0, a3, a5, a6, a4, a8, a9, a10, a7
                );
            }
            let r6: u32 = lf_checker_rt::callee_cdecl!(6, u32, 0x70);
            if r6 != 0 {
                let r7: u32 = lf_checker_rt::callee_thiscall!(
                    7, u32, r6, a1, a2, 0, 0, a3, a5, a6, a4, a8, a9, a10, a7
                );
                return lf_checker_rt::callee_thiscall!(8, u32, this, r7, 0);
            }
            lf_checker_rt::callee_thiscall!(9, u32, this, 0, 0)
        }
    }
);
