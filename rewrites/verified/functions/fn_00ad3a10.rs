// original: 0x00ad3a10 audio_voice_preset_apply
//! Voice preset apply: programs fetch/set/commit groups, sweeps a voice
//! table through a vtable hook, then tunes nine channel blocks.
//!
//! Specification. The routine takes one selector word and returns a masked
//! bit value. The head selects one of three programs: selector 0 programs
//! two preset groups, selector 1 programs two other groups, any other
//! selector programs none; selectors 0 and 1 then both run five shared
//! groups. Each group fetches a handle with `(0x10, 0)`, applies a setter
//! with that handle and the group's `(slot, value)` pair when the handle
//! is non-zero (else contributes zero), and commits the result.
//!
//! The middle sweep walks a table object: for each live entry whose flag
//! byte is clear, whose kind word is 2, whose level value is not below
//! zero, whose state word is not 2 and whose masked bits meet the global
//! mask, it invokes the entry's vtable slot `0x8C` with `(0x1E, 0, 0xFF,
//! -1)`; any failed test clears the entry's bit word instead.
//!
//! The tail runs nine channel blocks. Each block fetches a handle with
//! `(0x10, 0)`, sets it with the block's `(slot, value)` pair (a zero
//! handle selects a null object), invokes the object's vtable slot 8
//! twice, and folds the two answers into the object's word at +4: with
//! `t = ans1 mod 16` and `e = (16 - t) mod 16`, `x = ((((ans2 + e) / 16)
//! << 14) ^ word) & mask` is xored back into the word, where the division
//! is truncating and every step wraps. The routine returns the last
//! block's masked value. It writes no globals.

use lf_checker_rt::{callee_cdecl, callee_thiscall, global};

const LOOP_TABLE: u32 = 0x012E_22A4;
const LOOP_MASK: u32 = 0x0159_AF24;

const ID_FHEAD: u32 = 1;
const ID_FTAIL: u32 = 2;
const ID_SET: u32 = 3;
const ID_COMMIT: u32 = 4;
// Indirect targets (contract ids 5 and 6) are reached through the
// fabricated objects, never through the stub table.

const FETCH_A: u32 = 0x10;
const FETCH_B: u32 = 0;
const WORD_MASK: u32 = 0x01FF_C000;

const HEAD_A: [(u32, u32); 3] = [(0x13, 1), (0x17, 1), (0x18, 0x12)];
const HEAD_B: [(u32, u32); 3] = [(0x13, 1), (0x17, 1), (0x18, 0xFF)];
const HEAD_S: [(u32, u32); 5] = [(0x1A, 0xFF), (0x16, 2), (0x0F, 0), (0x06, 0), (0x0A, 1)];
const TAIL: [(u32, u32); 9] = [
    (0x0A, 1),
    (0x06, 1),
    (0x13, 0),
    (0x17, 0),
    (0x18, 0),
    (0x1A, 0xFF),
    (0x19, 0xFFFF_FFFF),
    (0x16, 0),
    (0x0F, 0x0F),
];

#[inline(always)]
fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}

#[inline(always)]
fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write(v) }
}

#[inline(always)]
fn g32(file_va: u32) -> u32 {
    unsafe { global::<u32>(file_va).read() }
}

#[inline(always)]
fn fetch_set(id: u32, slot: u32, value: u32) -> u32 {
    let h = callee_cdecl!(id, u32, FETCH_A, FETCH_B);
    if h != 0 {
        callee_thiscall!(ID_SET, u32, h, slot, value)
    } else {
        0
    }
}

#[inline(always)]
fn group(slot: u32, value: u32) {
    let s = fetch_set(ID_FHEAD, slot, value);
    callee_cdecl!(ID_COMMIT, u32, s);
}

#[inline(always)]
fn vcall8(obj: u32) -> u32 {
    let vt = rd32(obj);
    let tgt = rd32(vt + 8);
    let f: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(tgt as usize) };
    f(obj)
}

fn tail_block(slot: u32, value: u32) -> u32 {
    let h = callee_cdecl!(ID_FTAIL, u32, FETCH_A, FETCH_B);
    let obj = if h != 0 {
        callee_thiscall!(ID_SET, u32, h, slot, value)
    } else {
        0
    };
    let a1 = vcall8(obj);
    let t = (a1 as i32) % 16;
    let e = (16i32.wrapping_sub(t)) % 16;
    let a2 = vcall8(obj);
    let sum = (a2 as i32).wrapping_add(e);
    let sh = (sum / 16) as u32;
    let sh = sh.wrapping_shl(14);
    let w = rd32(obj + 4);
    let x = (sh ^ w) & WORD_MASK;
    wr32(obj + 4, w ^ x);
    x
}

fn body(sel: u32) -> u32 {
    if sel == 0 {
        for &(s, v) in &HEAD_A {
            group(s, v);
        }
        for &(s, v) in &HEAD_S {
            group(s, v);
        }
    } else if sel == 1 {
        for &(s, v) in &HEAD_B {
            group(s, v);
        }
        for &(s, v) in &HEAD_S {
            group(s, v);
        }
    }

    let table = g32(LOOP_TABLE);
    let count = rd32(table + 8) as i32;
    if count > 0 {
        let mut i: i32 = 0;
        while i < count {
            let flags = rd32(table + 4);
            let fl = unsafe { ((flags + i as u32) as *const u8).read() };
            if fl & 0x80 == 0 {
                let stride = rd32(table + 12);
                let ent = stride
                    .wrapping_mul(i as u32)
                    .wrapping_add(rd32(table));
                if ent != 0 && rd32(ent + 0x1304) == 2 {
                    let fobj = rd32(ent + 0x20);
                    let fv = f32::from_bits(rd32(fobj + 0x28));
                    if 0.0f32 > fv {
                        wr32(ent + 8, 0);
                    } else if rd32(ent + 0x144) == 2 {
                        wr32(ent + 8, 0);
                    } else {
                        let c = rd32(ent + 0x0C);
                        let b = rd32(ent + 8);
                        let g = g32(LOOP_MASK);
                        if (g & ((!c) & b)) == 0 {
                            wr32(ent + 8, 0);
                        } else {
                            let vt = rd32(ent);
                            let tgt = rd32(vt + 0x8C);
                            let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                                unsafe { core::mem::transmute(tgt as usize) };
                            f(ent, 0x1E, 0, 0xFF, 0xFFFF_FFFF);
                        }
                    }
                }
            }
            i += 1;
        }
    }

    let mut ans = 0;
    for &(s, v) in &TAIL {
        ans = tail_block(s, v);
    }
    ans
}

lf_checker_rt::export!(cdecl, fn_00ad3a10(sel: u32) -> u32 {
    body(sel)
});

