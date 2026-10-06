// original: 0x00C5AAB0 peds_wait_decide (proposed)
use lf_checker_rt as rt;

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
unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj)
    }
}


/// Decide one ped's waiting behaviour for this tick and queue the events.
///
/// `this` is a small state object (`+0x00` a signed entry id, `+0x04` an
/// index into the table at file address 0x12b60a0, `+0x14` an init flag
/// with a lazily built record at `+0x0c`). `esi` (`[ebp+8]`) is the ped:
/// task owner at `+0x224` (list head at `+0x2e0`, probe base at `+0x44`,
/// event base at `+0x84`, float base at `+0x10`), vehicle pointers at
/// `+0xb30`/`+0xab0`, task words at `+0x2b0`, flag bytes at `+0x26c`/`+0x268`
/// and flag words at `+0x29c`/`+0x270`.
///
/// Flow: resolve the ped through two helpers (either answering null ends
/// the call at once, as does a set word at owner `+0x50`); check the entry
/// id, either by a signed divide of a global dividend by a table divisor
/// whose remainder must equal the id, or, for id -1, by initialising the
/// record and asking a validator; compute one vehicle flag per ped
/// (in-vehicle with matching type, else the secondary slot); probe and
/// search the ped's list twice; check the two peds share a vehicle; pick
/// one of two task-event builders from two flag bytes; optionally scale
/// two constants (15.0 or 20.0 by -0.5/+0.5/7.0) into stack structs for a
/// float call; search two more lists and, when both flags are clear, run
/// the event block (two probes, a task-array gate, two resolver calls and
/// up to two event builders); search for the wait entry and either clear
/// or set the wait bit and queue its events; run the wait-state builder
/// when exactly the secondary flag is set; then always run the tail
/// (resolve the current task, check its kind, probe, and queue one event).
/// Three shapes end early: the two resolution failures and the owner-word
/// check skip everything including the tail, while a failed id check runs
/// the tail without the closing call.
///
/// All id, priority, kind and bound comparisons are unsigned; only the
/// entry-id divide is signed (quotient discarded, remainder compared).
/// List searches exit early when a priority rises between nodes.
/// The `edi == esi` edge is excluded from the proof: the original reads
/// flag bytes it never wrote on that path, so no rewrite can match it.
///
/// Original: 0x00C5AAB0 (thiscall, one stack word, void).
const P_TASK: u32 = 0x224;
const P_B30: u32 = 0xb30;
const P_AB0: u32 = 0xab0;
const P_26C: u32 = 0x26c;
const P_268: u32 = 0x268;
const P_270: u32 = 0x270;
const P_29C: u32 = 0x29c;
const P_2B0: u32 = 0x2b0;
const P_21C: u32 = 0x21c;
const P_A60: u32 = 0xa60;
const P_D68: u32 = 0xd68;
const G2_U32: u32 = 0x11735b4;
const G2_DIVD: u32 = 0x12b41d4;
const G2_TAB: u32 = 0x12b60a0;
const F15_ADR: u32 = 0x1049210;
const F20_ADR: u32 = 0x1049214;
const FM05_ADR: u32 = 0xfe8d7c;
const F05_ADR: u32 = 0xfe8830;
const EV_AE5C: u32 = 0xecae5c;
const EV_AF0C: u32 = 0xecaf0c;
const EV_AEB4: u32 = 0xecaeb4;
const EV_AFBC: u32 = 0xecafbc;
const EV_AF64: u32 = 0xecaf64;
const EV_8734: u32 = 0xe98734;

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

/// One owner-list search: node `+4` is the id, `+8` holds the priority in
/// bits 1..3, `+0xc` the next node. Exits early (miss) when the priority
/// rises between consecutive nodes; all comparisons unsigned.
#[inline(always)]
unsafe fn search_list(head: u32, target: u32) -> bool {
    unsafe {
        if head == 0 {
            return false;
        }
        let mut prev = (rd32(head + 8) >> 1) & 7;
        let mut cur = head;
        loop {
            let p = (rd32(cur + 8) >> 1) & 7;
            if prev < p {
                return false;
            }
            if rd32(cur + 4) == target {
                return true;
            }
            prev = p;
            cur = rd32(cur + 0x0c);
            if cur == 0 {
                return false;
            }
        }
    }
}

/// Secondary-slot chain: non-null slot whose word `+0x28` masked with
/// 0x3c0 equals 0x80 and whose word `+0x1304` equals 2.
#[inline(always)]
unsafe fn ab0_chain(ped: u32) -> u8 {
    unsafe {
        let a = rd32(ped + P_AB0);
        if a == 0 {
            return 0;
        }
        if rd32(a + 0x28) & 0x3c0 != 0x80 {
            return 0;
        }
        if rd32(a + 0x1304) != 2 {
            return 0;
        }
        1
    }
}

