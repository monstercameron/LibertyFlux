// original: 0x00D18170 slide_cover_task_dispatch (proposed)

/// Dispatch a cover-slide task step: probe the task, collect wanted cover
/// actions, and run them through the task's action list.
///
/// `this` is the task, `arg0` the ped, `arg1` the action list (a count at
/// +0x4, ids at +0x8, a parallel zeroed array at +0x48, capacity 16); the
/// third stack word is unread. The head probes first: unless the skip flag
/// (+0x60 bit 3) is set it resolves a small integer state through three
/// helpers, optionally queues one action directly, and then either runs an
/// x87-evaluated branch, a solver block with two indirect (vtable slot 3)
/// type queries that can queue two more actions, or nothing. The tail
/// always runs: it lazily initialises a shared handle (a global), reads a
/// mode from the handle's block, and either ensures five wanted ids are in
/// the list (mode 1), checks the ped-to-anchor distance against 225 and
/// queues four actions when beyond it (mode 2), or does neither; then two
/// conditional single ensures, an unconditional probe with another
/// conditional ensure, a final gate that can queue two last actions, and
/// returns void. Ensuring appends the id (plus a zero parallel word) only
/// when absent and the list is not full.
///
/// Original: 0x00D18170 (thiscall, three stack words, void).
lf_checker_rt::export!(thiscall, rw_00D18170(this: u32, arg0: u32, arg1: u32, _arg2: u32) -> u32 {
    unsafe {
        const C_THIRTY: u32 = 0xfe8b48;
        const C_DIST2: u32 = 0xfe8c00;
        const C_LEVEL: u32 = 0xe9d785;
        const GLOBAL_HANDLE: u32 = 0x167e3b4;
        const ID_A: u32 = 0x76e;
        const ID_B: u32 = 0x76f;
        const ID_C: u32 = 0x76d;
        const ID_D: u32 = 0x773;
        const ID_E: u32 = 0x774;
        const ID_F: u32 = 0x776;
        const BLEND_HIGH_BITS: u32 = 0x3f400000;
        const LIST_CAP: i32 = 0x10;

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
        #[inline(always)]
        unsafe fn site(arg1: u32, id: u32, x: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, arg1, id, x);
            }
        }
        #[inline(always)]
        unsafe fn ensure(list: u32, id: u32) {
            unsafe {
                let count = rd32(list + 4) as i32;
                if count > 0 {
                    let mut i = 0i32;
                    while i < count {
                        let at = (list + 8).wrapping_add((i as u32).wrapping_mul(4));
                        if rd32(at) == id {
                            return;
                        }
                        i += 1;
                    }
                }
                if count == LIST_CAP {
                    return;
                }
                wr32((list + 8).wrapping_add((count as u32).wrapping_mul(4)), id);
                let count2 = rd32(list + 4);
                wr32((list + 0x48).wrapping_add(count2.wrapping_mul(4)), 0);
                wr32(list + 4, (count + 1) as u32);
            }
        }
        #[inline(always)]
        unsafe fn vcall(vp: u32) -> u32 {
            unsafe {
                let slot: u32 = rd32(rd32(vp).wrapping_add(0xc));
                let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
                f(vp)
            }
        }
        #[inline(always)]
        unsafe fn tail(this: u32, arg0: u32, arg1: u32) {
            unsafe {
                let gaddr = lf_checker_rt::relocated(GLOBAL_HANDLE);
                let mut g = rd32(gaddr);
                if g == 0 {
                    let r: u32 = lf_checker_rt::callee_cdecl!(10, u32, 0x20020);
                    if r == 0 {
                        g = 0;
                    } else {
                        g = lf_checker_rt::callee_thiscall!(11, u32, r);
                    }
                    wr32(gaddr, g);
                }
                let inner = rd32(arg0 + 0x224);
                let blk: u32 = lf_checker_rt::callee_thiscall!(12, u32, g, rd32(inner + 0xd8));
                let mode = (rd32(blk + 0x8f4) >> 7) & 3;
                if mode == 1 {
                    ensure(arg1, ID_A);
                    ensure(arg1, ID_B);
                    ensure(arg1, ID_D);
                    ensure(arg1, ID_E);
                    ensure(arg1, ID_F);
                } else if mode == 2 {
                    let e20 = rd32(arg0 + 0x20);
                    let dy = sub(rdf(e20 + 0x34), rdf(this + 0x24));
                    let dx = sub(rdf(e20 + 0x30), rdf(this + 0x20));
                    let dz = sub(rdf(e20 + 0x38), rdf(this + 0x28));
                    let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    if d2 > f32::from_bits(rd32(lf_checker_rt::relocated(C_DIST2))) {
                        site(arg1, ID_A, 0);
                        site(arg1, ID_D, 0);
                        site(arg1, ID_E, 0);
                        site(arg1, ID_F, 0);
                    }
                }
                let d68 = rd32(arg0 + 0xd68);
                if d68 != 0 {
                    let mut f20a = [0u32; 1];
                    let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, d68, f20a.as_ptr() as u32, 0);
                    let o3c = rd32(this + 0x3c);
                    let transient = rd32(o3c + 0x20).wrapping_add(0x30);
                    let mut f20b = [0u32; 1];
                    let a14: u32 = lf_checker_rt::callee_cdecl!(14, u32, d68, transient, f20b.as_ptr() as u32);
                    if (a14 & 0xff) != 0 {
                        ensure(arg1, ID_B);
                    }
                }
                let mut f20c = [0u32; 1];
                let a15: u32 =
                    lf_checker_rt::callee_thiscall!(15, u32, this, arg0, f20c.as_ptr() as u32);
                if (a15 & 0xff) != 0 {
                    ensure(arg1, ID_B);
                }
                let a16: u32 = lf_checker_rt::callee_cdecl!(16, u32, arg0);
                if (a16 & 0xff) == 0 {
                    ensure(arg1, ID_B);
                }
                let o3c = rd32(this + 0x3c);
                if rd32(o3c + 0xb30) != 0 && rd8(o3c + 0x26c) & 4 != 0 {
                    let a17: u32 = lf_checker_rt::callee_cdecl!(17, u32, arg0, o3c);
                    if (a17 & 0xff) == 0 {
                        site(arg1, ID_D, 0);
                        site(arg1, ID_F, 0);
                    }
                }
            }
        }

        let inner224 = rd32(arg0 + 0x224);
        let s1: u32 = lf_checker_rt::callee_thiscall!(1, u32, inner224, 1);
        let mut esi = s1;
        if rd8(this + 0x60) & 8 != 0 {
            tail(this, arg0, arg1);
            return 0;
        }
        let t3c = rd32(this + 0x3c);
        let mut f14 = [0u32; 1];
        let ok: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi, t3c, f14.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            tail(this, arg0, arg1);
            return 0;
        }
        if esi != 0 {
            esi = lf_checker_rt::callee_thiscall!(3, u32, esi, t3c);
        } else {
            esi = 1;
        }
        if rd32(arg0 + 0x26c) & 0x400000 != 0 && esi != 2 {
            site(arg1, ID_A, 0);
        }
        let lvl = rd8(rd32(arg0 + 0x224) + 0x2c8);
        let lim = rd8(lf_checker_rt::relocated(C_LEVEL));
        if (lvl as i8) > (lim as i8) {
            if esi != 2 {
                site(arg1, ID_A, 0);
                tail(this, arg0, arg1);
                return 0;
            }
        }
        let e: u32 = lf_checker_rt::callee_cdecl!(5, u32, arg0, t3c);
        if (e & 0xff) != 0 {
            site(arg1, ID_B, 0);
            site(arg1, ID_C, 0);
            tail(this, arg0, arg1);
            return 0;
        }
        if esi != 2 {
            let f14f = f32::from_bits(f14[0]);
            let thirty = f32::from_bits(rd32(lf_checker_rt::relocated(C_THIRTY)));
            if thirty > f14f {
                let q: f32 = lf_checker_rt::callee_thiscall!(6, f32, this, arg0);
                if q > rdf(this + 0x6c) {
                    site(arg1, ID_A, 0);
                }
                tail(this, arg0, arg1);
                return 0;
            } else {
                core::hint::black_box(rd32(arg0 + 0xd68));
            }
        }
        let cobj: u32 = lf_checker_rt::callee_thiscall!(7, u32, inner224);
        if ((rd32(cobj + 0x8f4) >> 13) & 3) >= 3 {
            if esi != 2 {
                let mut f20 = [0u32; 1];
                let mut f18 = [0u32; 1];
                let mut f1c = [0u32; 1];
                let h: u32 = lf_checker_rt::callee_cdecl!(
                    8,
                    u32,
                    f20.as_mut_ptr() as u32,
                    f18.as_mut_ptr() as u32,
                    f1c.as_mut_ptr() as u32,
                    arg0,
                    t3c
                );
                if (h as i32) > 1 {
                    esi = ((h as i32) / 2) as u32;
                    if (f18[0] as i32) < (esi as i32) {
                        let f14f = f32::from_bits(f14[0]);
                        let thirty =
                            f32::from_bits(rd32(lf_checker_rt::relocated(C_THIRTY)));
                        if f14f > thirty {
                            let vp = rd32(this + 8);
                            let mut proceed = true;
                            if vp != 0 {
                                if vcall(vp) == ID_A {
                                    proceed = false;
                                } else if vcall(vp) == ID_B {
                                    proceed = false;
                                }
                            }
                            if proceed {
                                if (f1c[0] as i32) > (esi as i32) {
                                    site(arg1, ID_C, BLEND_HIGH_BITS);
                                    site(arg1, ID_B, BLEND_HIGH_BITS);
                                }
                            }
                        }
                    } else {
                        site(arg1, ID_A, 0);
                    }
                }
            }
        }
        tail(this, arg0, arg1);
        0
    }
});
