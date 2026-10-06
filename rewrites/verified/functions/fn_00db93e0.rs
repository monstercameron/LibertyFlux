// original: 0x00db93e0 UIMusicViewer::vf97
//! Rewrite of UIMusicViewer::vf97 (music viewer tick).
//!
//! Specification. Thiscall with one stack argument; returns a view pointer,
//! a slot call result, or a compared bound depending on the path. Three
//! gated phases (A, B, C) run in series: each phase first compares a slot-0
//! hook on the worker object against a string-fed compare callee, and skips
//! the phase on mismatch. A taken phase runs its 16-hook float phase (eight
//! samples on the worker plus eight on `this`, blended with the shared
//! scale constant, ordered reductions, two clamped runtime counters, and a
//! four-way ordered-greater dispatch). A dispatch-true arm on phase A calls
//! one direct callee; on phases B/C it runs a flag-selected section (a
//! counted loop over an array object, or an element-array scan with
//! per-element inner loops), a shared join, and epilogue A (one-arg slot
//! call, latch write, returns the view pointer) — or, on the flag-set arm,
//! a bounds check with epilogue B (returns the bound) or a computed tail
//! jump (equivalent to a plain call plus return on every observed channel,
//! including the stack pointer). Dispatch-false and skip arms converge on
//! epilogue B or the next gate; all three gates skipped returns the last
//! compare result.
//!
//! Float comparisons replicate the compare-and-branch flag semantics
//! explicitly instead of `f32::min/max`, which differ on NaN (unordered
//! comparisons keep the old value here). The original compares in both
//! operand orders, so two mirror helpers exist; mixing them up fails
//! verification. Arithmetic operand order is source order (no reassociation
//! without fast-math flags); float results match bit for bit.
//!
//! Provenance: continued from lane r-b111's staged proof (same logic,
//! export renamed); the body was lifted unchanged into `run_viewer_tick`
//! so the wrong version shares every line except the flagged blend.
//!
//! Wrong version (`rw_aq10_93e0_mut`): one behavioural change, the first
//! blend of the float phase subtracts the held term instead of adding it.
//! It disturbs the ordered reduction and flips dispatch decisions, so every
//! stage that runs a float phase must fail against it.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// A virtual sample hook: thiscall with no stack args, float result on ST0.
type VSample = extern "thiscall" fn(u32) -> f32;

/// A virtual query hook: thiscall with no stack args, integer result in EAX.
type VHook = extern "thiscall" fn(u32) -> u32;

/// A virtual one-arg hook: thiscall with one stack arg, integer result.
type VHook1 = extern "thiscall" fn(u32, u32) -> u32;

/// Call the sample hook at vtable `slot` on `obj`, like the original.
#[inline(always)]
unsafe fn vsample(obj: u32, slot: usize) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(slot) as *const u32);
    let f: VSample = core::mem::transmute(target as usize);
    f(obj)
}

/// Call the query hook at vtable `slot` on `obj`, like the original.
#[inline(always)]
unsafe fn vhook(obj: u32, slot: usize) -> u32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(slot) as *const u32);
    let f: VHook = core::mem::transmute(target as usize);
    f(obj)
}

/// Call the one-arg hook at vtable `slot` on `obj`, like the original.
#[inline(always)]
unsafe fn vhook1(obj: u32, slot: usize, arg: u32) -> u32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(slot) as *const u32);
    let f: VHook1 = core::mem::transmute(target as usize);
    f(obj, arg)
}

/// The `comiss new,old` + `jbe keep` idiom: keep `old` unless `new` is
/// ordered-greater. NaN on either side keeps `old` (unordered jumps).
#[inline(always)]
fn unless_greater(new: f32, old: f32) -> f32 {
    if new > old {
        new
    } else {
        old
    }
}

/// The mirrored `comiss old,new` + `jbe keep` idiom: keep `old` unless `old`
/// itself is ordered-greater than `new`. NaN on either side keeps `old`.
#[inline(always)]
fn unless_old_greater(old: f32, new: f32) -> f32 {
    if old > new {
        new
    } else {
        old
    }
}

