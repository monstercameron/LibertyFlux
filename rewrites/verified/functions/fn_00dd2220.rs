// original: 0x00dd2220 UIMontageContainer.PositionBar

const COOKIE_CALLEE: u32 = 2;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

/// Stage-B helpers shared by the prefix rewrite and its mutant.
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}
/// Load a virtual slot from an object's table.
#[inline(always)]
unsafe fn vslot(obj: u32, slot: u32) -> u32 {
    unsafe { rd32(rd32(obj) + slot) }
}
/// Indirect thiscall with no stack arguments.
#[inline(always)]
unsafe fn vind0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj)
    }
}
/// Indirect thiscall with one stack argument.
#[inline(always)]
unsafe fn vind1(obj: u32, slot: u32, a0: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj, a0)
    }
}
/// Indirect thiscall with two stack arguments.
#[inline(always)]
unsafe fn vind2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj, a0, a1)
    }
}
/// Indirect thiscall with four stack arguments.
#[inline(always)]
unsafe fn vind4(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj, a0, a1, a2, a3)
    }
}
/// Indirect thiscall with six stack arguments (a 24-byte struct by value).
#[inline(always)]
unsafe fn vind6(obj: u32, slot: u32, w: &[u32; 6]) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj, w[0], w[1], w[2], w[3], w[4], w[5])
    }
}
/// Indirect thiscall with seven stack arguments (a key plus 24 bytes).
#[inline(always)]
unsafe fn vind7(obj: u32, slot: u32, k: u32, w: &[u32; 6]) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj, k, w[0], w[1], w[2], w[3], w[4], w[5])
    }
}
/// Indirect thiscall with eight stack arguments (two keys plus 24 bytes).
#[inline(always)]
unsafe fn vind8(obj: u32, slot: u32, k0: u32, k1: u32, w: &[u32; 6]) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj, k0, k1, w[0], w[1], w[2], w[3], w[4], w[5])
    }
}
/// Indirect thiscall returning a float in ST0.
#[inline(always)]
unsafe fn vind_f32(obj: u32, slot: u32) -> f32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(vslot(obj, slot) as usize);
        f(obj)
    }
}

//
const SLOT_GATE: u32 = 0x140;
const SLOT_PARENT: u32 = 0x48;
const SLOT_FINAL: u32 = 0x13c;
const SLOT_APPLY7: u32 = 0x114;
const SLOT_APPLY8: u32 = 0x110;
const SLOT_APPLY6: u32 = 0x4c;
const SLOT_SET2: u32 = 0x10c;
const SLOT_SET1: u32 = 0x170;
const SLOT_FLAG28: u32 = 0x28;
const SLOT_SHOW: u32 = 0x120;
const SLOT_MEASURE: u32 = 0x94;
const SLOT_SETUP4: u32 = 0x1cc;
const SLOT_TINT: u32 = 0x208;
const SLOT_TEXT: u32 = 0x1e0;
const SLOT_A: u32 = 0x1fc;
const SLOT_B: u32 = 0x1f8;
const SLOT_C: u32 = 0x1d4;
const SLOT_SAMPLE: u32 = 0x20c;
const SLOT_224: u32 = 0x224;
const SLOT_17C: u32 = 0x17c;
const SLOT_188: u32 = 0x188;
const SLOT_44: u32 = 0x44;
const SLOT_FETCH: u32 = 0x1e0;
const GLOBAL_HUB: u32 = 0x1981a4c;
const F_TEN: u32 = 0x41200000;
const F_FIFTEEN: u32 = 0x41700000;
const F_FIVE: u32 = 0x40A00000;
const F_NEG_FIVE: u32 = 0xC0A00000;
const F_NEG_FIVE_HI: u32 = 0xC0A00000;
const F_SEVEN: u32 = 0x40E00000;
const F_NEG_NINETEEN: u32 = 0xC1980000;
const F_EIGHTEEN: u32 = 0x41900000;
const F_NEG_SEVEN: u32 = 0xC0E00000;
const F_NEG_THREE: u32 = 0xC0400000;
const TINT_PANEL: u32 = 0xff797979;

