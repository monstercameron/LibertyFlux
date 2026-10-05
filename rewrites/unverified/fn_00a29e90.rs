// original: 0x00a29e90 pedtask_validate_candidate

/// Validate a candidate entity for a ped task through range and state gates.
///
/// `this` is the task object (its first word points at the owner record) and
/// `ent` the candidate; `flags`, `farg` and `extra` are parameters, `out` an
/// optional byte the success path sets. Returns al: 1 when the candidate is
/// accepted, 0 otherwise; upper return bytes are stale.
///
/// Behaviour: the transform callee runs over the anchor geometry (see
/// below); a distance strictly above the reported radius rejects. The probe
/// callee must accept `(ent, flags, extra)` (a non-zero answer rejects). The
/// kind field `([ent+0x28] >> 6) & 0xf` dispatches: kind 3 runs the main gate
/// (the owner must differ from the candidate, bit 0x20 of `[ent+0x24]` set,
/// bit 0x100 of `[ent+0x260]` clear, the pair check silent, bit 0x2000000 of
/// `[owner+0x264]` clear, the candidate's vtable slot 0x128 polled with a
/// triple check on a positive answer, the gate callee positive, the global
/// word zero, `[ent+0x219]` set, and the two `[.+0x228]` views agreeing on
/// a non-(-1) `[.+0x580]` value); kinds 2, 4 and 6 take their own accept
/// paths (4 needs the probe callee, bit 0x100000 clear and a non-null
/// `[ent+0x34]` pointing at a non-zero word; 6 polls the fetch callee and
/// dispatches two of its vtable slot 4 answers against 6 and 7); every other
/// kind rejects.
/// The narrow path (flag 0x800 set, or both probe callees positive with
/// `[ent+0x210]` zero) runs the final check and, on silence, sets `[out]`
/// when non-null before rejecting; a positive final check accepts.
///
/// The transform callee takes `(ent, owner, &blk)` with a 10-word block
/// pre-filled with the float argument, the y/x anchor deltas and the z
/// delta at words 0, 1, 2 and 8; words 0, 4, 5 and 8 are reread as the
/// radius and the three distance components. The callee is scripted by the
/// contract to fill words 0 through 8. The distance test is an unsigned
/// `ja` over the ordered comparison, so an unordered (NaN) pair falls
/// through exactly like Rust's `>` returning false.
///
/// Original: 0x00a29e90 (thiscall, five stack words; returns al).
#[allow(clippy::all)]
#[allow(unsafe_code)]
unsafe fn run_00a29e90(
    this: u32,
    flags: u32,
    ent: u32,
    farg: u32,
    out: u32,
    extra: u32,
    mutate_kind2: bool,
) -> u32 {
    unsafe {
        const ENT_ANCHOR: u32 = 0x20;
        const ENT_KIND: u32 = 0x28;
        const ANCHOR_POS: u32 = 0x30;
        const TRANSFORM_CALLEE: u32 = 1;
        const PROBE_CALLEE: u32 = 2;
        const PAIR_CALLEE: u32 = 3;
        const TRIPLE_CALLEE: u32 = 4;
        const GATE_CALLEE: u32 = 5;
        const PROBE_B_CALLEE: u32 = 6;
        const PROBE_C_CALLEE: u32 = 7;
        const FINAL_CALLEE: u32 = 8;
        const PROBE_D_CALLEE: u32 = 9;
        const FETCH_CALLEE: u32 = 11;
        const G_WORD: u32 = 0x110e2fc;

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
        /// Call a vtable slot thiscall-style, exactly like the original's
        /// load-and-call through the fabricated object.
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                tgt(obj)
            }
        }

        let owner = rd32(this);
        let aa = rd32(ent.wrapping_add(ENT_ANCHOR));
        let p = if aa != 0 {
            aa.wrapping_add(ANCHOR_POS)
        } else {
            ent.wrapping_add(0x10)
        };
        let base = rd32(owner.wrapping_add(ENT_ANCHOR));
        let dx = sub(rdf(p), rdf(base.wrapping_add(0x30)));
        let dy = sub(rdf(p.wrapping_add(4)), rdf(base.wrapping_add(0x34)));
        let dz = sub(rdf(p.wrapping_add(8)), rdf(base.wrapping_add(0x38)));
        let mut blk = [0u32; 10];
        blk[0] = farg;
        blk[1] = dy.to_bits();
        blk[2] = dx.to_bits();
        blk[8] = dz.to_bits();
        let blk_ptr = blk.as_mut_ptr() as u32;
        let _: u32 =
            lf_checker_rt::callee_cdecl!(TRANSFORM_CALLEE, u32, ent, owner, blk_ptr);
        let c4 = f32::from_bits(blk[4]);
        let c5 = f32::from_bits(blk[5]);
        let c8 = f32::from_bits(blk[8]);
        let radius = f32::from_bits(blk[0]);
        // Operand order is the original's: (c4*c4 + c5*c5) + c8*c8.
        let dist2 = add(add(mul(c4, c4), mul(c5, c5)), mul(c8, c8));
        let r2 = mul(radius, radius);
        if dist2 > r2 {
            return 0;
        }

        let probe: u32 =
            lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, this, ent, flags, extra);
        if probe != 0 {
            return 0;
        }

        let kind = (rd32(ent.wrapping_add(ENT_KIND)) >> 6) & 0xf;
        if kind != 3 {
            // Kind dispatch.
            if kind == 2 {
                // MUTANT (mut_00a29e90): return 0 here instead of 1.
                return if mutate_kind2 { 0 } else { 1 };
            }
            if kind == 4 {
                let pd: u32 =
                    lf_checker_rt::callee_thiscall!(PROBE_D_CALLEE, u32, ent);
                if pd == 0 {
                    return 0;
                }
                if rd32(ent.wrapping_add(ENT_KIND)) & 0x100000 != 0 {
                    return 0;
                }
                let t = rd32(ent.wrapping_add(0x34));
                if t == 0 {
                    return 0;
                }
                if rd32(t) != 0 {
                    return 1;
                }
                return 0;
            }
            if kind == 6 {
                let u: u32 =
                    lf_checker_rt::callee_thiscall!(FETCH_CALLEE, u32, ent);
                if u == 0 {
                    return 1;
                }
                let u2: u32 =
                    lf_checker_rt::callee_thiscall!(FETCH_CALLEE, u32, ent);
                if vcall(u2, 4) == 6 {
                    return 0;
                }
                let u3: u32 =
                    lf_checker_rt::callee_thiscall!(FETCH_CALLEE, u32, ent);
                if vcall(u3, 4) == 7 {
                    return 0;
                }
                return 1;
            }
            return 0;
        }

        // Main gate (kind 3). Every path below either returns or falls
        // through to the narrow section.
        if owner != ent
            && rd8(ent.wrapping_add(0x24)) & 0x20 != 0
            && rd32(ent.wrapping_add(0x260)) & 0x100 == 0
        {
            let pair: u32 = lf_checker_rt::callee_cdecl!(PAIR_CALLEE, u32, owner, ent);
            if pair == 0 {
                if rd32(owner.wrapping_add(0x264)) & 0x2000000 == 0 {
                    let v = vcall(ent, 0x128);
                    let mut ok = v == 0;
                    if !ok {
                        let t: u32 = lf_checker_rt::callee_cdecl!(
                            TRIPLE_CALLEE,
                            u32,
                            1u32,
                            rd32(owner.wrapping_add(0xba4)),
                            rd32(ent.wrapping_add(0xba4))
                        );
                        ok = t == 0;
                    }
                    if ok {
                        let gate: u32 =
                            lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, ent);
                        if gate != 0 {
                            let gw: u32 = lf_checker_rt::global::<u32>(G_WORD).read();
                            if gw == 0 && rd8(ent.wrapping_add(0x219)) != 0 {
                                let e1 = rd32(owner.wrapping_add(0x228));
                                let e2 = rd32(ent.wrapping_add(0x228));
                                let v1 = if e1 != 0 {
                                    rd32(e1.wrapping_add(0x580)) as i32
                                } else {
                                    -1
                                };
                                let v2 = if e2 != 0 {
                                    rd32(e2.wrapping_add(0x580)) as i32
                                } else {
                                    -1
                                };
                                if v1 == v2 && v1 != -1 {
                                    return 0;
                                }
                            }
                        }
                    }
                    if !ok {
                        return 0;
                    }
                }
            } else {
                return 0;
            }
        } else {
            return 0;
        }

        // Narrow section: reached by fall-through from every surviving
        // main-gate path.
        {
            if flags & 0x800 == 0 {
                let pb: u32 =
                    lf_checker_rt::callee_thiscall!(PROBE_B_CALLEE, u32, ent);
                if pb != 0 {
                    let pc: u32 =
                        lf_checker_rt::callee_thiscall!(PROBE_C_CALLEE, u32, ent);
                    if pc == 0 {
                        return 0;
                    }
                    if rd8(ent.wrapping_add(0x210)) != 0 {
                        return 0;
                    }
                }
            }
            let fin: u32 = lf_checker_rt::callee_cdecl!(FINAL_CALLEE, u32, ent, owner);
            if fin != 0 {
                return 1;
            }
            if out != 0 {
                (out as *mut u8).write(1);
            }
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00a29e90(
    this: u32, flags: u32, ent: u32, farg: u32, out: u32, extra: u32) -> u32 {
    unsafe { run_00a29e90(this, flags, ent, farg, out, extra, false) }
});

lf_checker_rt::export!(thiscall, mut_00a29e90(
    this: u32, flags: u32, ent: u32, farg: u32, out: u32, extra: u32) -> u32 {
    unsafe { run_00a29e90(this, flags, ent, farg, out, extra, true) }
});
