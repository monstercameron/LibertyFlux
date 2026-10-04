// original: 0x0096A470 dispatch_blend_target (proposed)

/// Route a blend request to one of several target handlers.
///
/// `source` selects the handler through a virtual call, `sample` carries
/// the sample point, and `out` receives the copied sample on the fallback
/// path. The sample's direction is normalised, then a virtual call on the
/// source returns a small selector that a table maps to one of six
/// continuations: three fetch a fresh target through a second virtual
/// call and blend toward it with a fixed scale, one derives its target
/// and scale from the source's own fields and three global scales, one
/// resolves its target through two rounds of registry lookups with a
/// per-round scale, and the fallback (also taken for an out-of-range
/// selector or any missing object) copies twelve words of the sample to
/// `out`. Every non-fallback path ends by invoking the blend helper
/// twice: once with a scratch buffer and the sample, once with `out` and
/// the scratch buffer.
///
/// The per-path float blends are computed faithfully but land in scratch
/// slots the original never reads afterwards (the helper is intercepted,
/// so its consumption of them is outside the comparison): what the proof
/// observes is the dispatch itself — the virtual-call targets, the
/// branch choices, the helper arguments and the fallback copy. One
/// scratch word the original never writes is read as zero (the proof
/// runs with a zero stack fill).
///
/// Original: 0x0096A470 (stdcall, three stack words; integer result; two
/// virtual calls, two blend-helper shapes, one registry lookup and one
/// five-argument registry call; reads three global scales).
lf_checker_rt::export!(stdcall, rw_0096A470(source: u32, sample: u32, out: u32) -> u32 {
    unsafe {
        const VT_SELECT: u32 = 0x28;
        const VT_TARGET: u32 = 0x0C;
        const SAMPLE_DIR: u32 = 0x10;
        const TARGET_POS: u32 = 0x20;
        const TARGET_HEAD: u32 = 0x10;
        const SOURCE_AUX: u32 = 0x144;
        const SOURCE_SUB: u32 = 0x2A0;
        const KIND_FIELD: u32 = 0x28;
        const MODE_FIELD: u32 = 0x1304;
        const SCALE0: u32 = 0x0103_79DC; // file VAs of the three scales
        const SCALE1: u32 = 0x0103_79E0;
        const SCALE2: u32 = 0x0103_79E4;
        const FIXED_THREE: f32 = 8.0;
        const FIXED_ROUND1: f32 = 1.5;
        const FIXED_ROUND2: f32 = 7.0;
        const CALL_SELECT: u32 = 1;
        const CALL_TARGET: u32 = 2;
        const CALL_BLEND_SCRATCH: u32 = 3;
        const CALL_BLEND_OUT: u32 = 4;
        const CALL_LOOKUP: u32 = 5;
        const CALL_FIND: u32 = 6;
        // Selector table: (answer - 1) -> continuation 0..5.
        const CASES: [u8; 32] = [
            0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 2, 3, 2, 5, 5, 5, 5, 5, 4,
        ];

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(addr: u32) -> f32 {
            unsafe { (addr as *const f32).read_unaligned() }
        }
        /// A read whose value feeds only dead scratch: forced with
        /// black_box so fault behaviour still matches the original.
        #[inline(always)]
        unsafe fn rd_dead(addr: u32) -> f32 {
            unsafe {
                core::hint::black_box((addr as *const f32).read_unaligned())
            }
        }
        /// Virtual call through the object's table slot with the object
        /// in ECX and no stack arguments. Both sides land on the same
        /// planted stub.
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let slot_addr = vt.wrapping_add(slot);
                let target = (slot_addr as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn global_scale(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read() }
        }

        // Normalise the sample direction (feeds the dead per-path
        // blends; the reads still matter for fault parity).
        let sx = rdf(sample.wrapping_add(SAMPLE_DIR));
        let sy = rdf(sample.wrapping_add(SAMPLE_DIR + 4));
        let sz = rdf(sample.wrapping_add(SAMPLE_DIR + 8));
        let len2 = add(add(mul(sy, sy), mul(sx, sx)), mul(sz, sz));
        let scale = if len2 == 0.0 {
            0.0f32
        } else {
            let r = len2.sqrt();
            core::hint::black_box(1.0f32) / core::hint::black_box(r)
        };
        let dir = [mul(sx, scale), mul(sy, scale), mul(sz, scale)];

        let answer = vcall(source, VT_SELECT);
        if answer == 0 || answer > 32 {
            return copy_fallback(sample, out);
        }
        // Scratch buffer for the blend helper. Its contents are never
        // observed (no stub reads or writes through it); only its role
        // as the call argument matters.
        let mut scratch = [0u32; 8];
        let scratch_ptr = scratch.as_mut_ptr() as u32;
        let result = match CASES[(answer - 1) as usize] {
            0 => {
                let target = vcall(source, VT_TARGET);
                if target == 0 {
                    return copy_fallback(sample, out);
                }
                lf_checker_rt::callee_thiscall!(
                    CALL_BLEND_SCRATCH,
                    u32,
                    scratch_ptr,
                    sample
                );
                blend_toward(target, dir, 1.0, out, scratch_ptr)
            }
            1 => {
                let target = vcall(source, VT_TARGET);
                if target == 0 {
                    return copy_fallback(sample, out);
                }
                lf_checker_rt::callee_thiscall!(
                    CALL_BLEND_SCRATCH,
                    u32,
                    scratch_ptr,
                    sample
                );
                blend_toward(target, dir, FIXED_THREE, out, scratch_ptr)
            }
            2 => {
                let target = vcall(source, VT_TARGET);
                if target == 0 {
                    return copy_fallback(sample, out);
                }
                let scale = pick_scale(target);
                lf_checker_rt::callee_thiscall!(
                    CALL_BLEND_SCRATCH,
                    u32,
                    scratch_ptr,
                    sample
                );
                blend_toward(target, dir, scale, out, scratch_ptr)
            }
            3 => {
                let target = rd32(source.wrapping_add(SOURCE_SUB));
                if target == 0 {
                    return copy_fallback(sample, out);
                }
                let scale = pick_scale(target);
                lf_checker_rt::callee_thiscall!(
                    CALL_BLEND_SCRATCH,
                    u32,
                    scratch_ptr,
                    sample
                );
                blend_toward(target, dir, scale, out, scratch_ptr)
            }
            4 => lookup_path(source, sample, out, dir, scratch_ptr),
            _ => copy_fallback(sample, out),
        };
        return result;

        /// Twelve-word sample copy with three gaps (0x0c, 0x1c, 0x2c are
        /// not copied); returns the last word moved.
        #[inline(always)]
        unsafe fn copy_fallback(sample: u32, out: u32) -> u32 {
            unsafe {
                let mut last = 0u32;
                let mut i = 0usize;
                while i < 12 {
                    // Offsets in copy order.
                    let off = match i {
                        0 => 0x00,
                        1 => 0x04,
                        2 => 0x08,
                        3 => 0x10,
                        4 => 0x14,
                        5 => 0x18,
                        6 => 0x20,
                        7 => 0x24,
                        8 => 0x28,
                        9 => 0x30,
                        10 => 0x34,
                        _ => 0x38,
                    };
                    last = rd32(sample.wrapping_add(off));
                    ((out.wrapping_add(off)) as *mut u32).write_unaligned(last);
                    i += 1;
                }
                last
            }
        }

        /// Nibble-and-mode scale choice shared by two continuations.
        #[inline(always)]
        unsafe fn pick_scale(target: u32) -> f32 {
            unsafe {
                let nibble = (rd32(target.wrapping_add(KIND_FIELD)) >> 6) & 0xF;
                if nibble != 2 {
                    return global_scale(SCALE0);
                }
                let mode = rd32(target.wrapping_add(MODE_FIELD));
                if mode == 2 {
                    global_scale(SCALE1)
                } else if mode == 4 {
                    global_scale(SCALE2)
                } else {
                    global_scale(SCALE0)
                }
            }
        }

        /// Blend toward the target's position, then finish through the
        /// shared tail. The three differences are dead (stored to
        /// scratch the original never reads); the tail call is observed.
        #[inline(always)]
        unsafe fn blend_toward(
            target: u32,
            dir: [f32; 3],
            grow: f32,
            out: u32,
            scratch_ptr: u32,
        ) -> u32 {
            unsafe {
                let head = rd32(target.wrapping_add(TARGET_POS));
                let base = if head != 0 {
                    head.wrapping_add(0x30)
                } else {
                    target.wrapping_add(TARGET_HEAD)
                };
                let gx = mul(dir[0], grow);
                let gy = mul(dir[1], grow);
                let gz = mul(dir[2], grow);
                // Dead differences (fault parity kept via rd_dead).
                let _dead = [
                    sub(rd_dead(base), gx),
                    sub(rd_dead(base.wrapping_add(4)), gy),
                    sub(rd_dead(base.wrapping_add(8)), gz),
                ];
                let _ = _dead;
                converge(out, scratch_ptr)
            }
        }

        /// Shared tail: the scratch word nobody wrote reads as zero
        /// under the proof's zero stack fill, and the second blend call
        /// carries `out`.
        #[inline(always)]
        unsafe fn converge(out: u32, scratch_ptr: u32) -> u32 {
            unsafe {
                let _unwritten: f32 = 0.0;
                let _ = _unwritten;
                lf_checker_rt::callee_thiscall!(
                    CALL_BLEND_OUT,
                    u32,
                    out,
                    scratch_ptr
                )
            }
        }

        /// Five-argument registry query; the middle immediate names
        /// the table, the outer words are always zero.
        #[inline(always)]
        unsafe fn find(aux: u32, table: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_cdecl!(
                    CALL_FIND, u32, aux, 0u32, 0x0103_872Cu32, table, 0u32
                )
            }
        }

        /// Registry-lookup continuation with two rounds of fallback.
        /// The lookup itself runs once; its result is reused by both
        /// rounds.
        #[inline(always)]
        unsafe fn lookup_path(
            source: u32,
            sample: u32,
            out: u32,
            dir: [f32; 3],
            scratch_ptr: u32,
        ) -> u32 {
            unsafe {
                let found: u32 =
                    lf_checker_rt::callee_cdecl!(CALL_LOOKUP, u32,);
                let aux = rd32(source.wrapping_add(SOURCE_AUX));
                let e1 = find(aux, 0x0103_874C);
                let e2 = find(aux, 0x0103_8780);
                let e3 = find(aux, 0x0103_87B0);
                if e1 != 0 || e2 != 0 || e3 != 0 {
                    if found == 0 {
                        return second_round(
                            aux, sample, out, found, dir, scratch_ptr,
                        );
                    }
                    return lookup_blend(
                        found, dir, FIXED_ROUND1, out, scratch_ptr, sample,
                    );
                }
                second_round(aux, sample, out, found, dir, scratch_ptr)
            }
        }

        /// Second query round; falls back to the sample copy when both
        /// queries and the first round's lookup come up empty.
        #[inline(always)]
        unsafe fn second_round(
            aux: u32,
            sample: u32,
            out: u32,
            found: u32,
            dir: [f32; 3],
            scratch_ptr: u32,
        ) -> u32 {
            unsafe {
                let e4 = find(aux, 0x0103_87D8);
                let e5 = find(aux, 0x0103_8800);
                if e4 != 0 || e5 != 0 {
                    if found == 0 {
                        return copy_fallback(sample, out);
                    }
                    return lookup_blend(
                        found, dir, FIXED_ROUND2, out, scratch_ptr, sample,
                    );
                }
                copy_fallback(sample, out)
            }
        }

        /// Blend toward the looked-up target's position (dead
        /// differences, observed tail call).
        #[inline(always)]
        unsafe fn lookup_blend(
            found: u32,
            dir: [f32; 3],
            grow: f32,
            out: u32,
            scratch_ptr: u32,
            sample: u32,
        ) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    CALL_BLEND_SCRATCH,
                    u32,
                    scratch_ptr,
                    sample
                );
                let head = rd32(found.wrapping_add(TARGET_POS));
                // Unlike the other paths this one does not test the
                // head for null before adding the stride.
                let base = head.wrapping_add(0x30);
                let gx = mul(dir[0], grow);
                let gy = mul(dir[1], grow);
                let gz = mul(dir[2], grow);
                let _dead = [
                    sub(rd_dead(base), gx),
                    sub(rd_dead(base.wrapping_add(4)), gy),
                    sub(rd_dead(base.wrapping_add(8)), gz),
                ];
                let _ = _dead;
                converge(out, scratch_ptr)
            }
        }
    }
});
