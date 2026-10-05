// original: 0x009C1B50 task_drive_follow (proposed)

use lf_checker_rt::{callee_cdecl, callee_thiscall, global, relocated};

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}


const F5_TASK_OBJ: u32 = 0x10;
const F5_TASK_KEY: u32 = 0x14;
const F5_TASK_MODE: u32 = 0x18;
const F5_TASK_OTHER: u32 = 0x1c;
const F5_TASK_DEADLINE: u32 = 0x28;
const F5_TASK_A: u32 = 0x80;
const F5_TASK_C: u32 = 0x84;
const F5_TASK_CA: u32 = 0xca;
const F5_TASK_CD: u32 = 0xcd;
const F5_A_INFO: u32 = 0xa4;
const F5_C_INFO: u32 = 0xa4;
const F5_TIMER: u32 = 0x170;
const F5_GUARD: u32 = 0x16c;
const F5_LINK: u32 = 0x20;
const F5_PARAM: u32 = 0x180;
const F5_TARGET: u32 = 0x190;
const F5_STR: u32 = 0xe94cdc;
/// Bit the sample block ORs into its frame word (byte 1, value 2), later
/// covered by the submit snapshot.
const F5_EPATH_MARK: u32 = 0x200;
const F5_G_ACTIVE: u32 = 0x11f7060;
const F5_G_A: u32 = 0x12088b4;
const F5_G_B: u32 = 0x0f1c040;
const F5_G_MODE: u32 = 0x1037720;
const F5_G_CLOCK: u32 = 0x11735b4;
const F5_G_TMULT: u32 = 0x115d968;
const F5_G_TBASE: u32 = 0x115d988;
const F5_G_VEC: u32 = 0x103ac14;
const F5_C_DEAD: u32 = 0xfe8df8;
const F5_C_BLEND: u32 = 0xfe8a24;
const F5_C_ABS: u32 = 0xfe8f80;
const F5_C_CMP0: u32 = 0xfe8800;
const F5_C_CMP1: u32 = 0xfe8684;
const F5_C_ONE: u32 = 0xfe88e8;
const F5_C_HALF: u32 = 0xfe8830;

/// Follow-task update of one task record.
///
/// `task` points to the record: position floats at `+0/+4/+8`, linked
/// object at `+0x10`, key at `+0x14`, mode at `+0x18`, other object at
/// `+0x1c`, deadline at `+0x28`, two sub-objects at `+0x80`/`+0x84`, flag
/// bytes at `+0xca`/`+0xcd`.
///
/// Behaviour: after the three global guards, poll sub-object `+0x80`
/// through callee 1 (one out word) and, on the polled value, run an
/// announce sequence through callees 2-6 (the compared-zero scratch read
/// is zero under the contract's zero fill); then handle sub-object `+0x84`
/// (a short release pair, or a float-returning sample through callee 7
/// plus build calls). Blend a table lookup by two bytes of sub-object
/// `+0x80` into callee 25. When the linked object's timer is live and its
/// guard differs, score the base vectors, compare against a lookup hit and
/// either blend towards the target (callees 11-14, as in `0x9c1940`) or
/// fall through to a distance-scaled vector submit (callees 15-20) and a
/// second move submit (callees 21-22, callee 13, callee 14). A zero answer
/// from either submit skips its virtual apply.
///
/// Original: 0x009C1B50 (cdecl, one stack word). The contract does not
/// compare the return value: on the early paths it is an entry register,
/// not behaviour (recorded narrowing).
lf_checker_rt::export!(cdecl, rw_009C1B50(task: u32) -> u32 {
    unsafe { f5_run(task, true) }
});

