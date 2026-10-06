// original: 0x00d09170 ped_create_gun_task
// Rewrite of ped_create_gun_task. The full specification is the doc
// comment on the export below.

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, global};

// ---------------------------------------------------------------------------
// Named addresses, offsets and magic values.
// ---------------------------------------------------------------------------

/// Pool pointer the allocator call reads its `this` from.
const G_POOL: u32 = 0x0167_E2A0;
/// Global threshold compared SIGNED against 2 (`jge`).
const G_CMP_THRESHOLD: u32 = 0x011D_6FD4;
/// Read-only float constants (bit patterns moved, never computed).
const F_A: u32 = 0x00FE_8978;
const F_B: u32 = 0x00FE_8DA0;
const F_C: u32 = 0x00FE_8A24;
const F_D: u32 = 0x00FE_8D94;
const F_E: u32 = 0x0171_DE54;

/// Task id selecting the extended configuration.
const TASK_GUN_EXTENDED: u32 = 12;
/// Virtual slot (in dwords) of the ped predicate: byte offset 0x128.
const PED_VIRT_SLOT_WORDS: u32 = 0x48;

// Task flag words.
const FLAG_NEW: u32 = 0x4000_5A40;
const FLAG_EXTENDED: u32 = 0x0100_8001;
const FLAG_STATE_3: u32 = 0x0400_0000;
const FLAG_B: u32 = 0x4200_0000;
const FLAG_C: u32 = 0x4200_0009;
const FLAG_SEAL: u32 = 0x0002_0140;
const FLAG_DONE: u32 = 0x0000_0008;
const FLAG_TAIL: u32 = 0x0000_8043;
// Fixed float slot values (bit patterns).
const SLOT_F1: u32 = 0x3F49_0FDB;
const SLOT_F1_NEG: u32 = 0xBF49_0FDB;
const SLOT_C_LO: u32 = 0xBEC9_0FDB;
const SLOT_C_HI: u32 = 0x3EC9_0FDB;
const TEN_F32_BITS: u32 = 0x4120_0000;
const ONE_F32_BITS: u32 = 0x3F80_0000;
const NEG_ONE_F32_BITS: u32 = 0xBF80_0000;

// Ped object layout.
const PED_ZERO0: u32 = 0xD70;
const PED_ZERO1: u32 = 0xD74;
const PED_ZERO2: u32 = 0xD78;
const PED_MODE0: u32 = 0x218;
const PED_MODE1: u32 = 0x219;
const PED_STATE: u32 = 0x2B0;
const PED_INNER: u32 = 0xD68;
const PED_AUX: u32 = 0x224;
const PED_FLAGS: u32 = 0x264;
const PED_OTHER: u32 = 0x20;
const PED_RATE: u32 = 0x230;

// Small-object (`this`) layout.
const THIS_LINK: u32 = 0x30;
const THIS_GATE: u32 = 0x34;
const THIS_BITS: u32 = 0x35;
const THIS_ANSWER: u32 = 0x50;

// Task object layout.
const TASK_WORDS: u32 = 0x4C; // four u16 words at +0x4C..+0x53
const TASK_MODE: u32 = 0x6C;
const TASK_KIND: u32 = 0x70;
const TASK_FLAGS: u32 = 0x74;
const TASK_F0: u32 = 0x80;
const TASK_F1: u32 = 0x84;
const TASK_F2: u32 = 0x88;
const TASK_F3: u32 = 0x8C;
const TASK_F4: u32 = 0x90;

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
fn wr_u16(base: u32, off: u32, v: u16) {
    unsafe { ((base.wrapping_add(off)) as *mut u16).write_unaligned(v) }
}
/// Read-modify-write OR, in the original's order (read first, so a null
/// base faults on the read exactly like the original).
#[inline(always)]
fn rmw_or(base: u32, off: u32, mask: u32) -> u32 {
    let v = rd_u32(base, off) | mask;
    wr_u32(base, off, v);
    v
}