/// Vehicle flag: with the in-vehicle bit set and a non-null vehicle the
/// flag is whether the vehicle's word `+0x1304` equals 2, else the
/// secondary-slot chain decides.
#[inline(always)]
unsafe fn veh_flag(ped: u32) -> u8 {
    unsafe {
        if rd8(ped + P_26C) & 4 == 0 {
            return ab0_chain(ped);
        }
        let b = rd32(ped + P_B30);
        if b == 0 {
            return ab0_chain(ped);
        }
        if rd32(b + 0x1304) == 2 {
            1
        } else {
            0
        }
    }
}

macro_rules! ev3 {
    ($b:expr, $p:expr, $d:expr, $esi:expr, $id:expr) => {{
        let mut ev = [0u32; 4];
        rt::callee_thiscall!($b, u32, ev.as_mut_ptr() as u32);
        ev[0] = rt::relocated($id);
        rt::callee_thiscall!($p, u32, rd32($esi + P_TASK) + 0x84, ev.as_mut_ptr() as u32, 0, 1);
        rt::callee_thiscall!($d, u32, ev.as_mut_ptr() as u32);
    }};
}

/// The tail: resolve the current task through the other ped, check its
/// kind word, probe, and queue the wake event. Runs on every path except
/// the three earliest exits.
#[inline(always)]
unsafe fn tail2(esi: u32, edi: u32) {
    unsafe {
        let t = rt::callee_thiscall!(42, u32, rd32(edi + P_TASK) + 0x2e0);
        if t == 0 {
            return;
        }
        if rd32(t + 0x28) & 0x3c0 != 0xc0 {
            return;
        }
        if rt::callee_thiscall!(43, u32, rd32(esi + P_TASK) + 0x84, 0x24) != 0 {
            return;
        }
        if rt::callee_thiscall!(44, u32, rd32(esi + P_TASK), t) & 0xff != 0 {
            return;
        }
        let mut s40 = [0u32; 4];
        rt::callee_thiscall!(45, u32, s40.as_mut_ptr() as u32, t);
        let mut ev = [0u32; 4];
        ev[0] = rt::relocated(EV_8734);
        rt::callee_thiscall!(46, u32, rd32(esi + P_TASK) + 0x84, ev.as_mut_ptr() as u32, 0, 1);
        rt::callee_thiscall!(47, u32, ev.as_mut_ptr() as u32);
    }
}

