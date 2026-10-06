// original: 0x00C59E60 peds_tasks_tick (proposed)

/// Per-frame update of one ped's task state: run every task subsystem in
/// turn, then scan the fixed ped-task table for entries near this ped.
///
/// `this` is the task manager (`+0x08` a millisecond counter; sub-objects
/// at `+0x0c/+0x3c/+0xcc/+0xe8/+0x100/+0x118/+0x124/+0x208`). `ped` (`[ebp+8]`)
/// is the ped: vtable at `+0x00` (slot `+0x128` answers whether it is alive),
/// position matrix at `+0x20`, task owner at `+0x224`, vehicle at `+0xb30`,
/// flag bytes at `+0x118/+0x211/+0x212/+0x219/+0xa60` and flag words at
/// `+0x28` (bit 21), `+0x260` (bit 31), `+0x26c` (bits 0 and 2, dword bit 13)
/// and `+0x270` (bit 23).
///
/// Phases in order: three task-list updates through the owner, the sibling
/// update this function calls directly, one conditional phase per flag
/// group, a millisecond counter advanced by truncated frame time (unsigned
/// compared against 300; the equal case takes the quiet path), a task-owner
/// liveness walk, a guarded block that builds task events on the stack, a
/// task-id probe chain with a shared countdown, a vehicle check, a target
/// flag set-or-clear, and finally a 32-entry scan of the fixed table at
/// file address 0x1292460 (stride 0xd0): live entries (word `+8` equal to 1
/// with a non-null object at `+0`) get a proximity test against the ped and,
/// when close, a normalized pull vector through the event builder.
///
/// All comparisons of counts and bounds are unsigned; float edge cases
/// (NaN, negative zero) follow the original's comiss/jbe folding, and the
/// length-squared zero test keeps the original's lahf/test/jp shape
/// (zero, including negative zero, yields a zero vector; anything else is
/// scaled by one over its root).
///
/// Original: 0x00C59E60 (thiscall, one stack word, void).
use lf_checker_rt as rt;

const VT_ALIVE: u32 = 0x128;
const VT_OWNER_STEP: u32 = 0x1c;
const VT_LOOP_STEP: u32 = 0xec;
const PED_POS: u32 = 0x20;
const PED_TASK: u32 = 0x224;
const PED_B30: u32 = 0xb30;
const THIS_CTR: u32 = 0x08;
const CTR_LIM: u32 = 0x12c;
const G_TIME: u32 = 0x11735bc;
const G_U32: u32 = 0x11735b4;
const G_IDX: u32 = 0x1036f14;
const TBL_A: u32 = 0x11a8808;
const G_DEC: u32 = 0x16dcc24;
const G_BIT: u32 = 0x16dce58;
const G_LIM: u32 = 0x16dce54;
const LOOP_TAB: u32 = 0x1292460;
const LOOP_END: u32 = 0x1293e60;
const LOOP_STRIDE: u32 = 0xd0;
const OWNER: u32 = 0x12e2420;
const C1000_ADR: u32 = 0xfe8c58;
const C25_ADR: u32 = 0xed9a68;
const CN1000_ADR: u32 = 0xfe8e04;
const C025_ADR: u32 = 0xfe87e4;
const C05_ADR: u32 = 0xfe8830;
const C2_ADR: u32 = 0xfe8684;
const C10_ADR: u32 = 0xfe88e8;

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
/// comiss(x, y) followed by jbe: taken unless x is ordered-above y.
/// NaN input takes the branch, exactly like the hardware flag fold.
#[inline(always)]
fn jbe(x: f32, y: f32) -> bool {
    !(x > y)
}
/// The fistp-qword-with-truncate step. Only the low dword of the stored
/// 64-bit value is used. Invalid (NaN, out of i64 range) stores the
/// 64-bit indefinite 0x8000000000000000 whose low dword is zero.
#[inline(always)]
fn trunc_low(t: f32) -> u32 {
    const LIM: f32 = 9223372036854775808.0;
    if t.is_nan() || t >= LIM || t < -LIM {
        0
    } else {
        (t as i64) as u32
    }
}
#[inline(always)]
unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj)
    }
}
#[inline(always)]
unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj, a0)
    }
}

