// original: 0x00a264b0 ped_task_aim_update (proposed)

/// Advance a ped task's aim solution by one tick.
///
/// `this` is the task object and seven words follow on the stack: `aim`
/// (three floats), `target` (an object), a blend factor float, three flag
/// bytes (`use_target`, `do_blocks`, `alt_rate`) and an unused seventh word.
/// The routine seeds a 64-entry scratch table from three globals, folds the
/// aim direction with the task's stored position, optionally runs two
/// solver passes (each a fill call, a continuation fill call and a combine
/// call over the table, then a minimum reduction), smooths two stored
/// fields, and finishes with a direction blend, a sine shaping call and four
/// shared-global writes. It returns 1 when the second pass found anything.
///
/// The two C runtime helpers (stack probing and the security-cookie check)
/// run natively in the original and are not part of the rewrite. Every
/// floating-point operation is written in the original's operand order with
/// pinned evaluation order. The return value is a clean 0 or 1.
///
/// Original: 0x00a264b0 (thiscall, seven stack arguments, the last unread).
lf_checker_rt::export!(thiscall, rw_00a264b0(
    this: u32, aim: u32, target: u32, blend: u32, use_target: u32,
    do_blocks: u32, alt_rate: u32, _unused: u32,
) -> u32 {
    unsafe {
        const TABLE_N: usize = 64;
        const TABLE_STRIDE: usize = 0x60;
        const MODE_ACTIVE: u32 = 2;
        const G_MODE: u32 = 0x011d6fd4;
        const G_T0: u32 = 0x01b4b320;
        const G_T1: u32 = 0x01b4b324;
        const G_T2: u32 = 0x01b4b328;
        const G_ALT_RATE: u32 = 0x0103c764;
        const G_SECOND: u32 = 0x0103c0d8;
        const G_OBJ: u32 = 0x012b9c78;
        const G_OUT0: u32 = 0x012dd610;
        const G_OUT1: u32 = 0x012dd614;
        const G_OUT2: u32 = 0x012dd618;
        const G_OUT3: u32 = 0x012dd61c;
        const C_ONE: u32 = 0x00fe88e8;
        const C_RATE0: u32 = 0x00fe8864;
        const C_RELAX: u32 = 0x00fe87d0;
        const C_LO: u32 = 0x00e9b9f0;
        const C_HI: u32 = 0x00e9b9f4;
        const C_FRAC: u32 = 0x00e9b9d0;
        const C_BLEND: u32 = 0x00e9a400;
        const C_DEG_BASE: u32 = 0x00e8121c;
        const C_DEG2RAD: u32 = 0x00fe8728;
        const C_HALF: u32 = 0x00fe8830;
        const C_RATE1: u32 = 0x00fe8c1c;
        const C_TAIL: u32 = 0x00fe87e0;
        const C_ABS: u32 = 0x00fe8f80;
        const C_SMOOTH_A: u32 = 0x00fe87e8;
        const C_SMOOTH_B: u32 = 0x00fe879c;
        const FILL_FLAG: u32 = 0x3e4ccccd;
        const KIND_FULL: u32 = 0x40;

        #[inline(always)]
        unsafe fn rd32(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(base: u32, off: u32) -> u8 {
            unsafe { ((base + off) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read() }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            let x = core::hint::black_box(a);
            let y = core::hint::black_box(b);
            core::hint::black_box(x + y)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            let x = core::hint::black_box(a);
            let y = core::hint::black_box(b);
            core::hint::black_box(x - y)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let x = core::hint::black_box(a);
            let y = core::hint::black_box(b);
            core::hint::black_box(x * y)
        }
        #[inline(always)]
        unsafe fn fld(base: u32, off: u32) -> f32 {
            unsafe { f32::from_bits(rd32(base, off)) }
        }
        #[inline(always)]
        unsafe fn fst(base: u32, off: u32, v: f32) {
            unsafe { wr32(base, off, v.to_bits()) }
        }

        // Scratch table: 64 entries of 0x60 bytes plus a couple of spill
        // words the last entry's wide stores touch.
        let mut table = [0u32; TABLE_N * (TABLE_STRIDE / 4) + 8];
        let tbase = table.as_mut_ptr() as u32;
        #[inline(always)]
        unsafe fn tw(base: u32, w: usize, v: u32) {
            unsafe { ((base + (w * 4) as u32) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn tw16(base: u32, w: usize, v: u16) {
            unsafe { ((base + (w * 4) as u32) as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn tw8(base: u32, w: usize, b: usize, v: u8) {
            unsafe { ((base + (w * 4 + b) as u32) as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn tr(base: u32, w: usize) -> u32 {
            unsafe { ((base + (w * 4) as u32) as *const u32).read_unaligned() }
        }

        let mut x0 = gf(C_ONE);
        let mut x3 = gf(C_RATE0);
        let mut edx = target;
        let esi = this;
        let mut x4 = fld(esi, 0x1b0);
        let mut x5 = 0.0f32;
        let mut x7 = fld(esi, 0x1b4);
        let mut x6 = fld(esi, 0x1b8);
        x4 = fmul(x4, f32::from_bits(blend));
        let mut x0stash = x0;
        let mut x3init = x3;
        if (alt_rate & 0xff) != 0 {
            x3 = gf(G_ALT_RATE);
            x3init = x3;
        }
        x0 = gf(G_T2);
        let mut x1 = gf(G_T1);
        let mut x2 = gf(G_T0);
        // Seed the table (indices in words from each entry's base).
        for i in 0..TABLE_N {
            let b = i * (TABLE_STRIDE / 4);
            tw(tbase, b + 4, 0);
            tw(tbase, b + 8, x2.to_bits());
            tw(tbase, b + 9, x1.to_bits());
            tw(tbase, b + 10, x0.to_bits());
            tw(tbase, b + 12, x2.to_bits());
            tw(tbase, b + 13, x1.to_bits());
            tw(tbase, b + 14, x0.to_bits());
            tw(tbase, b + 16, x2.to_bits());
            tw(tbase, b + 17, x1.to_bits());
            tw(tbase, b + 18, x0.to_bits());
            tw(tbase, b + 20, 0);
            tw(tbase, b + 21, 0);
            tw(tbase, b + 23, 0xffff);
            tw8(tbase, b + 24, 0, 0);
            tw8(tbase, b + 24, 2, 0);
            tw8(tbase, b + 24, 3, 0);
            tw(tbase, b + 22, 0);
        }
        let mode_active = g32(G_MODE) == MODE_ACTIVE;
        x5 = fld(esi, 0x144);
        x1 = fld(esi, 0x148);
        x0 = fld(esi, 0x140);
        x2 = fld(aim, 8);
        x7 = fadd(x7, x5);
        x6 = fadd(x6, x1);
        x0 = fadd(x0, x4);
        x4 = fld(aim, 4);
        let mut count: u32 = 0;
        let mut s40 = x7;
        let s20 = x6;
        x6 = fld(aim, 0);
        x6 = fmul(x6, x3);
        x7 = x2;
        x7 = fmul(x7, x3init);
        x6 = fadd(x6, x0);
        x0 = fld(esi, 0x140);
        x7 = fadd(x7, s20);
        x3 = x4;
        x3 = fmul(x3, x3init);
        let mut s30 = x6;
        let mut xtmp = x7.to_bits();
        x3 = fadd(x3, s40);
        x7 = x6;
        x7 = fsub(x7, x0);
        let s50 = x0;
        x0 = gf(C_RELAX);
        x4 = fmul(x4, x0);
        s30 = x7;
        x7 = x3;
        x7 = fsub(x7, x5);
        x3 = fsub(x3, x4);
        x2 = fmul(x2, x0);
        let s54 = x5;
        let s4c = x7;
        x7 = f32::from_bits(xtmp);
        x7 = fsub(x7, x1);
        let _s74 = x3;
        x3 = f32::from_bits(xtmp);
        let s58 = x1;
        xtmp = 0;
        s40 = x7;
        x7 = fld(aim, 0);
        x7 = fmul(x7, x0);
        x0 = x3;
        x0 = fsub(x0, x2);
        x6 = fsub(x6, x7);
        let _s78 = x0;
        let _s70 = x6;
        // Mode-gated clamp check; either exit-early flag combination jumps
        // straight to the tail with the pristine stash.
        let mut early = false;
        if mode_active {
            x0 = gf(C_LO);
            x1 = s20;
            if x3 > x0 || x1 > x0 {
                x0 = gf(C_HI);
                if x0 > x3 || x0 > x1 {
                    early = true;
                }
            }
        }
        let mut edi: u32 = 0;
        // xmm3 on the paths below: the tail reloads or carries it per path.
        let mut tail_x3 = x0stash;
        if !early && (do_blocks & 0xff) != 0 {
            // Target selection for the auxiliary slot.
            if (use_target & 0xff) != 0 {
                if (rd32(target, 0x26c) & 4) != 0 {
                    edx = rd32(target, 0xb30);
                    if edx != 0 {
                        xtmp = rd32(edx, 0x38);
                    }
                }
            } else if edx != 0 {
                xtmp = rd32(edx, 0x38);
            }
            let obj = g32(G_OBJ);
            let _s48 = obj;
            let id1ans = lf_checker_rt::callee_cdecl!(1, u32,);
            let id1slot = id1ans;
            // First solver pass. Frame-pointer arguments are skipped in the
            // contract; the table words they point at are the checked state.
            let mut s50l = s50;
            let ans2 = lf_checker_rt::callee_thiscall!(
                2, u32, obj, &mut s50l as *mut f32 as u32, FILL_FLAG, tbase,
                xtmp, id1ans, 0xffff_ffffu32, 7, KIND_FULL, 0
            );
            count = ans2;
            if (ans2 as i32) < KIND_FULL as i32 {
                let left = KIND_FULL.wrapping_sub(ans2);
                let mut s70l = _s70;
                let cont = tbase + ans2.wrapping_mul(TABLE_STRIDE as u32);
                let add = lf_checker_rt::callee_thiscall!(
                    3, u32, obj, &mut s50l as *mut f32 as u32,
                    &mut s70l as *mut f32 as u32, FILL_FLAG, cont, xtmp,
                    id1slot, 0xffff_ffffu32, 7, 1, left, 0
                );
                count = count.wrapping_add(add);
            }
            lf_checker_rt::callee_cdecl!(4, u32, &mut count as *mut u32 as u32, tbase);
            edi = count;
            if (edi as i32) > 0 {
                for i in 0..edi as usize {
                    let v = f32::from_bits(tr(tbase, i * (TABLE_STRIDE / 4) + 16));
                    if !(v > x0stash) {
                        x0stash = v;
                    }
                }
            }
            x0 = fld(esi, 0x1f8);
            x3 = x0stash;
            if !(x3 > x0) {
                x0 = x3;
            }
            fst(esi, 0x1f8, x0);
            let flag: u8 = rd8(esi, 0x216);
            if (flag & 1) == 0 {
                x0 = fld(esi, 0x268);
                x3 = fsub(x3, x0);
                x3 = fmul(x3, fld(esi, 0x26c));
                x3 = fadd(x3, x0);
            }
            fst(esi, 0x268, x3);
            let do_second = g8(G_SECOND) != 0 && flag == 0;
            if do_second {
                // Smoothstep-style relaxation of the stash toward +0x1e0.
                x2 = fld(esi, 0x1e0);
                x3 = fsub(x3, x2);
                x1 = x3;
                x3 = gf(C_ONE);
                x0 = x1;
                x0 = f32::from_bits(x0.to_bits() & g32(C_ABS));
                x3 = fsub(x3, x0);
                x3 = fmul(x3, gf(C_SMOOTH_A));
                x3 = fadd(x3, gf(C_SMOOTH_B));
                x3 = fmul(x3, x1);
                x3 = fadd(x3, x2);
                x0stash = x3;
            }
            if do_second && edi != 0 {
                // Second solver pass over the same table.
                x0 = s50;
                x3 = s30;
                x4 = fld(aim, 0);
                x5 = fld(aim, 4);
                x6 = fld(aim, 8);
                x1 = s4c;
                x2 = s40;
                edi = obj;
                let mut s60 = x0;
                x0 = s54;
                let _s64 = x0;
                x0 = s58;
                let _s70b = x0;
                x0 = x3init;
                x4 = fmul(x4, x0);
                x5 = fmul(x5, x0);
                x6 = fmul(x6, x0);
                x0 = 0.0;
                x3 = fmul(x3, x0);
                x1 = fmul(x1, x0);
                x3 = fadd(x3, fld(esi, 0x140));
                x2 = fmul(x2, x0);
                x1 = fadd(x1, fld(esi, 0x144));
                x2 = fadd(x2, fld(esi, 0x148));
                x3 = fsub(x3, x4);
                x1 = fsub(x1, x5);
                x2 = fsub(x2, x6);
                let (a6176, a6172, a6168) = (x3, x1, x2);
                let mut a6176l = a6176;
                let ans2b = lf_checker_rt::callee_thiscall!(
                    2, u32, edi, &mut s60 as *mut f32 as u32, FILL_FLAG,
                    tbase, xtmp, id1slot, 0xffff_ffffu32, 7, KIND_FULL, 0
                );
                let _ = (a6172, a6168);
                count = ans2b;
                if (ans2b as i32) < KIND_FULL as i32 {
                    let left = KIND_FULL.wrapping_sub(ans2b);
                    let cont = tbase + ans2b.wrapping_mul(TABLE_STRIDE as u32);
                    let add = lf_checker_rt::callee_thiscall!(
                        3, u32, edi, &mut s60 as *mut f32 as u32,
                        &mut a6176l as *mut f32 as u32, FILL_FLAG, cont,
                        xtmp, id1slot, 0xffff_ffffu32, 7, 1, left, 0
                    );
                    count = count.wrapping_add(add);
                }
                lf_checker_rt::callee_cdecl!(
                    4, u32, &mut count as *mut u32 as u32, tbase
                );
                edi = count;
                if (edi as i32) > 0 {
                    x0 = fld(esi, 0x1e0);
                    x3 = gf(C_HALF);
                    x3 = fsub(x3, x0);
                    x3 = fmul(x3, gf(C_BLEND));
                    x3 = fadd(x3, x0);
                    fst(esi, 0x1e0, x3);
                    tail_x3 = x3;
                } else {
                    x3 = x0stash;
                    fst(esi, 0x1e0, x3);
                    tail_x3 = x3;
                }
            } else {
                fst(esi, 0x1e0, x3);
                tail_x3 = x3;
            }
        }
        // Tail blend. xmm3 arrives per path (pristine stash on early exit).
        x3 = tail_x3;
        x0 = fld(aim, 0);
        x4 = x3init;
        x1 = fld(aim, 4);
        x2 = fld(aim, 8);
        x5 = s4c;
        x6 = s40;
        x0 = fmul(x0, x4);
        x1 = fmul(x1, x4);
        x2 = fmul(x2, x4);
        x4 = s30;
        x4 = fmul(x4, x3);
        x5 = fmul(x5, x3);
        x6 = fmul(x6, x3);
        x3 = fsub(x3, gf(C_SMOOTH_A));
        x4 = fsub(x4, x0);
        x5 = fsub(x5, x1);
        x6 = fsub(x6, x2);
        x0 = 0.0;
        x3 = fmul(x3, gf(C_FRAC));
        x4 = fadd(x4, fld(esi, 0x140));
        x5 = fadd(x5, fld(esi, 0x144));
        let x3_le0 = !(x3 > x0);
        x6 = fadd(x6, fld(esi, 0x148));
        fst(esi, 0x140, x4);
        fst(esi, 0x144, x5);
        fst(esi, 0x148, x6);
        if x3_le0 {
            x3 = x0;
        } else {
            x0 = gf(C_ONE);
            if !(x0 > x3) {
                x3 = x0;
            }
        }
        if (use_target & 0xff) == 0 {
            x3 = fmul(x3, gf(C_RATE1));
            x0 = gf(C_DEG_BASE);
            x0 = fsub(x0, x3);
            x0 = fmul(x0, gf(C_DEG2RAD));
            let sin_bits = lf_checker_rt::callee_cdecl!(6, u32, x0.to_bits());
            x3 = f32::from_bits(sin_bits);
            x3 = fadd(x3, gf(C_ONE));
            x3 = fmul(x3, gf(C_HALF));
        }
        if (alt_rate & 0xff) == 0 {
            x3 = fmul(x3, gf(C_TAIL));
            x3 = fadd(x3, fld(esi, 0x148));
            fst(esi, 0x148, x3);
        }
        x0 = fld(esi, 0x140);
        x1 = fld(esi, 0x144);
        x2 = fld(esi, 0x148);
        wr32(lf_checker_rt::relocated(G_OUT0), 0, x0.to_bits());
        wr32(lf_checker_rt::relocated(G_OUT1), 0, x1.to_bits());
        wr32(lf_checker_rt::relocated(G_OUT2), 0, x2.to_bits());
        x0 = fld(esi, 0x14c);
        wr32(lf_checker_rt::relocated(G_OUT3), 0, x0.to_bits());
        if (edi as i32) > 0 {
            1
        } else {
            0
        }
    }
});
