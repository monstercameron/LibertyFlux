// original: 0x00AE8800 collect_timing_visible (proposed)

/// Collect the visible timed entries over three passes.
///
/// Arguments (cdecl): the first word is unread; `start` and `end` delimit
/// an array of 8-byte entries (object pointer, float tag); `ctx` is the
/// timing context. Returns nothing meaningful (leftover EAX; `ret: none`).
///
/// Setup resolves two providers: callee 1 yields an optional info block
/// (its dword at `+0x1304` equal to 4 arms loop one), and callees 2-4
/// resolve an optional reference object. When both reference answers agree
/// and are nonzero and the float at `+0x60` exceeds 48, callee 5 runs with
/// the reference floats and two integer-to-float-to-integer round trips of
/// context words, arming the per-entry second shot.
///
/// Loop one counts entries whose class row (`CLASS_TABLE[kind at +0x2E]`)
/// lacks bit 3 at `+0x40`, whose virtual slot `+0xC` answer is not 3 or
/// fails callee 7 `(object, entry, 1, 1)`, whose parent chain (two depths
/// of `+0x4C`) clears the mode-gated state checks, and whose `+0x24` bit 7
/// is set. It runs when the global loop flag is set, else only when loop
/// one is armed.
///
/// Loop two scores each live entry. One with `+0x8` bit 29 set is dropped
/// through virtual slot `+0x44` and nulled. Otherwise bit 31 of `+0x8` is
/// cleared and the entry is polled like in loop one; callee 7 accepting
/// skips it. A set class bit 3 with a nonzero `+0x34` skips it, else
/// virtual slot `+0x40` runs first. Then `uncovered` (`~+0xC & +0x8`) is
/// formed, `+0x5C` bit 14 cleared, and while the global level mask touches
/// it callee 10 runs (a zero answer zeroes the level byte `+0x63`); the
/// surviving entries always continue (the flag selecting the other branch
/// is provably 0: one store, no other writer, stub writes nothing).
/// With a nonzero parent and a zero `+0x61` byte, the low 24 bits join the
/// parent's `+0x58`. The distance from the context origin (`+0x910`,
/// `+0x914`, `+0x918`) to the entry point (`+0x20` plus `0x30`, else
/// `+0x10`) is measured, and callee 11 (virtual slot `+0x5C`) yields a
/// radius. `+0x24` bit 24 selects 0, else the armed flag and bits 7 and 6
/// with the loop-one count (over 5 selects 0) select the outcome or a
/// two-stage angular test through callee 12 (vector-register argument and
/// result): the distance is normalised by `1/(d + 1e-5)`, the first answer
/// (degrees) must exceed 5, and the second answer minus the first is
/// compared against `10 * 0.0055556 * pi`, selecting 1 on strictly greater.
/// When the second shot is armed, callee 13 runs with the out-vector slot,
/// the radius and 1; a nonzero answer skips the entry. A selected entry
/// whose parent chain clears the mode checks and (with the global extra
/// flag set) callee 14 `(kind, global, 0)` bumps the reject counter; an
/// unselected one is collected (counter below the global limit, fewer than
/// 200 collected, loop one counted nothing). The bottom clears the context
/// bit (from `+0x900`) from the mask words and nulls the entry when `+0x8`
/// bit 30 is set.
///
/// When the reject counter passes the limit the global overflow byte is
/// set; otherwise every collected entry is re-checked through the same
/// mode checks and callee 14. The float operation order is the original's,
/// pinned through `black_box` helpers. The original's stack-cookie check
/// calls are reproduced as preserve-mode stub calls with a dummy value:
/// the cookie itself lives in the original's frame and is unobservable.
/// Original: 0x00AE8800 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00AE8800(_a0: u32, start: u32, end: u32, ctx: u32) -> u32 {
    unsafe {
        const CLASS_TABLE: u32 = 0x01295CD8;
        const LOOP_FLAG: u32 = 0x015AE64F;
        const FACTORY: u32 = 0x0103E498;
        const REF_GATE: f32 = 48.0;
        const KIND: u32 = 0x2E;
        const OPTS: u32 = 0x24;
        const STATE: u32 = 0x28;
        const SEEN: u32 = 0x0C;
        const BITS: u32 = 0x08;
        const PARENT: u32 = 0x4C;
        const COUNT2: u32 = 0x34;
        const MASK_A: u32 = 0x54;
        const MASK_B: u32 = 0x58;
        const FLAGS: u32 = 0x5C;
        const LEVEL: u32 = 0x63;
        const PATH61: u32 = 0x61;
        const PT_LINK: u32 = 0x20;
        const PT_INLINE: u32 = 0x10;
        const CLASS_FLAGS: u32 = 0x40;
        const LEVEL_MASK: u32 = 0x0159AF28;
        const MODE_WORD: u32 = 0x016DD67C;
        const EXTRA_FLAG: u32 = 0x01032784;
        const EXTRA_ARG: u32 = 0x012B4138;
        const LIMIT: u32 = 0x015B0E70;
        const COLLECTED: u32 = 0x01593BB0;
        const OVERFLOW: u32 = 0x012FB3B3;
        const LOW24: u32 = 0x00FF_FFFF;
        const MAX_COLLECT: u32 = 200;
        const CTX_BITSEL: u32 = 0x900;
        const CTX_X: u32 = 0x910;
        const CTX_Y: u32 = 0x914;
        const CTX_Z: u32 = 0x918;
        const CTX_DX: u32 = 0x920;
        const CTX_DY: u32 = 0x924;
        const CTX_DZ: u32 = 0x928;
        const DEG_LIMIT: f32 = 5.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let g32 = lf_checker_rt::global::<u32>;
        let g8 = lf_checker_rt::global::<u8>;
        let table = lf_checker_rt::relocated(CLASS_TABLE);
        // The stack-cookie check: preserve-mode stub, dummy register value.
        let cookie = || unsafe {
            lf_checker_rt::callee_thiscall!(15, u32, 0u32);
        };

        // Setup: loop-one arming.
        let info: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0u32);
        let mut armed1 = start & 0xFF;
        if info != 0 {
            if rd32(info + 0x1304) == 4 {
                armed1 = 1;
            }
        } else {
            armed1 = 0;
        }
        // Setup: reference object.
        let f: u32 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            lf_checker_rt::relocated(FACTORY),
            2u32,
            0u32
        );
        let r1: u32 = lf_checker_rt::callee_thiscall!(3, u32, f);
        let r2: u32 = lf_checker_rt::callee_thiscall!(4, u32, g32(0x0103E49C).read_unaligned());
        let mut armed2 = false;
        if r1 == r2 && r1 != 0 {
            let w = f32::from_bits(rd32(r1 + 0x60));
            if w > REF_GATE {
                // Integer round trips through float multiply and truncate.
                // Contract values stay small so truncation matches exactly.
                let t1 = mul((rd32(ctx + 0x754) as i32) as f32, f32::from_bits(rd32(ctx + 0x72C)));
                let f1 = (core::hint::black_box(t1) as i32) as f32;
                let t2 = mul((rd32(ctx + 0x750) as i32) as f32, f32::from_bits(rd32(ctx + 0x728)));
                let f2 = (core::hint::black_box(t2) as i32) as f32;
                let mut buf = [0u32; 52];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    5,
                    u32,
                    buf.as_mut_ptr() as u32,
                    r1.wrapping_add(0x10),
                    rd32(r1 + 0x64),
                    rd32(r1 + 0x68),
                    0x4234_0000u32,
                    f2.to_bits(),
                    f1.to_bits(),
                    1u32
                );
                armed2 = true;
            }
        }
        let class_of = |obj: u32| unsafe {
            let idx = ((obj + KIND) as *const i16).read_unaligned() as i32;
            (table as *const u32).offset(idx as isize).read_unaligned()
        };
        let poll = |class: u32| unsafe {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(class) + 0x0C) as usize);
            f(class)
        };
        // Loop-one mode gate shared by all three chain checks. True means
        // the entry is skipped (not counted / not bumped).
        let chain_skip = |obj: u32| { unsafe {
            if g32(MODE_WORD).read_unaligned() == 0 {
                return false;
            }
            let p = rd32(obj + PARENT);
            let gp = if p != 0 { rd32(p + PARENT) } else { 0 };
            if p != 0 && (rd32(p + STATE) >> 0x19) & 1 == 0 && rd8(p + OPTS) & 0x80 != 0 {
                return true;
            }
            if gp != 0 && (rd32(gp + STATE) >> 0x19) & 1 == 0 {
                return true;
            }
            false
        } };
        // Two-stage angular test; `incoming` is the pre-test selector
        // (always 0 at both call sites).
        let angular = |d: f32, dx: f32, dy: f32, dz: f32, radius: f32, incoming: u32| {
            unsafe {
                let eps = f32::from_bits(g32(0x00FE8670).read_unaligned());
                let one = f32::from_bits(g32(0x00FE88E8).read_unaligned());
                let k = div(one, add(d, eps));
                let sx = mul(dx, k);
                let sy = mul(dy, k);
                let sz = mul(dz, k);
                let rr = mul(radius, radius);
                let dd = mul(d, d);
                let m = div(d, add(rr, dd).sqrt());
                let a1 = f32::from_bits(lf_checker_rt::callee_cdecl!(12, u32, m.to_bits()));
                let c180 = f32::from_bits(g32(0x00E81218).read_unaligned());
                let cinv = f32::from_bits(g32(0x00FE87F0).read_unaligned());
                let mut deg = mul(a1, c180);
                deg = mul(deg, cinv);
                let lim5 = f32::from_bits(g32(0x0103F6F4).read_unaligned());
                if !(deg > lim5) {
                    return incoming;
                }
                let w = add(
                    add(
                        mul(sy, f32::from_bits(rd32(ctx + CTX_DY))),
                        mul(sx, f32::from_bits(rd32(ctx + CTX_DX))),
                    ),
                    mul(sz, f32::from_bits(rd32(ctx + CTX_DZ))),
                );
                let a2 = f32::from_bits(lf_checker_rt::callee_cdecl!(12, u32, w.to_bits()));
                let c10 = f32::from_bits(g32(0x0103F6EC).read_unaligned());
                let cs = f32::from_bits(g32(0x00FE86F0).read_unaligned());
                let cpi = f32::from_bits(g32(0x00FE8AA0).read_unaligned());
                let d2 = core::hint::black_box(a2) - core::hint::black_box(a1);
                let mut lim = mul(c10, cs);
                lim = mul(lim, cpi);
                if lim > d2 {
                    1
                } else {
                    incoming
                }
            }
        };
        g32(COLLECTED).write_unaligned(0);
        let mut rejects = 0u32;
        let mut counted = 0u32;
        let mut any_counted = false;
        let run_loop1 =
            g8(LOOP_FLAG).read() != 0 || armed1 != 0;
        if start == end {
            cookie();
            return 0;
        }
        if run_loop1 {
            let mut cur = start;
            while cur != end {
                let obj = rd32(cur);
                if obj != 0 {
                    let class = class_of(obj);
                    if rd8(class + CLASS_FLAGS) & 8 == 0
                        && (poll(class) & 0xFF != 3
                            || lf_checker_rt::callee_cdecl!(7, u32, obj, cur, 1u32, 1u32) & 0xFF == 0)
                        && !chain_skip(obj)
                        && rd8(obj + OPTS) & 0x80 != 0
                    {
                        counted += 1;
                        any_counted = true;
                    }
                }
                cur = cur.wrapping_add(8);
            }
        }
        let bitsel = rd32(ctx + CTX_BITSEL);
        let mut slots = [0u32; MAX_COLLECT as usize];
        let mut nslots = 0u32;
        let mut cur = start;
        while cur != end {
            let obj = rd32(cur);
            if obj != 0 {
                if rd32(obj + BITS) & 0x2000_0000 != 0 {
                    let drop: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(obj) + 0x44) as usize);
                    drop(obj);
                    wr32(cur, 0);
                } else {
                    wr32(obj + BITS, rd32(obj + BITS) & 0x7FFF_FFFF);
                    let class = class_of(obj);
                    let mut skip_rest = false;
                    if poll(class) & 0xFF == 3
                        && lf_checker_rt::callee_cdecl!(7, u32, obj, cur, 1u32, 1u32) & 0xFF != 0
                    {
                        skip_rest = true;
                    }
                    if !skip_rest {
                        if rd8(class + CLASS_FLAGS) & 8 != 0 {
                            if rd32(obj + COUNT2) == 0 {
                                let rel: extern "thiscall" fn(u32) -> u32 =
                                    core::mem::transmute(rd32(rd32(obj) + 0x40) as usize);
                                rel(obj);
                            }
                        } else {
                            let uncovered = !rd32(obj + SEEN) & rd32(obj + BITS);
                            wr16(obj + FLAGS, rd16(obj + FLAGS) & 0xBFFF);
                            if g32(LEVEL_MASK).read_unaligned() & uncovered != 0 {
                                let a: u32 = lf_checker_rt::callee_cdecl!(10, u32);
                                if a & 0xFF == 0 {
                                    wr8(obj + LEVEL, 0);
                                }
                            }
                            // [E+0x12] is always 0 here: the only store
                            // writes 0 and no stub writes the slot, so the
                            // flag-1 and flag-nonzero exits below are dead
                            // and scoring always continues.
                            let parent = rd32(obj + PARENT);
                            if parent != 0 && rd8(obj + PATH61) == 0 {
                                wr32(
                                    parent + MASK_B,
                                    rd32(parent + MASK_B) | (uncovered & LOW24),
                                );
                            }
                            let pp = rd32(obj + PT_LINK);
                            let (px, py, pz) = if pp != 0 {
                                (
                                    f32::from_bits(rd32(pp + 0x30)),
                                    f32::from_bits(rd32(pp + 0x34)),
                                    f32::from_bits(rd32(pp + 0x38)),
                                )
                            } else {
                                (
                                    f32::from_bits(rd32(obj + PT_INLINE)),
                                    f32::from_bits(rd32(obj + PT_INLINE + 4)),
                                    f32::from_bits(rd32(obj + PT_INLINE + 8)),
                                )
                            };
                            let dx = sub(px, f32::from_bits(rd32(ctx + CTX_X)));
                            let dy = sub(py, f32::from_bits(rd32(ctx + CTX_Y)));
                            let dz = sub(pz, f32::from_bits(rd32(ctx + CTX_Z)));
                            let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                            let d = core::hint::black_box(d2).sqrt();
                            let range_fn: extern "thiscall" fn(u32, u32) -> f32 =
                                core::mem::transmute(rd32(rd32(obj) + 0x5C) as usize);
                            let mut ov = [0u32; 3];
                            let radius = range_fn(obj, ov.as_mut_ptr() as u32);
                            let opts = rd32(obj + OPTS);
                            let sel: u32;
                            if opts & 0x0100_0000 != 0 {
                                sel = 0;
                            } else if armed1 == 0 {
                                sel = angular(d, dx, dy, dz, radius, 0);
                            } else if opts & 0x80 != 0 {
                                sel = 1;
                            } else if opts & 0x40 == 0 {
                                sel = 0;
                            } else if counted > 5 {
                                sel = 0;
                            } else {
                                sel = angular(d, dx, dy, dz, radius, 0);
                            }
                            let mut skip_entry = false;
                            if armed2 {
                                let mut buf = [0u32; 52];
                                let rr: u32 = lf_checker_rt::callee_thiscall!(
                                    13,
                                    u32,
                                    buf.as_mut_ptr() as u32,
                                    ov.as_mut_ptr() as u32,
                                    radius.to_bits(),
                                    1u32
                                );
                                if rr & 0xFF != 0 {
                                    skip_entry = true;
                                }
                            }
                            if !skip_entry {
                                if sel != 0 {
                                    if !chain_skip(obj) {
                                        let mut bump = true;
                                        if g8(EXTRA_FLAG).read() != 0 {
                                            let kind = ((obj + KIND) as *const i16)
                                                .read_unaligned() as i32
                                                as u32;
                                            let ok: u32 = lf_checker_rt::callee_cdecl!(
                                                14,
                                                u32,
                                                kind,
                                                g32(EXTRA_ARG).read_unaligned(),
                                                0u32
                                            );
                                            if ok & 0xFF == 0 {
                                                bump = false;
                                            }
                                        }
                                        if bump {
                                            rejects += 1;
                                        }
                                    }
                                } else if rejects <= g32(LIMIT).read_unaligned()
                                    && g32(COLLECTED).read_unaligned() < MAX_COLLECT
                                    && !any_counted
                                {
                                    let n = g32(COLLECTED).read_unaligned();
                                    g32(COLLECTED).write_unaligned(n + 1);
                                    slots[n as usize] = obj;
                                    nslots += 1;
                                }
                            }
                        }
                    }
                    if rd32(obj + BITS) & 0x4000_0000 != 0 {
                        let keep = !(1u32.wrapping_shl(bitsel & 31)) | 0xFF00_0000;
                        wr32(obj + MASK_A, rd32(obj + MASK_A) & keep);
                        wr32(obj + MASK_B, rd32(obj + MASK_B) & keep);
                        wr32(obj + BITS, rd32(obj + BITS) & 0xBFFF_FFFF);
                        wr32(cur, 0);
                    }
                }
            }
            cur = cur.wrapping_add(8);
        }
        if rejects > g32(LIMIT).read_unaligned() {
            g8(OVERFLOW).write(1);
            cookie();
            return 0;
        }
        let total = g32(COLLECTED).read_unaligned();
        if (total as i32) > 0 {
            let mut i = 0u32;
            while (i as i32) < total as i32 {
                let obj = slots[i as usize];
                if !chain_skip(obj) && g8(EXTRA_FLAG).read() != 0 {
                    let kind =
                        ((obj + KIND) as *const i16).read_unaligned() as i32 as u32;
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        14,
                        u32,
                        kind,
                        g32(EXTRA_ARG).read_unaligned(),
                        0u32
                    );
                }
                i += 1;
            }
        }
        cookie();
        0
    }
});