/// Run one helper/fetch + keyed-apply + release triple on `panel`.
/// The fetch callee answers a pointer to 24 bytes; those six words travel
/// to the apply slot by value after the key.
#[inline(always)]
unsafe fn triple(
    panel: u32, ctx: u32, f0: u32, f1: u32, key: u32, rel: u32,
) {
    unsafe {
        let h = lf_checker_rt::callee_thiscall!(7, u32, ctx, f0, f1);
        let w = [
            rd32(h),
            rd32(h + 4),
            rd32(h + 8),
            rd32(h + 12),
            rd32(h + 16),
            rd32(h + 20),
        ];
        vind7(panel, SLOT_APPLY7, key, &w);
        lf_checker_rt::callee_thiscall!(9, u32, rel);
    }
}

/// Build a five-panel UI container, populate it from a list, and report.
///
/// Calling convention: thiscall, `this` in ecx, one stack word `list`
/// (the callee pops 4 bytes), 32-bit status in eax: the gate answer on the early-exit path,
/// otherwise the final virtual call's answer.
///
/// Behaviour. The entry gate calls virtual slot +0x140 on `this`; when the
/// low byte of its answer is nonzero (an 8-bit test, no signedness) the
/// full 32-bit answer is returned at once. Otherwise five panels are built:
/// three small ones (panel A kept in a local; B at `this`+0x1e4 and C at
/// +0x1f4), a medium one (D at +0x1e0 with an inner object at +0x21c and a
/// float source at +0x1e0), and a large one (G at +0x1e8). Each panel is
/// allocated, constructed with a class tag and its parent value, and
/// initialised; then fetch/keyed-apply/release triples run on it, each
/// fetch answering 24 bytes that travel to the apply call by value after
/// an integer key (panel C additionally takes a two-key apply). Panel D is
/// combined through panel B twice. Panel G takes a colour word, two string
/// tags, two flags and a count, and panel C's measure slot receives a
/// float sampled from panel D's source through the x87 ST0 channel
/// (bit-exact; only moved, never computed here).
///
/// When `list` is null the scalar state (+0x200/+0x204/+0x20c/+0x21a) is
/// stored and the tail runs. Otherwise `list` is kept at +0x1f8 and its
/// count is read: a zero count takes the short path (flag bytes set, one
/// toggle call, join the tail). A nonzero count enters the region, where
/// two loops run. Loop 1 builds one item per index (allocate, construct
/// through panel D, nine-word build call, float/int/flag wiring from the
/// item's fields at +0x44/+0x24/+0x20, first/last selectors, a second ST0
/// float stored to the hub object, and a register call), and loop 2 walks
/// the same indices comparing neighbouring fields (+0x24 against 2 or 0,
/// +0x20 against 1 or 0) to pick navigation calls. Every loop bound is a
/// SIGNED 32-bit compare (entry jle, back-edge jl, loop-2 jge/jle), and
/// the region entry itself is an exact-zero test (test/jne), so negative
/// counts enter the region but skip both loops. The tail pushes the +0x218
/// flag byte into panel G, resolves a key through two hub calls (either
/// storing a derived pointer at +0x1bc or zero), and returns virtual slot
/// +0x13c on `this`.
///
/// Edge cases. A failed allocation (any of the six) or a null list item
/// dereferences null and faults; those paths are not covered by the proof
/// (see `narrowed`). Counts above 4 are not trialled: the count is the
/// trip count, so large values would not terminate. All float traffic is
/// bit-exact moves of scripted values.
///
/// Original: 0x00dd2220 (thiscall, this + one stack word).
lf_checker_rt::export!(thiscall, rw_prefix(this: u32, list: u32) -> u32 {
    unsafe {
        let mut scratch = [0u32; 8];
        let ctx = scratch.as_mut_ptr() as u32;
        let gate = vind0(this, SLOT_GATE);
        if gate & 0xFF != 0 {
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return gate;
        }
        // Panel A (kept in a local, never stored to `this`).
        let raw_a = lf_checker_rt::callee_cdecl!(3, u32, 0x25c);
        if raw_a == 0 {
            unreachable!("stage C: allocation failure");
        }
        let pa = vind0(this, SLOT_PARENT);
        let panel_a = lf_checker_rt::callee_thiscall!(5, u32, raw_a, lf_checker_rt::relocated(0xefaad8), pa);
        lf_checker_rt::callee_thiscall!(6, u32, panel_a, 0, 0, ctx, 0xFFFFFFFF);
        triple(panel_a, ctx, 0, 0, 6, ctx);
        triple(panel_a, ctx, 0, 0, 0x18, ctx);
        // Panel B at [this+0x1e4].
        let raw_b = lf_checker_rt::callee_cdecl!(3, u32, 0x25c);
        if raw_b == 0 {
            unreachable!("stage C: allocation failure");
        }
        let pb = vind0(this, SLOT_PARENT);
        let panel_b = lf_checker_rt::callee_thiscall!(5, u32, raw_b, lf_checker_rt::relocated(0xefabf0), pb);
        wr32(this + 0x1e4, panel_b);
        lf_checker_rt::callee_thiscall!(6, u32, panel_b, 0, 0, ctx, 0xFFFFFFFF);
        triple(panel_b, ctx, 0, F_FIFTEEN, 4, ctx);
        triple(panel_b, ctx, F_FIVE, 0, 2, ctx);
        triple(panel_b, ctx, F_NEG_FIVE, 0, 8, ctx);
        triple(panel_b, ctx, 0, F_NEG_FIVE_HI, 0x10, ctx);
        // Panel C at [this+0x1f4].
        let raw_c = lf_checker_rt::callee_cdecl!(3, u32, 0x25c);
        if raw_c == 0 {
            unreachable!("stage C: allocation failure");
        }
        let pc = vind0(this, SLOT_PARENT);
        let panel_c = lf_checker_rt::callee_thiscall!(5, u32, raw_c, lf_checker_rt::relocated(0xefac00), pc);
        wr32(this + 0x1f4, panel_c);
        let fmt_c = lf_checker_rt::callee_cdecl!(10, u32, ctx, 0x3e);
        lf_checker_rt::callee_thiscall!(34, u32, panel_c, 0, 0, fmt_c, 0xFFFFFFFF);
        triple(panel_c, ctx, 0, F_SEVEN, 4, ctx);
        triple(panel_c, ctx, 0, F_NEG_NINETEEN, 0x10, ctx);
        // Two-key apply on panel C.
        let h = lf_checker_rt::callee_thiscall!(7, u32, ctx, 0, 0);
        let w = [
            rd32(h),
            rd32(h + 4),
            rd32(h + 8),
            rd32(h + 12),
            rd32(h + 16),
            rd32(h + 20),
        ];
        vind8(panel_c, SLOT_APPLY8, 2, 8, &w);
        lf_checker_rt::callee_thiscall!(9, u32, ctx);
        wr32(panel_c + 0x1d8, 0);
        vind1(panel_c, SLOT_SHOW, 0);
        // Panel D at [this+0x1e0].
        let raw_d = lf_checker_rt::callee_cdecl!(3, u32, 0x224);
        if raw_d == 0 {
            unreachable!("stage C: allocation failure");
        }
        let pd1 = vind0(this, SLOT_PARENT);
        let pd2 = vind0(this, SLOT_PARENT);
        let mk = lf_checker_rt::callee_cdecl!(11, u32, lf_checker_rt::relocated(0xefac20), pd2);
        let panel_d = lf_checker_rt::callee_thiscall!(12, u32, raw_d, mk, pd1);
        wr32(this + 0x1e0, panel_d);
        lf_checker_rt::callee_thiscall!(13, u32, panel_d, 2, F_TEN);
        wr32(panel_d + 0x20c, F_TEN);
        wr32(panel_d + 0x1f8, 2);
        let inner = rd32(panel_d + 0x21c);
        wr32(inner + 0x1f4, 2);
        // Six-word apply through panel B, result feeds panel D.
        let h = lf_checker_rt::callee_thiscall!(7, u32, ctx, 0, 0);
        let w = [
            rd32(h),
            rd32(h + 4),
            rd32(h + 8),
            rd32(h + 12),
            rd32(h + 16),
            rd32(h + 20),
        ];
        let comb = vind6(panel_b, SLOT_APPLY6, &w);
        vind2(panel_d, SLOT_SET2, 4, comb);
        lf_checker_rt::callee_thiscall!(9, u32, ctx);
        triple(panel_d, ctx, F_EIGHTEEN, 0, 2, ctx);
        triple(panel_d, ctx, F_NEG_SEVEN, 0, 8, ctx);
        let h = lf_checker_rt::callee_thiscall!(7, u32, ctx, 0, F_NEG_THREE);
        let w = [
            rd32(h),
            rd32(h + 4),
            rd32(h + 8),
            rd32(h + 12),
            rd32(h + 16),
            rd32(h + 20),
        ];
        let comb = vind6(panel_b, SLOT_APPLY6, &w);
        vind2(panel_d, SLOT_SET2, 0x10, comb);
        lf_checker_rt::callee_thiscall!(9, u32, ctx);
        let tail_parent = vind0(this, SLOT_APPLY6);
        vind1(panel_d, SLOT_SET1, tail_parent);
        vind1(panel_d, SLOT_FLAG28, 0);
        vind1(inner, SLOT_SHOW, 1);
        wr8(panel_d + 0x218, 0);
        let hub_a = lf_checker_rt::callee_thiscall!(29, u32, lf_checker_rt::relocated(GLOBAL_HUB), lf_checker_rt::relocated(0xefac38));
        wr32(this + 0x1ec, hub_a);
        let hub_b = lf_checker_rt::callee_thiscall!(29, u32, lf_checker_rt::relocated(GLOBAL_HUB), lf_checker_rt::relocated(0xefac44));
        wr32(this + 0x1f0, hub_b);
        // Panel G at [this+0x1e8].
        let raw_g = lf_checker_rt::callee_cdecl!(3, u32, 0x610);
        if raw_g == 0 {
            unreachable!("stage C: allocation failure");
        }
        let pg = vind0(this, SLOT_PARENT);
        let panel_g =
            lf_checker_rt::callee_thiscall!(30, u32, raw_g, lf_checker_rt::relocated(0xefac54), pg);
        wr32(this + 0x1e8, panel_g);
        let fmt_g = lf_checker_rt::callee_cdecl!(10, u32, ctx, 7);
        vind4(panel_g, SLOT_SETUP4, F_EIGHTEEN, fmt_g, 0, 2);
        let mut tint = TINT_PANEL;
        vind1(panel_g, SLOT_TINT, (&mut tint as *mut u32) as u32);
        vind2(panel_g, SLOT_TEXT, lf_checker_rt::relocated(0xefac70), 0);
        vind1(panel_g, SLOT_A, 0);
        vind1(panel_g, SLOT_B, 0);
        vind1(panel_g, SLOT_C, 0x12);
        // Sampled float from panel D's source into panel C's measure.
        let source = rd32(panel_d + 0x1e0);
        let sample = vind_f32(source, SLOT_SAMPLE);
        vind1(panel_c, SLOT_MEASURE, sample.to_bits());
        wr32(this + 0x200, 0);
        wr32(this + 0x204, 0xFFFFFFFF);
        wr32(this + 0x20c, 0);
        wr8(this + 0x21a, 0);

        if list != 0 {
            // List region: count-gated loops over the list at `list`.
            wr32(this + 0x1f8, list);
            let c0 = lf_checker_rt::callee_thiscall!(37, u32, list);
            let hub = rd32(this + 0x1ec);
            let hubb = rd32(this + 0x1f0);
            if c0 == 0 {
                wr8(this + 0x218, 1);
                wr8(hub + 0x1f4, 1);
                lf_checker_rt::callee_thiscall!(55, u32, hubb, 1);
            } else {
                wr8(this + 0x218, 0);
                wr8(hub + 0x1f4, 0);
                lf_checker_rt::callee_thiscall!(55, u32, hubb, 0);
                lf_checker_rt::callee_cdecl!(
                    56,
                    u32,
                    lf_checker_rt::relocated(0xefac80),
                    0
                );
                let inner = rd32(panel_d + 0x21c);
                vind1(inner, SLOT_SHOW, 1);
                // Loop 1 over k in 0..count (all bounds SIGNED: jle/jl).
                let c1 = lf_checker_rt::callee_thiscall!(37, u32, list);
                if (c1 as i32) > 0 {
                    let mut k = 0u32;
                    loop {
                        lf_checker_rt::callee_cdecl!(39, u32, ctx, 0, 0x100);
                        let item_a =
                            lf_checker_rt::callee_thiscall!(38, u32, list, k);
                        let mut p = item_a;
                        while rd8(p) != 0 {
                            p = p.wrapping_add(1);
                        }
                        let len = p.wrapping_sub(item_a);
                        let item_b =
                            lf_checker_rt::callee_thiscall!(38, u32, list, k);
                        lf_checker_rt::callee_cdecl!(
                            40,
                            u32,
                            ctx,
                            item_b,
                            len.wrapping_sub(3)
                        );
                        lf_checker_rt::callee_cdecl!(
                            41,
                            u32,
                            ctx,
                            lf_checker_rt::relocated(0xefac90),
                            3
                        );
                        let raw = lf_checker_rt::callee_cdecl!(3, u32, 0x31c);
                        if raw == 0 {
                            unreachable!("stage D: item allocation failure");
                        }
                        let r1 = vind0(panel_d, SLOT_PARENT);
                        let _r2 = vind0(panel_d, SLOT_PARENT);
                        let mk = lf_checker_rt::callee_cdecl!(
                            11,
                            u32,
                            lf_checker_rt::relocated(0xefac94),
                            _r2
                        );
                        let item = lf_checker_rt::callee_thiscall!(
                            42, u32, ctx, mk, r1
                        );
                        lf_checker_rt::callee_thiscall!(
                            43, u32, item, ctx,
                            lf_checker_rt::relocated(0xefb008), ctx,
                            0x43780000, 0x431b0000, 2, 0, 0, 2
                        );
                        let item_c =
                            lf_checker_rt::callee_thiscall!(38, u32, list, k);
                        vind1(item, SLOT_224, rd32(item_c + 0x44));
                        let item_d =
                            lf_checker_rt::callee_thiscall!(38, u32, list, k);
                        let f24 = rd32(item_d + 0x24);
                        let item_e =
                            lf_checker_rt::callee_thiscall!(38, u32, list, k);
                        let f20 = rd32(item_e + 0x20);
                        lf_checker_rt::callee_thiscall!(45, u32, item, f20, f24);
                        let t1 = vind0(this, SLOT_APPLY6);
                        vind1(item, SLOT_SET1, t1);
                        let t2 = vind0(this, SLOT_APPLY6);
                        vind1(item, SLOT_17C, t2);
                        let t3 = vind0(this, SLOT_APPLY6);
                        vind1(item, SLOT_188, t3);
                        vind1(item, SLOT_FLAG28, 1);
                        vind1(item, SLOT_44, 1);
                        if k == 0 {
                            lf_checker_rt::callee_thiscall!(49, u32, item, 1);
                        }
                        let cend =
                            lf_checker_rt::callee_thiscall!(37, u32, list);
                        if k == cend.wrapping_sub(1) {
                            lf_checker_rt::callee_thiscall!(50, u32, item, 1);
                        }
                        let f: f32 =
                            lf_checker_rt::callee_thiscall!(51, f32, list);
                        wr32(hub + 0x1f0, f.to_bits());
                        lf_checker_rt::callee_thiscall!(52, u32, hub, item, k, 1);
                        k = k.wrapping_add(1);
                        let cb =
                            lf_checker_rt::callee_thiscall!(37, u32, list);
                        if !((k as i32) < (cb as i32)) {
                            break;
                        }
                    }
                }
                // Loop 2 over j in 0..count (bounds SIGNED: jle/jge/jl).
                let c2e = lf_checker_rt::callee_thiscall!(37, u32, list);
                if (c2e as i32) > 0 {
                    let mut j = 0u32;
                    loop {
                        let it2 = vind1(panel_d, SLOT_FETCH, j);
                        let cc =
                            lf_checker_rt::callee_thiscall!(37, u32, list);
                        if !((j as i32) >= ((cc.wrapping_sub(1)) as i32)) {
                            let a = lf_checker_rt::callee_thiscall!(
                                38, u32, list, j
                            );
                            let mut done = false;
                            if rd32(a + 0x24) == 2 {
                                let b = lf_checker_rt::callee_thiscall!(
                                    38,
                                    u32,
                                    list,
                                    j.wrapping_add(1)
                                );
                                if rd32(b + 0x20) == 1 {
                                    lf_checker_rt::callee_thiscall!(
                                        53, u32, it2, 3
                                    );
                                    done = true;
                                }
                            }
                            if !done {
                                let a2 = lf_checker_rt::callee_thiscall!(
                                    38, u32, list, j
                                );
                                if rd32(a2 + 0x24) == 0 {
                                    let b2 = lf_checker_rt::callee_thiscall!(
                                        38,
                                        u32,
                                        list,
                                        j.wrapping_add(1)
                                    );
                                    if rd32(b2 + 0x20) == 1 {
                                        lf_checker_rt::callee_thiscall!(
                                            53, u32, it2, 1
                                        );
                                    }
                                }
                            }
                        }
                        if (j as i32) > 0 {
                            let m = lf_checker_rt::callee_thiscall!(
                                38,
                                u32,
                                list,
                                j.wrapping_sub(1)
                            );
                            let mut done = false;
                            if rd32(m + 0x24) == 2 {
                                let n = lf_checker_rt::callee_thiscall!(
                                    38, u32, list, j
                                );
                                if rd32(n + 0x20) == 1 {
                                    lf_checker_rt::callee_thiscall!(
                                        54, u32, it2, 3
                                    );
                                    done = true;
                                }
                            }
                            if !done {
                                let m2 = lf_checker_rt::callee_thiscall!(
                                    38,
                                    u32,
                                    list,
                                    j.wrapping_sub(1)
                                );
                                if rd32(m2 + 0x24) == 2 {
                                    let n2 = lf_checker_rt::callee_thiscall!(
                                        38, u32, list, j
                                    );
                                    if rd32(n2 + 0x20) == 0 {
                                        lf_checker_rt::callee_thiscall!(
                                            54, u32, it2, 2
                                        );
                                    }
                                }
                            }
                        }
                        j = j.wrapping_add(1);
                        let cb2 =
                            lf_checker_rt::callee_thiscall!(37, u32, list);
                        if !((j as i32) < (cb2 as i32)) {
                            break;
                        }
                    }
                }
            }
            wr8(this + 0x21a, 1);
            lf_checker_rt::callee_thiscall!(57, u32, panel_d);
            lf_checker_rt::callee_thiscall!(58, u32, this);
        }
        // Tail.
        let flag = rd8(this + 0x218);
        vind1(panel_g, SLOT_SHOW, (flag != 0) as u32);
        wr32(this + 0x20c, 0);
        wr32(this + 0x210, 0);
        let key = lf_checker_rt::callee_cdecl!(31, u32, lf_checker_rt::relocated(0xefaca4));
        let check = lf_checker_rt::callee_thiscall!(32, u32, lf_checker_rt::relocated(GLOBAL_HUB), key);
        if check & 0xFF != 0 {
            let extra = lf_checker_rt::callee_thiscall!(29, u32, lf_checker_rt::relocated(GLOBAL_HUB), lf_checker_rt::relocated(0xefacb4));
            wr32(this + 0x1bc, extra.wrapping_add(0x1f8));
        } else {
            wr32(this + 0x1bc, 0);
        }
        let out = vind1(this, SLOT_FINAL, 1);
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        out
    }
});