/// Build a gun-task object for a ped and configure its flag words.
///
/// Signature: `thiscall(this, ped, task_id) -> task` (two stack words,
/// callee cleans 8). `this` is a small object the function reads flag bytes
/// from (`+0x30` must be a pointer or null, `+0x34`/`+0x35` are flag bytes,
/// `+0x50` receives one answer byte); `ped` is the large ped object (offsets
/// up to `+0xD78`); `task_id` selects extra configuration when it is 12.
/// Returns the new task, allocated from the task pool and constructed by the
/// gun-task constructor.
///
/// Behaviour. The entry stores zero the ped's words at `+0xD70..+0xD78`,
/// then splits on two ped bytes: `+0x218 == 0` with `+0x219 != 0` takes the
/// short path (allocate, construct with constant arguments, set flag bit
/// `0x40005A40`, and when `task_id == 12` set more flag bits and two float
/// slots from read-only constants chosen by `([this+0x35] & 2)` and the
/// `& 0x18` masked word behind `[ped+0xD68]`). Every other combination takes
/// the long path: poll three predicate helpers, run two frame-pointer
/// helpers that fill caller slots, run a 7-out-pointer helper, optionally
/// call the ped's virtual slot `0x128/4` with the ped as `this`, run a
/// 3-out-pointer helper, and then either construct variant B (arguments from
/// the helper outputs, flag words from a SIGNED global threshold compare)
/// or variant C (constant arguments, fixed float slots).
///
/// Floating point: the function only MOVES single-precision bit patterns
/// between read-only constants, its frame and the task object; it performs
/// no float arithmetic, so bit-exactness needs no operand pinning.
///
/// Signedness: every compared value uses an exact-equality or bit test
/// EXCEPT the global at `G_CMP_THRESHOLD`, which is compared SIGNED
/// (`jge`): values `0x80000000..0xFFFFFFFF` count as below 2.
///
/// Faults: the function faults (null `+0x74` flag write) exactly when the
/// pool allocator or the constructor answers null; both sides do the same
/// read-modify-write, so the fault address matches.
///
/// Calling conventions (verified against each callee's prologue/epilogue,
/// not assumed from the call sites): the pool allocator is
/// `thiscall(pool)` with no stack arguments; the constructor is
/// `thiscall(newmem, 8 args)`; the three predicate helpers take only the
/// ped (`stdcall`, 1 arg) even where the caller pushes extra words, which
/// are dead stack garbage the callee never reads and are therefore NOT
/// reproduced here; the 5-argument filler is `cdecl`; the remaining
/// helpers are `thiscall`/`stdcall`/`cdecl` as called below.
lf_checker_rt::export!(thiscall, rw_00d09170(this: u32, ped: u32, task_id: u32) -> u32 {
    let mut obj = this;
    // Entry stores.
    wr_u32(ped, PED_ZERO2, 0);
    wr_u32(ped, PED_ZERO1, 0);
    wr_u32(ped, PED_ZERO0, 0);

    if rd_u8(ped, PED_MODE0) != 0 || rd_u8(ped, PED_MODE1) == 0 {
        return path_b(this, ped, task_id);
    }

    // ---- Short path: allocate + construct with constant arguments. ----
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    if alloc == 0 {
        obj = 0;
    } else {
        obj = callee_thiscall!(2u32, u32, alloc,
            1, 0, 0, NEG_ONE_F32_BITS, 0, 1, 1, ONE_F32_BITS);
    }
    let mut flags = rmw_or(obj, TASK_FLAGS, FLAG_NEW);
    if task_id != TASK_GUN_EXTENDED {
        return block_a(ped, task_id, obj);
    }
    flags |= FLAG_EXTENDED;
    wr_u32(obj, TASK_FLAGS, flags);
    // The `lea` is never null, so the null check always falls through and
    // only the value comparison remains.
    if rd_u32(ped, PED_STATE) == 3 {
        flags |= FLAG_STATE_3;
        wr_u32(obj, TASK_FLAGS, flags);
    }
    let bits = rd_u8(this, THIS_BITS);
    let masked = rd_u32(rd_u32(ped, PED_INNER), 0) & 0x18;
    let f_const: u32;
    let (f1_val, f2_val): (u32, u32);
    if bits & 2 == 0 {
        f_const = unsafe { global::<u32>(F_B).read() };
        f1_val = SLOT_F1_NEG;
        f2_val = 0;
    } else {
        f_const = unsafe { global::<u32>(F_A).read() };
        f1_val = 0;
        f2_val = SLOT_F1;
    }
    // The F1/F2 stores are skipped when the mask differs and the
    // predicate answers zero or the flag byte has bits 4-5 set; the seal
    // and the F0 store always run.
    let do_store = masked as u8 == 0x18 || {
        let d = callee_stdcall!(3u32, u32, ped);
        d & 0xFF != 0 && bits & 0x30 == 0
    };
    if do_store {
        wr_u32(obj, TASK_F1, f1_val);
        wr_u32(obj, TASK_F2, f2_val);
    }
    tail_a_checks(ped, obj);
    rmw_or(obj, TASK_FLAGS, FLAG_SEAL);
    wr_u32(obj, TASK_F0, f_const);
    block_a(ped, task_id, obj)
});

