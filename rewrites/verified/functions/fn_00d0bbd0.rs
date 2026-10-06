// original: 0x00d0bbd0 DRAW_GUN
// Rewrite of DRAW_GUN. The full specification is the doc comment on the
// export below.
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, global, relocated};

// ---------------------------------------------------------------------------
// Named addresses, offsets and magic values.
// ---------------------------------------------------------------------------

const G_POOL: u32 = 0x0167_E2A0;
/// Byte tested in case 0 (planted per-trial through its dword).
const G_FLAG_DWORD: u32 = 0x0171_DE5C;
/// Read-only float constants (addresses; values read as bit patterns).
const F_4: u32 = 0x00FE_8AB8; // 4.0
const F_033: u32 = 0x00FE_8800; // 0.33
const F_C1: u32 = 0x00FE_8684;
const F_C2: u32 = 0x00FE_87D8; // 0.2
const F_C3: u32 = 0x00FE_88BC; // 0.9
const F_900: u32 = 0x00E9_CAE0; // 900.0

const HUNDRED_F32: u32 = 0x42C8_0000;
const ONE_F32: u32 = 0x3F80_0000;
const FIVE_F32: u32 = 0x40A0_0000;
const ONE_HALF_F32: u32 = 0x3FC0_0000;
const TWO_HALF_F32: u32 = 0x4020_0000;
const TEN_F32: u32 = 0x4120_0000;

// Gun ids.
const GUN_FIRST: u32 = 0x76E;
const GUN_LAST: u32 = 0x779;
const GUN_SUB: u32 = 0x76D;
const GUN_C8: u32 = 0xC8;
const GUN_13F: u32 = 0x13F;

// Small-object (`this`) layout.
const THIS_SUB: u32 = 0x8;
const THIS_LINK: u32 = 0x3C;
const THIS_AUX: u32 = 0x54;
const THIS_RATE: u32 = 0x58;
const THIS_FLAGS: u32 = 0x60;
const THIS_F1: u32 = 0x6C;
const THIS_F2: u32 = 0x70;

// Ped object layout.
const PED_POS: u32 = 0x20;
const PED_AUX: u32 = 0x224;
const PED_FLAGS: u32 = 0x264;
const PED_MODE: u32 = 0x26C;
const PED_SLOT: u32 = 0x2B0;
const PED_HELPER: u32 = 0x570;
const PED_BIG: u32 = 0xB30;
const PED_INNER: u32 = 0xD68;
const PED_W: u32 = 0x21C;

#[inline(always)]
fn rd_u32(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
}
#[inline(always)]
fn wr_u32(base: u32, off: u32, v: u32) {
    unsafe { ((base.wrapping_add(off)) as *mut u32).write_unaligned(v) }
}
#[inline(always)]
fn rd_u8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}
#[inline(always)]
fn wr_u8(base: u32, off: u32, v: u8) {
    unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
}
#[inline(always)]
fn rd_f(base: u32, off: u32) -> f32 {
    f32::from_bits(rd_u32(base, off))
}
/// Zero-argument cdecl call (the `callee_cdecl!` macro needs at least one
/// argument, so this mirrors its expansion directly).
#[inline(always)]
fn call_cdecl_0(id: u32) -> u32 {
    let f: extern "cdecl" fn() -> u32 =
        unsafe { core::mem::transmute(lf_checker_rt::callee_addr(id) as usize) };
    f()
}

// ---------------------------------------------------------------------------
// Bit-exact SSE scalar arithmetic. Each helper is one SSE instruction on
// the low lane, matching the original's opcode and operand order exactly;
// the Rust compiler cannot reassociate across these calls. Comparisons
// use `!(a > b)` / `!(a >= b)`, which match `comiss`+`jbe`/`jb` in every
// case including NaN (unordered takes the branch), regardless of which
// instruction the compiler chooses for the comparison itself.
// ---------------------------------------------------------------------------