/// Shared body; `do_vtable` selects the virtual apply calls.
unsafe fn f5_run(task: u32, do_vtable: bool) -> u32 {
    unsafe {
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
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        if global::<u32>(F5_G_ACTIVE).read_unaligned() == 1 {
            return 0;
        }
        let ga = global::<u32>(F5_G_A).read_unaligned();
        if ga != global::<u32>(F5_G_B).read_unaligned() {
            return ga;
        }
        if global::<u32>(F5_G_MODE).read_unaligned() == 0x12 {
            return ga;
        }
        let clock = global::<u32>(F5_G_CLOCK).read_unaligned();
        let ca = rd8(task + F5_TASK_CA);
        let a = rd32(task + F5_TASK_A);
        let key = rd32(task + F5_TASK_KEY);
        let obj = rd32(task + F5_TASK_OBJ);
        // Scratch read of an unstored frame word; zero under zero fill.
        const FILL0: u32 = 0;
        if ca != 0 {
            if a != 0 {
                let mut slot = [0u32; 1];
                let _: u32 =
                    callee_thiscall!(1, u32, a, (&mut slot[0] as *mut u32) as u32);
                let al = if slot[0] == 0 {
                    f5_announce(task, a, ca);
                    u32::from(FILL0 == 0)
                } else {
                    0
                };
                f5_report(task, al);
            }
        } else if a != 0 {
            let mut slot = [0u32; 1];
            let _: u32 =
                callee_thiscall!(1, u32, a, (&mut slot[0] as *mut u32) as u32);
            // Both halves re-read the slot and set the flag from it; the
            // compared value here is 1, so the flag is (slot == 1).
            let al = u32::from(slot[0] == 1);
            if slot[0] == 1 {
                f5_announce(task, a, ca);
            }
            f5_report(task, al);
        }
        let c = rd32(task + F5_TASK_C);
        let mut fstp_bits = 0u32;
        let mut e_ran = false;
        if ca != 0 {
            if c != 0 {
                let _: u32 = callee_cdecl!(2, u32, rd32(c.wrapping_add(F5_C_INFO)));
                let _: u32 = callee_thiscall!(4, u32, c, 0);
            }
        } else if c != 0 {
            let f: f32 = callee_thiscall!(7, f32, task.wrapping_add(0x30), 0);
            fstp_bits = f.to_bits();
            let flt = if rd32(task + F5_TASK_MODE) == 3
                && rd32(task + F5_TASK_DEADLINE) > clock
            {
                f32::from_bits(global::<u32>(F5_C_DEAD).read_unaligned())
            } else {
                f
            };
            let _: u32 = callee_thiscall!(8, u32, c, flt.to_bits());
            let z2 = [0u32; 2];
            let _: u32 =
                callee_thiscall!(23, u32, (&z2[0] as *const u32) as u32, 0xab);
            // The original ORs 2 into a frame byte no snapshot covers;
            // unobservable, not mirrored.
            let z2b = [0u32; 2];
            let _: u32 = callee_cdecl!(
                24, u32, rd32(c.wrapping_add(F5_C_INFO)),
                (&z2b[0] as *const u32) as u32
            );
            e_ran = true;
        }
        if a != 0 {
            let mut x0 = 0.0f32;
            if rd32(task + F5_TASK_MODE) == 3 && rd32(task + F5_TASK_DEADLINE) > clock {
                x0 = f32::from_bits(global::<u32>(F5_C_DEAD).read_unaligned());
            }
            let dl = rd8(a.wrapping_add(4)) as u32;
            let edx = if dl == 0xff {
                0
            } else {
                let cl2 = rd8(a.wrapping_add(0x40)) as u32;
                let tmult = global::<u32>(F5_G_TMULT).read_unaligned();
                let tbase = global::<u32>(F5_G_TBASE).read_unaligned();
                dl.wrapping_mul(tmult).wrapping_add(rd32(
                    tbase.wrapping_add(cl2.wrapping_mul(0x6f40)).wrapping_add(0x6f14),
                ))
            };
            let _: u32 = callee_thiscall!(25, u32, edx, x0.to_bits());
        }
        let timer = rdf(obj.wrapping_add(F5_TIMER));
        let guard = rd32(obj.wrapping_add(F5_GUARD));
        let live = timer > 0.0 && (guard == 0 || guard != rd32(task + F5_TASK_OTHER));
        if live {
            let ob = obj;
            // The key resolves before the vector reads (a faulting read
            // must still log this call first, like the original).
            let hit: u32 = callee_cdecl!(9, u32, key);
            let ln = rd32(ob.wrapping_add(F5_LINK));
            let c10 = rdf(ln.wrapping_add(0x10));
            let c14 = rdf(ln.wrapping_add(0x14));
            let d180 = rdf(ob.wrapping_add(0x180));
            let d184 = rdf(ob.wrapping_add(0x184));
            let mut x1 = add(mul(c14, d184), mul(c10, d180));
            let c18 = rdf(ln.wrapping_add(0x18));
            let d188 = rdf(ob.wrapping_add(0x188));
            x1 = add(x1, mul(c18, d188));
            let mask = global::<u32>(F5_C_ABS).read_unaligned();
            x1 = f32::from_bits(x1.to_bits() & mask);
            let e2 = rd32(ob.wrapping_add(F5_GUARD));
            let x0 = if e2 != 0 && rd32(e2.wrapping_add(0x28)) & 0x3c0 == 0x80 {
                rdf(hit.wrapping_add(0xd4))
            } else {
                rdf(hit.wrapping_add(0xd0))
            };
            let mut go_g = x1 > x0;
            if !go_g {
                let a10: u32 = callee_cdecl!(10, u32,);
                if a10 & 0xff == 0 {
                    return f5_tail(task, fstp_bits, e_ran, do_vtable);
                }
                go_g = true;
            }
            let _ = go_g;
            // Blend towards the target (same shape as 0x9c1940).
            let k = f32::from_bits(global::<u32>(F5_C_BLEND).read_unaligned());
            let mut buf = [0u32; 8];
            for i in 0..3u32 {
                let cc = rdf(task.wrapping_add(i * 4));
                let tt = rdf(ob.wrapping_add(F5_TARGET).wrapping_add(i * 4));
                buf[i as usize] = add(cc, mul(sub(tt, cc), k)).to_bits();
            }
            let anchor = (&mut buf[4] as *mut u32) as u32;
            let _: u32 = callee_thiscall!(11, u32, anchor);
            let pp = (&mut buf[0] as *mut u32) as u32;
            let r: u32 = callee_cdecl!(
                12, u32, task, pp, rd32(task + F5_TASK_OTHER), anchor,
                0x86, 0xffff_ffff, 4
            );
            if r != 0 && do_vtable {
                let zero3 = [0u32; 3];
                f5_vtable(obj, (&zero3[0] as *const u32) as u32);
            }
            return callee_cdecl!(
                14, u32, task, 1, ob.wrapping_add(F5_PARAM),
                rd32(ob.wrapping_add(F5_GUARD))
            );
        }
        f5_tail(task, fstp_bits, e_ran, do_vtable)
    }
}

