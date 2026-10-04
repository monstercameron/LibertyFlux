// original: 0x00b07a40 cam_target_track_update (proposed)

/// Camera target-tracking update (thiscall, `this` in ECX, no stack arguments,
/// returns 1 in AL, 0 on one early path). While a global singleton exists:
/// exit 0 when a gate callee answers nonzero and a global inhibit flag is set;
/// when the singleton is neither active nor in mode 1, decay three rate
/// floats by a global factor, clear the tracked-target slot and return 1.
/// Otherwise run an update callee and, when the singleton is active, the
/// object flag bit 0 is clear and the singleton yields a candidate, resolve
/// the tracked slot through a global table (base, flag bytes, count, stride):
/// an empty slot searches top-down for the entry equal to the candidate,
/// skipping flagged, null and state-mismatched entries; a filled slot is kept
/// only if some unflagged entry is non-null. An unresolved slot runs the lazy
/// init (seed the origin slot from a global, maybe write two constant
/// globals) and returns 1.
/// A resolved slot asks a watcher callee for a detail object (selector 1,
/// then 2) and runs one of two near-identical float blocks: publish three
/// source floats and a zero seed (the original reads the seed from an
/// uninitialized stack slot; the contract defines the fill as 0) to globals,
/// difference two live pre-values into a frame buffer (two further slots read
/// uninitialized stack in the original and are defined 0), ask two transform
/// callees to consume the buffer (frame pointers skipped, pre-values
/// snapshotted; the buffer outputs are never read back), then scale the first
/// callee's x87 answer and accumulate it into the detail object. The first
/// block additionally requires the detail's big-object field to exceed a
/// global threshold. Both finish by zeroing two detail words and setting the
/// init flag. Float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00b07a40(this: u32) -> u32 {
    unsafe {
        const TR_SLOT: u32 = 0x154;
        const TR_FLAG0: u32 = 0x1C4;
        const TR_INIT: u32 = 0x1D8;
        const TR_ORIGIN: u32 = 0x1D4;
        const TR_RATE0: u32 = 0x1CC;
        const TR_RATE1: u32 = 0x1C8;
        const TR_RATE2: u32 = 0x1D0;
        const TR_SRC: u32 = 0x170;
        const G_INHIB: u32 = 0x017F5EB3;
        const G_DECAY: u32 = 0x00FE88BC;
        const G_SEED: u32 = 0x011735C4;
        const G_TABLE: u32 = 0x018B6F1C;
        const G_K: u32 = 0x00FE8830;
        const G_PUB0: u32 = 0x016154C0;
        const G_PUB1: u32 = 0x016154C4;
        const G_PUB2: u32 = 0x016154C8;
        const G_PUBS: u32 = 0x016154CC;
        const G_CONST: u32 = 0x3E4CCCCD;
        const G_C0: u32 = 0x01032350;
        const G_C1: u32 = 0x0103234C;
        const SING_ACTIVE: u32 = 0x210;
        const SING_MODE: u32 = 0xA70;
        const SING_CAND: u32 = 0x1E4;
        const ENT_STATE0: u32 = 0x218;
        const ENT_STATE1: u32 = 0x219;
        const ENT_VEC: u32 = 0x20;
        const DET_VEC: u32 = 0x40;
        const DET_BIG: u32 = 0x12C;
        const BIG_FIELD: u32 = 0x1450;
        const DET_ACC: u32 = 0x308;
        const DET_Z0: u32 = 0x310;
        const DET_Z1: u32 = 0x30C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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

        /// Lazy init shared by the unresolved-slot exits. Returns 1.
        unsafe fn init_exit(this: u32) -> u32 {
            unsafe {
                if rd8(this + TR_INIT) == 0 {
                    wr32(this + TR_ORIGIN, rd32(lf_checker_rt::relocated(G_SEED)));
                    let g: u32 = lf_checker_rt::callee_cdecl!(9, u32,);
                    if (g as u8) == 0 {
                        wr32(lf_checker_rt::relocated(G_C0), G_CONST);
                        wr32(lf_checker_rt::relocated(G_C1), G_CONST);
                    }
                    wr8(this + TR_INIT, 1);
                }
                1
            }
        }

        /// One tracking float block. `detail` is the watcher's non-null
        /// answer; `first` selects the threshold-gated block (callees 6/7)
        /// or the plain one (callees 10/11). Returns the detail object.
        unsafe fn float_block(this: u32, detail: u32, first: bool) -> u32 {
            unsafe {
                let slot = rd32(this + TR_SLOT);
                let d0 = rdf(detail + DET_VEC);
                let src = rd32(slot + ENT_VEC);
                let d1 = rdf(detail + DET_VEC + 4);
                let s0 = rdf(src + 0x30);
                let s1 = rdf(src + 0x34);
                let s2 = rdf(src + 0x38);
                let d2 = rdf(detail + DET_VEC + 8);
                wrf(lf_checker_rt::relocated(G_PUBS), 0.0);
                wrf(lf_checker_rt::relocated(G_PUB0), s0);
                wrf(lf_checker_rt::relocated(G_PUB1), s1);
                wrf(lf_checker_rt::relocated(G_PUB2), s2);
                let _ = (d1, d2);
                let v_a = sub(s0, d0);
                let v_b = sub(rdf(this + TR_SRC), d0);
                let buf = [v_b.to_bits(), v_a.to_bits(), 0u32, 0u32];
                let p0 = (&buf[0] as *const u32) as u32;
                let p1 = (&buf[1] as *const u32) as u32;
                let p2 = (&buf[2] as *const u32) as u32;
                let p3 = (&buf[3] as *const u32) as u32;
                let k = rdf(lf_checker_rt::relocated(G_K));
                if first {
                    let ans: f32 = lf_checker_rt::callee_cdecl!(6, f32, p0, p1, p2, p3);
                    let _: f32 = lf_checker_rt::callee_cdecl!(7, f32, p0, p1, p2, p3);
                    let big = rd32(detail + DET_BIG);
                    if big != 0 && rdf(big + BIG_FIELD) > k {
                        wrf(detail + DET_ACC, add(mul(ans, k), rdf(detail + DET_ACC)));
                    }
                } else {
                    let ans: f32 = lf_checker_rt::callee_cdecl!(10, f32, p0, p1, p2, p3);
                    let _: f32 = lf_checker_rt::callee_cdecl!(11, f32, p0, p1, p2, p3);
                    wrf(detail + DET_ACC, add(mul(ans, k), rdf(detail + DET_ACC)));
                }
                detail
            }
        }

        let sing: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if sing == 0 {
            return 1;
        }
        let g: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        if (g as u8) != 0 && rd8(lf_checker_rt::relocated(G_INHIB)) != 0 {
            return 0;
        }
        let active = rd8(sing + SING_ACTIVE) != 0;
        if !active && rd32(sing + SING_MODE) != 1 {
            let k = rdf(lf_checker_rt::relocated(G_DECAY));
            wrf(this + TR_RATE0, mul(rdf(this + TR_RATE0), k));
            wrf(this + TR_RATE1, mul(rdf(this + TR_RATE1), k));
            wrf(this + TR_RATE2, mul(rdf(this + TR_RATE2), k));
            wr32(this + TR_SLOT, 0);
            return 1;
        }
        lf_checker_rt::callee_thiscall!(3, u32, this);
        if active {
            let mut resolved = false;
            if rd8(this + TR_FLAG0) & 1 == 0 {
                let cand = rd32(sing + SING_CAND);
                if cand != 0 {
                    let t = rd32(lf_checker_rt::relocated(G_TABLE));
                    let cnt = rd32(t + 8);
                    if rd32(this + TR_SLOT) == 0 {
                        if cnt != 0 {
                            let fl = rd32(t + 4);
                            let stride = rd32(t + 0xC);
                            let base = rd32(t);
                            let mut i = cnt;
                            while i != 0 {
                                i -= 1;
                                if rd8(fl + i) & 0x80 != 0 {
                                    continue;
                                }
                                let ent = base.wrapping_add(stride.wrapping_mul(i));
                                if ent == 0 {
                                    continue;
                                }
                                if rd8(ent + ENT_STATE0) == 0 && rd8(ent + ENT_STATE1) != 0 {
                                    continue;
                                }
                                if ent == cand {
                                    wr32(this + TR_SLOT, ent);
                                    break;
                                }
                            }
                        }
                        resolved = rd32(this + TR_SLOT) != 0;
                    } else {
                        // Validity probe: keep the slot unless no usable
                        // entry exists at all.
                        let mut keep = false;
                        if cnt != 0 {
                            let fl = rd32(t + 4);
                            let stride = rd32(t + 0xC);
                            let base = rd32(t);
                            let mut i = cnt;
                            while i != 0 {
                                i -= 1;
                                if rd8(fl + i) & 0x80 != 0 {
                                    continue;
                                }
                                if base.wrapping_add(stride.wrapping_mul(i)) != 0 {
                                    keep = true;
                                    break;
                                }
                            }
                        }
                        if keep {
                            resolved = true;
                        } else {
                            wr32(this + TR_SLOT, 0);
                        }
                    }
                }
            }
            if !resolved {
                wr32(this + TR_SLOT, 0);
                return init_exit(this);
            }
            if rd8(this + TR_INIT) == 0 {
                wr32(this + TR_ORIGIN, rd32(lf_checker_rt::relocated(G_SEED)));
            }
            let d: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 1, 0);
            let detail = if d != 0 {
                float_block(this, d, true)
            } else {
                let d2: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, 2, 0);
                if d2 == 0 {
                    return init_exit(this);
                }
                float_block(this, d2, false)
            };
            wr32(detail + DET_Z0, 0);
            wr32(detail + DET_Z1, 0);
            wr8(this + TR_INIT, 1);
            return 1;
        }
        wr32(this + TR_SLOT, 0);
        1
    }
});