/// Second masked check of the short path (conditional F3/F4 stores).
fn tail_a_checks(ped: u32, obj: u32) {
    let masked2 = rd_u32(rd_u32(ped, PED_INNER), 0) & 0x18;
    if masked2 as u8 != 0x18 {
        let d = callee_stdcall!(3u32, u32, ped);
        if d & 0xFF == 0 {
            wr_u32(obj, TASK_F3, 0);
            wr_u32(obj, TASK_F4, unsafe { global::<u32>(F_E).read() });
        }
    }
}

/// Shared block of the short path: slot setup, the filler call, the word
/// stores, then the seal epilogue. Here the filler fills E+0x14/E+0x18
/// (both lea's use `[esp+0x30]`) and the words are read back from the
/// same two slots. (The E+0x1C low byte the caller also sets is never
/// passed anywhere nor read back on this path, so it is not reproduced.)
fn block_a(ped: u32, task_id: u32, obj: u32) -> u32 {
    let mut e14 = 0xFFFF_FFFFu32;
    let mut e18 = 0xFFFF_FFFFu32;
    d11eb0_call(ped, task_id, obj, &mut e14, &mut e18);
    epilogue_seal(obj)
}

/// The three 1-argument predicate calls, the 5-argument filler call with
/// two out-pointers, and the conditional word stores. The short path
/// passes its E+0x14/E+0x18 pair, variant B its E+0x1C/E+0x18 pair (its
/// second lea uses `[esp+0x38]`); both read the words back from the same
/// pair they passed.
///
/// NOTE: the caller's extra pushed words (a copy of edx, copies of eax and
/// of one frame word) are dead stack garbage: the callees pop only their
/// single argument and never read the rest, and nothing below the incoming
/// stack pointer is observable. They are not reproduced.
fn d11eb0_call(
    ped: u32,
    task_id: u32,
    obj: u32,
    out0: &mut u32,
    out1: &mut u32,
) {
    let _d3 = callee_stdcall!(3u32, u32, ped);
    let _d4 = callee_stdcall!(4u32, u32, ped);
    let d5 = callee_stdcall!(5u32, u32, ped);
    let a = d5 & 0xFF;
    let d6 = callee_cdecl!(6u32, u32, ped, out0 as *mut u32 as u32,
        out1 as *mut u32 as u32, task_id, a);
    if d6 & 0xFF != 0 {
        let lo = (*out0 & 0xFFFF) as u16;
        let hi = (*out1 & 0xFFFF) as u16;
        wr_u16(obj, TASK_WORDS, lo);
        wr_u16(obj, TASK_WORDS + 2, hi);
        wr_u16(obj, TASK_WORDS + 4, lo);
        wr_u16(obj, TASK_WORDS + 6, hi);
    }
}

/// Seal epilogue shared by every returning path except variant C.
fn epilogue_seal(obj: u32) -> u32 {
    rmw_or(obj, TASK_FLAGS, FLAG_DONE);
    wr_u8(obj, TASK_KIND, 4);
    obj
}