#[target_feature(enable = "sse2")]
unsafe fn sse_sub(a: f32, b: f32) -> f32 {
    use core::arch::x86::*;
    _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(a), _mm_set_ss(b)))
}
#[target_feature(enable = "sse2")]
unsafe fn sse_mul(a: f32, b: f32) -> f32 {
    use core::arch::x86::*;
    _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(a), _mm_set_ss(b)))
}
#[target_feature(enable = "sse2")]
unsafe fn sse_add(a: f32, b: f32) -> f32 {
    use core::arch::x86::*;
    _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(a), _mm_set_ss(b)))
}
#[target_feature(enable = "sse2")]
unsafe fn sse_cvt_i32(x: u32) -> f32 {
    use core::arch::x86::*;
    _mm_cvtss_f32(_mm_cvtepi32_ps(_mm_set1_epi32(x as i32)))
}

/// Build the gun task for `gunid`, or return the entry task when the
/// ped already has what the entry block constructs.
///
/// Signature: `thiscall(this, gunid, ped) -> task` (two stack words,
/// callee cleans 8). Returns a task from the pool, or null/zero when the
/// allocator fails on a path that tolerates it.
///
/// Behaviour. The loop head clears the `0x200` flag bit, zeroes the
/// result slot and the rate slot, then (unless the gun id is one of four
/// special values or the ped gates fail) allocates and constructs the
/// entry task. When that task exists it is returned at once; otherwise
/// the id is dispatched: ids `0x76E..=0x779` index a 12-way jump table,
/// `0x76D`/`0xC8`/`0x13F` take dedicated tails, anything else returns the
/// (null) result slot. Case 0 can rewrite the id to `0x779` and loop back
/// (the id slot doubles as loop-carried state, which is why the stack
/// comparison is off for this function: the value is observed through the
/// calls the next iteration makes instead).
///
/// Floating point: two squared-distance sums (`subss`/`mulss`/`addss`)
/// and two integer-scaled chains (`cvtdq2ps`/`mulss`/`addss`) run through
/// the SSE helpers above in the original's exact operand order; the only
/// x87 use is storing one callee's ST0 float result to the frame. All
/// `comiss` branches take the jump on unordered (NaN), matched with
/// negated comparisons.
///
/// Signedness: every integer comparison is exact-equality or a bit test
/// EXCEPT `(an instruction of the original); jl` (site 2 of callee 5), `(an instruction of the original); jge`
/// against the halved frame word (SIGNED, can be negative), and the
/// jump-table range check `(an instruction of the original); ja` (UNSIGNED: gun ids below
/// `0x76E` wrap to huge values and miss the table).
///
/// Faults: several paths dereference a null allocator/constructor answer
/// (flag writes at small offsets); both sides fault identically. The
/// indirect call reads the entry object's `+0x8` vtable slot `0xC` and
/// calls it with that object as `this`.
///
/// Calling conventions (verified against each callee's prologue/epilogue
/// except the encrypted pool allocator, whose shape comes from its call
/// sites): ctors are `thiscall` with the pushed counts; fillers are
/// `cdecl`; three helpers take a frame pointer in ECX (`thiscall` with
/// skipped ECX, snapshot-compared).
lf_checker_rt::export!(thiscall, rw_00d0bbd0(this: u32, gunid: u32, ped: u32) -> u32 {
    let mut gun = gunid;
    // Frame slots (values only; out-pointer slots are passed by address).
    // Multi-word slots are arrays: the checker snapshots them as one
    // consecutive run, which separate locals do not guarantee.
    let mut e10: u32 = 0;
    let mut e14: u32 = 0;
    let mut e20_3 = [0u32; 3];
    let mut e2c: u32 = 0;
    // E+0x30 holds id3's callee-written outputs and is loop-carried: the
    // original never clears it on loop-back, so zero it once (the checker's
    // stack fill is 0) and let later iterations see the leftovers.
    let mut e30_3 = [0u32; 3];
    loop {
        // ---- Loop head. ----
        wr_u32(this, THIS_FLAGS, rd_u32(this, THIS_FLAGS) & !0x200);
        e10 = 0;
        wr_u32(this, THIS_RATE, 0);
        let g = gun;
        if g != 0xC8 && g != 0x774 && g != 0x772 && g != 0x770
            && rd_u32(ped, PED_BIG) != 0
            && rd_u8(ped, PED_MODE) & 4 != 0
        {
            let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
            let alloc = callee_thiscall!(1u32, u32, pool);
            if alloc != 0 {
                e10 = callee_thiscall!(2u32, u32, alloc, 0, 1, 0, 0);
            }
        }
        // ---- Entry distance block. ----
        let edx = rd_u32(ped, PED_INNER);
        if edx != 0 {
            let a = rd_u32(rd_u32(this, THIS_LINK), 0x20);
            let b = rd_u32(ped, PED_POS);
            let d30 = unsafe { sse_sub(rd_f(a, 0x30), rd_f(b, 0x30)) };
            let d34 = unsafe { sse_sub(rd_f(a, 0x34), rd_f(b, 0x34)) };
            let d38 = unsafe { sse_sub(rd_f(a, 0x38), rd_f(b, 0x38)) };
            // Note the order: E+0x20=xmm0, E+0x24=xmm2, E+0x28=xmm1.
            e20_3 = [d30.to_bits(), d34.to_bits(), d38.to_bits()];
            let al = callee_cdecl!(3u32, u32, edx, ped,
                e20_3.as_mut_ptr() as u32,
                e30_3.as_mut_ptr() as u32, 0);
            if al & 0xFF == 0 {
                callee_thiscall!(4u32, u32, ped);
            } else {
                let t1 = unsafe { sse_sub(f32::from_bits(e30_3[0]), rd_f(b, 0x30)) };
                let t2 = unsafe { sse_sub(f32::from_bits(e30_3[1]), rd_f(b, 0x34)) };
                let t0 = unsafe { sse_sub(f32::from_bits(e30_3[2]), rd_f(b, 0x38)) };
                let q2 = unsafe { sse_mul(t2, t2) };
                let q1 = unsafe { sse_mul(t1, t1) };
                let q0 = unsafe { sse_mul(t0, t0) };
                let s = unsafe { sse_add(q2, q1) };
                let s = unsafe { sse_add(s, q0) };
                let c4 = f32::from_bits(unsafe { global::<u32>(F_4).read() });
                if s > c4 {
                    callee_thiscall!(4u32, u32, ped);
                }
            }
        }
        if e10 != 0 {
            return e10;
        }
        // ---- Dispatch on the (possibly rewritten) gun id. ----
        let id = gun;
        // SIGNED greater-than (`jle` takes the sub-dispatch).
        if (id as i32) > (GUN_SUB as i32) {
            let idx = id.wrapping_add(0xFFFF_F892);
            if idx > 0xB {
                return e10;
            }
            match idx {
                0 => {
                    let r = case_0(this, ped, &mut gun, &mut e10, &mut e14,
                        &mut e30_3);
                    if let Some(v) = r {
                        return v;
                    }
                    continue;
                }
                1 => return case_1(this, ped, &mut e10, &mut e14,
                    &mut e20_3, &mut e2c),
                2 => return case_2(this, ped, &mut e10),
                3 => return case_3(this, ped, &mut e10),
                4 => return case_4(this, ped),
                5 => return case_5(this, ped),
                6 => return case_6(this, ped, &mut e10),
                7 | 10 => return e10,
                8 => return case_8(this, ped),
                9 => return case_9(this, ped),
                _ => return case_11(this, ped),
            }
        }
        if id == GUN_SUB {
            return tail_76d(this, ped);
        }
        let t = id.wrapping_sub(GUN_C8);
        if t == 0 {
            callee_thiscall!(4u32, u32, ped);
            return e10;
        }
        if t.wrapping_sub(0x77) != 0 {
            return e10;
        }
        return tail_13f(this, ped);
    }
});