/// Shared body. `signed_bound` selects the deliberately wrong counter
/// comparison (signed instead of unsigned) used only by the mutant.
#[inline(always)]
unsafe fn body(this: u32, ped: u32, signed_bound: bool) {
    unsafe {
        let t224 = rd32(ped + PED_TASK);
        rt::callee_thiscall!(1, u32, this + 0x0c, ped, t224.wrapping_add(0x10c), 0x10);
        if (vcall0(ped, VT_ALIVE) & 0xff) != 0 {
            let inner = t224.wrapping_add(0x150);
            if rt::callee_thiscall!(3, u32, inner) != 0 {
                let got = rt::callee_thiscall!(4, u32, inner);
                rt::callee_thiscall!(5, u32, this + 0x118, ped, got);
            }
        }
        rt::callee_thiscall!(6, u32, this + 0xcc, ped, t224.wrapping_add(0x168), 0x10);
        rt::callee_thiscall!(7, u32, this + 0x100, ped);
        if (rt::callee_thiscall!(8, u32, ped) & 0xff) != 0 && rd8(ped + 0xa60) != 2 {
            rt::callee_thiscall!(9, u32, this + 0x124, ped);
        }
        rt::callee_thiscall!(10, u32, this + 0x3c, ped);
        rt::callee_thiscall!(11, u32, this + 0xe8, ped);
        rt::callee_thiscall!(12, u32, t224.wrapping_add(0x208), ped);
        let f26c = rd32(ped + 0x26c);
        let counter = f26c & 1 == 0
            && rd8(ped + 0x118) & 1 == 0
            && (f26c & 0x2000 != 0 || (rt::callee_cdecl!(13, u32, ped) & 0xff) != 0);
        if counter {
            let step = trunc_low(mul(rdf(rt::relocated(G_TIME)), rdf(rt::relocated(C1000_ADR))));
            let sum = rd32(this + THIS_CTR).wrapping_add(step);
            wr32(this + THIS_CTR, sum);
            let over = if signed_bound {
                (sum as i32) > CTR_LIM as i32
            } else {
                sum > CTR_LIM
            };
            if over {
                let mut ev = [0u32; 2];
                rt::callee_thiscall!(14, u32, ev.as_mut_ptr() as u32);
                let mut req = [0u32; 2];
                rt::callee_thiscall!(15, u32, t224.wrapping_add(0x84), req.as_mut_ptr() as u32, 0, 1);
                rt::callee_thiscall!(16, u32, ev.as_mut_ptr() as u32);
            }
        } else if f26c & 0x2000 != 0 {
            let first = vcall0(t224, VT_OWNER_STEP);
            if rt::callee_thiscall!(18, u32, first.wrapping_add(0x20)) != 0 {
                let second = vcall0(t224, VT_OWNER_STEP);
                let obj = rt::callee_thiscall!(18, u32, second.wrapping_add(0x20));
                if vcall0(obj, 4) != 0x21 {
                    wr32(ped + 0x26c, f26c & !0x2000);
                }
            } else {
                wr32(ped + 0x26c, f26c & !0x2000);
            }
            wr32(this + THIS_CTR, 0);
        } else {
            wr32(this + THIS_CTR, 0);
        }
        if (rd32(ped + 0x28) >> 0x15) & 1 != 0 {
            if (vcall0(ped, VT_ALIVE) & 0xff) == 0 || (rt::callee_cdecl!(20, u32,) & 0xff) == 0 {
                e_fallback(t224);
            } else {
                let found = rt::callee_thiscall!(21, u32, rt::relocated(OWNER), ped);
                let idx = rd32(rt::relocated(G_IDX));
                if idx != 0xffffffff {
                    let slot = rd32(rt::relocated(TBL_A).wrapping_add(idx.wrapping_mul(4)));
                    if slot != 0
                        && (rt::callee_thiscall!(22, u32, slot) & 0xff) == 0
                        && found != 0
                    {
                        e_full(ped, found);
                    }
                }
            }
        }
        if rd32(ped + 0x26c) & 4 != 0 {
            let veh = rd32(ped + PED_B30);
            if veh != 0 && rd32(veh + 0x1300) == 2 && rd8(ped + 0x219) == 0 {
                let probe = rt::callee_cdecl!(33, u32, 0);
                if rd32(probe.wrapping_add(0xab4)) == veh {
                    let probe2 = rt::callee_cdecl!(33, u32, 0);
                    let mut req = [0u32; 8];
                    rt::callee_thiscall!(34, u32, req.as_mut_ptr() as u32, probe2, veh, 5);
                    let mut req2 = [0u32; 8];
                    req2[4] = 0x2c2;
                    rt::callee_thiscall!(
                        35, u32, t224.wrapping_add(0x84), req2.as_mut_ptr() as u32, 0, 1
                    );
                    let mut ev = [0u32; 2];
                    rt::callee_thiscall!(36, u32, ev.as_mut_ptr() as u32);
                }
            }
        }
        probe_chain(ped, t224);
        if rd32(ped + 0x26c) & 4 != 0 {
            let veh = rd32(ped + PED_B30);
            if veh != 0 && rd32(ped + 0x260) & 0x80000000 != 0 {
                let quick = (rt::callee_thiscall!(42, u32, veh.wrapping_add(0x10d0)) & 0xff) != 0;
                let mut construct = quick;
                if !construct {
                    let near = rdf(veh.wrapping_add(0x10d8));
                    let floor = rdf(rt::relocated(CN1000_ADR));
                    if jbe(near, floor) || !(0.0 > near) {
                        construct = (rd32(veh + 0x28) >> 0x15) & 1 != 0;
                    } else {
                        construct = true;
                    }
                }
                if construct {
                    let mut ev = [0u32; 2];
                    rt::callee_thiscall!(43, u32, ev.as_mut_ptr() as u32, veh);
                    let mut req = [0u32; 2];
                    rt::callee_thiscall!(56, u32, t224.wrapping_add(0x84), req.as_mut_ptr() as u32, 0, 1);
                    let mut ev2 = [0u32; 2];
                    rt::callee_thiscall!(44, u32, ev2.as_mut_ptr() as u32);
                }
            }
        }
        flag_block(ped, t224);
        let step = vcall0(t224, VT_OWNER_STEP);
        if rt::callee_thiscall!(49, u32, step) != 0x8b {
            scan_table(ped);
        }
    }
}