/// Shared body. `unsigned_div` selects the deliberately wrong entry-id
/// divide (unsigned instead of signed) used only by the mutant.
#[inline(always)]
unsafe fn body2(this: u32, esi: u32, unsigned_div: bool) {
    unsafe {
        let r9 = rt::callee_thiscall!(0, u32, esi);
        if r9 == 0 {
            return;
        }
        let mecx = r9.wrapping_add(8);
        let edi = rt::callee_thiscall!(1, u32, mecx);
        if edi == 0 {
            return;
        }
        if rd32(rd32(esi + P_TASK) + 0x50) != 0 {
            return;
        }
        let eid = rd32(this) as i32;
        if eid == -1 {
            if rd8(this + 0x14) == 0 {
                wr32(this + 0x0c, rd32(rt::relocated(G2_U32)));
                wr32(this + 0x10, 0);
                wr8(this + 0x14, 1);
            }
            if rt::callee_thiscall!(2, u32, this + 0x0c) & 0xff == 0 {
                tail2(esi, edi);
                return;
            }
            wr32(this + 0x0c, rd32(rt::relocated(G2_U32)));
            wr32(this + 0x10, 0x64);
            wr8(this + 0x14, 1);
        } else {
            let idx = rd32(this + 4);
            let slot = rd32(rt::relocated(G2_TAB).wrapping_add(idx.wrapping_mul(4)));
            let dvd = rd32(rt::relocated(G2_DIVD)) as i32;
            let dvs = rd32(slot + 4) as i32;
            let _q = dvd / dvs;
            let rem = if unsigned_div {
                (dvd as u32 % dvs as u32) as i32
            } else {
                dvd % dvs
            };
            if rem != eid {
                tail2(esi, edi);
                return;
            }
        }
        rt::callee_thiscall!(3, u32, this);
        if edi == esi {
            // Excluded from the proof (see doc comment): the original
            // reads flag bytes it never wrote on this path.
            rt::callee_thiscall!(41, u32, this);
            tail2(esi, edi);
            return;
        }
        let f19 = veh_flag(edi);
        let f1a = veh_flag(esi);
        let f1b = if rt::callee_thiscall!(4, u32, rd32(esi + P_TASK) + 0x44, 0x2de) != 0 {
            1u8
        } else {
            0u8
        };
        let f18 = if search_list(rd32(rd32(edi + P_TASK) + 0x2e0), 0x2de) {
            1u8
        } else {
            0u8
        };
        let mut f20: u32 = if rd8(esi + P_26C) & 4 != 0 {
            1
        } else if rt::callee_thiscall!(5, u32, rd32(esi + P_TASK) + 0x44, 0x2de) != 0 {
            1
        } else {
            0
        };
        let dl: u8 = if f19 != 0 {
            1
        } else if rd8(edi + P_26C) & 4 != 0
            && rt::callee_thiscall!(6, u32, rd32(edi + P_TASK) + 0x2e0, 0x2e2, 0) & 0xff == 0
        {
            1
        } else if search_list(rd32(rd32(edi + P_TASK) + 0x2e0), 0x2de) {
            if rt::callee_thiscall!(7, u32, rd32(edi + P_TASK) + 0x2e0, 0x2de, 5) < 0x1d {
                1
            } else {
                0
            }
        } else {
            0
        };
        if rd8(esi + P_26C) & 4 != 0 {
            let sb = rd32(esi + P_B30);
            if sb != 0 && rd8(edi + P_26C) & 4 != 0 {
                let eb = rd32(edi + P_B30);
                if eb != 0 && sb != eb {
                    f20 = 0;
                }
            }
        }
        if dl != 0 {
            if f20 == 0 {
                let mut cflag: u8 = 0;
                if rd32(rd32(esi + P_21C) + 0x12c) == 0x10
                    && rt::callee_thiscall!(8, u32, rd32(esi + P_TASK) + 0x2e0, 0x391, 0) & 0xff != 0
                {
                    cflag = 1;
                }
                if rd8(esi + P_268) & 2 == 0 && cflag == 0
                    && rt::callee_thiscall!(9, u32, rd32(esi + P_TASK) + 0x84, 0x36) == 0
                {
                    let mut s40 = [0u32; 4];
                    rt::callee_thiscall!(10, u32, s40.as_mut_ptr() as u32, rd32(edi + P_B30));
                    rt::callee_thiscall!(11, u32, rd32(esi + P_TASK) + 0x84, s40.as_mut_ptr() as u32, 0, 1);
                    rt::callee_thiscall!(12, u32, s40.as_mut_ptr() as u32);
                }
            }
        } else if f20 != 0 {
            if rd8(esi + P_268) & 2 == 0
                && rt::callee_thiscall!(13, u32, rd32(esi + P_TASK) + 0x84, 0x37) == 0
            {
                ev3!(14, 15, 16, esi, EV_AE5C);
            }
        }
        if !(rd32(rd32(esi + P_21C) + 0x12c) == 2 && rd8(esi + P_A60) != 2) {
            let f15 = rdf(rt::relocated(F15_ADR));
            let mut x1 = f15;
            if rt::callee_thiscall!(17, u32, mecx) != 0 {
                // No null check: the original dereferences the answer at
                // once, so a null here faults on both sides.
                let m3 = rt::callee_thiscall!(18, u32, mecx);
                if vcall0(m3, 0x128) & 0xff == 0 {
                    x1 = rdf(rt::relocated(F20_ADR));
                }
            }
            let neg = mul(x1, rdf(rt::relocated(FM05_ADR)));
            let pos = mul(x1, rdf(rt::relocated(F05_ADR)));
            let mut s50 = [0u32, neg.to_bits(), 0xc0e00000, 0];
            let mut s3c = [0u32, pos.to_bits(), 0x40e00000, 0];
            rt::callee_thiscall!(20, u32, rd32(esi + P_TASK) + 0x10,
                s50.as_mut_ptr() as u32, s3c.as_mut_ptr() as u32, x1.to_bits(), edi, 0);
        }
        let mut cl15: u8 = 0;
        let mut ran_events = true;
        if f20 == 0 && dl == 0 {
            let f14 = if search_list(rd32(rd32(esi + P_TASK) + 0x2e0), 0x76c) {
                1u8
            } else {
                0u8
            };
            let ehead = rd32(rd32(edi + P_TASK) + 0x2e0);
            let f16 = if search_list(ehead, 0x13f) { 1u8 } else { 0u8 };
            let f17 = f16;
            if rt::callee_thiscall!(21, u32, rd32(esi + P_TASK) + 0x2e0, 0x40c, 0) & 0xff != 0 {
                cl15 = 1;
            } else if rt::callee_thiscall!(22, u32, rd32(esi + P_TASK) + 0x2e0, 0x417, 0) & 0xff != 0
            {
                cl15 = 1;
            }
            let tidx = rd32(esi + P_2B0).wrapping_add(3).wrapping_mul(3);
            let tword = rd32(esi.wrapping_add(P_2B0).wrapping_add(tidx.wrapping_mul(4)));
            if tword == 0 || cl15 != 0 {
                if f14 != 0 {
                    wr32(esi + P_270, rd32(esi + P_270) & 0xffdfffff);
                }
            } else if f14 != cl15 {
                wr32(esi + P_270, rd32(esi + P_270) & 0xffdfffff);
                ran_events = false;
            } else if f16 != cl15 || f17 != cl15 || rd8(esi + P_29C) & 4 != 0
                || rd32(esi + P_29C) & 0x20000000 != 0
            {
            } else {
                let b1 = rt::callee_thiscall!(23, u32, edi + P_2B0);
                let b2 = rt::callee_thiscall!(24, u32, esi + P_2B0);
                let found6 = search_list(rd32(rd32(edi + P_TASK) + 0x2e0), 0x422);
                let cl: u8 = if b1 != 0 && rd32(b1 + 0x18) != 0 && rd32(b1 + 0x18) != 0x2e {
                    0
                } else if found6 {
                    0
                } else {
                    1
                };
                let al2: u8 = if b2 == 0 || rd32(b2 + 0x18) == 0 || rd32(b2 + 0x18) == 0x2e {
                    1
                } else {
                    0
                };
                if cl == 0 {
                    if al2 != 0 {
                        ev3!(25, 26, 27, esi, EV_AF0C);
                    }
                } else if al2 == 0 {
                    ev3!(28, 29, 30, esi, EV_AEB4);
                }
            }
            if ran_events {
                if !search_list(rd32(rd32(edi + P_TASK) + 0x2e0), 0x41e) {
                    wr32(esi + P_270, rd32(esi + P_270) & 0xffdfffff);
                    if cl15 != 0 {
                        ev3!(32, 33, 34, esi, EV_AFBC);
                    }
                } else {
                    let w270 = rd32(esi + P_270);
                    if w270 & 0x200000 == 0
                        && rt::callee_thiscall!(31, u32, rd32(esi + P_TASK) + 0x2e0, 0x417, 0) & 0xff != 0
                        && rd32(esi + P_D68) == 0
                    {
                        wr32(esi + P_270, w270 | 0x200000);
                    }
                    let bit21 = (rd32(esi + P_270) >> 0x15) & 1;
                    if bit21 == 0 && cl15 == 0 {
                        ev3!(35, 36, 37, esi, EV_AF64);
                    } else if bit21 != 0 && cl15 != 0 {
                        ev3!(32, 33, 34, esi, EV_AFBC);
                    }
                }
            }
        }
        if f1b == 0 && f18 == 0 && f1a != 0 {
            let mut proceed = true;
            if f19 != 0 {
                let ea = rd32(edi + P_AB0);
                let sa = rd32(esi + P_AB0);
                if ea == 0 || sa == 0 || ea == sa {
                    proceed = false;
                }
            }
            if proceed {
                let mut vc: u32 = 0;
                let mut ok = false;
                let sa = rd32(esi + P_AB0);
                if sa != 0 && rd32(sa + 0x28) & 0x3c0 == 0x80 && rd32(sa + 0x1304) == 2 {
                    vc = sa;
                    ok = true;
                } else if rd8(esi + P_26C) & 4 != 0 {
                    let sb = rd32(esi + P_B30);
                    if sb != 0 && rd32(sb + 0x1304) == 2 {
                        vc = sb;
                        ok = true;
                    }
                }
                // vc is never 0 here (both paths test non-null); the
                // original's null check is dead but mirrored.
                if ok && vc != 0 && rd32(esi + P_268) & 0x400 == 0 {
                    let mut s40 = [0u32; 4];
                    rt::callee_thiscall!(38, u32, s40.as_mut_ptr() as u32, vc);
                    rt::callee_thiscall!(39, u32, rd32(esi + P_TASK) + 0x84, s40.as_mut_ptr() as u32, 0, 1);
                    rt::callee_thiscall!(40, u32, s40.as_mut_ptr() as u32);
                }
            }
        }
        let _ = cl15;
        rt::callee_thiscall!(41, u32, this);
        tail2(esi, edi);
    }
}

rt::export!(thiscall, rw_00C5AAB0(this: u32, ped: u32) -> u32 {
    unsafe { body2(this, ped, false) };
    0
});