// ---------------------------------------------------------------------------
// Case 0 (gun 0x76E): the searching path. Returns Some(task) to return, or
// None to rewrite the gun id to 0x779 and loop back.
// ---------------------------------------------------------------------------
#[allow(clippy::too_many_arguments)]
fn case_0(
    this: u32,
    ped: u32,
    gun: &mut u32,
    e10: &mut u32,
    e14: &mut u32,
    e30_3: &mut [u32; 3],
) -> Option<u32> {
    let eax0 = rd_u32(ped, PED_POS).wrapping_add(0x30);
    // Push order (0,0,-1,-1,ecx,overwritten,eax,edi): the ecx word is
    // overwritten with 100.0 before the call, so its stale value is dead.
    let d = callee_cdecl!(5u32, u32, ped, eax0, HUNDRED_F32,
        0xFFFF_FFFF, 0xFFFF_FFFF, 0, 0);
    *e10 = d;
    let zf = rd_f(this, THIS_F2);
    if !(0.0f32 >= zf) {
        return Some(case_0_tail(this, ped, e10, e14, e30_3));
    }
    let flag: u32 = unsafe { global::<u32>(G_FLAG_DWORD).read() };
    if flag & 0xFF00 != 0 {
        *gun = 0x779;
        return None;
    }
    let ecx1 = rd_u32(ped, PED_POS).wrapping_add(0x30);
    let d2 = callee_cdecl!(5u32, u32, ped, ecx1, HUNDRED_F32, 0x76E,
        0xFFFF_FFFF, 0, 0);
    if (d2 as i32) < 4 {
        return Some(case_0_tail(this, ped, e10, e14, e30_3));
    }
    let eax2 = rd_u32(ped, PED_POS).wrapping_add(0x30);
    let d3 = callee_cdecl!(5u32, u32, ped, eax2, HUNDRED_F32, 0x779,
        0xFFFF_FFFF, 0, 0);
    // `(an instruction of the original); cdq; (an instruction of the original); (an instruction of the original)`: the frame word
    // is never written, so it reads zero under the defined fill.
    let half: u32 = 0;
    if (d3 as i32) >= (half as i32) {
        return Some(case_0_tail(this, ped, e10, e14, e30_3));
    }
    *gun = 0x779;
    None
}