/// Announce sequence shared by the two flag halves (callees 2-5).
unsafe fn f5_announce(task: u32, a: u32, ca: u8) {
    unsafe {
        let _: u32 = callee_cdecl!(2, u32, rd32(a.wrapping_add(F5_A_INFO)));
        let cd = rd8(task + F5_TASK_CD) as u32;
        let _: u32 = callee_cdecl!(3, u32, cd);
        let _: u32 = callee_thiscall!(4, u32, a, 0);
        let _: u32 = callee_cdecl!(
            5, u32, rd32(task + F5_TASK_KEY), rd32(task + F5_TASK_OBJ),
            task.wrapping_add(F5_TASK_A), ca as u32
        );
    }
}

/// Report call shared by the two flag halves (callee 6).
unsafe fn f5_report(task: u32, al: u32) {
    unsafe {
        let _: u32 = callee_cdecl!(
            6, u32, rd32(task + F5_TASK_OBJ), rd32(task + F5_TASK_KEY),
            rd8(task + F5_TASK_CD) as u32, rd8(task + F5_TASK_CA) as u32, al
        );
    }
}

/// Virtual apply through slot 8 (one callee id, two sites). The original
/// calls through the object's function table; the checker answers that
/// slot with the callee 13 stub on the original side, so the rewrite
/// issues the same call through the runtime on this side.
unsafe fn f5_vtable(obj: u32, ptr: u32) -> u32 {
    unsafe { callee_thiscall!(13, u32, obj, ptr, 0, 0) }
}

