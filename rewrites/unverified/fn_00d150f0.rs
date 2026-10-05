// original: 0x00d150f0 CTaskComplexCombat::vf5 (symbols)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated};
//
// State update for a complex combat task. `this` is the task object, `a1` is
// the actor, `a2` is a small mode and `a3` is a task object (the original
// keeps `a1` in edi and `a3` in esi; `a3`'s stack slot is reused as scratch
// afterwards, so the contract switches the stack comparison off). Returns 1
// when the task stays active, 0 when it ends this tick.
//
// Behaviour. Guard: when `a3` is non-null, its type id (virtual slot +4)
// is 0x31, 0x74 or 9, its target (slot +0x34) is this task's target
// (+0x3c) and the readiness check (callee 3) passes, flag 0x4000 is set on
// the task state (+0x60, clearing bits 0x28). Dispatch: mode 2 or a null
// `a3` goes straight to the tail. Otherwise the type id selects one of six
// switch targets (decoded from the two tables past the function end):
// 2/11 end the task (return 0); 79 ends it unless state bit 0x400 is set;
// 9 walks the child chain at +0x224 (virtual slots +0x20/+0xc); 36/103 and
// 15/31/41/49/116 share the main path, with a flag cleared for the first
// pair; every other id goes to the tail.
// Main path: the target's crowd flags (+0x28, masked 0x3c0) must read 0xc0
// and the target must still match; then a per-target check (callee 5), a
// stance check (callee 6, must answer below 2), and, for type 0x31/0x74, a
// table lookup (callee 7) feeding two parameter calls (callees 8, 9).
// A random tick (callee 10) scaled by a fixed step feeds three threshold
// comparisons (0.5, 0.33, 0.66) that select one of three parameter blocks
// for the pose call (callee 12); a status check (slot +0xc on the +8
// object) may then latch a byte. The path joins a final child check: an
// empty child ends the task (return 0, bumping the child's refcount at
// +4), otherwise the finish call (callee 14) runs and the tail follows.
// Tail: unless latched, a virtual approve call (slot +0x14 on the +8
// object) runs with the actor, the mode and the task; refusal ends it.
// word is maintained and state bit 0x400 is set/cleared from whether the
// type id is 0x4f, and 1 is returned.
// Original convention: thiscall (this in ECX, three stack words).
lf_checker_rt::export!(thiscall, rw_00D150F0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x3c;
        const STATE: u32 = 0x60;
        const VT_TYPE: u32 = 0x04;
        const VT_STATUS: u32 = 0x0c;
        const VT_CHILD: u32 = 0x20;
        const VT_TARGET: u32 = 0x34;
        const READY_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(obj)
            }
        }

        // Guard.
        if a3 != 0 {
            let tgt = vcall0(a3, VT_TARGET);
            if tgt == rd32(this + TARGET) {
                let t = vcall0(a3, VT_TYPE);
                let pass = t == 0x31
                    || {
                        let t2 = vcall0(a3, VT_TYPE);
                        t2 == 0x74 || {
                            let t3 = vcall0(a3, VT_TYPE);
                            t3 == 9
                        }
                    };
                if pass {
                    let ok: u32 = lf_checker_rt::callee_thiscall!(READY_CALLEE, u32, a1);
                    if (ok as u8) != 0 {
                        let st = rd32(this + STATE);
                        if st & 0x4000 == 0 {
                            unsafe {
                                ((this + STATE) as *mut u32)
                                    .write_unaligned((st & 0xffffffd7) | 0x4000)
                            };
                        }
                    }
                }
            }
        }
        // Dispatch.
        if a2 == 2 || a3 == 0 {
            return tail(this, a1, a3, a2);
        }
        let mut flag: u8 = 1;
        let t = vcall0(a3, VT_TYPE);
        // Table decode: default unless t-2 fits in the 0x73-entry table.
        let case: u8 = if t.wrapping_sub(2) > 0x72 {
            0
        } else {
            match t {
                2 | 11 => 1,
                79 => 2,
                9 => 3,
                36 | 103 => {
                    flag = 0;
                    4
                }
                15 | 31 | 41 | 49 | 116 => 4,
                _ => 0,
            }
        };
        match case {
            // End-of-task returns run `(an instruction of the original)` with the small switch-table
            // index still in eax, so they yield exactly 0.
            1 => return 0,
            2 => {
                if rd32(this + STATE) & 0x400 == 0 {
                    return tail(this, a1, a3, a2);
                }
                return 0;
            }
            3 => {
                let c = rd32(a1 + 0x224);
                let v = vcall0(c, VT_CHILD);
                if rd32(v + 0xc) != 0 {
                    let c2 = rd32(a1 + 0x224);
                    let v2 = vcall0(c2, VT_CHILD);
                    let o = rd32(v2 + 0xc);
                    if vcall0(o, VT_STATUS) != 0x76c {
                        let c3 = rd32(a1 + 0x224);
                        let v3 = vcall0(c3, VT_CHILD);
                        let o3 = rd32(v3 + 0xc);
                        if vcall0(o3, VT_STATUS) != 0x16c {
                            return tail(this, a1, a3, a2);
                        }
                    }
                }
                let c4 = rd32(a1 + 0x224);
                let v4 = vcall0(c4, VT_CHILD);
                if rd32(v4 + 8) == 0 {
                    unsafe {
                        ((a3 + 4) as *mut u32)
                            .write_unaligned(rd32(a3 + 4).wrapping_add(1))
                    };
                    return v4 & 0xffffff00;
                }
                let c5 = rd32(a1 + 0x224);
                let v5 = vcall0(c5, VT_CHILD);
                let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, v5);
                return tail(this, a1, a3, a2);
            }
            4 => {
                let ans = vcall0(a3, VT_TARGET);
                if rd32(ans + 0x28) & 0x3c0 != 0xc0 {
                    return child_check(this, a1, a3, a2);
                }
                let ebp = rd32(this + TARGET);
                if vcall0(a3, VT_TARGET) != ebp {
                    return child_check(this, a1, a3, a2);
                }
                if flag == 0 {
                    return child_check(this, a1, a3, a2);
                }
                if ebp == 0 {
                    return tick_path(this, a1, a3, a2);
                }
                let per: u32 = lf_checker_rt::callee_thiscall!(5, u32, ebp);
                if (per as u8) == 0 {
                    return tick_path(this, a1, a3, a2);
                }
                let w = rd32(a1 + 0x21c);
                if rd32(w + 0x12c) != 2 {
                    return tick_path(this, a1, a3, a2);
                }
                let q = rd32(ebp + 0x228);
                let qarg = if q != 0 { q.wrapping_add(0x70) } else { 0 };
                let r: u32 = lf_checker_rt::callee_thiscall!(6, u32, qarg);
                if (r as i32) >= 2 {
                    return tick_path(this, a1, a3, a2);
                }
                let t2 = vcall0(a3, VT_TYPE);
                let deep = t2 == 0x31 || vcall0(a3, VT_TYPE) == 0x74;
                if deep {
                    let p: u32 = lf_checker_rt::callee_cdecl!(7, u32, rd32(a3 + 0x48));
                    let kind = if rd32(p + 0xc) == 4 { 0x1eu32 } else { 0x0e };
                    let base = rd32(rd32(this + TARGET) + 0x20);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        8,
                        u32,
                        lf_checker_rt::relocated(0x128aa90),
                        base.wrapping_add(0x30),
                        kind,
                        0
                    );
                }
                let edx = rd32(this + TARGET);
                let e = rd32(edx + 0x228);
                let earg = if e != 0 { e.wrapping_add(0x70) } else { 0 };
                let base2 = rd32(edx + 0x20);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    9,
                    u32,
                    earg,
                    base2.wrapping_add(0x30),
                    2,
                    0,
                    1
                );
                return tick_path(this, a1, a3, a2);
            }
            _ => return tail(this, a1, a3, a2),
        }

        /// Shared tail of the main path: scaled tick, threshold ladder,
        /// status latch, then the child check.
        unsafe fn tick_path(this: u32, a1: u32, a3: u32, a2: u32) -> u32 {
            unsafe {
                const TICK_CALLEE: u32 = 10;
                const STEP: f32 = f32::from_bits(0x38000100);
                const TH0: f32 = f32::from_bits(0x3f000000);
                const TH1: f32 = f32::from_bits(0x3ea8f5c3);
                const TH2: f32 = f32::from_bits(0x3f28f5c3);
                const ONE_BITS: u32 = 0x3f800000;
                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn rd8(a: u32) -> u8 {
                    unsafe { (a as *const u8).read() }
                }
                #[inline(always)]
                fn mul(a: f32, b: f32) -> f32 {
                    core::hint::black_box(a) * core::hint::black_box(b)
                }
                #[inline(always)]
                fn below_eq(a: f32, b: f32) -> bool {
                    !(core::hint::black_box(a) > core::hint::black_box(b))
                }
                #[inline(always)]
                unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
                    unsafe {
                        let vt = rd32(obj);
                        let f: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt + slot) as usize);
                        f(obj)
                    }
                }
                unsafe fn pose(a1: u32, this: u32, tag: u32) {
                    unsafe {
                        #[inline(always)]
                        unsafe fn rd32(a: u32) -> u32 {
                            unsafe { (a as *const u32).read_unaligned() }
                        }
                        // The tag is a relocated image address in the original.
                        let tag = lf_checker_rt::relocated(tag);
                        let d: u32 = lf_checker_rt::callee_cdecl!(11, u32, rd32(this + 0x3c));
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            12, u32, a1.wrapping_add(0x570), tag, 0, 0, 0, 0xffffffff, 0, 0,
                            ONE_BITS, d, 0
                        );
                    }
                }
                let m = rd32(this + 0x60) & 0xffffefff;
                unsafe { ((this + 0x60) as *mut u32).write_unaligned(m) };
                let tick: u32 = lf_checker_rt::callee_cdecl!(TICK_CALLEE, u32,);
                let x = mul((tick as i32) as f32, STEP);
                if rd8(a1 + 0x26c) & 4 == 0 {
                    if !below_eq(TH0, x) {
                        let child = rd32(this + 8);
                        if child == 0 || vcall0(child, 0x0c) != 0x76f {
                            if !below_eq(TH1, x) {
                                let w = rd32(a1 + 0x21c);
                                if rd32(w + 0x12c) == 2 {
                                    pose(a1, this, 0xedf4bc);
                                }
                            } else if !below_eq(TH2, x) {
                                if rd8(a1 + 0x26c) & 4 == 0 {
                                    pose(a1, this, 0xedf4c8);
                                }
                            }
                            // The fourth block is unreachable: it needs the
                            // flag flipped with no call between the tests.
                            return status_latch(this, a1, a3, a2);
                        }
                        let w = rd32(a1 + 0x21c);
                        if rd32(w + 0x12c) == 2 {
                            pose(a1, this, 0xedf4b0);
                        }
                    }
                }
                status_latch(this, a1, a3, a2)
            }
        }

        /// Status latch plus the joining child check.
        unsafe fn status_latch(this: u32, a1: u32, a3: u32, a2: u32) -> u32 {
            unsafe {
                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
                    unsafe {
                        let vt = rd32(obj);
                        let f: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt + slot) as usize);
                        f(obj)
                    }
                }
                let child = rd32(this + 8);
                if vcall0(child, 0x0c) == 0x774 {
                    let o = rd32(a1 + 0xb30);
                    if o != 0 {
                        let ok: u32 = lf_checker_rt::callee_thiscall!(13, u32, o, a1);
                        if (ok as u8) != 0 {
                            unsafe { ((child + 0x4c) as *mut u8).write(1) };
                        }
                    }
                }
                child_check(this, a1, a3, a2)
            }
        }

        /// Joining child check: empty child ends the task, else finish + tail.
        unsafe fn child_check(this: u32, a1: u32, a3: u32, a2: u32) -> u32 {
            unsafe {
                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
                    unsafe {
                        let vt = rd32(obj);
                        let f: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt + slot) as usize);
                        f(obj)
                    }
                }
                let c = rd32(a1 + 0x224);
                let v = vcall0(c, 0x20);
                if rd32(v + 8) != 0 {
                    let c2 = rd32(a1 + 0x224);
                    let v2 = vcall0(c2, 0x20);
                    let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, v2);
                    return tail(this, a1, a3, a2);
                }
                unsafe {
                    ((a3 + 4) as *mut u32).write_unaligned(rd32(a3 + 4).wrapping_add(1))
                };
                v & 0xffffff00
            }
        }

        /// Approve-call tail. `mode` is a2 on the fall-in paths.
        unsafe fn tail(this: u32, a1: u32, a3: u32, mode: u32) -> u32 {
            unsafe {
                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn rd8(a: u32) -> u8 {
                    unsafe { (a as *const u8).read() }
                }
                #[inline(always)]
                unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
                    unsafe {
                        let vt = rd32(obj);
                        let f: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt + slot) as usize);
                        f(obj)
                    }
                }
                // The original re-reads a2 from its own stack frame here;
                // every path carries the incoming a2 unchanged, so the
                // rewrite threads it as `mode`.
                let ebp = rd32(this + 8);
                if rd8(ebp + 0xc) & 1 == 0 {
                    let vt = rd32(ebp);
                    let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(rd32(vt + 0x14) as usize);
                    let ok = f(ebp, a1, mode, a3);
                    if (ok as u8) == 0 {
                        // `(an instruction of the original)` keeps the upper bytes of the answer.
                        return ok & 0xffffff00;
                    }
                    let c = rd32(ebp + 0xc) | 2;
                    unsafe { ((ebp + 0xc) as *mut u32).write_unaligned(c) };
                }
                if rd32(a1 + 0xd68) != 0 {
                    let f = rd32(a1 + 0x26c) & 0xfffff7ff;
                    unsafe { ((a1 + 0x26c) as *mut u32).write_unaligned(f) };
                }
                let bit = if a3 != 0 && vcall0(a3, 0x04) == 0x4f { 1u32 } else { 0 };
                let m = rd32(this + 0x60);
                let w = ((bit << 10) ^ m) & 0x400;
                unsafe { ((this + 0x60) as *mut u32).write_unaligned(m ^ w) };
                // (an instruction of the original).
                w | 1
            }
        }

    }
});