/// Fallback event (short request, tagged 0xe9dd14) for the gated block.
#[inline(always)]
unsafe fn e_fallback(t224: u32) {
    unsafe {
        let mut ev = [0u32; 2];
        rt::callee_thiscall!(30, u32, ev.as_mut_ptr() as u32);
        let mut req = [0u32; 2];
        req[0] = rt::relocated(0xe9dd14);
        rt::callee_thiscall!(31, u32, t224.wrapping_add(0x84), req.as_mut_ptr() as u32, 0, 1);
        ev[0] = rt::relocated(0xe9dd14);
        rt::callee_thiscall!(32, u32, ev.as_mut_ptr() as u32);
    }
}

/// Full gated block: seed, blend, finalize, then an optional heavy blend
/// when the finalize step reports new work (low two bits of its word).
#[inline(always)]
unsafe fn e_full(ped: u32, found: u32) {
    unsafe {
        let mut seed = [0u32; 11];
        let shared = rd32(rt::relocated(G_U32));
        rt::callee_thiscall!(23, u32, seed.as_mut_ptr() as u32, 0, shared, 5);
        let mut blend = [0u32; 10];
        let scaled = mul(rdf(rt::relocated(G_TIME)), rdf(rt::relocated(C25_ADR)));
        let extra = rd32(found.wrapping_add(0x44));
        rt::callee_thiscall!(24, u32, blend.as_mut_ptr() as u32, extra, scaled.to_bits(), 5, 0, 0);
        let mut fin = [0u32; 10];
        let mut work = [0u32; 10];
        rt::callee_thiscall!(25, u32, fin.as_mut_ptr() as u32, ped, work.as_mut_ptr() as u32);
        let mut req = [0u32; 4];
        req[0] = 0x41a00000;
        req[1] = 3 | ((rd8(ped.wrapping_add(0x210)) as u32) << 16);
        rt::callee_thiscall!(26, u32, ped.wrapping_add(0x570), req.as_mut_ptr() as u32);
        if seed[10] & 0x0c != 0 {
            let heavy = rd32(found.wrapping_add(0x44));
            let _: f32 = rt::callee_cdecl!(27, f32, heavy, ped, 5, rdf(rt::relocated(C25_ADR)).to_bits(), 0, 0, 0, 0, 0);
        }
        let mut tmp = [0u32; 2];
        rt::callee_thiscall!(28, u32, tmp.as_mut_ptr() as u32);
        let mut fin2 = [0u32; 2];
        rt::callee_thiscall!(29, u32, fin2.as_mut_ptr() as u32);
    }
}