/// Tail of `f5_run`: the distance-scaled vector submit and the second move
/// submit. `fstp_bits`/`e_ran` mirror the frame slot the sample block may
/// have stored.
unsafe fn f5_tail(task: u32, fstp_bits: u32, e_ran: bool, do_vtable: bool) -> u32 {
    unsafe {
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
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let obj = rd32(task + F5_TASK_OBJ);
        let a15: u32 = callee_cdecl!(15, u32,);
        let c0 = f32::from_bits(global::<u32>(F5_C_CMP0).read_unaligned());
        let c1 = f32::from_bits(global::<u32>(F5_C_CMP1).read_unaligned());
        let prod = mul(a15 as i32 as f32, c1);
        // `comiss`/`jb`: jump unless the constant is greater or equal (a
        // NaN takes the jump on both sides via the negated compare).
        if !(c0 >= prod) {
            return f5_submit2(task, obj, do_vtable);
        }
        let olink = rd32(obj.wrapping_add(F5_LINK));
        let base = if olink != 0 {
            olink.wrapping_add(0x30)
        } else {
            obj.wrapping_add(0x10)
        };
        let dx = sub(rdf(base), rdf(task));
        let dy = sub(rdf(base.wrapping_add(4)), rdf(task.wrapping_add(4)));
        let len2 = add(mul(dy, dy), mul(dx, dx));
        // The lahf/jp sequence zeroes the factor exactly when the squared
        // length compares equal to zero (signed zeros included).
        let k = if len2 == 0.0 {
            0.0
        } else {
            div(
                f32::from_bits(global::<u32>(F5_C_ONE).read_unaligned()),
                len2.sqrt(),
            )
        };
        let g = f32::from_bits(global::<u32>(F5_G_VEC).read_unaligned());
        let dxk = mul(dx, k);
        let dyk = mul(dy, k);
        let k0 = mul(k, 0.0);
        let gc = mul(
            g,
            f32::from_bits(global::<u32>(F5_C_HALF).read_unaligned()),
        );
        let v0 = add(mul(dxk, gc), rdf(base));
        let v1 = add(mul(dyk, gc), rdf(base.wrapping_add(4)));
        let v2 = add(rdf(base.wrapping_add(8)), mul(gc, k0));
        let z2 = [0u32; 2];
        let _: u32 =
            callee_thiscall!(16, u32, (&z2[0] as *const u32) as u32);
        // Mirror of the twenty frame words the submit snapshot covers, in
        // slot order: the string constant, zeros, the maybe-stored sample,
        // the computed vector words, and trailing constants.
        let cdc = relocated(F5_STR);
        let mut m = [
            cdc, 0, 0,
            if e_ran { fstp_bits } else { 0 },
            v0.to_bits(), v1.to_bits(), v2.to_bits(), 0,
            g.to_bits(), 0, 0,
            // The sample block ORs 2 into the second byte of this frame
            // word; the submit snapshot covers it, so mirror the bit.
            if e_ran { F5_EPATH_MARK } else { 0 },
            dxk.to_bits(), dyk.to_bits(), k0.to_bits(), 0,
            1, 0, 3, 1,
        ];
        let a17: u32 = callee_thiscall!(
            17, u32, 0, (&mut m[0] as *mut u32) as u32, 0, 1
        );
        let _: u32 = callee_thiscall!(18, u32, a17);
        // The submit flags its guard word through the buffer on some
        // trials (the contract writes it); a nonzero guard runs the
        // release call with the guard value held, like the original.
        if m[17] != 0 {
            let _: u32 = callee_thiscall!(
                19, u32, m[17], (&m[17] as *const u32) as u32
            );
        }
        let m20 = [cdc, 0];
        let _: u32 =
            callee_thiscall!(20, u32, (&m20[0] as *const u32) as u32);
        f5_submit2(task, obj, do_vtable)
    }
}

/// Second move submit shared by the vector and skip paths.
unsafe fn f5_submit2(task: u32, obj: u32, do_vtable: bool) -> u32 {
    unsafe {
        let mut buf = [0u32; 8];
        let anchor = (&mut buf[4] as *mut u32) as u32;
        let _: u32 = callee_thiscall!(21, u32, anchor);
        let olink = rd32(obj.wrapping_add(F5_LINK));
        let base = if olink != 0 {
            olink.wrapping_add(0x30)
        } else {
            obj.wrapping_add(0x10)
        };
        let r: u32 = callee_cdecl!(
            22, u32, task, base, rd32(task + F5_TASK_OTHER), anchor,
            0x86, 0xffff_ffff, 4
        );
        if r == 0 {
            return 0;
        }
        if do_vtable {
            let zero3 = [0u32; 3];
            f5_vtable(obj, (&zero3[0] as *const u32) as u32);
        }
        let zero3b = [0u32; 3];
        callee_cdecl!(
            14, u32, task, 1,
            (&zero3b[0] as *const u32) as u32, 0
        )
    }
}