/// Tail shared by case 0's non-looping exits.
#[allow(clippy::too_many_arguments)]
fn case_0_tail(
    this: u32,
    ped: u32,
    e10: &mut u32,
    e14: &mut u32,
    e30_3: &mut [u32; 3],
) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        *e10 = 0;
    } else {
        *e10 = callee_thiscall!(6u32, u32, alloc, rd_u32(this, THIS_LINK));
    }
    let d7 = callee_cdecl!(7u32, u32, rd_u32(this, THIS_LINK));
    let hc = (ped.wrapping_add(PED_HELPER)) as u32;
    callee_thiscall!(8u32, u32, hc, relocated(0xEDF92C), 0, 0, 0, 0xFFFF_FFFF, 0, 0,
        ONE_F32, d7, 0);
    let f: f32 = callee_thiscall!(9u32, f32, this, ped);
    *e14 = f.to_bits();
    let g = rd_f(this, THIS_F1);
    if !(g >= f) {
        return case_0_epilogue(this, ped, e10);
    }
    let a = rd_u32(rd_u32(this, THIS_LINK), 0x20).wrapping_add(0x30);
    let t = callee_cdecl!(10u32, u32, ped, a, 0, 0, 0, 0, 0);
    if t & 0xFF != 0 {
        // All three `lea`s run with the same stack depth and yield E+0x30.
        callee_thiscall!(11u32, u32,
            &mut e30_3[0] as *mut u32 as u32, ped);
        callee_thiscall!(12u32, u32,
            &mut e30_3[0] as *mut u32 as u32, ped);
        callee_thiscall!(13u32, u32,
            &mut e30_3[0] as *mut u32 as u32);
    }
    let o = *e10;
    wr_u8(o, 0x40, rd_u8(o, 0x40) | 8);
    wr_u32(this, THIS_F1, 0);
    case_0_epilogue(this, ped, e10)
}