/// Task-id probe chain with the shared countdown.
#[inline(always)]
unsafe fn probe_chain(ped: u32, t224: u32) {
    unsafe {
        let mode = rd32(ped + 0xa74);
        if mode == 1 || mode == 2 {
            return;
        }
        if rd8(ped + 0x211) == 0 && rd8(ped + 0x212) == 0 {
            return;
        }
        let left = rd32(rt::relocated(G_DEC));
        if left != 0 {
            wr32(rt::relocated(G_DEC), left.wrapping_sub(1));
            return;
        }
        for (n, id) in [(0, 0x191u32), (1, 0xd9), (2, 0x836), (3, 0xfa)] {
            let hit = rt::callee_thiscall!(37, u32, t224.wrapping_add(0x44), id);
            if hit != 0 {
                if n == 0 {
                    if rd8(hit.wrapping_add(0x18)) & 2 != 0 {
                        return;
                    }
                } else {
                    return;
                }
            }
        }
        let mut seed = [0u32; 4];
        rt::callee_thiscall!(38, u32, seed.as_mut_ptr() as u32, 0, 0, 0x37);
        let mut req = [0u32; 2];
        if (rt::callee_thiscall!(39, u32, t224.wrapping_add(0x84), req.as_mut_ptr() as u32) & 0xff) == 0 {
            let mut big = [0u32; 20];
            rt::callee_thiscall!(40, u32, big.as_mut_ptr() as u32);
            let mat = rd32(ped + PED_POS);
            big[4] = rd32(mat.wrapping_add(0x30));
            big[5] = rd32(mat.wrapping_add(0x34));
            big[6] = rd32(mat.wrapping_add(0x38));
            let mut small = [0u32; 3];
            let _: f32 = rt::callee_cdecl!(
                41, f32, 0, ped, 7, 0x41200000,
                small.as_mut_ptr() as u32, big.as_mut_ptr() as u32,
                0, 0, 0
            );
            wr32(rt::relocated(G_DEC), 5);
        }
        let mut fin = [0u32; 2];
        rt::callee_thiscall!(29, u32, fin.as_mut_ptr() as u32);
    }
}

/// Set-or-clear of the ped target flag (bit 23 of +0x270).
#[inline(always)]
unsafe fn flag_block(ped: u32, t224: u32) {
    unsafe {
        let cand = rt::callee_cdecl!(33, u32, 0);
        let set = cand != 0
            && rd32(cand.wrapping_add(0xab0)) != 0
            && rd32(ped + PED_B30) != 0
            && (rt::callee_thiscall!(45, u32, rd32(ped + PED_B30), ped) & 0xff) != 0
            && rd32(cand.wrapping_add(0xab0)) == rd32(ped + PED_B30)
            && rd8(ped + 0xa60) != 2
            && (vcall0(ped, VT_ALIVE) & 0xff) == 0;
        if set {
            if rd32(ped + 0x270) & 0x800000 != 0 && rd32(rd32(ped + PED_B30).wrapping_add(0x12ec)) != 0 {
                return;
            }
        } else {
            wr32(ped + 0x270, rd32(ped + 0x270) & !0x800000);
            return;
        }
        let mut ev = [0u32; 2];
        rt::callee_thiscall!(46, u32, ev.as_mut_ptr() as u32, cand, rd32(cand.wrapping_add(0xab0)));
        let mut req = [0u32; 2];
        rt::callee_thiscall!(56, u32, t224.wrapping_add(0x84), req.as_mut_ptr() as u32, 0, 1);
        wr32(ped + 0x270, rd32(ped + 0x270) | 0x800000);
        let mut ev2 = [0u32; 2];
        rt::callee_thiscall!(47, u32, ev2.as_mut_ptr() as u32);
    }
}