/// One 16-hook float phase with ordered reductions, clamped runtime
/// counters and the four-way dispatch. All three phases in the original
/// share this shape; returns the dispatch decision.
#[inline(always)]
unsafe fn float_phase(this: u32, worker: u32, mut_blend: bool) -> bool {
    let k = *global::<f32>(0xfe8830);
    let a = vsample(worker, 0xc8);
    let b = vsample(worker, 0xb8);
    let s_lo0 = a - b * k;
    let c = vsample(worker, 0xb8);
    let held0 = c * k;
    let d = vsample(worker, 0xc8);
    // MUTANT LINE: the wrong version takes the subtract arm.
    let s_hi0 = if mut_blend { d - held0 } else { d + held0 };
    let e = vsample(worker, 0xd0);
    let f = vsample(worker, 0xc0);
    let s_lo1 = e - f * k;
    let g = vsample(worker, 0xc0);
    let held1 = g * k;
    let h = vsample(worker, 0xd0);
    let s_hold = h + held1;
    let i = vsample(this, 0xc8);
    let j = vsample(this, 0xb8);
    let s_lo2 = i - j * k;
    let kk = vsample(this, 0xb8);
    let held2 = kk * k;
    let l = vsample(this, 0xc8);
    let s_hi1 = l + held2;
    let m = vsample(this, 0xd0);
    let n = vsample(this, 0xc0);
    let s_lo3 = m - n * k;
    let o = vsample(this, 0xc0);
    let held3 = o * k;
    let p = vsample(this, 0xd0);
    let y = p + held3;
    let x7 = unless_greater(s_lo2, s_lo0);
    let x6 = unless_old_greater(s_hi0, s_hi1);
    let x5 = unless_greater(s_lo3, s_lo1);
    let x4 = unless_old_greater(s_hold, y);
    let cap = *global::<f32>(0xfe88e8);
    let mut x0 = (*global::<i32>(0x18b7a8c) as f32) * *global::<f32>(0x17accf0);
    x0 = unless_greater(0.0, x0);
    if x0 > cap {
        x0 = cap;
    }
    let mut x1 = (*global::<i32>(0x18b7a80) as f32) * *global::<f32>(0x17acce8);
    x1 = unless_greater(0.0, x1);
    if x1 > cap {
        x1 = cap;
    }
    x1 > x7 && x6 > x1 && x0 > x5 && x4 > x0
}