/// Case 0's flag checks and float-gated return.
fn case_0_epilogue(this: u32, ped: u32, e10: &mut u32) -> u32 {
    wr_u32(this, THIS_FLAGS, rd_u32(this, THIS_FLAGS) | 0x100);
    let t = rd_u32(ped, PED_AUX);
    let r = callee_thiscall!(14u32, u32, t);
    if rd_u8(r, 0x8F4) & 0x1C != 4 {
        let t2 = rd_u32(ped, PED_AUX);
        let r2 = callee_thiscall!(14u32, u32, t2);
        if rd_u8(r2, 0x8F4) & 0x1C != 0 {
            return *e10;
        }
    }
    let n = call_cdecl_0(15);
    let v = unsafe { sse_cvt_i32(n) };
    let c1 = f32::from_bits(unsafe { global::<u32>(F_C1).read() });
    let v = unsafe { sse_mul(v, c1) };
    let c0 = f32::from_bits(unsafe { global::<u32>(F_033).read() });
    if !(c0 > v) {
        return *e10;
    }
    wr_u32(this, THIS_FLAGS, rd_u32(this, THIS_FLAGS) | 0x200);
    *e10
}

// ---------------------------------------------------------------------------
// Case 11 (gun 0x779): allocate, construct, return (null-tolerant).
// ---------------------------------------------------------------------------
fn case_11(this: u32, ped: u32) -> u32 {
    let d7 = callee_cdecl!(7u32, u32, rd_u32(this, THIS_LINK));
    let hc = ped.wrapping_add(PED_HELPER);
    callee_thiscall!(8u32, u32, hc, relocated(0xEDF93C), 0, 0, 0, 0xFFFF_FFFF, 0, 0,
        ONE_F32, d7, 0);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    callee_thiscall!(16u32, u32, alloc, rd_u32(this, THIS_LINK))
}

// ---------------------------------------------------------------------------
// Case 1 (gun 0x76F): near/far probe with table lookup, then construct.
// ---------------------------------------------------------------------------
#[allow(clippy::too_many_arguments)]
fn case_1(
    this: u32,
    ped: u32,
    e10: &mut u32,
    e14: &mut u32,
    e20_3: &mut [u32; 3],
    e2c: &mut u32,
) -> u32 {
    if rd_u32(this, THIS_FLAGS) & 0x100 == 0 {
        *e10 = (*e10 & 0xFFFF_FF00) | 1;
    } else {
        let a = rd_u32(ped, PED_POS);
        let b = rd_u32(rd_u32(this, THIS_LINK), 0x20);
        let x1 = unsafe { sse_sub(rd_f(a, 0x30), rd_f(b, 0x30)) };
        let x2 = unsafe { sse_sub(rd_f(a, 0x34), rd_f(b, 0x34)) };
        let x0 = unsafe { sse_sub(rd_f(a, 0x38), rd_f(b, 0x38)) };
        let q2 = unsafe { sse_mul(x2, x2) };
        let q1 = unsafe { sse_mul(x1, x1) };
        let q0 = unsafe { sse_mul(x0, x0) };
        let s = unsafe { sse_add(q2, q1) };
        let s = unsafe { sse_add(s, q0) };
        let c = f32::from_bits(unsafe { global::<u32>(F_900).read() });
        if c > s {
            *e10 = (*e10 & 0xFFFF_FF00) | 1;
        } else {
            *e10 &= 0xFFFF_FF00;
        }
    }
    let pa = rd_u32(ped, PED_POS);
    *e20_3 = [rd_u32(pa, 0x30), rd_u32(pa, 0x34), rd_u32(pa, 0x38)];
    let r17 = callee_thiscall!(17u32, u32, ped);
    *e14 = r17;
    let taux = rd_u32(ped, PED_AUX);
    if rd_u8(taux, 0x38) & 1 != 0 && r17 != 0 {
        let hp = r17.wrapping_add(8);
        let e18slot = hp;
        let m = callee_thiscall!(18u32, u32, hp);
        if m != ped {
            let s = rd_u32(r17, 0x50);
            let k = callee_thiscall!(19u32, u32, e18slot, ped);
            let idx = k.wrapping_add(1).wrapping_mul(3);
            let o0 = rd_u32(s, idx.wrapping_mul(8));
            let o1 = rd_u32(s, idx.wrapping_mul(8).wrapping_add(4));
            let o2 = rd_u32(s, idx.wrapping_mul(8).wrapping_add(8));
            *e20_3 = [o0, o1, o2];
            // E+0x3C is never written: reads zero under the defined fill.
            *e2c = 0;
            return case_1_alloc(this, ped, e10, e20_3);
        }
    }
    if rd_u8(taux, 0x38) & 1 != 0 {
        let ax = rd_u32(ped, PED_POS).wrapping_add(0x30);
        let t2 = taux.wrapping_add(0x10);
        let al = callee_thiscall!(20u32, u32, t2, ax, 0);
        if al & 0xFF == 0 {
            callee_cdecl!(21u32, u32, ped,
                &mut e20_3[0] as *mut u32 as u32);
            return case_1_alloc(this, ped, e10, e20_3);
        }
    }
    let inner = rd_u32(ped, PED_INNER);
    if inner != 0 {
        let _ = callee_thiscall!(22u32, u32, inner,
            &mut e20_3[0] as *mut u32 as u32, 0);
    }
    case_1_alloc(this, ped, e10, e20_3)
}

