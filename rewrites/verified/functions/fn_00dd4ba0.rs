// original: 0x00dd4ba0 UIMontageContainer::vf88

/// Long update routine for a montage UI container (thiscall, result in EAX).
///
/// Runs an armed-state prologue, a list synchronisation block, then either a
/// panel-selection block (two identity gates choose between a ranked pick, a
/// direct float path, or an idle guard) or nothing at all; the panel path
/// always continues into a float measurement chain with a threshold maze and
/// two tick-counter checks, while the idle path skips all of that and joins
/// at the tick check. After a counter-driven session refresh with a float
/// blend tail, a final probe value is scaled, chopped to an integer with x87
/// truncation semantics, and published to the list object when changed. The
/// routine returns the list object, or its parameter sub-object when the
/// published value changed.
///
/// Details the rewrite mirrors exactly: several calls take a leftover word
/// pushed for the lookup above them as an extra argument; two builder helpers
/// receive a never-written frame word (kept identical on both sides through
/// the contract's stack fill) and build their argument lists with explicit
/// stores after `(an instruction of the original)`; one frame word read before any write is fixed
/// nonzero by the same fill so its call block always runs; every float
/// comparison is written in the original's comiss+jbe/ja shape so NaN takes
/// the same path; the trailing conversion reproduces x87 `fistp` with forced
/// truncation including its indefinite value for NaN and out-of-range inputs.
///
/// Original: 0x00dd4ba0 (thiscall, `this` in ECX).
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, relocated};

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
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

/// Call `slot` of the object at `obj` with no stack arguments (thiscall).
#[inline(always)]
unsafe fn vcall0(obj: u32, slot: u32, this: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(this)
    }
}

/// Call `slot` of the object at `obj` with one stack argument (thiscall).
#[inline(always)]
unsafe fn vcall1(obj: u32, slot: u32, this: u32, a0: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(this, a0)
    }
}

const G88_STATE: u32 = 0x018B6C8C;
const G88_TICK_A: u32 = 0x018B7A8C;
const G88_TICK_B: u32 = 0x018B7A80;
const G88_SCALE_A: u32 = 0x017ACCF0;
const G88_SCALE_B: u32 = 0x017ACCE8;
const G88_DIVISOR: u32 = 0x01057B98;
const G88_COUNTER: u32 = 0x01173594;
const G88_CAP: u32 = 0x00FE88E8;
const G88_UNIT: u32 = 0x00FE8830;
const G88_FINAL_SCALE: u32 = 0x00EFB214;
const CTX88_A: u32 = 0x01981A4C;
const CTX88_B: u32 = 0x019D2F18;
const TAG88_A: u32 = 0x00EFACC4;
const TAG88_B: u32 = 0x00EFACE4;
const TAG88_C: u32 = 0x00EFACF4;
const TAG88_D: u32 = 0x00EFAD04;
const TAG88_E: u32 = 0x00EFAD14;
const TAG88_F: u32 = 0x00EFAD24;
const TAG88_G: u32 = 0x00EFAD30;
const TAG88_H: u32 = 0x00EFAD40;
const B88_LIST: u32 = 0x1E0;
const B88_AUX: u32 = 0x1E8;
const B88_CFG: u32 = 0x1EC;
const B88_OUT: u32 = 0x1F0;
const B88_VIEW: u32 = 0x1F4;
const B88_SRC: u32 = 0x1F8;
const B88_WANT: u32 = 0x204;
const B88_STAMP: u32 = 0x208;
const B88_SLOT: u32 = 0x20C;
const B88_MODE: u32 = 0x218;
const B88_KIND: u32 = 0x219;
const B88_TICK: u32 = 0x21A;
const B88_ARMED: u32 = 0x21B;
const B88_PHASE: u32 = 0x21C;
const B88_LEVEL: u32 = 0x220;
const L88_COUNT: u32 = 0x1FC;
const L88_LAST: u32 = 0x1F8;
const STACK_FILL88: u32 = 0x12345678;

