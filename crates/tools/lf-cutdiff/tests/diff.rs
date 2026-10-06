//! Differential cases: the cutscene object.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order, floats bit for bit. Each case also runs a
//! deliberately wrong lift, which must be caught at least once. 32-bit
//! target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_cutdiff::rewrites::*;
    use lf_cutdiff::rt::{self, StubKind};
    use lf_world::cutscene_object::{
        Accumulator, BoneRow, BoundsRect, BoundsScale, CutsceneObject, CutsceneWorld, PoseRecord,
        UpdateEntry, UpdateScalars, WorldBounds,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        F32_EDGE, Fake, Image, Rng, Stubs, U32_EDGE, VTable, assert_only_changed, check_virtual,
        cookie, lift_from, random_matrix, random_scales, read_matrix, words, write_matrix,
    };

    // Object field offsets, as the verified rewrites use them.
    const VT: usize = 0x00;
    const TRIPLE: usize = 0x10;
    const ATTACH: usize = 0x20;
    const HELPER: usize = 0x80;
    const HFLAG: usize = 0xBC;
    const FLAG2A0: usize = 0x2A0;
    const RFA: usize = 0x2A9;
    const RFB: usize = 0x2AA;
    const RADIUS: usize = 0x2E0;
    const CORNER_A: usize = 0x2F0;
    const CORNER_B: usize = 0x300;
    const MEMBER_A: usize = 0x310;
    const MODE: usize = 0x314;
    const OBJ_SIZE: usize = 0x320;
    const MAT_SIZE: usize = 0x3C;
    // Teardown-only field offsets.
    const FLAGS24: usize = 0x24;
    const TBLIDX: usize = 0x2E;
    const CTX: usize = 0xD0;
    const GATE: usize = 0xD4;
    const MEMBER_B: usize = 0x290;
    const BLOCK0: usize = 0x29C;
    const BLOCK1: usize = 0x298;
    const BLOCK2: usize = 0x294;
    const DONE: usize = 0x2AC;
    const SCRIPT: usize = 0x2C;
    const CHAIN: usize = 0x34;
    // Relocated addresses the teardown reads.
    const DTOR_VTABLE: u32 = 0x00EC_B99C;
    const REGISTRY_G: u32 = 0x0129_5CD8;
    const FIXED_CTX: u32 = 0x0117_37D0;

    /// A test object: the 32-bit image plus the blocks it points at. Kept
    /// alive together so addresses stay valid.
    #[allow(dead_code)]
    struct Fixture {
        obj: Image,
        mat: Image,
        vtable: VTable,
        member_obj: Image,
        member_vt: VTable,
        helper_vt: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng, attached: bool, member: bool) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let mat = Image::random(MAT_SIZE, rng);
            let mut vtable = VTable::random(24, rng);
            let mut member_obj = Image::random(0x10, rng);
            let mut member_vt = VTable::random(10, rng);
            let mut helper_vt = VTable::random(4, rng);
            let stubs = Stubs::new();
            vtable.set(0x54, stubs.pose);
            vtable.set(0x58, stubs.followup);
            vtable.set(0x24, stubs.guard_a);
            vtable.set(0x28, stubs.guard_b);
            member_vt.set(0x20, stubs.successor);
            member_vt.set(0x00, stubs.destroy);
            member_vt.set(0x08, stubs.probe);
            helper_vt.set(0x08, stubs.helper);
            obj.w32(VT, vtable.addr());
            obj.w32(ATTACH, if attached { mat.addr() } else { 0 });
            obj.w32(HELPER, helper_vt.addr());
            obj.w32(MEMBER_A, if member { member_obj.addr() } else { 0 });
            member_obj.w32(0, member_vt.addr());
            Self {
                obj,
                mat,
                vtable,
                member_obj,
                member_vt,
                helper_vt,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        /// The lifted object owning the same words the image holds.
        fn lift(&self) -> CutsceneObject {
            let mat = if self.obj.r32(ATTACH) == 0 {
                None
            } else {
                Some(&self.mat)
            };
            lift_from(
                &self.obj, mat, TRIPLE, ATTACH, HELPER, HFLAG, FLAG2A0, RFA, RFB, RADIUS, CORNER_A,
                CORNER_B, MEMBER_A, MODE, FLAGS24, TBLIDX, CTX, GATE, MEMBER_B, BLOCK0, BLOCK1,
                BLOCK2, DONE, SCRIPT, CHAIN,
            )
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use super::{
            Accumulator, BoundsRect, BoundsScale, CutsceneObject, CutsceneWorld, PoseRecord,
            UpdateEntry, UpdateScalars, WorldBounds,
        };
        use lf_core::Handle32;

        /// Confuses the first state slot with the second.
        pub fn state0_as_1(o: &CutsceneObject) -> bool {
            o.mode == 1
        }

        /// Confuses the second state slot with the third.
        pub fn state1_as_2(o: &CutsceneObject) -> bool {
            o.mode == 2
        }

        /// Confuses the third state slot with the first.
        pub fn state2_as_0(o: &CutsceneObject) -> bool {
            o.mode == 0
        }

        /// Inverts the flag test.
        pub fn flag_zero(o: &CutsceneObject) -> bool {
            o.flag_word == 0
        }

        /// Negates the radius.
        pub fn radius_neg(o: &CutsceneObject) -> f32 {
            -o.bound_radius()
        }

        /// Swaps the attached and inline sources.
        pub fn describe_swapped(o: &CutsceneObject, out: &mut [u32; 3]) {
            *out = match &o.attached {
                Some(_) => o.inline_triple,
                None => [0xDEAD_BEEF, 0xDEAD_BEEF, 0xDEAD_BEEF],
            };
        }

        /// Answers the head word instead of the tail.
        pub fn pose_head(r: &PoseRecord) -> u32 {
            r.head
        }

        /// Skips the follow-up slot, answering the tail.
        pub fn pose_no_followup<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            out: &mut PoseRecord,
        ) -> u32 {
            let _ = o;
            *out = world.pose_slot();
            out.tail
        }

        /// Never forwards to the member.
        pub fn forward_first<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            word: u32,
        ) -> u32 {
            let _ = o;
            world.notify(word)
        }

        /// Requires both refresh flags.
        pub fn refresh_and<W: CutsceneWorld>(o: &CutsceneObject, world: &mut W) -> u32 {
            if o.refresh_a != 0 && o.refresh_b != 0 {
                world.refresh_hook();
                world.refresh_run()
            } else {
                0
            }
        }

        /// Issues the helper command without clearing the flag.
        pub fn helper_no_clear<W: CutsceneWorld>(o: &CutsceneObject, world: &mut W) -> u32 {
            world.helper_command(o.helper, 0, 0xFFFF_FFFE)
        }

        /// Mode 0 through the first emitter only.
        pub fn draw_a_only<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            a0: u32,
            a1: u32,
            a2: u32,
        ) {
            if o.mode == 0 {
                world.draw_mode0_a(a0, a1);
            } else if o.mode == 1 {
                if let Some(t) = world.draw_emit(a0, a1, a2) {
                    world.mark_emitted(t);
                }
            } else if o.mode == 2 {
                if !o.is_flag_word_nonzero() {
                    let _ = world.draw_emit(a0, a1, a2);
                }
            }
        }

        /// Mode 1 without the mark.
        pub fn draw_no_mark<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            a0: u32,
            a1: u32,
            a2: u32,
        ) {
            if o.mode == 0 {
                world.draw_mode0_a(a0, a1);
                world.draw_mode0_b(a0, a1);
            } else if o.mode == 1 {
                let _ = world.draw_emit(a0, a1, a2);
            } else if o.mode == 2 {
                if !o.is_flag_word_nonzero() {
                    let _ = world.draw_emit(a0, a1, a2);
                }
            }
        }

        #[inline(always)]
        fn wadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        #[inline(always)]
        fn wmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        /// The box with the minimum corner added instead of subtracted.
        pub fn box_min_plus<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            scales: &BoundsScale,
            out: &mut WorldBounds,
        ) {
            let buf = world.placement_matrix(o.placement);
            let word = |i: usize| f32::from_bits(buf[i]);
            let masked = |i: usize| f32::from_bits(buf[i] & scales.abs_mask);
            let (ax, ay, az) = (o.corner_a[0], o.corner_a[1], o.corner_a[2]);
            let (bx, by, bz) = (o.corner_b[0], o.corner_b[1], o.corner_b[2]);
            let px_sum = wmul(scales.gx, wadd(bx, ax));
            let pz_sum = wmul(wadd(bz, az), scales.gz);
            let py_sum = wmul(scales.gy, wadd(by, ay));
            let sub = |a: f32, b: f32| core::hint::black_box(a) - core::hint::black_box(b);
            let px_dif = wmul(sub(bx, ax), scales.gx);
            let pz_dif = wmul(sub(bz, az), scales.gz);
            let py_dif = wmul(sub(by, ay), scales.gy);
            let b0_px = wmul(word(0), px_sum);
            let b4_py = wmul(word(4), py_sum);
            let b1_px = wmul(word(1), px_sum);
            let cx_a = wadd(b4_py, b0_px);
            let b8_pz = wmul(word(8), pz_sum);
            let b2_px = wmul(word(2), px_sum);
            let cx_b = wadd(cx_a, b8_pz);
            let b9_pz = wmul(word(9), pz_sum);
            let b10_pz = wmul(word(10), pz_sum);
            let cx = wadd(cx_b, word(12));
            let b5_py = wmul(word(5), py_sum);
            let cy_a = wadd(b5_py, b1_px);
            let cy_b = wadd(cy_a, b9_pz);
            let b6_py = wmul(word(6), py_sum);
            let cy = wadd(cy_b, word(13));
            let cz_a = wadd(b6_py, b2_px);
            let cz_b = wadd(cz_a, b10_pz);
            let cz = wadd(cz_b, word(14));
            let ex = wadd(
                wadd(wmul(masked(1), py_dif), wmul(masked(0), px_dif)),
                wmul(masked(2), pz_dif),
            );
            let ey = wadd(
                wadd(wmul(masked(5), py_dif), wmul(masked(4), px_dif)),
                wmul(masked(6), pz_dif),
            );
            let ez = wadd(
                wadd(wmul(masked(9), py_dif), wmul(masked(8), px_dif)),
                wmul(masked(10), pz_dif),
            );
            // Wrong: the minimum adds the extent like the maximum does.
            out.min = [wadd(cx, ex), wadd(cy, ey), wadd(cz, ez)];
            out.lo_tag = buf[3];
            out.max = [wadd(cx, ex), wadd(cy, ey), wadd(cz, ez)];
            out.hi_tag = buf[7];
        }

        /// The rectangle skipping the second corner (first corner twice).
        pub fn rect_skip_second<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            out: &mut BoundsRect,
        ) {
            let corners = [
                (o.corner_a[0], o.corner_a[1], o.corner_a[2]),
                (o.corner_a[0], o.corner_a[1], o.corner_a[2]),
                (o.corner_b[0], o.corner_a[1], o.corner_a[2]),
                (o.corner_a[0], o.corner_b[1], o.corner_b[2]),
            ];
            *out = BoundsRect {
                min_x: f32::from_bits(0x4974_2400),
                max_y: f32::from_bits(0xC974_2400),
                max_x: f32::from_bits(0xC974_2400),
                min_y: f32::from_bits(0x4974_2400),
            };
            for (cx, cy, cz) in corners {
                let (rx, ry) = match &o.attached {
                    Some(m) => {
                        let rx = wadd(
                            wadd(
                                wadd(wmul(m.vy[0], cy), wmul(m.vx[0], cx)),
                                wmul(m.vz[0], cz),
                            ),
                            m.origin[0],
                        );
                        let ry = wadd(
                            wadd(
                                wadd(wmul(m.vy[1], cy), wmul(m.vx[1], cx)),
                                wmul(m.vz[1], cz),
                            ),
                            m.origin[1],
                        );
                        (rx, ry)
                    }
                    None => {
                        let p = world.transform_corner(o.placement, [cx, cy, cz]);
                        (p[0], p[1])
                    }
                };
                if rx < out.min_x {
                    out.min_x = rx;
                }
                if out.max_x < rx {
                    out.max_x = rx;
                }
                if ry < out.min_y {
                    out.min_y = ry;
                }
                if out.max_y < ry {
                    out.max_y = ry;
                }
            }
        }

        /// The update with the blend dispatch swapped: mode 1 runs the
        /// four-bone average, anything else the two-bone blend.
        pub fn update_jk_swap<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            cfg: &UpdateScalars,
            acc: &mut Accumulator,
        ) -> u32 {
            world.entry_notify();
            if cfg.entry_seq_byte == 0 {
                world.entry_second();
            }
            if world.guard_a() & 0xff != 0 {
                let member = o.member_b.expect("member linked");
                world.member_probe(member);
                let ent = world.table_entry(o.table_index);
                if ent.flag != 0 {
                    let block = world.early_block();
                    let w = world.early_word(block);
                    return world.early_tail(w);
                }
                return world.early_call(
                    i32::from(o.table_index),
                    Handle32::raw_or_zero(o.blocks[2]),
                    Handle32::raw_or_zero(o.member_a),
                );
            }
            let r2 = world.guard_b();
            if r2 & 0xff == 0 {
                return r2;
            }
            let ent = world.table_entry(o.table_index);
            let edx = if cfg.sel != -1 { cfg.sel } else { cfg.edx_alt };
            let w2c = u32::from(o.script_word);
            let go_g = if edx > 0x14 {
                true
            } else {
                let eax = cfg.eax;
                let mut ecx = cfg.ecx;
                if edx > 0x13 {
                    if eax != -1 {
                        ecx = eax;
                    }
                    ecx > ((w2c & 0x3f) as i32)
                } else if edx < 6 {
                    true
                } else if edx >= 7 {
                    false
                } else {
                    if eax != -1 {
                        ecx = eax;
                    }
                    ecx < ((w2c & 0x3f) as i32)
                }
            };
            let go_g = if go_g {
                true
            } else {
                let x = wmul((w2c as i32) as f32, cfg.win_scale);
                cfg.win_lo > x || cfg.win_hi > x
            };
            if go_g && (cfg.setup_flag & 2) == 0 {
                world.setup_primary(ent.id, o.attached_id, [0x32, 0x33, 1, 1, 0x3F80_0000, 0]);
                world.setup_secondary(
                    ent.id,
                    o.attached_id,
                    [0x34, 0xffff_ffff, 0x35, 1, 0, 1, 0x3F80_0000, 0],
                );
                let chain = o.store_chain.expect("chain linked");
                world.store_setup(chain, cfg.store_val);
            }
            let idx = ent.index_words;
            // Wrong: the dispatch is swapped.
            if ent.mode == 1 {
                if (idx[0] as i32) < 0
                    || (idx[1] as i32) < 0
                    || (idx[2] as i32) < 0
                    || (idx[3] as i32) < 0
                {
                    return idx[3];
                }
                return wrong_average_k(o, world, cfg, &ent, idx);
            }
            if (idx[0] as i32) < 0 {
                return idx[2];
            }
            if (idx[2] as i32) < 0 {
                return idx[2];
            }
            wrong_blend_j(o, world, cfg, acc, &ent, idx[0], idx[2])
        }

        /// The average, as the swapped update runs it.
        fn wrong_average_k<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            cfg: &UpdateScalars,
            ent: &UpdateEntry,
            idx: [u32; 4],
        ) -> u32 {
            let r1 = world.bone_row(idx[0]);
            let r2 = world.bone_row(idx[1]);
            let r3 = world.bone_row(idx[2]);
            let r4 = world.bone_row(idx[3]);
            let (x1, y1, z1) = (r1.xyz[0], r1.xyz[1], r1.xyz[2]);
            let (x2, y2, z2) = (r2.xyz[0], r2.xyz[1], r2.xyz[2]);
            let (x3, y3, z3) = (r3.xyz[0], r3.xyz[1], r3.xyz[2]);
            let (x4, y4, z4) = (r4.xyz[0], r4.xyz[1], r4.xyz[2]);
            let qx = wmul(wadd(wadd(x3, wadd(x2, x1)), x4), cfg.qk);
            let qy = wmul(wadd(wadd(y3, wadd(y2, y1)), y4), cfg.qk);
            let qz = wmul(wadd(wadd(z3, wadd(z2, z1)), z4), cfg.qk);
            let m = o.attached.as_ref().expect("record attached");
            let zero = 0.0f32;
            let sub = |a: f32, b: f32| core::hint::black_box(a) - core::hint::black_box(b);
            let sqr = |x: f32| core::hint::black_box(x) * core::hint::black_box(x);
            let sqrt = |x: f32| core::hint::black_box(x).sqrt();
            let b0 = sub(
                wadd(wmul(m.vy[0], zero), wmul(m.vx[0], zero)),
                wmul(m.vz[0], cfg.k0),
            );
            let b4 = sub(
                wadd(wmul(m.vy[1], zero), wmul(m.vx[1], zero)),
                wmul(m.vz[1], cfg.k0),
            );
            let b8 = sub(
                wadd(wmul(m.vy[2], zero), wmul(m.vx[2], zero)),
                wmul(m.vz[2], cfg.k0),
            );
            let c0 = wadd(wadd(m.vy[0], wmul(m.vx[0], zero)), wmul(m.vz[0], zero));
            let c4 = wadd(wadd(m.vy[1], wmul(m.vx[1], zero)), wmul(m.vz[1], zero));
            let c8 = wadd(wadd(m.vy[2], wmul(m.vx[2], zero)), wmul(m.vz[2], zero));
            let len_a = sqrt(wadd(
                wadd(sqr(sub(y2, y4)), sqr(sub(x2, x4))),
                sqr(sub(z2, z4)),
            ));
            let len_b = sqrt(wadd(
                wadd(sqr(sub(y1, y3)), sqr(sub(x1, x3))),
                sqr(sub(z1, z3)),
            ));
            let arg5 = wmul(wmul(wadd(len_a, len_b), cfg.kn), cfg.k_a5);
            let arg5 = wmul(arg5, cfg.kn);
            let len_c = sqrt(wadd(
                wadd(sqr(sub(y3, y4)), sqr(sub(x3, x4))),
                sqr(sub(z3, z4)),
            ));
            let len_d = sqrt(wadd(
                wadd(sqr(sub(y2, y1)), sqr(sub(x2, x1))),
                sqr(sub(z2, z1)),
            ));
            let arg4 = wmul(wmul(wadd(len_c, len_d), cfg.kn), cfg.k_a4);
            let arg4 = wmul(arg4, cfg.kn);
            let arg6 = wmul(wmul(wmul(ent.weight, cfg.wx), cfg.k_a6), cfg.kn);
            world.submit_k(
                o.attached_id,
                [qx.to_bits(), qy.to_bits(), qz.to_bits(), 0],
                [b0.to_bits(), b4.to_bits(), b8.to_bits(), 0],
                [c0.to_bits(), c4.to_bits(), c8.to_bits(), 0],
                [arg4.to_bits(), arg5.to_bits(), arg6.to_bits(), cfg.k_a8],
            )
        }

        /// The blend, as the swapped update runs it.
        fn wrong_blend_j<W: CutsceneWorld>(
            o: &CutsceneObject,
            world: &mut W,
            cfg: &UpdateScalars,
            acc: &mut Accumulator,
            ent: &UpdateEntry,
            idx_a: u32,
            idx_b: u32,
        ) -> u32 {
            let r1 = world.bone_row(idx_a);
            let sub = |a: f32, b: f32| core::hint::black_box(a) - core::hint::black_box(b);
            let sqr = |x: f32| core::hint::black_box(x) * core::hint::black_box(x);
            let sqrt = |x: f32| core::hint::black_box(x).sqrt();
            let dk = sub(cfg.k0, cfg.k1);
            let r2 = world.bone_row(idx_b);
            let (t1x, t1y, t1z) = (r1.xyz[0], r1.xyz[1], r1.xyz[2]);
            let (t2x, t2y, t2z) = (r2.xyz[0], r2.xyz[1], r2.xyz[2]);
            let ix = wadd(wmul(t1x, cfg.k1), wmul(t2x, dk));
            let iy = wadd(wmul(t1y, cfg.k1), wmul(t2y, dk));
            let iz = wadd(wmul(t1z, cfg.k1), wmul(t2z, dk));
            let wgt = wmul(ent.weight, cfg.wgt_scale);
            let (g0, g1, g2) = if acc.flag & 1 != 0 {
                (acc.vals[0], acc.vals[1], acc.vals[2])
            } else {
                acc.flag |= 1;
                acc.vals = [0.0, 0.0, 0.0];
                (0.0, 0.0, 0.0)
            };
            let jx = wadd(ix, g0);
            let jy = wadd(iy, g1);
            let jz = wadd(iz, g2);
            let dx = sub(t1x, t2x);
            let dy = sub(t1y, t2y);
            let dz = sub(t1z, t2z);
            let m = o.attached.as_ref().expect("record attached");
            let zero = 0.0f32;
            let b0 = sub(
                wadd(wmul(m.vy[0], zero), wmul(m.vx[0], zero)),
                wmul(m.vz[0], cfg.k0),
            );
            let b4 = sub(
                wadd(wmul(m.vy[1], zero), wmul(m.vx[1], zero)),
                wmul(m.vz[1], cfg.k0),
            );
            let b8 = sub(
                wadd(wmul(m.vy[2], cfg.e18_scale), wmul(m.vx[2], zero)),
                wmul(m.vz[2], cfg.k0),
            );
            let a0 = wadd(wadd(wmul(m.vx[0], zero), m.vy[0]), wmul(m.vz[0], zero));
            let a4 = wadd(wadd(wmul(m.vx[1], zero), m.vy[1]), wmul(m.vz[1], zero));
            let a8 = wadd(wadd(wmul(m.vx[2], zero), m.vy[2]), wmul(m.vz[2], zero));
            let arg4 = wmul(wmul(cfg.j_a4, wgt), cfg.kn);
            let shade = wadd(
                sqrt(wadd(wadd(sqr(dy), sqr(dx)), sqr(dz))),
                wmul(wgt, cfg.wx),
            );
            let arg5 = wmul(wmul(shade, cfg.j_a5), cfg.kn);
            let arg6 = wmul(wmul(cfg.j_a6, wgt), cfg.kn);
            world.submit_j(
                o.attached_id,
                [jx.to_bits(), jy.to_bits(), jz.to_bits(), 0],
                [b0.to_bits(), b4.to_bits(), b8.to_bits(), 0],
                [a0.to_bits(), a4.to_bits(), a8.to_bits(), 0],
                [arg4.to_bits(), arg5.to_bits(), arg6.to_bits(), cfg.j_a8],
            )
        }

        /// Tears down in every mode (the mode-0 gate dropped).
        pub fn destroy_always<W: CutsceneWorld>(o: &mut CutsceneObject, world: &mut W) -> u32 {
            if let Some(ha) = o.member_a.take() {
                world.destroy_member(ha);
            }
            for slot in o.blocks.iter_mut() {
                if let Some(blk) = slot.take() {
                    world.teardown_block(blk);
                    world.free_block(blk);
                }
            }
            if let Some(hb) = o.member_b.take() {
                world.destroy_member(hb);
            }
            if world.ask_registry() & 0xff != 0 {
                let word = world.registry_word(o.table_index);
                if word != -1 && o.gate_d4 != 0 && world.ask_gate(word as u32) & 0xff != 0 {
                    world.registry_run();
                    world.context_run(o.ctx, 3);
                    world.registry_tell(word as u32);
                }
            }
            if o.mode == 1 {
                o.flags_24 |= 0x0400_0000;
            }
            o.done_2ac = 0;
            world.base_destroy()
        }
    }

    #[test]
    fn predicates_match() {
        let mut rng = Rng(0xC050);
        let mut caught = [0u32; 4];
        let modes = [0u32, 1, 2, 3, 7, 0x7FFF_FFFF, 0x8000_0000, 0xFFFF_FFFF];
        for (i, mode) in modes.iter().cycle().take(64).enumerate() {
            let mut fx = Fixture::build(&mut rng, false, false);
            fx.obj.w32(MODE, *mode);
            let flag = if i < U32_EDGE.len() {
                U32_EDGE[i]
            } else {
                rng.u32()
            };
            fx.obj.w32(FLAG2A0, flag);
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let _guard = rt::script_lock();
            rt::set_script(&[]);
            let r9 = unsafe { fn_00c66020::rw_c66020(this) };
            let r10 = unsafe { fn_00c66030::rw_c66030(this) };
            let r11 = unsafe { fn_00c66010::rw_c66010(this) };
            let rf = unsafe { fn_00c66130::rw_00c66130(this) };
            assert_eq!(rt::take_numbered(), vec![]);
            assert_eq!(rt::take_virtual(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let o = fx.lift();
            assert_eq!(r9, u32::from(o.is_state_0()), "vf9 mode {mode}");
            assert_eq!(r10, u32::from(o.is_state_1()), "vf10 mode {mode}");
            assert_eq!(r11, u32::from(o.is_state_2()), "vf11 mode {mode}");
            assert_eq!(rf, u32::from(o.is_flag_word_nonzero()), "flag {flag:#x}");
            if wrong::state0_as_1(&o) != o.is_state_0() {
                caught[0] += 1;
            }
            if wrong::state1_as_2(&o) != o.is_state_1() {
                caught[1] += 1;
            }
            if wrong::state2_as_0(&o) != o.is_state_2() {
                caught[2] += 1;
            }
            if wrong::flag_zero(&o) != o.is_flag_word_nonzero() {
                caught[3] += 1;
            }
        }
        assert!(caught[0] > 0, "wrong vf9 never caught");
        assert!(caught[1] > 0, "wrong vf10 never caught");
        assert!(caught[2] > 0, "wrong vf11 never caught");
        assert!(caught[3] > 0, "wrong flag test never caught");
    }

    #[test]
    fn bound_radius_matches() {
        let mut rng = Rng(0x8AD1);
        let mut caught = 0;
        for i in 0..80u32 {
            let bits = if (i as usize) < F32_EDGE.len() {
                F32_EDGE[i as usize].to_bits()
            } else {
                rng.u32()
            };
            let mut fx = Fixture::build(&mut rng, false, false);
            fx.obj.w32(RADIUS, bits);
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let _guard = rt::script_lock();
            rt::set_script(&[]);
            let r = unsafe { fn_00c659c0::rw_00c659c0(this) };
            assert_eq!(rt::take_numbered(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let o = fx.lift();
            assert_eq!(r.to_bits(), o.bound_radius().to_bits(), "bits {bits:#x}");
            if wrong::radius_neg(&o).to_bits() != r.to_bits() {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong radius never caught");
    }

    #[test]
    fn describe_matches() {
        let mut rng = Rng(0xDE5C);
        let mut caught = 0;
        for i in 0..80u32 {
            let attached = i % 2 == 0;
            let mut fx = Fixture::build(&mut rng, attached, false);
            if attached {
                let m = random_matrix(&mut rng);
                write_matrix(&mut fx.mat, &m);
            }
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let out = Image::zeroed(12);
            let out_addr = out.addr();
            let _guard = rt::script_lock();
            rt::set_script(&[]);
            let r = unsafe { fn_00c65900::rw_c65900(this, out_addr) };
            assert_eq!(rt::take_numbered(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert_eq!(r, out_addr, "vf21 answers out");
            let o = fx.lift();
            let mut lo = [0u32; 3];
            o.describe_into(&mut lo);
            assert_eq!(out.r32(0), lo[0], "word 0 attached {attached}");
            assert_eq!(out.r32(4), lo[1], "word 1 attached {attached}");
            assert_eq!(out.r32(8), lo[2], "word 2 attached {attached}");
            let mut wo = [0u32; 3];
            wrong::describe_swapped(&o, &mut wo);
            if wo != lo {
                caught += 1;
            }
            let _ = out;
        }
        assert!(caught > 0, "wrong describe never caught");
    }

    /// A random pose record image plus its four words.
    fn pose_record(rng: &mut Rng) -> (Image, [u32; 4]) {
        let mut rec = Image::zeroed(16);
        let head = rng.u32();
        // Middle words travel as floats: edge patterns and arbitrary bits.
        let mid0 = if rng.u32() % 2 == 0 {
            F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()].to_bits()
        } else {
            rng.u32()
        };
        let mid1 = if rng.u32() % 2 == 0 {
            F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()].to_bits()
        } else {
            rng.u32()
        };
        let tail = rng.u32();
        rec.w32(0, head);
        rec.w32(4, mid0);
        rec.w32(8, mid1);
        rec.w32(12, tail);
        (rec, [head, mid0, mid1, tail])
    }

    #[test]
    fn pose_matches() {
        let mut rng = Rng(0x905E);
        let mut caught = 0;
        for _ in 0..60u32 {
            let fx = Fixture::build(&mut rng, false, false);
            let (rec, words4) = pose_record(&mut rng);
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let out = Image::zeroed(16);
            let out_addr = out.addr();
            let _guard = rt::script_lock();
            rt::set_script(&[]);
            rt::set_virtual(&[("pose", vec![rec.addr()])]);
            let r = unsafe { fn_00c65930::rw_c65930(this, out_addr) };
            let virt = rt::take_virtual();
            assert_eq!(rt::take_numbered(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            // The slot call carries a scratch address: its presence, not
            // its value, is compared.
            assert_eq!(virt.len(), 1, "one pose-slot call");
            assert_eq!(virt[0].0, "pose");
            assert_eq!(virt[0].1[0], this);
            assert_ne!(virt[0].1[1], 0, "scratch address passed");
            assert_eq!(r, words4[3], "vf20 answers the tail word");
            let o = fx.lift();
            let mut fake = Fake::new();
            fake.answer_block("pose", vec![words4.to_vec()]);
            let mut lo = PoseRecord {
                head: 0,
                mid0: 0.0,
                mid1: 0.0,
                tail: 0,
            };
            let lr = o.pose_into(&mut fake, &mut lo);
            assert_eq!(out.r32(0), lo.head);
            assert_eq!(out.r32(4), lo.mid0.to_bits());
            assert_eq!(out.r32(8), lo.mid1.to_bits());
            assert_eq!(out.r32(12), lo.tail);
            assert_eq!(lr, r);
            assert_eq!(fake.log, vec![("pose".to_string(), vec![])]);
            if wrong::pose_head(&lo) != r {
                caught += 1;
            }
            let _ = (out, rec);
        }
        assert!(caught > 0, "wrong pose answer never caught");
    }

    #[test]
    fn pose_followup_matches() {
        let mut rng = Rng(0xF011);
        let mut caught = 0;
        for _ in 0..60u32 {
            let fx = Fixture::build(&mut rng, false, false);
            let (rec, words4) = pose_record(&mut rng);
            let follow = rng.u32();
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let out = Image::zeroed(16);
            let out_addr = out.addr();
            let _guard = rt::script_lock();
            rt::set_script(&[]);
            rt::set_virtual(&[("pose", vec![rec.addr()]), ("followup", vec![follow])]);
            let r = unsafe { fn_00c65970::rw_c65970(this, out_addr) };
            let virt = rt::take_virtual();
            assert_eq!(rt::take_numbered(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert_eq!(virt.len(), 2, "pose slot then follow-up");
            assert_eq!(virt[0].0, "pose");
            assert_eq!(virt[0].1[0], this);
            assert_ne!(virt[0].1[1], 0, "scratch address passed");
            check_virtual(vec![virt[1].clone()], vec![("followup", vec![this])]);
            assert_eq!(r, follow, "vf23 answers the follow-up");
            let o = fx.lift();
            let mut fake = Fake::new();
            fake.answer_block("pose", vec![words4.to_vec()]);
            fake.answer("followup", vec![follow]);
            let mut lo = PoseRecord {
                head: 0,
                mid0: 0.0,
                mid1: 0.0,
                tail: 0,
            };
            let lr = o.pose_and_followup(&mut fake, &mut lo);
            assert_eq!(out.r32(0), lo.head);
            assert_eq!(out.r32(4), lo.mid0.to_bits());
            assert_eq!(out.r32(8), lo.mid1.to_bits());
            assert_eq!(out.r32(12), lo.tail);
            assert_eq!(lr, r);
            assert_eq!(
                fake.log,
                vec![
                    ("pose".to_string(), vec![]),
                    ("followup".to_string(), vec![]),
                ]
            );
            // The wrong version skips the follow-up: its log differs.
            let mut wf = Fake::new();
            wf.answer_block("pose", vec![words4.to_vec()]);
            let mut wo = lo;
            let wr = wrong::pose_no_followup(&o, &mut wf, &mut wo);
            if wr != r || wf.log != fake.log {
                caught += 1;
            }
            let _ = (out, rec);
        }
        assert!(caught > 0, "wrong follow-up never caught");
    }

    #[test]
    fn forward_matches() {
        let mut rng = Rng(0xF0A0);
        let mut caught = 0;
        for i in 0..60u32 {
            let linked = i % 2 == 0;
            let fx = Fixture::build(&mut rng, false, linked);
            let word = rng.u32();
            let first = rng.u32();
            // The successor answer differs from the notification answer
            // on linked cases, so the wrong version is caught there.
            let second = if linked { first ^ 0xFFFF_FFFF } else { 0 };
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let member = fx.obj.r32(MEMBER_A);
            let _guard = rt::script_lock();
            rt::set_script(&[(1, StubKind::Thiscall2, vec![first])]);
            rt::set_virtual(&[("successor", vec![second])]);
            let r = unsafe { fn_00c661a0::rw_00c661a0(this, word) };
            let numbered = rt::take_numbered();
            let virt = rt::take_virtual();
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert_eq!(numbered, vec![(1, vec![this, word])], "notify call");
            if linked {
                check_virtual(virt, vec![("successor", vec![member, word])]);
                assert_eq!(r, second, "linked answers the successor");
            } else {
                assert_eq!(virt, vec![], "no member, no successor call");
                assert_eq!(r, first, "unlinked answers the notification");
            }
            let o = fx.lift();
            let mut fake = Fake::new();
            fake.answer("notify", vec![first]);
            fake.answer("forward", vec![second]);
            let lr = o.forward_word(&mut fake, word);
            assert_eq!(lr, r);
            let mut expect = vec![("notify".to_string(), vec![word])];
            if linked {
                expect.push(("forward".to_string(), vec![words(o.member_a), word]));
            }
            assert_eq!(fake.log, expect, "lift calls");
            let mut wf = Fake::new();
            wf.answer("notify", vec![first]);
            let wr = wrong::forward_first(&o, &mut wf, word);
            if wr != r || wf.log != fake.log {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong forward never caught");
    }

    #[test]
    fn refresh_matches() {
        let mut rng = Rng(0x8EF8);
        let mut caught = 0;
        // Every flag combination: both clear, each alone, both set.
        let flags = [0u8, 1, 0x80, 0xFF];
        for (i, a) in flags.iter().cycle().take(64).enumerate() {
            let b = flags[(i * 3 + 1) % flags.len()];
            let mut fx = Fixture::build(&mut rng, false, false);
            fx.obj.w8(RFA, *a);
            fx.obj.w8(RFB, b);
            let tail = rng.u32();
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let _guard = rt::script_lock();
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![rng.u32()]),
                (2, StubKind::Thiscall1, vec![tail]),
            ]);
            let r = unsafe { fn_00c677b0::rw_00C677B0(this) };
            let numbered = rt::take_numbered();
            assert_eq!(rt::take_virtual(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let o = fx.lift();
            let mut fake = Fake::new();
            fake.answer("refresh", vec![tail]);
            let lr = o.maybe_refresh(&mut fake);
            if *a != 0 || b != 0 {
                assert_eq!(
                    numbered,
                    vec![(1, vec![this]), (2, vec![this])],
                    "hook then tail"
                );
                assert_eq!(r, tail, "refresh answers the tail");
                assert_eq!(
                    fake.log,
                    vec![
                        ("hook".to_string(), vec![]),
                        ("refresh".to_string(), vec![]),
                    ]
                );
            } else {
                assert_eq!(numbered, vec![], "no flags, no calls");
                assert_eq!(r, 0, "both clear answers zero");
                assert_eq!(fake.log, vec![]);
            }
            assert_eq!(lr, r);
            let mut wf = Fake::new();
            wf.answer("refresh", vec![tail]);
            let wr = wrong::refresh_and(&o, &mut wf);
            if wr != r || wf.log != fake.log {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong refresh never caught");
    }

    #[test]
    fn helper_matches() {
        let mut rng = Rng(0x4E48);
        let mut caught = 0;
        for i in 0..60u32 {
            let mut fx = Fixture::build(&mut rng, false, false);
            // The flag starts set (nonzero values across the byte range).
            let flag = if i < 16 {
                17 + (i as u8) * 13
            } else {
                rng.u8() | 1
            };
            fx.obj.w8(HFLAG, flag);
            let answer = rng.u32();
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let helper = this.wrapping_add(HELPER as u32);
            let _guard = rt::script_lock();
            rt::set_script(&[]);
            rt::set_virtual(&[("helper", vec![answer])]);
            let r = unsafe { fn_00c67800::rw_c67800(this) };
            let virt = rt::take_virtual();
            assert_eq!(rt::take_numbered(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[(HFLAG, 1)]);
            assert_eq!(fx.obj.r8(HFLAG), 0, "flag cleared");
            check_virtual(virt, vec![("helper", vec![helper, 0, 0xFFFF_FFFE])]);
            assert_eq!(r, answer);
            let mut o = fx.lift();
            // The lift runs from the pre-call state, like the rewrite did.
            o.helper_flag = flag;
            let mut fake = Fake::new();
            fake.answer("helper", vec![answer]);
            let lr = o.reset_helper(&mut fake);
            assert_eq!(lr, r);
            assert_eq!(o.helper_flag, 0, "lift clears the flag");
            assert_eq!(
                fake.log,
                vec![(
                    "helper".to_string(),
                    vec![words(Some(o.helper)), 0, 0xFFFF_FFFE],
                )]
            );
            // The wrong version never clears: compare against a fresh lift.
            let mut wo = fx.lift();
            wo.helper_flag = flag;
            let mut wf = Fake::new();
            wf.answer("helper", vec![answer]);
            let wr = wrong::helper_no_clear(&wo, &mut wf);
            if wr != r || wo.helper_flag != 0 {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong helper reset never caught");
    }

    #[test]
    fn draw_matches() {
        let mut rng = Rng(0xD8A0);
        let mut caught = [0u32; 2];
        let modes = [0u32, 0, 1, 1, 2, 2, 3, 7, 0xFFFF_FFFF];
        for (i, mode) in modes.iter().cycle().take(72).enumerate() {
            let mut fx = Fixture::build(&mut rng, false, false);
            fx.obj.w32(MODE, *mode);
            // Mode 2 runs both flag states; other modes take any word.
            let flag = if *mode == 2 {
                (i % 4 < 2) as u32
            } else {
                rng.u32()
            };
            fx.obj.w32(FLAG2A0, flag);
            let (a0, a1, a2, a3) = (rng.u32(), rng.u32(), rng.u32(), rng.u32());
            // The emit target: a block with a flaggable byte; the answer
            // alternates between the block and null on emitting modes.
            let mut emit = Image::random(0x60, &mut rng);
            let emit_addr = emit.addr();
            let emit_some = *mode == 1 && i % 3 != 2 || *mode == 2 && flag == 0 && i % 2 == 0;
            let emit_ans = if (*mode == 1 || *mode == 2) && emit_some {
                emit_addr
            } else {
                0
            };
            // The flag callee answers consistently with the flag word:
            // the lift reads the same value from its own field.
            let flag_ans = u32::from(flag != 0);
            let before = fx.obj.buf.to_vec();
            let emit_before = emit.buf[0x59];
            let this = fx.this();
            let _guard = rt::script_lock();
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![flag_ans]),
                (2, StubKind::Thiscall3, vec![rng.u32()]),
                (3, StubKind::Thiscall3, vec![rng.u32()]),
                (4, StubKind::Thiscall5, vec![emit_ans]),
            ]);
            let r = unsafe { fn_00c64520::rw_00c64520(this, a0, a1, a2, a3) };
            let numbered = rt::take_numbered();
            assert_eq!(rt::take_virtual(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert_eq!(r, 0, "draw always answers zero");
            let o = fx.lift();
            let mut fake = Fake::new();
            fake.answer("draw.emit", vec![emit_ans]);
            o.emit_draw_commands(&mut fake, a0, a1, a2);
            if *mode == 0 {
                assert_eq!(
                    numbered,
                    vec![(2, vec![this, a0, a1]), (3, vec![this, a0, a1])],
                    "mode 0 calls both emitters"
                );
                assert_eq!(
                    fake.log,
                    vec![
                        ("draw.a".to_string(), vec![a0, a1]),
                        ("draw.b".to_string(), vec![a0, a1]),
                    ]
                );
            } else if *mode == 1 {
                assert_eq!(numbered, vec![(4, vec![this, a0, a1, a2, 0xFFFF_FFFF])]);
                if emit_ans != 0 {
                    assert_eq!(
                        emit.buf[0x59],
                        emit_before | 1,
                        "mode 1 sets the target bit"
                    );
                    assert_eq!(
                        fake.log,
                        vec![
                            ("draw.emit".to_string(), vec![a0, a1, a2]),
                            ("draw.mark".to_string(), vec![emit_ans]),
                        ]
                    );
                } else {
                    assert_eq!(emit.buf[0x59], emit_before, "null target marks nothing");
                    assert_eq!(fake.log, vec![("draw.emit".to_string(), vec![a0, a1, a2])]);
                }
            } else if *mode == 2 {
                // The rewrite checks the flag through its callee; the lift
                // through its own predicate: each side against its own log.
                if flag == 0 {
                    assert_eq!(
                        numbered,
                        vec![(1, vec![this]), (4, vec![this, a0, a1, a2, 0xFFFF_FFFF])]
                    );
                    assert_eq!(fake.log, vec![("draw.emit".to_string(), vec![a0, a1, a2])]);
                } else {
                    assert_eq!(numbered, vec![(1, vec![this])], "flag set emits nothing");
                    assert_eq!(fake.log, vec![]);
                }
            } else {
                assert_eq!(numbered, vec![], "other modes call nothing");
                assert_eq!(fake.log, vec![]);
            }
            // Wrong versions: logs compared against fresh fakes.
            let mut wf = Fake::new();
            wf.answer("draw.emit", vec![emit_ans]);
            wrong::draw_a_only(&o, &mut wf, a0, a1, a2);
            if wf.log != fake.log {
                caught[0] += 1;
            }
            let mut wf = Fake::new();
            wf.answer("draw.emit", vec![emit_ans]);
            wrong::draw_no_mark(&o, &mut wf, a0, a1, a2);
            if wf.log != fake.log {
                caught[1] += 1;
            }
            let _ = emit;
        }
        assert!(caught[0] > 0, "wrong draw dispatch never caught");
        assert!(caught[1] > 0, "wrong draw mark never caught");
    }

    /// One update case: every knob the per-frame update reads.
    struct UCase {
        entry_byte: u8,
        guard_a: u32,
        guard_b: u32,
        flag: u8,
        mode: u32,
        weight: f32,
        indices: [u32; 4],
        early_word_val: u32,
        early_tail_ans: u32,
        early_call_ans: u32,
        sel: i32,
        edx_alt: i32,
        eax: i32,
        ecx: i32,
        w2c: u16,
        win_lo: f32,
        win_hi: f32,
        win_scale: f32,
        setup_flag: u32,
        store_val: f32,
        acc_flag: u32,
        acc_vals: [f32; 3],
        bones_a: [[f32; 3]; 8],
        bones_b: [[f32; 3]; 8],
        distinct_sets: bool,
        k0: f32,
        k1: f32,
        wgt_scale: f32,
        e18_scale: f32,
        kn: f32,
        wx: f32,
        j_a4: f32,
        j_a5: f32,
        j_a6: f32,
        j_a8: u32,
        k_a4: f32,
        k_a5: f32,
        k_a6: f32,
        k_a8: u32,
        qk: f32,
        submit_ans: u32,
        table_idx: u16,
        linked_b: bool,
        chain_linked: bool,
        attached: bool,
    }

    /// Small maze counters: edges and small values.
    fn random_counter(rng: &mut Rng) -> i32 {
        const EDGE: [i32; 10] = [-1, 0, 1, 5, 6, 7, 0x13, 0x14, 0x15, 0x100];
        if rng.u32() % 2 == 0 {
            EDGE[(rng.u32() as usize) % EDGE.len()]
        } else {
            (rng.u32() % 41) as i32 - 5
        }
    }

    /// Eight bone rows: edge patterns and arbitrary bits.
    fn random_bones(rng: &mut Rng) -> [[f32; 3]; 8] {
        let mut b = [[0.0f32; 3]; 8];
        for row in b.iter_mut() {
            for x in row.iter_mut() {
                *x = if rng.u32() % 4 == 0 {
                    F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()]
                } else {
                    rng.f32_bits()
                };
            }
        }
        b
    }

    /// A default case running the full two-bone blend.
    fn default_ucase(rng: &mut Rng) -> UCase {
        UCase {
            entry_byte: 0,
            guard_a: 0,
            guard_b: 1,
            flag: 0,
            mode: 1,
            weight: rng.f32_bits(),
            indices: [rng.u32() % 8, rng.u32() % 8, rng.u32() % 8, rng.u32() % 8],
            early_word_val: rng.u32(),
            early_tail_ans: rng.u32(),
            early_call_ans: rng.u32(),
            sel: 0,
            edx_alt: 0,
            eax: 0,
            ecx: 0,
            w2c: 0x10,
            win_lo: -100.0,
            win_hi: 100.0,
            win_scale: 1.0,
            setup_flag: 0,
            store_val: rng.f32_bits(),
            acc_flag: 1,
            acc_vals: [rng.f32_bits(), rng.f32_bits(), rng.f32_bits()],
            bones_a: random_bones(rng),
            bones_b: random_bones(rng),
            distinct_sets: false,
            k0: rng.f32_bits(),
            k1: rng.f32_bits(),
            wgt_scale: rng.f32_bits(),
            e18_scale: rng.f32_bits(),
            kn: rng.f32_bits(),
            wx: rng.f32_bits(),
            j_a4: rng.f32_bits(),
            j_a5: rng.f32_bits(),
            j_a6: rng.f32_bits(),
            j_a8: rng.u32(),
            k_a4: rng.f32_bits(),
            k_a5: rng.f32_bits(),
            k_a6: rng.f32_bits(),
            k_a8: rng.u32(),
            qk: rng.f32_bits(),
            submit_ans: rng.u32(),
            table_idx: 0,
            linked_b: true,
            chain_linked: true,
            attached: true,
        }
    }

    /// Sixteen matrix words: edge float bits, tags and arbitrary words.
    fn matrix_words(rng: &mut Rng) -> [u32; 16] {
        let mut m = [0u32; 16];
        for (i, w) in m.iter_mut().enumerate() {
            *w = if i % 3 == 0 {
                F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()].to_bits()
            } else {
                rng.u32()
            };
        }
        m
    }

    #[test]
    fn bounds_box_matches() {
        let mut rng = Rng(0xB027);
        let mut caught = 0;
        // A targeted ordinary case first, so the wrong version is caught
        // even if every random case were degenerate.
        let mut targeted = true;
        for i in 0..100u32 {
            let mut fx = Fixture::build(&mut rng, false, false);
            let (ca, cb, scales, mwords) = if targeted {
                targeted = false;
                (
                    [1.0f32, 2.0, 3.0],
                    [4.0f32, 5.0, 6.0],
                    BoundsScale {
                        gx: 1.0,
                        gy: 1.0,
                        gz: 1.0,
                        abs_mask: 0x7FFF_FFFF,
                    },
                    // Identity-ish matrix with translation and tags.
                    [
                        1.0f32.to_bits(),
                        0,
                        0,
                        0x1111_1111,
                        0,
                        1.0f32.to_bits(),
                        0,
                        0x2222_2222,
                        0,
                        0,
                        1.0f32.to_bits(),
                        0,
                        10.0f32.to_bits(),
                        20.0f32.to_bits(),
                        30.0f32.to_bits(),
                        0,
                    ],
                )
            } else if i < F32_EDGE.len() as u32 {
                (
                    [F32_EDGE[i as usize], rng.f32_bits(), rng.f32_bits()],
                    [rng.f32_bits(), rng.f32_bits(), F32_EDGE[i as usize]],
                    random_scales(&mut rng),
                    matrix_words(&mut rng),
                )
            } else {
                (
                    [rng.f32_bits(), rng.f32_bits(), rng.f32_bits()],
                    [rng.f32_bits(), rng.f32_bits(), rng.f32_bits()],
                    random_scales(&mut rng),
                    matrix_words(&mut rng),
                )
            };
            for (k, v) in ca.iter().enumerate() {
                fx.obj.w32(CORNER_A + k * 4, v.to_bits());
            }
            for (k, v) in cb.iter().enumerate() {
                fx.obj.w32(CORNER_B + k * 4, v.to_bits());
            }
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let placement = this.wrapping_add(TRIPLE as u32);
            let out = Image::zeroed(32);
            let out_addr = out.addr();
            let _guard = rt::script_lock();
            rt::set_script(&[(1, StubKind::MatrixWrite, vec![0])]);
            rt::set_writes(1, vec![mwords.to_vec()]);
            unsafe {
                rt::SCALE_X = scales.gx.to_bits();
                rt::SCALE_Y = scales.gy.to_bits();
                rt::SCALE_Z = scales.gz.to_bits();
                rt::ABS_MASK = scales.abs_mask;
            }
            let r = unsafe { fn_00c65560::rw_00c65560(this, out_addr) };
            let numbered = rt::take_numbered();
            assert_eq!(rt::take_virtual(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert_eq!(r, out_addr, "vf27 answers out");
            assert_eq!(numbered.len(), 1, "one matrix call");
            assert_eq!(numbered[0].0, 1);
            assert_eq!(numbered[0].1[0], placement);
            assert_ne!(numbered[0].1[1], 0, "scratch buffer passed");
            let o = fx.lift();
            let mut fake = Fake::new();
            fake.answer_block("matrix", vec![mwords.to_vec()]);
            let mut lo = WorldBounds {
                min: [0.0; 3],
                lo_tag: 0,
                max: [0.0; 3],
                hi_tag: 0,
            };
            o.world_bounds(&mut fake, &scales, &mut lo);
            assert_eq!(out.r32(0), lo.min[0].to_bits(), "min x case {i}");
            assert_eq!(out.r32(4), lo.min[1].to_bits(), "min y case {i}");
            assert_eq!(out.r32(8), lo.min[2].to_bits(), "min z case {i}");
            assert_eq!(out.r32(12), lo.lo_tag, "tag case {i}");
            assert_eq!(out.r32(16), lo.max[0].to_bits(), "max x case {i}");
            assert_eq!(out.r32(20), lo.max[1].to_bits(), "max y case {i}");
            assert_eq!(out.r32(24), lo.max[2].to_bits(), "max z case {i}");
            assert_eq!(out.r32(28), lo.hi_tag, "tag case {i}");
            assert_eq!(
                fake.log,
                vec![("matrix".to_string(), vec![words(Some(o.placement))])]
            );
            let mut wf = Fake::new();
            wf.answer_block("matrix", vec![mwords.to_vec()]);
            let mut wo = lo;
            wrong::box_min_plus(&o, &mut wf, &scales, &mut wo);
            if wo.min != lo.min {
                caught += 1;
            }
            let _ = out;
        }
        assert!(caught > 0, "wrong box never caught");
    }

    #[test]
    fn bounds_rect_matches() {
        let mut rng = Rng(0x8EC7);
        let mut caught = 0;
        // A targeted case first: the second corner extends every bound,
        // so the wrong version (which skips it) is caught for sure.
        let mut targeted = true;
        for i in 0..100u32 {
            let attached = i % 2 == 0;
            let mut fx = Fixture::build(&mut rng, attached, false);
            let (ca, cb) = if targeted {
                targeted = false;
                ([1.0f32, 1.0, 1.0], [50.0f32, 60.0, 70.0])
            } else if i < F32_EDGE.len() as u32 * 2 {
                (
                    [
                        F32_EDGE[i as usize % F32_EDGE.len()],
                        rng.f32_bits(),
                        rng.f32_bits(),
                    ],
                    [rng.f32_bits(), rng.f32_bits(), rng.f32_bits()],
                )
            } else {
                (
                    [rng.f32_bits(), rng.f32_bits(), rng.f32_bits()],
                    [rng.f32_bits(), rng.f32_bits(), rng.f32_bits()],
                )
            };
            for (k, v) in ca.iter().enumerate() {
                fx.obj.w32(CORNER_A + k * 4, v.to_bits());
            }
            for (k, v) in cb.iter().enumerate() {
                fx.obj.w32(CORNER_B + k * 4, v.to_bits());
            }
            let mat = if attached {
                // The targeted case pushes x through the corner sum, so the
                // skipped second corner is the unique x maximum.
                let m = if i == 0 {
                    read_matrix(&{
                        let mut id = Image::zeroed(MAT_SIZE);
                        id.w32(0, 1.0f32.to_bits());
                        id.w32(0x10, 1.0f32.to_bits());
                        id.w32(0x14, 1.0f32.to_bits());
                        id.w32(0x20, 1.0f32.to_bits());
                        id
                    })
                } else {
                    random_matrix(&mut rng)
                };
                write_matrix(&mut fx.mat, &m);
                Some(m)
            } else {
                None
            };
            // Four corner answers (x, y, plus an unread filler third).
            let mut corner_blocks = Vec::with_capacity(4);
            for _ in 0..4 {
                corner_blocks.push(vec![rng.u32(), rng.u32(), rng.u32()]);
            }
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let placement = this.wrapping_add(TRIPLE as u32);
            let out = Image::zeroed(16);
            let out_addr = out.addr();
            let _guard = rt::script_lock();
            rt::set_script(&[(1, StubKind::CornerWrite, vec![0, 0, 0, 0])]);
            rt::set_writes(1, corner_blocks.clone());
            let r = unsafe { fn_00c659d0::rw_00c659d0(this, out_addr) };
            let numbered = rt::take_numbered();
            let corners = rt::take_corners();
            assert_eq!(rt::take_virtual(), vec![]);
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert_eq!(r, out_addr, "vf26 answers out");
            let o = fx.lift();
            // The four pushed corners, in round order.
            let expect_corners = [
                [ca[0].to_bits(), ca[1].to_bits(), ca[2].to_bits()],
                [cb[0].to_bits(), cb[1].to_bits(), cb[2].to_bits()],
                [cb[0].to_bits(), ca[1].to_bits(), ca[2].to_bits()],
                [ca[0].to_bits(), cb[1].to_bits(), cb[2].to_bits()],
            ];
            let mut fake = Fake::new();
            if attached {
                assert_eq!(numbered, vec![], "attached path calls nothing");
                assert!(corners.is_empty(), "attached path pushes nothing");
                assert!(mat.is_some());
            } else {
                assert_eq!(numbered.len(), 4, "one corner call per round");
                for (k, (id, args)) in numbered.iter().enumerate() {
                    assert_eq!(*id, 1);
                    assert_eq!(args[1], placement, "round {k} placement");
                    assert_ne!(args[0], 0, "round {k} out buffer");
                    assert_ne!(args[2], 0, "round {k} corner vector");
                }
                assert_eq!(corners, expect_corners, "pushed corners in order");
                fake.answer_block(
                    "corner",
                    corner_blocks.iter().map(|b| b[..2].to_vec()).collect(),
                );
            }
            let mut lo = BoundsRect {
                min_x: 0.0,
                max_y: 0.0,
                max_x: 0.0,
                min_y: 0.0,
            };
            o.bounding_rect(&mut fake, &mut lo);
            assert_eq!(out.r32(0), lo.min_x.to_bits(), "min x case {i}");
            assert_eq!(out.r32(4), lo.max_y.to_bits(), "max y case {i}");
            assert_eq!(out.r32(8), lo.max_x.to_bits(), "max x case {i}");
            assert_eq!(out.r32(12), lo.min_y.to_bits(), "min y case {i}");
            if attached {
                assert_eq!(fake.log, vec![]);
            } else {
                let expect_log: Vec<(String, Vec<u32>)> = expect_corners
                    .iter()
                    .map(|c| {
                        (
                            "corner".to_string(),
                            vec![words(Some(o.placement)), c[0], c[1], c[2]],
                        )
                    })
                    .collect();
                assert_eq!(fake.log, expect_log, "lift corner calls");
            }
            let mut wf = Fake::new();
            if !attached {
                wf.answer_block(
                    "corner",
                    corner_blocks.iter().map(|b| b[..2].to_vec()).collect(),
                );
            }
            let mut wo = lo;
            wrong::rect_skip_second(&o, &mut wf, &mut wo);
            // On the detached path the pushed set is identical, so the
            // outputs agree and the differing corner sequence catches it.
            if wo.min_x.to_bits() != lo.min_x.to_bits()
                || wo.max_y.to_bits() != lo.max_y.to_bits()
                || wo.max_x.to_bits() != lo.max_x.to_bits()
                || wo.min_y.to_bits() != lo.min_y.to_bits()
                || wf.log != fake.log
            {
                caught += 1;
            }
            let _ = out;
        }
        assert!(caught > 0, "wrong rectangle never caught");
    }

    #[test]
    fn teardown_matches() {
        let mut rng = Rng(0xD708);
        let mut caught = 0;
        let mut full_runs = 0;
        for i in 0..48u32 {
            // Targeted arm coverage first, then breadth.
            let (mode, ask, word, gate, gate_ans, links) = match i {
                0 => (0u32, 1u32, 7i32, 1u32, 1u32, true), // full run
                1 => (0, 0, 7, 1, 1, true),                // ask says no
                2 => (0, 0x100, 7, 1, 1, true),            // ask low byte zero
                3 => (0, 1, -1, 1, 1, true),               // word is -1
                4 => (0, 1, 7, 0, 1, true),                // gate shut
                5 => (0, 1, 7, 1, 0, true),                // gate says no
                6 => (0, 1, 42, 1, 1, false),              // nothing linked
                7 => (1, 1, 7, 1, 1, true),                // mode 1 sets the bit
                8 => (2, 1, 7, 1, 1, true),                // mode 2 tears nothing
                9 => (3, 1, 7, 1, 1, true),                // other mode
                _ => (
                    [0u32, 0, 1, 2, 3][(i as usize) % 5],
                    [1u32, 0, 0x1FF, 0x100][(i as usize) % 4],
                    [-1i32, 0, 7, -5][(i as usize) % 4],
                    (i % 2) as u32,
                    ((i / 3) % 2) as u32,
                    i % 6 != 5,
                ),
            };
            let mut fx = Fixture::build(&mut rng, false, false);
            // Teardown-only extras: second member, blocks, context.
            let mut member_b_obj = Image::random(0x10, &mut rng);
            member_b_obj.w32(0, fx.member_vt.addr());
            let blocks_img = Image::random(12, &mut rng);
            let ctx_img = Image::random(8, &mut rng);
            let link_a = links && i % 5 != 4;
            let link_b = links && i % 7 != 6;
            let set = |k: u32| links && (i + k * 3) % (4 + k) != 3;
            let (set0, set1, set2) = (set(0), set(1), set(2));
            fx.obj
                .w32(MEMBER_A, if link_a { fx.member_obj.addr() } else { 0 });
            fx.obj
                .w32(MEMBER_B, if link_b { member_b_obj.addr() } else { 0 });
            fx.obj.w32(BLOCK0, if set0 { blocks_img.at(0) } else { 0 });
            fx.obj.w32(BLOCK1, if set1 { blocks_img.at(4) } else { 0 });
            fx.obj.w32(BLOCK2, if set2 { blocks_img.at(8) } else { 0 });
            fx.obj.w32(MODE, mode);
            fx.obj.w32(
                FLAGS24,
                [0u32, 0x0400_0000, 0xFFFF_FFFF, 0x1234_5678][(i as usize) % 4],
            );
            let idx = (i % 4) as u16;
            fx.obj.w16(TBLIDX, idx);
            // Four-entry table; the indexed entry carries the case word.
            let mut table = Image::zeroed(16);
            let mut entries = Vec::with_capacity(4);
            for k in 0..4u16 {
                let mut e = Image::random(0x5C, &mut rng);
                let w = if k == idx { word } else { rng.u32() as i32 };
                e.w16(0x58, w as u16);
                table.w32((k as usize) * 4, e.addr());
                entries.push(e);
            }
            let ctx_some = links && i % 3 != 1;
            fx.obj.w32(CTX, if ctx_some { ctx_img.addr() } else { 0 });
            fx.obj.w32(GATE, gate);
            fx.obj.w8(DONE, [0x5Au8, 0, 0xFF][(i as usize) % 3]);
            let stamp = Image::random(4, &mut rng);
            let fixed = Image::random(4, &mut rng);
            let base_ans = rng.u32();
            let before = fx.obj.buf.to_vec();
            let this = fx.this();
            let ha = fx.obj.r32(MEMBER_A);
            let hb = fx.obj.r32(MEMBER_B);
            let blks = [fx.obj.r32(BLOCK0), fx.obj.r32(BLOCK1), fx.obj.r32(BLOCK2)];
            let ctx_word = fx.obj.r32(CTX);
            let _guard = rt::script_lock();
            rt::set_script(&[
                (
                    2,
                    StubKind::Thiscall1,
                    vec![rng.u32(), rng.u32(), rng.u32()],
                ),
                (3, StubKind::Cdecl1, vec![rng.u32(), rng.u32(), rng.u32()]),
                (4, StubKind::Thiscall1, vec![ask]),
                (5, StubKind::Cdecl1, vec![gate_ans]),
                (6, StubKind::Thiscall2, vec![rng.u32()]),
                (7, StubKind::Thiscall2, vec![rng.u32()]),
                (8, StubKind::Cdecl1, vec![rng.u32()]),
                (9, StubKind::Thiscall1, vec![base_ans]),
            ]);
            // After the script: the map is cleared with it.
            rt::set_relocated(&[
                (DTOR_VTABLE, stamp.addr()),
                (REGISTRY_G, table.addr()),
                (FIXED_CTX, fixed.addr()),
            ]);
            rt::set_virtual(&[("destroy", vec![rng.u32(), rng.u32()])]);
            let r = unsafe { fn_00c64380::rw_00c64380(this) };
            let numbered = rt::take_numbered();
            let virt = rt::take_virtual();
            drop(_guard);
            // Expected calls, walked from the case inputs.
            let mut exp_n: Vec<(u32, Vec<u32>)> = vec![];
            let mut exp_v: Vec<(String, Vec<u32>)> = vec![];
            let mut exp_l: Vec<(String, Vec<u32>)> = vec![];
            if mode == 0 {
                if link_a {
                    exp_v.push(("destroy".to_string(), vec![ha, 1]));
                    exp_l.push(("destroy".to_string(), vec![ha]));
                }
                for b in blks.iter() {
                    if *b != 0 {
                        exp_n.push((2, vec![*b]));
                        exp_n.push((3, vec![*b]));
                        exp_l.push(("block.teardown".to_string(), vec![*b]));
                        exp_l.push(("block.free".to_string(), vec![*b]));
                    }
                }
                if link_b {
                    exp_v.push(("destroy".to_string(), vec![hb, 1]));
                    exp_l.push(("destroy".to_string(), vec![hb]));
                }
                exp_n.push((4, vec![this]));
                exp_l.push(("reg.ask".to_string(), vec![]));
                if ask & 0xff != 0 {
                    exp_l.push(("reg.word".to_string(), vec![idx as u32]));
                    if word != -1 && gate != 0 {
                        exp_n.push((5, vec![word as u32]));
                        exp_l.push(("reg.gate".to_string(), vec![word as u32]));
                        if gate_ans & 0xff != 0 {
                            exp_n.push((6, vec![fixed.addr(), 0]));
                            exp_n.push((7, vec![ctx_word, 3]));
                            exp_n.push((8, vec![word as u32]));
                            exp_l.push(("reg.run".to_string(), vec![]));
                            exp_l.push(("reg.ctx".to_string(), vec![ctx_word, 3]));
                            exp_l.push(("reg.tell".to_string(), vec![word as u32]));
                            full_runs += 1;
                        }
                    }
                }
            }
            exp_n.push((9, vec![this]));
            exp_l.push(("base".to_string(), vec![]));
            assert_eq!(numbered, exp_n, "numbered calls case {i}");
            assert_eq!(virt, exp_v, "virtual calls case {i}");
            assert_eq!(r, base_ans, "teardown answers the base");
            // Image effects: the stamp always lands; members, blocks, the
            // flag bit and the done byte per the arms taken.
            let mut changed = vec![(VT, 4)];
            assert_eq!(fx.obj.r32(VT), stamp.addr(), "table stamp case {i}");
            if mode == 0 {
                if link_a {
                    assert_eq!(fx.obj.r32(MEMBER_A), 0, "member A cleared");
                    changed.push((MEMBER_A, 4));
                }
                if link_b {
                    assert_eq!(fx.obj.r32(MEMBER_B), 0, "member B cleared");
                    changed.push((MEMBER_B, 4));
                }
                for (k, off) in [BLOCK0, BLOCK1, BLOCK2].iter().enumerate() {
                    if blks[k] != 0 {
                        assert_eq!(fx.obj.r32(*off), 0, "block {k} cleared");
                        changed.push((*off, 4));
                    }
                }
            }
            let flags_before = u32::from_le_bytes(before[FLAGS24..FLAGS24 + 4].try_into().unwrap());
            let flags_after = if mode == 1 {
                flags_before | 0x0400_0000
            } else {
                flags_before
            };
            assert_eq!(fx.obj.r32(FLAGS24), flags_after, "flag word case {i}");
            if flags_after != flags_before {
                changed.push((FLAGS24, 4));
            }
            assert_eq!(fx.obj.r8(DONE), 0, "done byte cleared");
            if before[DONE] != 0 {
                changed.push((DONE, 1));
            }
            assert_only_changed(&before, &fx.obj.buf, &changed);
            // The lift from the pre-call state (restoring the words the
            // rewrite overwrote, so both sides start equal).
            let mut fake = Fake::new();
            fake.answer("reg.ask", vec![ask]);
            fake.answer("reg.word", vec![word as u32]);
            fake.answer("reg.gate", vec![gate_ans]);
            fake.answer("base", vec![base_ans]);
            let mut pre = fx.lift();
            pre.member_a = if link_a { cookie(ha) } else { None };
            pre.member_b = if link_b { cookie(hb) } else { None };
            for (k, b) in blks.iter().enumerate() {
                pre.blocks[k] = cookie(*b);
            }
            pre.flags_24 = flags_before;
            pre.done_2ac = before[DONE];
            let mut wo = pre.clone();
            let lr = pre.tear_down(&mut fake);
            assert_eq!(lr, r, "lift answers the base");
            assert_eq!(fake.log, exp_l, "lift calls case {i}");
            assert_eq!(pre.member_a.is_none(), !link_a || mode == 0);
            assert_eq!(pre.member_b.is_none(), !link_b || mode == 0);
            for (k, b) in blks.iter().enumerate() {
                assert_eq!(pre.blocks[k].is_none(), *b == 0 || mode == 0, "block {k}");
            }
            assert_eq!(pre.flags_24, flags_after);
            assert_eq!(pre.done_2ac, 0);
            // The wrong version tears down in every mode.
            let mut wf = Fake::new();
            wf.answer("reg.ask", vec![ask]);
            wf.answer("reg.word", vec![word as u32]);
            wf.answer("reg.gate", vec![gate_ans]);
            wf.answer("base", vec![base_ans]);
            let wr = wrong::destroy_always(&mut wo, &mut wf);
            if wr != r || wf.log != fake.log || wo.member_a != pre.member_a {
                caught += 1;
            }
            let _ = (
                member_b_obj,
                blocks_img,
                ctx_img,
                table,
                entries,
                stamp,
                fixed,
            );
        }
        assert!(full_runs > 0, "no full registry run covered");
        assert!(caught > 0, "wrong teardown never caught");
    }

    #[test]
    fn update_matches() {
        const ONE: u32 = 0x3F80_0000;
        let mut rng = Rng(0x9DA7);
        let mut caught = 0;
        let mut full_j = 0;
        let mut full_k = 0;
        for i in 0..64u32 {
            let mut c = default_ucase(&mut rng);
            match i {
                0 => {} // full J
                1 => {
                    c.mode = 2;
                } // full K
                2 => {
                    c.entry_byte = 1;
                } // skip second entry call
                3 => {
                    c.guard_a = 1;
                    c.flag = 1;
                } // early tail
                4 => {
                    c.guard_a = 1;
                    c.flag = 0;
                    c.table_idx = 2;
                } // early call
                5 => {
                    c.guard_a = 0x100;
                } // low byte zero: not the early path
                6 => {
                    c.guard_b = 0x1234_5600;
                } // full-word zero-path return
                7 => {
                    c.sel = 0x14;
                    c.eax = -1;
                    c.ecx = 10;
                    c.w2c = 5;
                } // maze go
                8 => {
                    c.sel = 0x14;
                    c.eax = -1;
                    c.ecx = 3;
                    c.w2c = 5;
                } // maze falls to window (go)
                9 => {
                    c.sel = 0x14;
                    c.eax = -1;
                    c.ecx = 3;
                    c.w2c = 5;
                    c.win_lo = -5.0;
                    c.win_hi = -3.0;
                } // window stop
                10 => {
                    c.sel = 5;
                } // edx < 6: go
                11 => {
                    c.sel = 6;
                    c.eax = -1;
                    c.ecx = 2;
                    c.w2c = 9;
                } // go
                12 => {
                    c.sel = 6;
                    c.eax = -1;
                    c.ecx = 9;
                    c.w2c = 9;
                } // window go
                13 => {
                    c.sel = 7;
                    c.win_lo = -5.0;
                    c.win_hi = -3.0;
                    c.w2c = 0;
                    c.mode = 2;
                } // stop, no setup, K
                14 => {
                    c.sel = 0x20;
                    c.setup_flag = 2;
                } // go but gated: no setup
                15 => {
                    c.mode = 2;
                    c.indices = [0xFFFF_FFFF, 1, 2, 3];
                } // K guard exits
                16 => {
                    c.mode = 0;
                    c.indices = [0, 1, 2, 0x8000_0000];
                } // K guard returns the word
                17 => {
                    c.indices = [0xFFFF_FFFF, 1, 2, 3];
                } // J guard on idx_a
                18 => {
                    c.indices = [0, 1, 0x8000_0001, 3];
                } // J guard on idx_b
                19 => {
                    c.acc_flag = 0;
                    c.acc_vals = [7.0, 8.0, 9.0];
                } // acc zero path
                20 => {
                    c.attached = false;
                    c.guard_b = 0;
                } // detached early exit
                21 => {
                    c.attached = false;
                    c.guard_a = 1;
                    c.flag = 0;
                } // detached C path
                22 => {
                    c.attached = false;
                    c.mode = 2;
                    c.indices = [0, 0xFFFF_FFFF, 2, 3];
                } // detached K-guard exit
                23 => {
                    c.mode = 2;
                    c.distinct_sets = true;
                } // distinct bone sets
                24 => {
                    c.bones_a = [[f32::NAN; 3]; 8];
                    c.bones_a[0] = [1.0, f32::INFINITY, f32::from_bits(0x7F80_0001)];
                    c.kn = f32::NAN;
                    c.qk = f32::from_bits(0xFF80_0001);
                } // NaN-heavy J
                25 => {
                    c.sel = -1;
                    c.edx_alt = 3;
                } // alternate counter
                26 => {
                    c.sel = -1;
                    c.edx_alt = 6;
                    c.eax = 100;
                    c.w2c = 0;
                } // alt + window
                27 => {
                    c.mode = 0;
                } // mode 0 selects K
                28 => {
                    c.mode = 0xFFFF_FFFF;
                } // odd mode selects K
                29 => {
                    c.sel = 6;
                    c.eax = 5;
                    c.ecx = 99;
                    c.w2c = 0xFFFF;
                } // eax folds into ecx
                _ => {
                    c.guard_a = [0u32, 0, 0, 1][(i as usize) % 4];
                    c.guard_b = [1u32, 1, 1, 0, 0x100][(i as usize) % 5];
                    c.flag = (i % 2) as u8;
                    c.mode = [1u32, 2, 0, 1, 3][(i as usize) % 5];
                    c.sel = random_counter(&mut rng);
                    c.edx_alt = random_counter(&mut rng);
                    c.eax = random_counter(&mut rng);
                    c.ecx = random_counter(&mut rng);
                    c.w2c = rng.u32() as u16;
                    c.setup_flag = [0u32, 2, 3][(i as usize) % 3];
                    c.acc_flag = (i % 2) as u32;
                    c.table_idx = (i % 4) as u16;
                    c.distinct_sets = i % 5 == 0;
                    c.linked_b = i % 3 != 2;
                    c.chain_linked = i % 3 != 0;
                    c.attached = i % 4 != 3;
                }
            }
            // Path prediction, then safety: link what the path dereferences.
            let c_path = c.guard_a & 0xff != 0;
            let gb_zero = c.guard_b & 0xff == 0;
            let main_path = !c_path && !gb_zero;
            let k_pass = c.mode != 1
                && (c.indices[0] as i32) >= 0
                && (c.indices[1] as i32) >= 0
                && (c.indices[2] as i32) >= 0
                && (c.indices[3] as i32) >= 0;
            let j_pass = c.mode == 1 && (c.indices[0] as i32) >= 0 && (c.indices[2] as i32) >= 0;
            let reaches_math = main_path && ((c.mode != 1 && k_pass) || (c.mode == 1 && j_pass));
            let edx = if c.sel != -1 { c.sel } else { c.edx_alt };
            let w2cl = u32::from(c.w2c);
            let go1 = if edx > 0x14 {
                true
            } else {
                let eax = c.eax;
                let mut ecx = c.ecx;
                if edx > 0x13 {
                    if eax != -1 {
                        ecx = eax;
                    }
                    ecx > ((w2cl & 0x3f) as i32)
                } else if edx < 6 {
                    true
                } else if edx >= 7 {
                    false
                } else {
                    if eax != -1 {
                        ecx = eax;
                    }
                    ecx < ((w2cl & 0x3f) as i32)
                }
            };
            let go_g = if go1 {
                true
            } else {
                let x = (w2cl as i32) as f32 * c.win_scale;
                c.win_lo > x || c.win_hi > x
            };
            let setup_runs = main_path && go_g && (c.setup_flag & 2) == 0;
            if c_path {
                c.linked_b = true;
            }
            // The original loads the chain pointer eagerly on every main
            // path (storing only on setup): it stays linked there.
            if main_path {
                c.chain_linked = true;
            }
            if reaches_math {
                c.attached = true;
            }
            // Fixture: object plus the update's world.
            let mut fx = Fixture::build(&mut rng, c.attached, false);
            let mut member_b_obj = Image::random(0x10, &mut rng);
            member_b_obj.w32(0, fx.member_vt.addr());
            let blocks_img = Image::random(12, &mut rng);
            fx.obj.w32(
                MEMBER_A,
                if rng.u32() % 2 == 0 {
                    fx.member_obj.addr()
                } else {
                    0
                },
            );
            fx.obj
                .w32(MEMBER_B, if c.linked_b { member_b_obj.addr() } else { 0 });
            for (k, off) in [BLOCK0, BLOCK1, BLOCK2].iter().enumerate() {
                fx.obj.w32(
                    *off,
                    if rng.u32() % 2 == 0 {
                        blocks_img.at(k * 4)
                    } else {
                        0
                    },
                );
            }
            fx.obj.w32(MODE, rng.u32());
            fx.obj.w16(SCRIPT, c.w2c);
            fx.obj.w16(TBLIDX, c.table_idx);
            let mut entries = Vec::with_capacity(4);
            let mut params = Vec::with_capacity(4);
            for k in 0..4u16 {
                let mut e = Image::random(0xD0, &mut rng);
                let mut p = Image::random(0x3C, &mut rng);
                if k == c.table_idx {
                    e.w32(0x6C, c.mode);
                    e.w32(0x74, c.weight.to_bits());
                    e.w8(0x8C, c.flag);
                    for (n, off) in [0x24usize, 0x28, 0x34, 0x38].iter().enumerate() {
                        p.w32(*off, c.indices[n]);
                    }
                } else {
                    e.w32(0x6C, rng.u32());
                    e.w32(0x74, rng.u32());
                    e.w8(0x8C, rng.u8());
                    for off in [0x24usize, 0x28, 0x34, 0x38] {
                        p.w32(off, rng.u32() % 8);
                    }
                }
                e.w32(0xCC, p.addr());
                entries.push(e);
                params.push(p);
            }
            let entry_addr = entries[c.table_idx as usize].addr();
            let mut array_a = Image::random(0x240, &mut rng);
            let mut array_b = Image::random(0x240, &mut rng);
            for (k, row) in c.bones_a.iter().enumerate() {
                array_a.w32(k * 64 + 0x30, row[0].to_bits());
                array_a.w32(k * 64 + 0x34, row[1].to_bits());
                array_a.w32(k * 64 + 0x38, row[2].to_bits());
            }
            for (k, row) in c.bones_b.iter().enumerate() {
                array_b.w32(k * 64 + 0x30, row[0].to_bits());
                array_b.w32(k * 64 + 0x34, row[1].to_bits());
                array_b.w32(k * 64 + 0x38, row[2].to_bits());
            }
            let mut wrap_a = Image::zeroed(0x18);
            wrap_a.w32(0x14, array_a.addr());
            let mut wrap_b = Image::zeroed(0x18);
            wrap_b.w32(0x14, array_b.addr());
            let mut chain_p = Image::zeroed(8);
            let mut target = Image::random(0x94, &mut rng);
            chain_p.w32(4, target.addr());
            fx.obj
                .w32(CHAIN, if c.chain_linked { chain_p.addr() } else { 0 });
            let mut early_p = Image::random(0xEA0, &mut rng);
            early_p.w32(0xE98, c.early_word_val);
            let mat = random_matrix(&mut rng);
            if c.attached {
                write_matrix(&mut fx.mat, &mat);
            }
            let cfg = UpdateScalars {
                entry_seq_byte: c.entry_byte,
                sel: c.sel,
                edx_alt: c.edx_alt,
                eax: c.eax,
                ecx: c.ecx,
                win_lo: c.win_lo,
                win_hi: c.win_hi,
                win_scale: c.win_scale,
                setup_flag: c.setup_flag,
                store_val: c.store_val,
                k0: c.k0,
                k1: c.k1,
                wgt_scale: c.wgt_scale,
                e18_scale: c.e18_scale,
                kn: c.kn,
                wx: c.wx,
                j_a4: c.j_a4,
                j_a5: c.j_a5,
                j_a6: c.j_a6,
                j_a8: c.j_a8,
                k_a4: c.k_a4,
                k_a5: c.k_a5,
                k_a6: c.k_a6,
                k_a8: c.k_a8,
                qk: c.qk,
            };
            let mut acc = Accumulator {
                flag: c.acc_flag,
                vals: c.acc_vals,
            };
            let this = fx.this();
            let member_b = fx.obj.r32(MEMBER_B);
            let v294 = fx.obj.r32(BLOCK2);
            let v310 = fx.obj.r32(MEMBER_A);
            let mat_addr = if c.attached { fx.mat.addr() } else { 0 };
            let served = if c.distinct_sets {
                vec![wrap_a.addr(), wrap_b.addr(), wrap_a.addr(), wrap_b.addr()]
            } else {
                vec![wrap_a.addr(); 4]
            };
            let _guard = rt::script_lock();
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![rng.u32()]),
                (2, StubKind::Thiscall1, vec![rng.u32()]),
                (6, StubKind::Cdecl3, vec![c.early_call_ans]),
                (7, StubKind::Cdecl0, vec![early_p.addr()]),
                (8, StubKind::Cdecl1, vec![c.early_tail_ans]),
                (9, StubKind::Thiscall9, vec![rng.u32()]),
                (10, StubKind::Thiscall11, vec![rng.u32()]),
                (11, StubKind::Thiscall1, served.clone()),
                (12, StubKind::SubmitSnap, vec![c.submit_ans]),
                (13, StubKind::SubmitSnap, vec![c.submit_ans]),
            ]);
            unsafe {
                rt::UG[0] = ((c.entry_byte as u32) << 16) | (rng.u32() & 0xFF00_FFFF);
                for (k, e) in entries.iter().enumerate() {
                    rt::UTABLE[k] = e.addr();
                }
                rt::UG[2] = c.sel as u32;
                rt::UG[3] = c.edx_alt as u32;
                rt::UG[4] = c.eax as u32;
                rt::UG[5] = c.ecx as u32;
                rt::UG[6] = c.win_lo.to_bits();
                rt::UG[7] = c.win_hi.to_bits();
                rt::UG[8] = c.win_scale.to_bits();
                rt::UG[9] = c.setup_flag;
                rt::UG[10] = c.store_val.to_bits();
                rt::UG[11] = c.acc_flag;
                rt::UG[12] = c.acc_vals[0].to_bits();
                rt::UG[13] = c.acc_vals[1].to_bits();
                rt::UG[14] = c.acc_vals[2].to_bits();
                rt::UG[15] = c.k1.to_bits();
                rt::UG[16] = c.k0.to_bits();
                rt::UG[17] = c.wgt_scale.to_bits();
                rt::UG[18] = c.e18_scale.to_bits();
                rt::UG[19] = c.kn.to_bits();
                rt::UG[20] = c.wx.to_bits();
                rt::UG[21] = c.j_a4.to_bits();
                rt::UG[22] = c.j_a5.to_bits();
                rt::UG[23] = c.j_a6.to_bits();
                rt::UG[24] = c.j_a8;
                rt::UG[25] = c.k_a4.to_bits();
                rt::UG[26] = c.k_a5.to_bits();
                rt::UG[27] = c.k_a6.to_bits();
                rt::UG[28] = c.k_a8;
                rt::UG[29] = c.qk.to_bits();
            }
            rt::set_virtual(&[
                ("guard_a", vec![c.guard_a]),
                ("guard_b", vec![c.guard_b]),
                ("probe", vec![rng.u32()]),
            ]);
            let before = fx.obj.buf.to_vec();
            let target_before = target.buf.to_vec();
            let ug_before: [u32; 30] = unsafe { rt::UG };
            let r = unsafe { fn_00c661c0::rw_00c661c0(this) };
            let numbered = rt::take_numbered();
            let virt = rt::take_virtual();
            let snaps = rt::take_snaps();
            let ug_after: [u32; 30] = unsafe { rt::UG };
            drop(_guard);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            for (n, cell) in ug_before.iter().enumerate() {
                if main_path && c.mode == 1 && reaches_math && (11..=14).contains(&n) {
                    continue; // accumulator handled below
                }
                assert_eq!(ug_after[n], *cell, "shared word {n} case {i}");
            }
            // Expected calls, walked from the case inputs.
            let mut exp_n: Vec<(u32, Vec<u32>)> = vec![(1, vec![this])];
            if c.entry_byte == 0 {
                exp_n.push((2, vec![this]));
            }
            let mut exp_v: Vec<(String, Vec<u32>)> = vec![("guard_a".to_string(), vec![this])];
            let expects_return: u32;
            if c_path {
                exp_v.push(("probe".to_string(), vec![member_b]));
                if c.flag != 0 {
                    exp_n.push((7, vec![]));
                    exp_n.push((8, vec![c.early_word_val]));
                    expects_return = c.early_tail_ans;
                } else {
                    exp_n.push((6, vec![(c.table_idx as i16 as i32) as u32, v294, v310]));
                    expects_return = c.early_call_ans;
                }
            } else {
                exp_v.push(("guard_b".to_string(), vec![this]));
                if gb_zero {
                    expects_return = c.guard_b;
                } else {
                    if setup_runs {
                        exp_n.push((
                            9,
                            vec![this, 0x32, 0x33, 1, 1, ONE, entry_addr, mat_addr, 0],
                        ));
                        exp_n.push((
                            10,
                            vec![
                                this,
                                0x34,
                                0xFFFF_FFFF,
                                0x35,
                                1,
                                0,
                                1,
                                ONE,
                                entry_addr,
                                mat_addr,
                                0,
                            ],
                        ));
                    }
                    if c.mode != 1 {
                        if k_pass {
                            for _ in 0..4 {
                                exp_n.push((11, vec![this]));
                            }
                            expects_return = c.submit_ans;
                        } else {
                            expects_return = c.indices[3];
                        }
                    } else if j_pass {
                        for _ in 0..2 {
                            exp_n.push((11, vec![this]));
                        }
                        expects_return = c.submit_ans;
                    } else {
                        expects_return = c.indices[2];
                    }
                }
            }
            // The submit call carries scratch block pointers: shape-checked
            // separately, values compared against the lift below.
            let submit_args: Option<Vec<u32>>;
            if reaches_math {
                let (main, last) = numbered.split_at(numbered.len() - 1);
                assert_eq!(main, exp_n.as_slice(), "numbered calls case {i}");
                assert_eq!(last.len(), 1);
                let (sid, sargs) = &last[0];
                assert_eq!(*sid, if c.mode != 1 { 13 } else { 12 }, "submit id");
                assert_eq!(sargs[0], mat_addr, "submit record");
                assert_ne!(sargs[1], 0, "block pointer");
                assert_ne!(sargs[2], 0, "block pointer");
                assert_ne!(sargs[3], 0, "block pointer");
                assert_eq!(sargs[7], ONE, "constant one word");
                assert_eq!(snaps.len(), 1, "one snapshot");
                assert_eq!(snaps[0].len(), 12, "twelve block words");
                submit_args = Some(sargs.clone());
            } else {
                assert_eq!(numbered, exp_n, "numbered calls case {i}");
                assert!(snaps.is_empty(), "no submit, no snapshot");
                submit_args = None;
            }
            assert_eq!(virt, exp_v, "virtual calls case {i}");
            assert_eq!(r, expects_return, "return case {i}");
            // The lift on the same inputs.
            let o = fx.lift();
            let entry = UpdateEntry {
                id: cookie(entry_addr).expect("entry addr nonzero"),
                flag: c.flag,
                mode: c.mode,
                weight: c.weight,
                index_words: c.indices,
            };
            let pattern: Vec<u32> = if c.mode != 1 {
                c.indices.to_vec()
            } else {
                vec![c.indices[0], c.indices[2]]
            };
            let nbone = if reaches_math { pattern.len() } else { 0 };
            let mut brows = Vec::with_capacity(nbone);
            for (k, idx_k) in pattern.iter().take(nbone).enumerate() {
                // Math paths always carry in-array indices (guards and case
                // construction); anything else is a case bug and panics.
                assert!((*idx_k as usize) < 8, "bone index in array");
                let arr = if served[k] == wrap_a.addr() {
                    &c.bones_a
                } else {
                    &c.bones_b
                };
                brows.push(BoneRow {
                    set: cookie(served[k]).expect("wrapper addr nonzero"),
                    xyz: arr[*idx_k as usize],
                });
            }
            let mut fake = Fake::new();
            fake.answer_entries(vec![entry, entry]);
            fake.answer_brows(brows.clone());
            fake.answer("early.block", vec![early_p.addr()]);
            fake.answer("early.word", vec![c.early_word_val]);
            fake.answer("early.tail", vec![c.early_tail_ans]);
            fake.answer("early.call", vec![c.early_call_ans]);
            fake.answer("guard.a", vec![c.guard_a]);
            fake.answer("guard.b", vec![c.guard_b]);
            fake.answer("submit.j", vec![c.submit_ans]);
            fake.answer("submit.k", vec![c.submit_ans]);
            let lr = o.update(&mut fake, &cfg, &mut acc);
            assert_eq!(lr, r, "lift return case {i}");
            let mut exp_l: Vec<(String, Vec<u32>)> = vec![("entry.1".to_string(), vec![])];
            if c.entry_byte == 0 {
                exp_l.push(("entry.2".to_string(), vec![]));
            }
            exp_l.push(("guard.a".to_string(), vec![]));
            let table_log = (
                "table".to_string(),
                vec![
                    u32::from(c.table_idx),
                    entry_addr,
                    u32::from(c.flag),
                    c.mode,
                    c.weight.to_bits(),
                    c.indices[0],
                    c.indices[1],
                    c.indices[2],
                    c.indices[3],
                ],
            );
            if c_path {
                exp_l.push(("probe".to_string(), vec![member_b]));
                exp_l.push(table_log);
                if c.flag != 0 {
                    exp_l.push(("early.block".to_string(), vec![]));
                    exp_l.push(("early.word".to_string(), vec![early_p.addr()]));
                    exp_l.push(("early.tail".to_string(), vec![c.early_word_val]));
                } else {
                    exp_l.push((
                        "early.call".to_string(),
                        vec![(c.table_idx as i16 as i32) as u32, v294, v310],
                    ));
                }
            } else {
                exp_l.push(("guard.b".to_string(), vec![]));
                if !gb_zero {
                    exp_l.push(table_log);
                    if setup_runs {
                        exp_l.push((
                            "setup.9".to_string(),
                            vec![entry_addr, mat_addr, 0x32, 0x33, 1, 1, ONE, 0],
                        ));
                        exp_l.push((
                            "setup.10".to_string(),
                            vec![
                                entry_addr,
                                mat_addr,
                                0x34,
                                0xFFFF_FFFF,
                                0x35,
                                1,
                                0,
                                1,
                                ONE,
                                0,
                            ],
                        ));
                        exp_l.push((
                            "store".to_string(),
                            vec![chain_p.addr(), c.store_val.to_bits()],
                        ));
                    }
                    for (k, b) in brows.iter().enumerate() {
                        exp_l.push((
                            "bone".to_string(),
                            vec![
                                pattern[k],
                                served[k],
                                b.xyz[0].to_bits(),
                                b.xyz[1].to_bits(),
                                b.xyz[2].to_bits(),
                            ],
                        ));
                    }
                    if reaches_math {
                        let sargs = submit_args.clone().expect("submit recorded");
                        let mut s = vec![mat_addr];
                        s.extend_from_slice(&snaps[0]);
                        s.extend_from_slice(&[sargs[4], sargs[5], sargs[6], sargs[8]]);
                        exp_l.push((
                            (if c.mode != 1 { "submit.k" } else { "submit.j" }).to_string(),
                            s,
                        ));
                    }
                }
            }
            assert_eq!(fake.log, exp_l, "lift calls case {i}");
            // Effects: the store word and the accumulator.
            if setup_runs {
                assert_eq!(target.r32(0x90), c.store_val.to_bits(), "store word");
                assert_eq!(fake.store_cell, c.store_val.to_bits(), "fake store cell");
            } else {
                assert_eq!(target.buf.to_vec(), target_before, "no setup, no store");
            }
            assert_eq!(
                [ug_after[11], ug_after[12], ug_after[13], ug_after[14]],
                [
                    acc.flag,
                    acc.vals[0].to_bits(),
                    acc.vals[1].to_bits(),
                    acc.vals[2].to_bits()
                ],
                "accumulator case {i}"
            );
            // The wrong version swaps the blend dispatch. It runs
            // everywhere the lift runs except the narrowed domain it would
            // newly enter: detached with its swapped guards passing, where
            // the original dereferences blindly and the lift panics.
            let k_idx_pass = (c.indices[0] as i32) >= 0
                && (c.indices[1] as i32) >= 0
                && (c.indices[2] as i32) >= 0
                && (c.indices[3] as i32) >= 0;
            let j_idx_pass = (c.indices[0] as i32) >= 0 && (c.indices[2] as i32) >= 0;
            let w_reaches =
                main_path && ((c.mode == 1 && k_idx_pass) || (c.mode != 1 && j_idx_pass));
            let run_wrong = c.attached || !w_reaches;
            let wpattern: Vec<u32> = if c.mode != 1 {
                vec![c.indices[0], c.indices[2]]
            } else {
                c.indices.to_vec()
            };
            let mut wbrows = Vec::with_capacity(wpattern.len());
            for (k, idx_k) in wpattern.iter().enumerate() {
                let xyz = if (*idx_k as usize) < 8 {
                    let arr = if served[k] == wrap_a.addr() {
                        &c.bones_a
                    } else {
                        &c.bones_b
                    };
                    arr[*idx_k as usize]
                } else {
                    [0.0, 0.0, 0.0]
                };
                wbrows.push(BoneRow {
                    set: cookie(served[k]).expect("wrapper addr nonzero"),
                    xyz,
                });
            }
            let mut wf = Fake::new();
            wf.answer_entries(vec![entry, entry]);
            wf.answer_brows(wbrows);
            wf.answer("early.block", vec![early_p.addr()]);
            wf.answer("early.word", vec![c.early_word_val]);
            wf.answer("early.tail", vec![c.early_tail_ans]);
            wf.answer("early.call", vec![c.early_call_ans]);
            wf.answer("guard.a", vec![c.guard_a]);
            wf.answer("guard.b", vec![c.guard_b]);
            wf.answer("submit.j", vec![c.submit_ans]);
            wf.answer("submit.k", vec![c.submit_ans]);
            let mut wacc = Accumulator {
                flag: c.acc_flag,
                vals: c.acc_vals,
            };
            if run_wrong {
                let wr = wrong::update_jk_swap(&o, &mut wf, &cfg, &mut wacc);
                if wr != r || wf.log != fake.log || wacc != acc {
                    caught += 1;
                }
            }
            if reaches_math && c.mode == 1 {
                full_j += 1;
            }
            if reaches_math && c.mode != 1 {
                full_k += 1;
            }
            let _ = (
                member_b_obj,
                blocks_img,
                entries,
                params,
                array_a,
                array_b,
                wrap_a,
                wrap_b,
                chain_p,
                target,
                early_p,
            );
        }
        assert!(full_j > 0, "no full blend covered");
        assert!(full_k > 0, "no full average covered");
        assert!(caught > 0, "wrong update never caught");
    }
}

#[cfg(not(target_arch = "x86"))]
#[test]
fn no_rewrite_cases_on_host() {}