/// Long path: helper polling, out-pointer helpers, the virtual call, then
/// variant B or variant C.
fn path_b(this: u32, ped: u32, task_id: u32) -> u32 {
    let mut obj = this;
    let mut e14: u32 = 0;
    let mut e18: u32 = 0;
    let mut e1c: u32 = 0;
    let mut e20: u32 = 0;
    let mut e9: u8 = 0;
    e9 = 0;
    e1c = 1;
    let aux = rd_u32(ped, PED_AUX);
    let a7 = callee_thiscall!(7u32, u32, aux, 1);
    if a7 != 0 {
        let c30 = rd_u32(this, THIS_LINK);
        if c30 != 0 {
            let d8 = callee_thiscall!(8u32, u32, a7, c30);
            e1c = d8;
            e9 = u8::from(d8 == 2);
        }
    }
    let d9 = callee_stdcall!(9u32, u32, ped);
    wr_u8(obj, THIS_ANSWER, (d9 & 0xFF) as u8);
    e14 = 0;
    let mut al_cur: u8 = (d9 & 0xFF) as u8;
    let c30 = rd_u32(obj, THIS_LINK);
    if c30 != 0 {
        let inner = rd_u32(c30, PED_INNER);
        e18 = inner;
        if inner != 0 && rd_u32(ped, PED_FLAGS) & 0x200 != 0 {
            obj = rd_u32(ped, PED_OTHER);
            e20 = 0;
            let _w = callee_thiscall!(10u32, u32, inner,
                &mut e20 as *mut u32 as u32, 0);
            let hp = obj.wrapping_add(0x30);
            let v = e18;
            // Push order is (frameptr, esi, value), so arg 0 is the
            // VALUE, arg 1 the heap address and arg 2 the frame pointer.
            let d11 = callee_cdecl!(11u32, u32,
                v, hp, &mut e20 as *mut u32 as u32);
            al_cur = (d11 & 0xFF) as u8;
            // The callee sequence reloads the saved entry object here.
            obj = this;
            if d11 & 0xFF != 0 {
                // 7-out-pointer helper: six byte slots plus the e18 dword.
                let p = rd_u32(this, THIS_LINK);
                let h = rd_u32(p, PED_AUX).wrapping_add(0x2E0);
                let mut bs = [0u8; 8];
                e18 = 0;
                // Argument order matches the original's push order: arg 0
                // is the LAST push (the e18 dword slot), arg 6 the first
                // (byte slot 0), the rest count down the byte slots.
                let bp = bs.as_mut_ptr();
                let w12 = callee_thiscall!(12u32, u32, h,
                    &mut e18 as *mut u32 as u32,
                    unsafe { bp.add(5) } as u32,
                    unsafe { bp.add(4) } as u32,
                    unsafe { bp.add(3) } as u32,
                    unsafe { bp.add(2) } as u32,
                    unsafe { bp.add(1) } as u32,
                    bp as u32);
                al_cur = (w12 & 0xFF) as u8;
                let c = e18;
                if c != 0xFFFF_FFFF && c != 9 {
                    let a = (e14 & 0xFF) as u8;
                    e14 = if c == 0xC { u32::from(a) } else { 1 };
                }
            }
        }
    }
    // Virtual-slot predicate gate.
    if rd_u8(obj, THIS_GATE) == 0 && e1c == 3 {
        let vt = rd_u32(ped, 0);
        let slot = rd_u32(vt, PED_VIRT_SLOT_WORDS * 4);
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let av = f(ped) & 0xFF;
        al_cur = if av == 0 { 1 } else { (e14 & 0xFF) as u8 };
    }
    if e9 != 0 || rd_u8(obj, THIS_GATE) != 0 || al_cur != 0
        || rd_u8(obj, THIS_ANSWER) != al_cur
    {
        return path_c(ped, obj);
    }
    // ---- Variant B. ----
    e1c = 0;
    e14 = 0;
    e18 = 0;
    // Push order (frameptr, frameptr, frameptr, ped) with esp-relative
    // lea's: arg 1 fills e14, arg 2 fills e1c, arg 3 fills e18.
    let _ = callee_cdecl!(13u32, u32, ped,
        &mut e14 as *mut u32 as u32,
        &mut e1c as *mut u32 as u32,
        &mut e18 as *mut u32 as u32);
    let xmm: u32;
    if task_id == TASK_GUN_EXTENDED {
        xmm = unsafe { global::<u32>(F_C).read() };
        e18 = 7;
    } else {
        let t = e18;
        xmm = e1c;
        e18 = t;
    }
    e1c = xmm;
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    let built: u32;
    if alloc == 0 {
        built = 0;
    } else {
        let c = rd_u32(obj, THIS_LINK);
        let (sarg, carg): (u32, u32);
        if c != 0 {
            sarg = 0;
            carg = c;
        } else {
            sarg = obj.wrapping_add(0x20);
            carg = 0;
        }
        built = callee_thiscall!(2u32, u32, alloc,
            e18, carg, sarg, xmm, 0, 1, 1, ONE_F32_BITS);
    }
    rmw_or(built, TASK_FLAGS, FLAG_B);
    obj = built;
    // Variant B passes its E+0x1C/E+0x18 pair to the filler. (The flag
    // bit the caller also derives here lands in E+0x14, which is never
    // passed anywhere nor read back on this path, so it is not computed.)
    let mut be1c = 0xFFFF_FFFFu32;
    let mut be18 = 0xFFFF_FFFFu32;
    d11eb0_call(ped, task_id, built, &mut be1c, &mut be18);
    wr_u8(built, TASK_KIND, 4);
    let g: u32 = unsafe { global::<u32>(G_CMP_THRESHOLD).read() };
    if (g as i32) < 2 {
        let x = rd_u32(built, TASK_MODE);
        wr_u32(built, TASK_MODE, (x & 0xFFFF_04FF) | 0x400);
    }
    let x = rd_u32(built, TASK_MODE);
    rmw_or(built, TASK_FLAGS, FLAG_DONE);
    wr_u32(built, TASK_MODE, (x & 0xFF04_FFFF) | 0x0004_0000);
    if task_id != TASK_GUN_EXTENDED {
        return epilogue_seal(built);
    }
    wr_u32(ped, PED_RATE, TEN_F32_BITS);
    rmw_or(built, TASK_FLAGS, FLAG_TAIL);
    let mut c = rd_u32(built, TASK_FLAGS);
    if (c >> 0x19) & 1 != 0 {
        c &= 0xFDFF_FFFF;
        wr_u32(built, TASK_FLAGS, c);
    }
    let maw = rd_u32(rd_u32(ped, PED_INNER), 0) & 0x18;
    let xmm2: u32;
    if rd_u8(this, THIS_BITS) & 2 != 0 {
        xmm2 = unsafe { global::<u32>(F_A).read() };
        if maw as u8 == 0x18 {
            wr_u32(built, TASK_F1, 0);
            wr_u32(built, TASK_F2, SLOT_F1);
        }
    } else {
        xmm2 = unsafe { global::<u32>(F_B).read() };
        if maw as u8 == 0x18 {
            wr_u32(built, TASK_F1, SLOT_F1_NEG);
            wr_u32(built, TASK_F2, 0);
        }
    }
    wr_u32(built, TASK_F0, xmm2);
    let maw2 = rd_u32(rd_u32(ped, PED_INNER), 0) & 0x18;
    if maw2 as u8 == 0x18 {
        return epilogue_seal(built);
    }
    let d = callee_stdcall!(3u32, u32, ped);
    if d & 0xFF != 0 {
        return built;
    }
    wr_u32(built, TASK_F3, 0);
    wr_u32(built, TASK_F4, unsafe { global::<u32>(F_E).read() });
    built
}

/// Variant C: constant-argument construction with fixed float slots.
fn path_c(ped: u32, obj: u32) -> u32 {
    let e1c: u32;
    if rd_u8(obj, THIS_ANSWER) != 0 {
        e1c = unsafe { global::<u32>(F_D).read() };
    } else {
        e1c = unsafe { global::<u32>(F_C).read() };
    }
    let pool: u32 = unsafe { global::<u32>(G_POOL).read() };
    let alloc = callee_thiscall!(1u32, u32, pool);
    let built: u32;
    if alloc == 0 {
        built = 0;
    } else {
        let c = rd_u32(obj, THIS_LINK);
        let (sarg, carg): (u32, u32);
        if c != 0 {
            sarg = 0;
            carg = c;
        } else {
            sarg = obj.wrapping_add(0x20);
            carg = 0;
        }
        built = callee_thiscall!(2u32, u32, alloc,
            2, carg, sarg, e1c, 0, 1, 1, ONE_F32_BITS);
    }
    rmw_or(built, TASK_FLAGS, FLAG_C);
    wr_u32(built, TASK_F3, SLOT_C_LO);
    wr_u32(built, TASK_F4, SLOT_C_HI);
    built
}