/// Case 1's shared allocator/constructor tail.
fn case_1_alloc(
    this: u32,
    ped: u32,
    e10: &mut u32,
    e20_3: &mut [u32; 3],
) -> u32 {
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    // Push order (0, E+0x10-value, &E+0x20, [this+0x3C]): the lea
    // `[esp+0x28]` runs after two pushes, so it is the same E+0x20 slot
    // id22 just wrote through, not E+0x28.
    callee_thiscall!(23u32, u32, alloc, rd_u32(this, THIS_LINK),
        &mut e20_3[0] as *mut u32 as u32, *e10, 0)
}

// ---------------------------------------------------------------------------
// Small jump-table cases.
// ---------------------------------------------------------------------------

/// Case 2 (gun 0x770): flag-derived answer byte, faults on null.
fn case_2(this: u32, ped: u32, e10: &mut u32) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    wr_u32(this, THIS_FLAGS, rd_u32(this, THIS_FLAGS) | 8);
    wr_u32(this, THIS_AUX, 0);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        *e10 = 0;
    } else {
        *e10 = callee_thiscall!(24u32, u32, alloc,
            rd_u32(this, THIS_LINK));
    }
    let o = *e10;
    wr_u8(o, 0x69, ((rd_u32(this, THIS_FLAGS) >> 0x12) & 1) as u8);
    o
}

/// Case 3 (gun 0x771): scaled float chain into the rate slot.
fn case_3(this: u32, ped: u32, e10: &mut u32) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        *e10 = 0;
    } else {
        *e10 = callee_thiscall!(25u32, u32, alloc,
            rd_u32(this, THIS_LINK));
    }
    wr_u32(this, THIS_RATE, FIVE_F32);
    float_chain_store(this);
    *e10
}

/// Shared integer-scaled float chain: `rate = ((n*C1*C2)+C3) * rate`.
fn float_chain_store(this: u32) {
    let n = call_cdecl_0(15);
    let mut v = unsafe { sse_cvt_i32(n) };
    let c1 = f32::from_bits(unsafe { global::<u32>(F_C1).read() });
    v = unsafe { sse_mul(v, c1) };
    let c2 = f32::from_bits(unsafe { global::<u32>(F_C2).read() });
    v = unsafe { sse_mul(v, c2) };
    let c3 = f32::from_bits(unsafe { global::<u32>(F_C3).read() });
    v = unsafe { sse_add(v, c3) };
    v = unsafe { sse_mul(v, rd_f(this, THIS_RATE)) };
    wr_u32(this, THIS_RATE, v.to_bits());
}

/// Case 4 (gun 0x772).
fn case_4(this: u32, ped: u32) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    let t = callee_thiscall!(26u32, u32, ped);
    if t & 0xFF != 0 {
        callee_thiscall!(27u32, u32, ped, 0, 0xFFFF_FFFF);
    }
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    callee_thiscall!(28u32, u32, alloc, rd_u32(this, THIS_LINK))
}

/// Case 5 (gun 0x773).
fn case_5(this: u32, ped: u32) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    callee_thiscall!(29u32, u32, alloc, rd_u32(this, THIS_LINK))
}