#[inline(always)]
unsafe fn rd64(a: u32) -> u64 {
    unsafe { (a as *const u64).read_unaligned() }
}

#[inline(always)]
unsafe fn wr64(a: u32, v: u64) {
    unsafe { (a as *mut u64).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}

/// Call `slot` of the object at `obj` (thiscall, no stack args) with an x87
/// float result, for the `(an instruction of the original); (an instruction of the original)` sites.
#[inline(always)]
unsafe fn fcall0(obj: u32, slot: u32, this: u32) -> f32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(this)
    }
}

#[inline(always)]
fn fmul88(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn fadd88(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn fsub88(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn fdiv88(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

/// The trailing float-to-integer conversion: x87 `fistp qword` with the
/// control word forced to truncation. Out-of-range and NaN inputs produce the
/// x87 indefinite value rather than saturating.
fn fchop_to_i64(v: f32) -> i64 {
    const LIM: f32 = 9223372036854775808.0; // 2^63, exactly representable
    if v.is_nan() || v >= LIM || v < -LIM {
        i64::MIN
    } else {
        v as i64
    }
}


/// List synchronisation block. Returns the current index and the equality
/// flag the refresh block below reads back from the frame.
unsafe fn sync88(this: u32) -> (u32, bool) {
    unsafe {
        let list = rd32(this + B88_LIST);
        let slot = rd32(this + B88_SLOT);
        let active: u32 = callee_thiscall!(4, u32, relocated(CTX88_A), relocated(TAG88_A));
        wr8(active + 0x20B, 1);
        let _opened: u32 = callee_thiscall!(6, u32, relocated(CTX88_A), slot);
        // The store of that answer to the frame is dead (overwritten before
        // any read); only the calls below are observable.
        let mark: u32 = vcall1(list, 0x1E4, list, slot);
        let list2 = rd32(this + B88_LIST);
        if vcall0(list2, 0x1D4, list2) == 1 {
            wr8(this + B88_MODE, 1);
            let cfg = rd32(this + B88_CFG);
            wr8(cfg + 0x1F4, 1);
            let out = rd32(this + B88_OUT);
            let _: u32 = callee_thiscall!(7, u32, out, 1);
            let aux = rd32(this + B88_AUX);
            let _: u32 = vcall1(aux, 0x120, aux, 1);
            let _: u32 = vcall0(this, 0x1B0, this);
        }
        let list3 = rd32(this + B88_LIST);
        if vcall0(list3, 0x1D4, list3) > 1 {
            let sub = rd32(rd32(this + B88_LIST) + B88_LIST);
            if vcall0(sub, 0x21C, sub) == 0 {
                let list4 = rd32(this + B88_LIST);
                let probe: u32 = vcall1(list4, 0x1E0, list4, 1);
                let _: u32 = callee_thiscall!(8, u32, probe, 1);
            }
            let sub2 = rd32(rd32(this + B88_LIST) + B88_LIST);
            let prop: u32 = vcall0(sub2, 0x21C, sub2);
            let list5 = rd32(this + B88_LIST);
            if prop == vcall0(list5, 0x1D4, list5).wrapping_sub(1) {
                resync88(this);
            } else {
                let list6 = rd32(this + B88_LIST);
                if mark == vcall0(list6, 0x1D4, list6).wrapping_sub(1) {
                    resync88(this);
                }
            }
            let list7 = rd32(this + B88_LIST);
            let rank = vcall1(list7, 0x1E4, list7, slot) as i32;
            if rank > 0 {
                let list8 = rd32(this + B88_LIST);
                let rank2: u32 = vcall1(list8, 0x1E4, list8, slot);
                let elem: u32 = vcall1(list8, 0x1E0, list8, rank2.wrapping_sub(1));
                let _: u32 = callee_thiscall!(10, u32, elem, 0);
                let list9 = rd32(this + B88_LIST);
                // Two words are pushed for the lookup; it takes the top one
                // and the leftover zero becomes the second argument of the
                // helper call after it.
                let rank3: u32 = vcall1(list9, 0x1E4, list9, slot);
                let src = rd32(this + B88_SRC);
                let _: u32 = callee_thiscall!(11, u32, src, rank3.wrapping_sub(1), 0);
            }
            let list10 = rd32(this + B88_LIST);
            let ahead: u32 = vcall1(list10, 0x1E4, list10, slot);
            if (ahead as i32) < vcall0(list10, 0x1D4, list10).wrapping_sub(1) as i32 {
                let list11 = rd32(this + B88_LIST);
                let rank4: u32 = vcall1(list11, 0x1E4, list11, slot);
                let elem2: u32 = vcall1(list11, 0x1E0, list11, rank4.wrapping_add(1));
                let _: u32 = callee_thiscall!(12, u32, elem2, 0);
                let list12 = rd32(this + B88_LIST);
                let rank5: u32 = vcall1(list12, 0x1E4, list12, slot);
                let src2 = rd32(this + B88_SRC);
                let _: u32 = callee_thiscall!(13, u32, src2, rank5.wrapping_add(1), 0);
            }
        }
        // One frame word below was never written; the contract fills it with
        // a fixed nonzero word, so this call block always runs.
        debug_assert_ne!(STACK_FILL88, 0);
        {
            let src = rd32(this + B88_SRC);
            let _: u32 = callee_thiscall!(14, u32, src, mark);
            let src_arg = rd32(this + B88_SRC);
            let found: u32 = callee_stdcall!(15, u32, src_arg);
            let _: u32 = callee_thiscall!(16, u32, found);
        }
        let view = rd32(this + B88_VIEW);
        let panel: u32 = vcall0(view, 0xF8, view);
        let inner = rd32(panel);
        let cell = rd32(inner + 8);
        let tag: u32 = vcall0(this, 0x4C, this);
        wr32(cell + 4, tag);
        wr32(cell + 8, 0);
        let cfg = rd32(this + B88_CFG);
        let _: u32 = callee_thiscall!(17, u32, cfg);
        let list13 = rd32(this + B88_LIST);
        let cur: u32 = vcall1(list13, 0x1E4, list13, slot);
        let list14 = rd32(this + B88_LIST);
        let total: u32 = vcall0(list14, 0x1D4, list14);
        let eq_flag = cur.wrapping_add(1) == total;
        let sub3 = rd32(rd32(this + B88_LIST) + B88_LIST);
        let _: u32 = vcall0(sub3, 0x21C, sub3);
        (cur, eq_flag)
    }
}

/// Shared tail of the two resynchronisation paths.
unsafe fn resync88(this: u32) {
    unsafe {
        let list = rd32(this + B88_LIST);
        let n: u32 = vcall0(list, 0x1D4, list);
        let elem: u32 = vcall1(list, 0x1E0, list, n.wrapping_sub(2));
        let _: u32 = callee_thiscall!(9, u32, elem, 1);
    }
}

/// Phase dispatch and refresh join. `cur` is the current index saved above.
unsafe fn refresh88(this: u32, cur: u32, eq_flag: bool, skip_phase_clear: bool) {
    unsafe {
        let phase = rd8(this + B88_PHASE);
        if phase != 2 && phase != 3 {
            wr8(this + B88_ARMED, 1);
            let list = rd32(this + B88_LIST);
            let slot = rd32(this + B88_SLOT);
            let rank: u32 = vcall1(list, 0x1E4, list, slot);
            let _: u32 = callee_thiscall!(18, u32, list, rank);
            let cfg = rd32(this + B88_CFG);
            let _: u32 = callee_thiscall!(19, u32, cfg, cur, 0);
        } else {
            let want = rd32(this + B88_WANT) as i32;
            if want >= 0 {
                select88(this, want as u32);
            } else {
                let view = rd32(this + B88_VIEW);
                let panel: u32 = vcall0(view, 0xF8, view);
                let inner = rd32(panel);
                let found: u32 = callee_thiscall!(20, u32, rd32(inner + 8));
                let key: u32 = vcall0(found, 0x4C, found);
                let list = rd32(this + B88_LIST);
                let rank0: u32 = vcall1(list, 0x1E4, list, key);
                if rank0 as i32 >= 0 {
                    select88(this, rank0);
                }
            }
            let _: u32 = callee_thiscall!(2, u32, this);
            let cfg = rd32(this + B88_CFG);
            let _: u32 = callee_thiscall!(19, u32, cfg, cur, 1);
            let list = rd32(this + B88_LIST);
            if vcall0(list, 0x1D4, list) > 1 {
                let sub = rd32(rd32(this + B88_LIST) + B88_LIST);
                let prop: u32 = vcall0(sub, 0x21C, sub);
                let cfg2 = rd32(this + B88_CFG);
                let _: u32 = callee_thiscall!(22, u32, cfg2, prop);
            } else {
                let list2 = rd32(this + B88_LIST);
                if vcall0(list2, 0x1D4, list2) == 1 {
                    let cfg3 = rd32(this + B88_CFG);
                    let _: u32 = callee_thiscall!(22, u32, cfg3, 0);
                } else {
                    let cfg4 = rd32(this + B88_CFG);
                    let _: u32 = vcall0(cfg4, 0x1B0, cfg4);
                }
            }
        }
        let src = rd32(this + B88_SRC);
        let level: f32 = callee_thiscall!(23, f32, src);
        let blend: u32 = vcall1(this, 0x54, this, level.to_bits());
        let sess: u32 = callee_thiscall!(6, u32, relocated(CTX88_A), blend);
        let _: u32 = callee_thiscall!(24, u32, sess);
        let cfg = rd32(this + B88_CFG);
        let _: u32 = callee_thiscall!(3, u32, cfg);
        wr32(this + B88_SLOT, 0);
        let src2 = rd32(this + B88_SRC);
        if callee_thiscall!(25, u32, src2) as i32 > 0 {
            let sub = rd32(rd32(this + B88_LIST) + B88_LIST);
            let prop: u32 = vcall0(sub, 0x21C, sub);
            let src3 = rd32(this + B88_SRC);
            let arg = if eq_flag { prop } else { prop.wrapping_sub(1) };
            let val: f32 = callee_thiscall!(26, f32, src3, arg);
            let tag = if eq_flag { TAG88_B } else { TAG88_C };
            let active: u32 = callee_thiscall!(5, u32, relocated(CTX88_A), relocated(tag), val.to_bits());
            let _: u32 = callee_thiscall!(27, u32, active);
        } else {
            let active: u32 = callee_thiscall!(5, u32, relocated(CTX88_A), relocated(TAG88_D), 0);
            let _: u32 = callee_thiscall!(27, u32, active);
        }
        if !skip_phase_clear {
            wr8(this + B88_PHASE, 0);
        }
    }
}

/// Ranked-selection call shared by the two phase paths.
unsafe fn select88(this: u32, want: u32) {
    unsafe {
        let list = rd32(this + B88_LIST);
        let slot = rd32(this + B88_SLOT);
        // As in the sync block, the lookup takes the top word and the
        // leftover becomes the second argument of the helper after it.
        let rank: u32 = vcall1(list, 0x1E4, list, slot);
        let _: u32 = callee_thiscall!(21, u32, list, rank, want);
        wr8(list + 0x220, 1);
    }
}

/// Panel-selection block with its two identity gates. `frame` mirrors the
/// never-written frame word the two builder helpers receive.
unsafe fn panel88(this: u32, frame: &mut [u32; 2]) {
    unsafe {
        let state = rd32(lf_checker_rt::global::<u32>(G88_STATE) as u32);
        let flag = rd32(state + STATE_FLAG);
        let mut probe = 0u32;
        if flag != 0 {
            let sess: u32 = callee_thiscall!(6, u32, relocated(CTX88_A), flag);
            probe = vcall0(sess, 0, sess);
        }
        let list = rd32(this + B88_LIST);
        let want = rd32(this + B88_WANT);
        if (want as i32) < rd32(list + L88_COUNT) as i32 {
            return;
        }
        let gate1: u32 = callee_cdecl!(28, u32, relocated(TAG88_E));
        if probe != gate1 {
            let gate2: u32 = callee_cdecl!(28, u32, relocated(TAG88_F));
            if probe != gate2 {
                let phase = rd8(this + B88_PHASE);
                if phase != 1 && phase != 2 {
                    return;
                }
            }
        }
        pick88(this, frame);
    }
}

/// Panel pick for the equal and unequal index paths.
unsafe fn pick88(this: u32, frame: &mut [u32; 2]) {
    unsafe {
        let list = rd32(this + B88_LIST);
        let want = rd32(this + B88_WANT);
        let count = rd32(list + L88_COUNT);
        let frame_addr = frame.as_mut_ptr() as u32;
        if want == count {
            let elem: u32 = vcall1(list, 0x1E0, list, want);
            let view = rd32(this + B88_VIEW);
            let panel: u32 = vcall0(view, 0xF8, view);
            let inner = rd32(panel);
            let cell = rd32(inner + 8);
            let unit = rdf(lf_checker_rt::global::<u32>(G88_UNIT) as u32);
            let level = rdf(this + B88_LEVEL);
            if unit > level {
                if rd32(cell + 0xC) == 8 {
                    wr32(cell + 0xC, 2);
                    build88(cell, frame_addr, 0xC1200000, 0);
                }
            } else if rd32(this + B88_WANT) != rd32(list + L88_COUNT) {
                if rd32(cell + 0xC) == 2 {
                    wr32(cell + 0xC, 8);
                    build88(cell, frame_addr, 0, 0);
                }
            } else if rd32(cell + 0xC) == 8 {
                wr32(cell + 0xC, 2);
                build88(cell, frame_addr, 0xC1200000, 0);
            }
            let cfg = rd32(this + B88_CFG);
            let _: u32 = callee_thiscall!(32, u32, cfg, want, level.to_bits(), rd32(cell + 0xC));
            finish_common88(this, cell, elem);
        } else {
            let elem: u32 = vcall1(list, 0x1E0, list, want.wrapping_sub(1));
            let view = rd32(this + B88_VIEW);
            let panel: u32 = vcall0(view, 0xF8, view);
            let inner = rd32(panel);
            let cell = rd32(inner + 8);
            if rd32(cell + 0xC) == 2 {
                wr32(cell + 0xC, 8);
                let blob: u32 = callee_thiscall!(29, u32, frame_addr, 0, 0);
                wr64(cell + 0x10, rd64(blob));
                wr64(cell + 0x18, rd64(blob + 8));
                wr64(cell + 0x20, rd64(blob + 0x10));
                let _: u32 = callee_thiscall!(31, u32, frame_addr);
            }
            let level = rdf(this + B88_LEVEL);
            let cfg = rd32(this + B88_CFG);
            let _: u32 = callee_thiscall!(32, u32, cfg, want.wrapping_sub(1), level.to_bits(), rd32(cell + 0xC));
            finish_common88(this, cell, elem);
        }
    }
}

/// Builder pair shared by the equal-index paths: construct, attach, release.
unsafe fn build88(cell: u32, frame_addr: u32, a0: u32, a1: u32) {
    unsafe {
        let blob: u32 = callee_thiscall!(29, u32, frame_addr, a0, a1);
        let _: u32 = callee_thiscall!(30, u32, cell, blob);
        let _: u32 = callee_thiscall!(31, u32, frame_addr);
    }
}

/// Shared tail of both pick paths. `elem` is the picked element.
unsafe fn finish_common88(this: u32, cell: u32, elem: u32) {
    unsafe {
        let tag: u32 = vcall0(elem, 0x4C, elem);
        wr32(cell + 4, tag);
        wr32(cell + 8, 0);
        let view = rd32(this + B88_VIEW);
        let _: u32 = vcall1(view, 0x120, view, 1);
        let _: u32 = vcall1(view, 0x118, view, 1);
        let sub = rd32(rd32(this + B88_CFG) + 0x1E4);
        let _: u32 = vcall1(sub, 0x118, sub, 1);
        let _: u32 = vcall1(sub, 0x120, sub, 1);
    }
}

/// Float measurement chain and threshold maze. Every comparison is written in
/// the original's comiss+jbe/ja shape so NaN takes the same path.
unsafe fn measure88(this: u32) {
    unsafe {
        let cap = rdf(lf_checker_rt::global::<u32>(G88_CAP) as u32);
        let unit = rdf(lf_checker_rt::global::<u32>(G88_UNIT) as u32);
        let tick_a = rd32(lf_checker_rt::global::<u32>(G88_TICK_A) as u32) as i32 as f32;
        let mut lo = fmul88(tick_a, rdf(lf_checker_rt::global::<u32>(G88_SCALE_A) as u32));
        let mut m1c = lo;
        if 0.0 > lo {
            m1c = 0.0;
        }
        if lo > cap {
            m1c = cap;
        }
        let tick_b = rd32(lf_checker_rt::global::<u32>(G88_TICK_B) as u32) as i32 as f32;
        let hi_in = fmul88(tick_b, rdf(lf_checker_rt::global::<u32>(G88_SCALE_B) as u32));
        let mut m18 = 0.0f32;
        if !(0.0 > hi_in) {
            m18 = if hi_in > cap { cap } else { hi_in };
        }
        let list = rd32(this + B88_LIST);
        let view = rd32(this + B88_VIEW);
        let m_c8 = fcall0(list, 0xC8, list);
        let m_b8 = fcall0(list, 0xB8, list);
        let m20 = fsub88(m_c8, fmul88(m_b8, unit));
        let m_b8b = fcall0(list, 0xB8, list);
        let m14 = fmul88(m_b8b, unit);
        let m_c8b = fcall0(list, 0xC8, list);
        let mut m24 = fadd88(m_c8b, m14);
        let m_c0 = fcall0(view, 0xC0, view);
        let m28a = fmul88(m_c0, unit);
        let m_d0 = fcall0(view, 0xD0, view);
        let m2c = fadd88(m_d0, m28a);
        let m_d0b = fcall0(list, 0xD0, list);
        let m_c0b = fcall0(list, 0xC0, list);
        let m28 = fsub88(m_d0b, fmul88(m_c0b, unit));
        let state = rd32(lf_checker_rt::global::<u32>(G88_STATE) as u32);
        let signal: u32 = callee_thiscall!(33, u32, state);
        maze88(this, m1c, m18, m20, m24, m28, m2c, signal);
    }
}

/// Threshold maze over the measured values.
#[allow(clippy::too_many_arguments)]
unsafe fn maze88(
    this: u32, m1c: f32, m18: f32, m20: f32, m24: f32, m28: f32, m2c: f32,
    signal: u32,
) {
    unsafe {
        let mut flag14 = (signal & 0xFF) as u8;
        let mut cl = flag14;
        let al = u8::from(m1c > m28 && m2c > m1c);
        let flag13 = al;
        if cl == 0 && al == 1 && m18 > m20 {
            let list = rd32(this + B88_LIST);
            let t = fcall0(list, 0xB8, list);
            let divisor = rdf(lf_checker_rt::global::<u32>(G88_DIVISOR) as u32);
            let x0 = fadd88(fdiv88(t, divisor), m20);
            if x0 > m18 {
                flag14 = 1;
            } else {
                cl = flag14;
                flag14 = 0;
                if cl != 0 {
                    cl = 0;
                    maze_tail88(this, flag14, cl);
                    return;
                }
            }
        } else {
            flag14 = 0;
            if cl != 0 {
                cl = 0;
                maze_tail88(this, flag14, cl);
                return;
            }
        }
        if flag13 == 0 {
            maze_tail88(this, flag14, 0);
            return;
        }
        if m24 > m18 {
            let list = rd32(this + B88_LIST);
            let t2 = fcall0(list, 0xB8, list);
            let divisor = rdf(lf_checker_rt::global::<u32>(G88_DIVISOR) as u32);
            let x1 = fsub88(m24, fdiv88(t2, divisor));
            if m18 > x1 {
                maze_tail88(this, flag14, 1);
                return;
            }
        }
        maze_tail88(this, flag14, 0);
    }
}

/// Maze exit: stamp/kind update then the two counter checks.
unsafe fn maze_tail88(this: u32, flag14: u8, cl: u8) {
    unsafe {
        if flag14 != 0 {
            counter_d88(this);
            return;
        }
        let mut cl = cl;
        if cl == 0 {
            let counter = rd32(lf_checker_rt::global::<u32>(G88_COUNTER) as u32);
            wr32(this + B88_STAMP, counter);
            wr8(this + B88_KIND, cl);
        }
        let kind = rd8(this + B88_KIND);
        if kind == 1 {
            counter_d88(this);
            return;
        }
        if cl != 0 || kind == 2 {
            counter_c88(this);
        }
    }
}

/// First counter check: refresh when the counter ran past the stamp.
unsafe fn counter_c88(this: u32) {
    unsafe {
        let counter = rd32(lf_checker_rt::global::<u32>(G88_COUNTER) as u32);
        if counter.wrapping_sub(rd32(this + B88_STAMP)) > 0x14 {
            let sub = rd32(rd32(this + B88_LIST) + 0x21C);
            let _: u32 = callee_thiscall!(34, u32, sub, 0x42200000);
            wr32(this + B88_STAMP, counter);
            wr8(this + B88_KIND, 2);
        }
    }
}

/// Second counter check: same shape, other helper and kind.
unsafe fn counter_d88(this: u32) {
    unsafe {
        let counter = rd32(lf_checker_rt::global::<u32>(G88_COUNTER) as u32);
        if counter.wrapping_sub(rd32(this + B88_STAMP)) > 0x14 {
            let sub = rd32(rd32(this + B88_LIST) + 0x21C);
            let _: u32 = callee_thiscall!(35, u32, sub, 0x42200000);
            wr32(this + B88_STAMP, counter);
            wr8(this + B88_KIND, 1);
        }
    }
}

/// Idle-guard block reached when the phase is clear and the list is quiet.
unsafe fn idle88(this: u32) {
    unsafe {
        let view = rd32(this + B88_VIEW);
        if vcall0(view, 0x124, view) as u8 != 0 {
            let _: u32 = vcall1(view, 0x120, view, 0);
            let _: u32 = vcall1(view, 0x118, view, 0);
            let sub = rd32(rd32(this + B88_CFG) + 0x1E4);
            let _: u32 = vcall1(sub, 0x120, sub, 0);
            let _: u32 = vcall1(sub, 0x118, sub, 0);
        }
    }
}

/// Tick block: counter-driven session refresh with a float blend tail.
unsafe fn tick88(this: u32) {
    unsafe {
        let cfg = rd32(this + B88_CFG);
        wr8(this + B88_TICK, 0);
        let _: u32 = callee_thiscall!(3, u32, cfg);
        let probe: u32 = callee_thiscall!(36, u32, relocated(CTX88_B));
        if probe as i32 >= 0 {
            let active: u32 = callee_thiscall!(4, u32, relocated(CTX88_A), relocated(TAG88_G));
            let _: u32 = vcall0(active, 0x1B0, active);
            let _: u32 = vcall0(this, 0x1AC, this);
            let list = rd32(this + B88_LIST);
            if probe == 0 {
                let _: u32 = callee_thiscall!(37, u32, list, 0);
                let sub = rd32(rd32(this + B88_LIST) + B88_LIST);
                let q1: u32 = vcall0(sub, 0x220, sub);
                let _: u32 = vcall0(q1, 0x1AC, q1);
                let sub2 = rd32(rd32(this + B88_LIST) + B88_LIST);
                let q2: u32 = vcall0(sub2, 0x220, sub2);
                let _: u32 = vcall1(q2, 0x18, q2, 1);
            } else {
                let _: u32 = callee_thiscall!(38, u32, list, probe, 1);
            }
            let cfg2 = rd32(this + B88_CFG);
            let _: u32 = callee_thiscall!(22, u32, cfg2, probe);
        }
        let probe2: u32 = callee_thiscall!(36, u32, relocated(CTX88_B));
        let src = rd32(this + B88_SRC);
        let val: f32 = callee_thiscall!(26, f32, src, probe2);
        let active: u32 = callee_thiscall!(5, u32, relocated(CTX88_A), relocated(TAG88_H), val.to_bits());
        let _: u32 = callee_thiscall!(27, u32, active);
    }
}

unsafe fn body88(this: u32, skip_phase_clear: bool) -> u32 {
    unsafe {
        let mut frame = [STACK_FILL88; 2];
        if rd8(this + B88_ARMED) != 0 {
            let list = rd32(this + B88_LIST);
            if callee_thiscall!(1, u32, list) as u8 == 0 {
                wr8(this + B88_ARMED, 0);
                let _: u32 = callee_thiscall!(2, u32, this);
                let cfg = rd32(this + B88_CFG);
                let _: u32 = callee_thiscall!(3, u32, cfg);
            }
        }
        if rd32(this + B88_SLOT) != 0 {
            let (cur, eq_flag) = sync88(this);
            refresh88(this, cur, eq_flag, skip_phase_clear);
        }
        // The panel path always continues into the measurement block; the
        // idle path skips it and joins at the tick check below.
        let state = rd32(lf_checker_rt::global::<u32>(G88_STATE) as u32);
        if rd32(state + STATE_FLAG) != 0 {
            let list = rd32(this + B88_LIST);
            if vcall0(list, 0x1D4, list) != 0 {
                panel88(this, &mut frame);
                measure88(this);
            } else if rd8(this + B88_PHASE) != 0 {
                panel88(this, &mut frame);
                measure88(this);
            } else {
                idle88(this);
            }
        } else if rd8(this + B88_PHASE) != 0 {
            panel88(this, &mut frame);
            measure88(this);
        } else {
            idle88(this);
        }
        if rd8(this + B88_TICK) > 6 {
            tick88(this);
        }
        let tick = rd8(this + B88_TICK);
        if tick != 0 {
            wr8(this + B88_TICK, tick.wrapping_add(1));
        }
        // Trailing conversion: scale the probe, chop to integer, publish.
        let list = rd32(this + B88_LIST);
        let probe = fcall0(list, 0x8C, list);
        let scaled = fmul88(probe, rdf(lf_checker_rt::global::<u32>(G88_FINAL_SCALE) as u32));
        let chopped = fchop_to_i64(scaled) as u32;
        if chopped != rd32(list + L88_LAST) {
            wr32(list + L88_LAST, chopped);
            let sub = rd32(list + 0x21C);
            wr32(sub + 0x1F4, chopped);
            return sub;
        }
        list
    }
}

lf_checker_rt::export!(thiscall, rw_00dd4ba0(this: u32) -> u32 {
    unsafe { body88(this, false) }
});