/// Shared body: the correct rewrite runs it with `mut_blend = false`.
unsafe fn run_viewer_tick(this: u32, arg: u32, mut_blend: bool) -> u32 {
        // Entry: fetch the service object, prod its slot, fetch the worker
        // object selected by our stack argument, prod its slot 0.
        let svc: u32 =
            callee_thiscall!(1, u32, relocated(0x1981a4c), relocated(0xef332c));
        vhook(svc, 0x1b0);
        let worker: u32 =
            callee_thiscall!(3, u32, relocated(0x1981a4c), arg);
        let gate = vhook(worker, 0);
        let want: u32 = callee_cdecl!(5, u32, relocated(0xef3340));
        if gate == want {
            if float_phase(this, worker, mut_blend) {
                callee_thiscall!(20, u32, worker, 1);
            } else {
                vhook(this, 0x1a4);
                let other = *global::<u32>(0x18b6c8c);
                callee_thiscall!(22, u32, other);
            }
        }
        // Gate B.
        let gate_b = vhook(worker, 0);
        let want_b: u32 = callee_cdecl!(6, u32, relocated(0xef334c));
        if gate_b == want_b {
            if float_phase(this, worker, mut_blend) {
                // Dispatch taken: re-resolve the worker through its query
                // hook, then the flag-selected loop section.
                let q = vhook(worker, 0x54);
                let w2: u32 =
                    callee_thiscall!(3, u32, relocated(0x1981a4c), q);
                let flag = *((w2 as *const u8).add(0x1ec));
                if flag != 0 {
                    callee_thiscall!(24, u32, w2);
                    let arr = *((w2 as *const u32).add(0x1e4 / 4));
                    let mut taken = vhook(arr, 0x1d4);
                    if taken != 0 {
                        let mut esi = 0u32;
                        loop {
                            let view =
                                *((this as *const u32).add(0x1e0 / 4));
                            let inner =
                                *((view as *const u32).add(0x1ec / 4));
                            callee_thiscall!(26, u32, inner);
                            esi += 1;
                            taken = vhook(arr, 0x1d4);
                            if esi >= taken {
                                break;
                            }
                        }
                    }
                    let view = *((this as *const u32).add(0x1e0 / 4));
                    let inner = *((view as *const u32).add(0x1ec / 4));
                    callee_thiscall!(27, u32, inner, 1.0f32.to_bits());
                    vhook(view, 0x160);
                    *((view as *mut u8).add(0x218)) = 1;
                    let x = *((inner as *const u32).add(0x1f0 / 4));
                    let y = *((view as *const u32).add(0x1fc / 4));
                    if x < y {
                        // Tail jump: the original tears its frame down and
                        // jumps; a plain call plus return compares equal on
                        // every channel (proven by the tail1/tail2 runs).
                        return vhook1(inner, 0x120, 0);
                    }
                    return x;
                } else {
                    // Flag-clear arm: scan the element array, then the
                    // shared join and epilogue A.
                    let sview = *((this as *const u32).add(0x1e0 / 4));
                    let scan = *((sview as *const u32).add(0x1e0 / 4));
                    let mut ebp = 0u32;
                    let p0 = vhook(scan, 0x1d0);
                    let bound0 = *((p0 as *const u8).add(4) as *const u16);
                    if bound0 != 0 {
                        loop {
                            let p = vhook(scan, 0x1d0);
                            let q = *(p as *const u32);
                            let elem =
                                *((q as *const u32).add(ebp as usize));
                            let hv = vhook(elem, 0);
                            let cv: u32 =
                                callee_cdecl!(31, u32, relocated(0xef3364));
                            if hv == cv {
                                let ef = *((elem as *const u8).add(0x1ec));
                                if ef != 0 {
                                    callee_thiscall!(24, u32, elem);
                                    let arr2 =
                                        *((elem as *const u32).add(0x1e4 / 4));
                                    let mut n2 = vhook(arr2, 0x1d4);
                                    if n2 != 0 {
                                        let mut esi2 = 0u32;
                                        loop {
                                            let view2 = *((this as *const u32)
                                                .add(0x1e0 / 4));
                                            let inner2 = *((view2
                                                as *const u32)
                                                .add(0x1ec / 4));
                                            callee_thiscall!(26, u32, inner2);
                                            esi2 += 1;
                                            n2 = vhook(arr2, 0x1d4);
                                            if esi2 >= n2 {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            ebp += 1;
                            let p2 = vhook(scan, 0x1d0);
                            let bnd =
                                *((p2 as *const u8).add(4) as *const u16);
                            if (ebp as i32) >= (bnd as i32) {
                                break;
                            }
                        }
                    }
                    // Join: the original reloads the worker from its
                    // scribbled arg slot; the rewrite still holds it.
                    callee_thiscall!(32, u32, w2);
                    callee_thiscall!(
                        33,
                        u32,
                        relocated(0x1176888),
                        relocated(0xef3374)
                    );
                    let arr3 = *((w2 as *const u32).add(0x1e4 / 4));
                    let mut n3 = vhook(arr3, 0x1d4);
                    if n3 != 0 {
                        let mut esi3 = 0u32;
                        loop {
                            let view3 =
                                *((this as *const u32).add(0x1e0 / 4));
                            let inner3 =
                                *((view3 as *const u32).add(0x1ec / 4));
                            callee_thiscall!(34, u32, inner3);
                            esi3 += 1;
                            n3 = vhook(arr3, 0x1d4);
                            if esi3 >= n3 {
                                break;
                            }
                        }
                    }
                    // Second bounds check, then epilogue A.
                    let view4 = *((this as *const u32).add(0x1e0 / 4));
                    let inner4 = *((view4 as *const u32).add(0x1ec / 4));
                    let xx = *((inner4 as *const u32).add(0x1f0 / 4));
                    let yy = *((view4 as *const u32).add(0x1fc / 4));
                    let ep_arg = if xx >= yy {
                        1u32
                    } else {
                        callee_thiscall!(27, u32, inner4, 1.0f32.to_bits());
                        vhook(view4, 0x160);
                        0u32
                    };
                    vhook1(inner4, 0x120, ep_arg);
                    // Epilogue A returns the view pointer: the original
                    // reloads EAX from [this+0x1E0] after the call.
                    *((view4 as *mut u8).add(0x218)) = 1;
                    return view4;
                }
            } else {
                return vhook(this, 0x1a4);
            }
        }
        // Gate C. The taken arm mirrors gate B: third float phase,
        // flag-selected section, shared join (without the id33 call here)
        // and epilogue A, or the bounds check with epilogue B.
        let gate_c = vhook(worker, 0);
        let want_c: u32 = callee_cdecl!(7, u32, relocated(0xef3394));
        if gate_c == want_c {
            if float_phase(this, worker, mut_blend) {
                let q = vhook(worker, 0x54);
                let w2: u32 =
                    callee_thiscall!(3, u32, relocated(0x1981a4c), q);
                let flag = *((w2 as *const u8).add(0x1ec));
                if flag != 0 {
                    callee_thiscall!(24, u32, w2);
                    let arr = *((w2 as *const u32).add(0x1e4 / 4));
                    let mut taken = vhook(arr, 0x1d4);
                    if taken != 0 {
                        let mut esi = 0u32;
                        loop {
                            let view =
                                *((this as *const u32).add(0x1e0 / 4));
                            let inner =
                                *((view as *const u32).add(0x1ec / 4));
                            callee_thiscall!(26, u32, inner);
                            esi += 1;
                            taken = vhook(arr, 0x1d4);
                            if esi >= taken {
                                break;
                            }
                        }
                    }
                    let view = *((this as *const u32).add(0x1e0 / 4));
                    let inner = *((view as *const u32).add(0x1ec / 4));
                    callee_thiscall!(27, u32, inner, 1.0f32.to_bits());
                    vhook(view, 0x160);
                    *((view as *mut u8).add(0x218)) = 1;
                    let x = *((inner as *const u32).add(0x1f0 / 4));
                    let y = *((view as *const u32).add(0x1fc / 4));
                    if x < y {
                        // Second tail jump: plain call, as probed for the
                        // first (the checker equates both sides here).
                        return vhook1(inner, 0x120, 0);
                    }
                    return x;
                } else {
                    let sview = *((this as *const u32).add(0x1e0 / 4));
                    let scan = *((sview as *const u32).add(0x1e0 / 4));
                    let mut ebp = 0u32;
                    let p0 = vhook(scan, 0x1d0);
                    let bound0 = *((p0 as *const u8).add(4) as *const u16);
                    if bound0 != 0 {
                        loop {
                            let p = vhook(scan, 0x1d0);
                            let q = *(p as *const u32);
                            let elem =
                                *((q as *const u32).add(ebp as usize));
                            let hv = vhook(elem, 0);
                            let cv: u32 =
                                callee_cdecl!(36, u32, relocated(0xef33a4));
                            if hv == cv {
                                let ef = *((elem as *const u8).add(0x1ec));
                                if ef != 0 {
                                    callee_thiscall!(24, u32, elem);
                                    let arr2 =
                                        *((elem as *const u32).add(0x1e4 / 4));
                                    let mut n2 = vhook(arr2, 0x1d4);
                                    if n2 != 0 {
                                        let mut esi2 = 0u32;
                                        loop {
                                            let view2 = *((this as *const u32)
                                                .add(0x1e0 / 4));
                                            let inner2 = *((view2
                                                as *const u32)
                                                .add(0x1ec / 4));
                                            callee_thiscall!(26, u32, inner2);
                                            esi2 += 1;
                                            n2 = vhook(arr2, 0x1d4);
                                            if esi2 >= n2 {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            ebp += 1;
                            let p2 = vhook(scan, 0x1d0);
                            let bnd =
                                *((p2 as *const u8).add(4) as *const u16);
                            if (ebp as i32) >= (bnd as i32) {
                                break;
                            }
                        }
                    }
                    callee_thiscall!(32, u32, w2);
                    let arr3 = *((w2 as *const u32).add(0x1e4 / 4));
                    let mut n3 = vhook(arr3, 0x1d4);
                    if n3 != 0 {
                        let mut esi3 = 0u32;
                        loop {
                            let view3 =
                                *((this as *const u32).add(0x1e0 / 4));
                            let inner3 =
                                *((view3 as *const u32).add(0x1ec / 4));
                            callee_thiscall!(34, u32, inner3);
                            esi3 += 1;
                            n3 = vhook(arr3, 0x1d4);
                            if esi3 >= n3 {
                                break;
                            }
                        }
                    }
                    let view4 = *((this as *const u32).add(0x1e0 / 4));
                    let inner4 = *((view4 as *const u32).add(0x1ec / 4));
                    let xx = *((inner4 as *const u32).add(0x1f0 / 4));
                    let yy = *((view4 as *const u32).add(0x1fc / 4));
                    let ep_arg = if xx >= yy {
                        1u32
                    } else {
                        callee_thiscall!(27, u32, inner4, 1.0f32.to_bits());
                        vhook(view4, 0x160);
                        0u32
                    };
                    vhook1(inner4, 0x120, ep_arg);
                    *((view4 as *mut u8).add(0x218)) = 1;
                    return view4;
                }
            } else {
                return vhook(this, 0x1a4);
            }
        }
        want_c
}

export!(thiscall, rw_aq10_93e0(this: u32, arg: u32) -> u32 {
    unsafe { run_viewer_tick(this, arg, false) }
});

export!(thiscall, rw_aq10_93e0_mut(this: u32, arg: u32) -> u32 {
    unsafe { run_viewer_tick(this, arg, true) }
});