/// Case 6 (gun 0x774): flag-derived answer byte, faults on null on both
/// paths (the null path writes through the null result too).
fn case_6(this: u32, ped: u32, e10: &mut u32) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        *e10 = 0;
    } else {
        *e10 = callee_thiscall!(31u32, u32, alloc,
            rd_u32(this, THIS_LINK));
    }
    let o = *e10;
    wr_u8(o, 0x4C, ((rd_u32(this, THIS_FLAGS) >> 0xE) & 1) as u8);
    o
}

/// Case 8 (gun 0x776).
fn case_8(this: u32, ped: u32) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    callee_thiscall!(30u32, u32, alloc, rd_u32(this, THIS_LINK))
}

/// Case 9 (gun 0x777).
fn case_9(this: u32, ped: u32) -> u32 {
    callee_thiscall!(4u32, u32, ped);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    callee_thiscall!(32u32, u32, alloc, rd_u32(this, THIS_LINK))
}

// ---------------------------------------------------------------------------
// Sub-dispatch tails (gunid <= 0x76D).
// ---------------------------------------------------------------------------

/// Gun 0x76D: vtable check, then a 5-argument construction or the shared
/// float tail.
fn tail_76d(this: u32, ped: u32) -> u32 {
    let sub = rd_u32(this, THIS_SUB);
    if sub != 0 {
        let vt = rd_u32(sub, 0);
        let slot = rd_u32(vt, 0xC);
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        if f(sub) == 0x770 {
            callee_thiscall!(36u32, u32, ped, 1);
            let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
            let alloc = callee_thiscall!(1u32, u32, pool);
            if alloc == 0 {
                wr_u32(0, 0x14, 4);
                return 0;
            }
            let edx = rd_u32(this, THIS_LINK);
            let cx = rd_u32(edx, 0x20).wrapping_add(0x30);
            let o = callee_thiscall!(37u32, u32, alloc, edx, cx, 0, 1,
                FIVE_F32);
            wr_u32(o, 0x14, 4);
            wr_u32(o, 0x20, 0x770);
            return o;
        }
    }
    tail_float(this, ped)
}

/// Shared float tail: rate selection, float chain, 5-argument construct.
fn tail_float(this: u32, ped: u32) -> u32 {
    let d = callee_cdecl!(38u32, u32, ped, rd_u32(this, THIS_LINK));
    if d == 0x78B {
        wr_u32(this, THIS_RATE, ONE_HALF_F32);
    } else if rd_u32(ped, PED_INNER) != 0 {
        wr_u32(this, THIS_RATE, FIVE_F32);
    } else {
        let l = rd_u32(this, THIS_LINK);
        if l != 0 && rd_u32(l, 0xB30) != 0 && rd_u8(l, 0x26C) & 4 != 0 {
            wr_u32(this, THIS_RATE, ONE_F32);
        } else {
            wr_u32(this, THIS_RATE, TWO_HALF_F32);
        }
    }
    float_chain_store(this);
    callee_thiscall!(36u32, u32, ped, 1);
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    let edx = rd_u32(this, THIS_LINK);
    let cx = rd_u32(edx, 0x20).wrapping_add(0x30);
    callee_thiscall!(37u32, u32, alloc, edx, cx, 0, 1, 0)
}

/// Gun 0x13F: helper probes then a constant-argument construction.
fn tail_13f(this: u32, ped: u32) -> u32 {
    let h = ped.wrapping_add(PED_SLOT);
    let b1 = callee_thiscall!(33u32, u32, h);
    if b1 != 0 {
        let b2 = callee_thiscall!(33u32, u32, h);
        if rd_u32(b2, 0x18) == 0x2E {
            callee_thiscall!(34u32, u32, h, ped, 1);
        }
    }
    let w = rd_u32(ped, PED_W);
    if rd_u32(w, 0x12C) == 2 {
        let hc = ped.wrapping_add(PED_HELPER);
        callee_thiscall!(8u32, u32, hc, relocated(0xEDF9C0), 0, 0, 0, 0xFFFF_FFFF, 0,
            0, ONE_F32, 0, 0);
    }
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        return 0;
    }
    callee_thiscall!(35u32, u32, alloc, 0x10600)
}