/// Scan of the fixed 32-entry table for live entries near the ped.
#[inline(always)]
unsafe fn scan_table(ped: u32) {
    unsafe {
        let lim25 = rdf(rt::relocated(C025_ADR));
        let half = rdf(rt::relocated(C05_ADR));
        let c2 = rdf(rt::relocated(C2_ADR));
        let one = rdf(rt::relocated(C10_ADR));
        let mut entry = rt::relocated(LOOP_TAB);
        loop {
            if entry.wrapping_sub(0x10) != 0
                && rd32(entry.wrapping_add(8)) == 1
                && rd32(entry) != 0
            {
                let obj = rd32(entry);
                let mut arg = [0u32; 2];
                let triple = vcall1(obj, VT_LOOP_STEP, arg.as_mut_ptr() as u32);
                let sx = rdf(triple);
                let sy = rdf(triple.wrapping_add(4));
                let sz = rdf(triple.wrapping_add(8));
                let sum = add(add(mul(sx, sx), mul(sy, sy)), mul(sz, sz));
                if jbe(lim25, sum) {
                    // too far
                } else {
                    // A null liveness answer clears the flag and continues;
                    // it does not end the iteration.
                    let mut near_flag = 0u32;
                    if rt::callee_thiscall!(51, u32, ped) != 0 {
                        let anchor = rd32(entry.wrapping_add(0xc));
                        if anchor != 0 && rd32(anchor.wrapping_add(0x28)) & 0x3c0 == 0xc0 {
                            let a = rt::callee_thiscall!(51, u32, anchor);
                            let b = rt::callee_thiscall!(51, u32, ped);
                            near_flag = (a == b) as u32;
                        }
                    }
                    let base = rd32(obj.wrapping_add(0x20));
                    let pos = if base != 0 { base.wrapping_add(0x30) } else { obj.wrapping_add(0x10) };
                    let mat = rd32(ped + PED_POS);
                    let dx = sub(rdf(mat.wrapping_add(0x30)), rdf(pos));
                    let dy = sub(rdf(mat.wrapping_add(0x34)), rdf(pos.wrapping_add(4)));
                    let dz = sub(rdf(mat.wrapping_add(0x38)), rdf(pos.wrapping_add(8)));
                    let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    let bits = rd32(rt::relocated(G_BIT));
                    if bits & 1 == 0 {
                        wr32(rt::relocated(G_BIT), bits | 1);
                        wr32(rt::relocated(G_LIM), 0x42100000);
                    }
                    let mut al2 = 0u32;
                    if (rt::callee_thiscall!(52, u32, ped, obj, 0x3f860a92) & 0xff) != 0 {
                        let n = rt::callee_cdecl!(53, u32,) as i32 as f32;
                        if jbe(half, mul(n, c2)) {
                            al2 = 0;
                        } else {
                            al2 = 1;
                        }
                    }
                    if !jbe(rdf(rt::relocated(G_LIM)), d2) && (al2 != 0 || near_flag != al2) {
                        let k = if d2 == 0.0 {
                            0.0
                        } else {
                            let r = d2.sqrt();
                            core::hint::black_box(one) / core::hint::black_box(r)
                        };
                        let nx = mul(dx, k);
                        let ny = mul(dy, k);
                        let nz = mul(dz, k);
                        let base2 = rd32(obj.wrapping_add(0x20));
                        let pos2 = if base2 != 0 {
                            base2.wrapping_add(0x30)
                        } else {
                            obj.wrapping_add(0x10)
                        };
                        let mut out = [0u32; 22];
                        out[0] = nx.to_bits();
                        out[1] = ny.to_bits();
                        out[2] = nz.to_bits();
                        let mut req = [0u32; 20];
                        rt::callee_thiscall!(
                            54, u32, req.as_mut_ptr() as u32, pos2, 0x40c00000,
                            out.as_mut_ptr() as u32, 3, obj, 0, 0
                        );
                        let mut req2 = [0u32; 2];
                        rt::callee_thiscall!(56, u32, rd32(ped + PED_TASK).wrapping_add(0x84), req2.as_mut_ptr() as u32, 0, 1);
                        if rd32(out.as_mut_ptr() as u32 + 0x54) != 0 {
                            rt::callee_stdcall!(55, u32, out.as_mut_ptr() as u32 + 0x54);
                        }
                        req[0] = rt::relocated(0xe94cdc);
                        rt::callee_thiscall!(32, u32, req.as_mut_ptr() as u32);
                    }
                }
            }
            entry = entry.wrapping_add(LOOP_STRIDE);
            if (entry as i32) >= (rt::relocated(LOOP_END) as i32) {
                break;
            }
        }
    }
}

rt::export!(thiscall, rw_00C59E60(this: u32, ped: u32) -> u32 {
    unsafe { body(this, ped, false) };
    0
});
