// original: 0x00e49c80 E1_SELECT
//! Select-menu builder.
//!
//! Unless the entry gate (the object's slot 0x140) reports the menu already
//! built, computes the menu layout in single precision from two seeded words,
//! one float-stack answer, the display dimensions (each chosen from a global
//! pair by a helper answer) and three seeded globals, applying a 1.3x scale
//! to two stored words when the small-display mark is set and one of two
//! constant rows by layout selector; builds four texture panels and three
//! fonts through their helpers; publishes the selection, runs the four
//! one-word tail hooks, describes the trailing object past the menu fields
//! and hands it to the registrar, whose answer is the return value. Float
//! work runs in the original's operand order; the one integer-to-float step
//! converts a scalar, whose low lane rounds identically to the original's
//! vector conversion.
//!
//! 32-bit only. Build with `RUSTFLAGS="-C panic=abort" cargo build --release
//! --target i686-pc-windows-msvc` (a panic must become a fault, never an
//! unwind into the worker).

#![allow(unsafe_code)]
#![allow(clippy::pedantic)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![cfg(target_arch = "x86")]

use core::hint::black_box as bb;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}

#[inline(always)]
fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write(v) }
}

#[inline(always)]
fn rd_glob_f32(file_va: u32) -> f32 {
    unsafe { global::<f32>(file_va).read() }
}

#[inline(always)]
fn rd_glob_u32(file_va: u32) -> u32 {
    unsafe { global::<u32>(file_va).read() }
}

// Menu-object field offsets written or read (raw offsets; the merged
// structure file names this class UITitleMenu but none of these fields).
const F_PANEL_BG: u32 = 0x1EC;
const F_PANEL_RIGHT: u32 = 0x1F4;
const F_PANEL_LEFT: u32 = 0x1F0;
const F_PANEL_BOTTOM: u32 = 0x1F8;
const F_FONT_LEFT: u32 = 0x1FC;
const F_FONT_RIGHT: u32 = 0x200;
const F_FONT_CENTER: u32 = 0x204;
const F_SELECTED: u32 = 0x208;
const F_STATE: u32 = 0x1E8;
const F_COUNT: u32 = 0x1E0;
const F_MODE: u32 = 0x1E4;
const F_FLAGS: u32 = 0x20C;

// Vtable slots used on the menu object itself, on panel objects and on
// font objects. Slot numbers collide across the three object kinds, so the
// contract plants three separate vtable segments.
const V_GATE: u32 = 0x140;
const V_THIS_48: u32 = 0x48;
const V_THIS_4C: u32 = 0x4C;
const V_TAIL_28: u32 = 0x28;
const V_TAIL_24: u32 = 0x24;
const V_TAIL_18: u32 = 0x18;
const V_TAIL_13C: u32 = 0x13C;
const V_PANEL_104: u32 = 0x104;
const V_PANEL_114: u32 = 0x114;
const V_PANEL_120: u32 = 0x120;
const V_PANEL_48: u32 = 0x48;
const V_FONT_100: u32 = 0x100;
const V_FONT_1CC: u32 = 0x1CC;
const V_FONT_28: u32 = 0x28;
const V_FONT_118: u32 = 0x118;
const V_FONT_1FC: u32 = 0x1FC;
const V_FONT_200: u32 = 0x200;
const V_FONT_120: u32 = 0x120;
const V_FONT_1E0: u32 = 0x1E0;
const V_FONT_170: u32 = 0x170;
const V_FONT_17C: u32 = 0x17C;
const V_FONT_208: u32 = 0x208;
const V_FONT_1DC: u32 = 0x1DC;

// Game-string addresses passed as callee arguments (file VAs, relocated).
const S_TXD_LOOKUP: u32 = 0x00F17038;
const S_PANEL_BG: u32 = 0x00F17044;
const S_PANEL_BG_CFG: u32 = 0x00F17064;
const S_PANEL_RIGHT: u32 = 0x00F17080;
const S_PANEL_RIGHT_CFG: u32 = 0x00F17098;
const S_PANEL_LEFT: u32 = 0x00F170A4;
const S_PANEL_LEFT_CFG: u32 = 0x00F170BC;
const S_PANEL_BOTTOM: u32 = 0x00F170C8;
const S_PANEL_BOTTOM_CFG: u32 = 0x00F170E4;
const S_FONT_LEFT: u32 = 0x00F170F4;
const S_FONT_LEFT_NAME: u32 = 0x00F17104;
const S_FONT_RIGHT: u32 = 0x00F17110;
const S_FONT_RIGHT_NAME: u32 = 0x00F17120;
const S_FONT_CENTER: u32 = 0x00F1714C;
const S_FONT_CENTER_NAME: u32 = 0x00F1716C;

// Float constants read from read-only memory (file VAs, relocated).
const C_SCALE: u32 = 0x00FE892C; // 1.3, the small-display scale
const C_ONE_HALF: u32 = 0x00FE8960; // 1.5
const C_ONE_Q: u32 = 0x00FE8920; // 1.25
const C_FOUR: u32 = 0x00FE8AB8; // 4.0
const C_HALF: u32 = 0x00FE8830; // 0.5
const C_2048: u32 = 0x00FE8C70; // 2048.0
const C_WIDE: u32 = 0x00F17818; // 1.8947368
const C_405: u32 = 0x00F1781C; // 405.0
const C_485: u32 = 0x00F17820; // 485.0
const C_495: u32 = 0x00F17824; // 495.0
const C_648: u32 = 0x00F17828; // 648.0
const C_1152: u32 = 0x00F1782C; // 1152.0
const C_NEG182: u32 = 0x00F17830; // -182.0
const C_350: u32 = 0x00E9D128; // 350.0

// Writable globals read (file VAs, relocated).
const G_ASPECT_NUM: u32 = 0x01059654; // float: display aspect numerator
const G_W_LO: u32 = 0x0105C884; // int width when the first answer is silent
const G_W_HI: u32 = 0x0105C888; // int width when it answers
const G_H_LO: u32 = 0x0105C880; // int height when the second answer is silent
const G_H_HI: u32 = 0x0105C87C; // int height when it answers
const G_LAYOUT: u32 = 0x01160CC8; // layout selector: 1 and 2 share a path
const G_SMALL: u32 = 0x0116C250; // low byte 0x72 selects the small scale

const SMALL_MARK: u8 = 0x72;
const FIXED_ECX: u32 = 0x0118D7F0; // constant object for the float helpers
const FONT_MAGIC: u32 = 0x42100000; // 36.0, fixed first word for slot 0x1CC
const STRUCT_TAG: u32 = 0xDD682944; // fixed middle word of every 0x104 call
const TAIL_MAGIC: u32 = 0x00E49AA0; // value the tail stores past the object
const PANEL_SCRATCH_LAST: u32 = 0xBEFFFFFF; // fourth block's scratch word

/// Working frame. The original keeps eleven words above its entry stack
/// pointer plus a temp-object anchor higher up; several callee arguments
/// are pointers into those words, and the contract snapshots what they
/// point at, so the offsets here match the original's exactly.
#[repr(C)]
struct Frame {
    _lo: [u32; 3],
    txd_slot: u32, // +0x0C
    f10: f32,      // +0x10
    f14: f32,      // +0x14
    f18: f32,      // +0x18
    f1c: f32,      // +0x1C
    scratch: u32,  // +0x20
    f24: f32,      // +0x24
    f28: f32,      // +0x28
    f2c: f32,      // +0x2C
    f30: f32,      // +0x30
    _mid: [u32; 5],
    anchor: u32,   // +0x48, never written, like the original's
    _hi: u32,
}

const _: () = assert!(core::mem::offset_of!(Frame, txd_slot) == 0x0C);
const _: () = assert!(core::mem::offset_of!(Frame, f18) == 0x18);
const _: () = assert!(core::mem::offset_of!(Frame, scratch) == 0x20);
const _: () = assert!(core::mem::offset_of!(Frame, anchor) == 0x48);
const _: () = assert!(core::mem::offset_of!(Frame, _mid) == 0x34);

// Indirect helpers: load the object's table, load the slot, call through.
// Both sides land on the same planted stub.
#[inline(always)]
fn v0(obj: u32, slot: u32) -> u32 {
    let t = rd32(rd32(obj).wrapping_add(slot));
    let f: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(t as usize) };
    f(obj)
}

#[inline(always)]
fn v1(obj: u32, slot: u32, a0: u32) -> u32 {
    let t = rd32(rd32(obj).wrapping_add(slot));
    let f: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(t as usize) };
    f(obj, a0)
}

#[inline(always)]
fn v2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
    let t = rd32(rd32(obj).wrapping_add(slot));
    let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(t as usize) };
    f(obj, a0, a1)
}

#[inline(always)]
fn v3(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    let t = rd32(rd32(obj).wrapping_add(slot));
    let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(t as usize) };
    f(obj, a0, a1, a2)
}

#[inline(always)]
fn v4(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    let t = rd32(rd32(obj).wrapping_add(slot));
    let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(t as usize) };
    f(obj, a0, a1, a2, a3)
}

/// Panel call with one scalar word plus a 24-byte struct passed by value
/// (seven stack words; the callee cleans all of them).
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn v7(
    obj: u32, slot: u32, a0: u32, s0: u32, s1: u32, s2: u32, s3: u32, s4: u32,
    s5: u32,
) -> u32 {
    let t = rd32(rd32(obj).wrapping_add(slot));
    let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(t as usize) };
    f(obj, a0, s0, s1, s2, s3, s4, s5)
}

/// Panel call with three scalar words plus a 24-byte struct passed by value
/// (nine stack words; the callee cleans all of them).
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn v9(
    obj: u32, slot: u32, a0: u32, a1: u32, a2: u32, s0: u32, s1: u32, s2: u32,
    s3: u32, s4: u32, s5: u32,
) -> u32 {
    let t = rd32(rd32(obj).wrapping_add(slot));
    let f: extern "thiscall" fn(
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    ) -> u32 = unsafe { core::mem::transmute(t as usize) };
    f(obj, a0, a1, a2, s0, s1, s2, s3, s4, s5)
}

/// Ask the layout helper for a 24-byte record and read all six words back,
/// in the original's order.
#[inline(always)]
fn fresh_record(anchor: u32, w0: u32, w1: u32) -> [u32; 6] {
    let d = callee_thiscall!(16, u32, anchor, w0, w1);
    [
        rd32(d),
        rd32(d.wrapping_add(4)),
        rd32(d.wrapping_add(8)),
        rd32(d.wrapping_add(12)),
        rd32(d.wrapping_add(16)),
        rd32(d.wrapping_add(20)),
    ]
}

/// One slot-0x114 group on a panel: fetch a record, hand it to the panel
/// with one scalar word, run the anchor finalizer.
#[inline(always)]
fn group114(this: u32, field: u32, anchor: u32, push: u32, w0: u32, w1: u32) {
    let s = fresh_record(anchor, w0, w1);
    let panel = rd32(this.wrapping_add(field));
    v7(panel, V_PANEL_114, push, s[0], s[1], s[2], s[3], s[4], s[5]);
    callee_thiscall!(17, u32, anchor);
}

/// One slot-0x104 group on a panel: fetch a record, hand it to the panel
/// with three scalar words, run the anchor finalizer.
#[inline(always)]
fn group104(
    this: u32, field: u32, anchor: u32, first: u32, last: u32, w0: u32,
    w1: u32,
) {
    let s = fresh_record(anchor, w0, w1);
    let panel = rd32(this.wrapping_add(field));
    v9(
        panel, V_PANEL_104, last, STRUCT_TAG, first, s[0], s[1], s[2], s[3],
        s[4], s[5],
    );
    callee_thiscall!(17, u32, anchor);
}

/// One font-block record group: fetch a record, hand it to the sibling
/// panel with one scalar word, feed that answer into the font's slot 0x100,
/// run the anchor finalizer.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn group_font(
    this: u32, font_field: u32, panel_field: u32, anchor: u32, push: u32,
    pick: u32, w0: u32, w1: u32,
) {
    let s = fresh_record(anchor, w0, w1);
    let panel = rd32(this.wrapping_add(panel_field));
    let answer = v7(panel, V_PANEL_48, push, s[0], s[1], s[2], s[3], s[4], s[5]);
    let font = rd32(this.wrapping_add(font_field));
    v2(font, V_FONT_100, pick, answer);
    callee_thiscall!(17, u32, anchor);
}

/// Sign flip done the way the original does it: flip the sign bit.
#[inline(always)]
fn negate(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

/// Shared body of the correct export and the mutant. `invert_scale` is
/// false in the correct export; the mutant inverts the small-display test
/// so the 1.3x scale applies exactly when it should not.
fn run(this: u32, invert_scale: bool) -> u32 {
    // Gate: ask slot 0x140 whether the menu is already built; a nonzero
    // low byte skips the whole body and the gate's answer is the return.
    let gate = v0(this, V_GATE);
    if (gate as u8) != 0 {
        return gate;
    }

    let mut fr: Frame = unsafe { core::mem::zeroed() };
    let anchor = (&fr.anchor as *const u32) as u32;

    // Layout seeds: two words through a frame pointer, then two floats.
    let mut seeds = [0u32; 2];
    callee_cdecl!(2, u32, seeds.as_mut_ptr() as u32, 2);
    let mut x2 = bb(f32::from_bits(seeds[0]));
    let mut x3 = bb(f32::from_bits(seeds[1]));
    let small = unsafe { global::<u8>(G_SMALL).read() };
    // The 1.3x scale applies only to the two stored words; the running
    // values continue unscaled (the original multiplies scratch copies).
    if (small == SMALL_MARK) != invert_scale {
        let scale = bb(rd_glob_f32(C_SCALE));
        fr.f2c = bb(x2 * scale);
        fr.f30 = bb(x3 * scale);
    } else {
        fr.f2c = x2;
        fr.f30 = x3;
    }
    let one_half = bb(rd_glob_f32(C_ONE_HALF));
    x2 = bb(x2 * one_half);
    x3 = bb(x3 * one_half);
    fr.f24 = x2;
    fr.f28 = x3;

    // Height row: the helper answers in the float stack; the value is
    // stored to the frame and from there continues in single precision.
    let st0: f32 = callee_thiscall!(3, f32, relocated(FIXED_ECX), 1);
    let mut y1 = bb(rd_glob_f32(C_NEG182));
    let mut y0 = bb(st0);
    y0 = bb(y0 - rd_glob_f32(C_ONE_Q));
    y0 = bb(y0 * rd_glob_f32(C_FOUR));
    y0 = bb(y0 * rd_glob_f32(C_WIDE));
    y1 = bb(y1 - y0);
    fr.f1c = y1;

    // Display dimensions: each helper answer selects one of a pair.
    let a1 = callee_thiscall!(4, u32, relocated(FIXED_ECX));
    let wide: u32 = if (a1 as u8) != 0 {
        rd_glob_u32(G_W_HI)
    } else {
        rd_glob_u32(G_W_LO)
    };
    let a2 = callee_thiscall!(5, u32, relocated(FIXED_ECX));
    let high: u32 = if (a2 as u8) != 0 {
        rd_glob_u32(G_H_HI)
    } else {
        rd_glob_u32(G_H_LO)
    };
    // Integer to float, then the ratio chain. The original converts a
    // whole vector; only the low lane ever reaches a store or a callee,
    // and the scalar conversion rounds it identically.
    let mut r0 = bb((high as i32) as f32);
    let mut r1 = bb((wide as i32) as f32);
    let mut r2 = bb(unsafe { global::<f32>(G_ASPECT_NUM).read() });
    r1 = bb(r1 / r0);
    r2 = bb(r2 - rd_glob_f32(C_648));
    r0 = bb(rd_glob_f32(C_2048));
    let r4 = bb(rd_glob_f32(C_WIDE));
    let layout = rd_glob_u32(G_LAYOUT);
    let mut r3 = bb(r1);
    r3 = bb(r3 - rd_glob_f32(C_ONE_Q));
    r1 = bb(r1 * rd_glob_f32(C_1152));
    r2 = bb(r2 * r3);
    r0 = bb(r0 - r1);
    r1 = bb(rd_glob_f32(C_405));
    r2 = bb(r2 * r4);
    r0 = bb(r0 * rd_glob_f32(C_HALF));
    r2 = bb(r2 + rd_glob_f32(C_648));
    fr.f10 = r0;
    r0 = bb(rd_glob_f32(C_495));
    fr.f14 = r2;
    if layout == 2 || layout == 1 {
        r0 = bb(rd_glob_f32(C_485));
        r1 = bb(rd_glob_f32(C_350));
    }
    r0 = bb(r0 - r1);
    r0 = bb(r0 * r3);
    r0 = bb(r0 * r4);
    r0 = bb(r0 + r1);
    fr.f18 = r0;

    // Background panel block. The texture slot found here is reused as an
    // argument by all four panel blocks.
    fr.txd_slot = callee_cdecl!(6, u32, relocated(S_TXD_LOOKUP));
    let s_bg = callee_cdecl!(7, u32, 0x25C);
    let panel_bg = if s_bg != 0 {
        let ans = v0(this, V_THIS_48);
        callee_thiscall!(14, u32, s_bg, relocated(S_PANEL_BG), ans)
    } else {
        0
    };
    wr32(this.wrapping_add(F_PANEL_BG), panel_bg);
    fr.scratch = 0xFFFF_FFFF;
    callee_thiscall!(
        15, u32, panel_bg, fr.txd_slot, relocated(S_PANEL_BG_CFG),
        (&fr.scratch as *const u32) as u32, 0xFFFF_FFFF
    );
    group114(this, F_PANEL_BG, anchor, 4, 0, 0);
    group114(this, F_PANEL_BG, anchor, 0x10, 0, 0);
    group104(this, F_PANEL_BG, anchor, 2, 2, negate(fr.f10).to_bits(), 0);
    group104(this, F_PANEL_BG, anchor, 8, 8, fr.f10.to_bits(), 0);
    wr32(rd32(this.wrapping_add(F_PANEL_BG)).wrapping_add(0x1D8), 0);
    v1(rd32(this.wrapping_add(F_PANEL_BG)), V_PANEL_120, 1);

    // Right panel block.
    let s_right = callee_cdecl!(8, u32, 0x25C);
    let panel_right = if s_right != 0 {
        let ans = v0(this, V_THIS_48);
        callee_thiscall!(14, u32, s_right, relocated(S_PANEL_RIGHT), ans)
    } else {
        0
    };
    wr32(this.wrapping_add(F_PANEL_RIGHT), panel_right);
    fr.scratch = 0xFFFF_FFFF;
    callee_thiscall!(
        15, u32, panel_right, fr.txd_slot, relocated(S_PANEL_RIGHT_CFG),
        (&fr.scratch as *const u32) as u32, 0xFFFF_FFFF
    );
    group114(this, F_PANEL_RIGHT, anchor, 4, 0, 0);
    group114(this, F_PANEL_RIGHT, anchor, 0x10, 0, 0);
    group104(this, F_PANEL_RIGHT, anchor, 8, 2, negate(fr.f14).to_bits(), 0);
    v1(rd32(this.wrapping_add(F_PANEL_RIGHT)), V_PANEL_120, 1);

    // Left panel block.
    let s_left = callee_cdecl!(9, u32, 0x25C);
    let panel_left = if s_left != 0 {
        let ans = v0(this, V_THIS_48);
        callee_thiscall!(14, u32, s_left, relocated(S_PANEL_LEFT), ans)
    } else {
        0
    };
    wr32(this.wrapping_add(F_PANEL_LEFT), panel_left);
    fr.scratch = 0xFFFF_FFFF;
    callee_thiscall!(
        15, u32, panel_left, fr.txd_slot, relocated(S_PANEL_LEFT_CFG),
        (&fr.scratch as *const u32) as u32, 0xFFFF_FFFF
    );
    group114(this, F_PANEL_LEFT, anchor, 4, 0, 0);
    group114(this, F_PANEL_LEFT, anchor, 0x10, 0, 0);
    group104(this, F_PANEL_LEFT, anchor, 2, 8, fr.f14.to_bits(), 0);
    v1(rd32(this.wrapping_add(F_PANEL_LEFT)), V_PANEL_120, 1);

    // Bottom panel block. Its scratch word differs from the other three.
    let s_bottom = callee_cdecl!(10, u32, 0x25C);
    let panel_bottom = if s_bottom != 0 {
        let ans = v0(this, V_THIS_48);
        callee_thiscall!(14, u32, s_bottom, relocated(S_PANEL_BOTTOM), ans)
    } else {
        0
    };
    wr32(this.wrapping_add(F_PANEL_BOTTOM), panel_bottom);
    fr.scratch = PANEL_SCRATCH_LAST;
    callee_thiscall!(
        15, u32, panel_bottom, fr.txd_slot, relocated(S_PANEL_BOTTOM_CFG),
        (&fr.scratch as *const u32) as u32, 0xFFFF_FFFF
    );
    group104(this, F_PANEL_BOTTOM, anchor, 2, 2, fr.f18.to_bits(), 0);
    group104(
        this, F_PANEL_BOTTOM, anchor, 8, 8, negate(fr.f18).to_bits(), 0,
    );
    group104(this, F_PANEL_BOTTOM, anchor, 0x10, 4, 0, 0xC1C80000);
    group104(this, F_PANEL_BOTTOM, anchor, 0x10, 0x10, 0, 0xC2A00000);
    v1(rd32(this.wrapping_add(F_PANEL_BOTTOM)), V_PANEL_120, 1);

    // Left font block.
    let s_fleft = callee_cdecl!(11, u32, 0x610);
    let font_left = if s_fleft != 0 {
        let first = v0(this, V_THIS_48);
        let second = v0(this, V_THIS_48);
        let name = callee_cdecl!(18, u32, relocated(S_FONT_LEFT), second);
        callee_thiscall!(19, u32, s_fleft, name, first)
    } else {
        0
    };
    wr32(this.wrapping_add(F_FONT_LEFT), font_left);
    let fc_ans = callee_cdecl!(
        20, u32, (&fr.scratch as *const u32) as u32, 0x41
    );
    v4(font_left, V_FONT_1CC, FONT_MAGIC, fc_ans, 0, 2);
    group_font(this, F_FONT_LEFT, F_PANEL_LEFT, anchor, 1, 1, 0, 0);
    group_font(
        this, F_FONT_LEFT, F_PANEL_LEFT, anchor, 0x10, 4, 0,
        fr.f1c.to_bits(),
    );
    v1(font_left, V_FONT_1FC, 0);
    v1(font_left, V_FONT_118, 1);
    v2(font_left, V_FONT_1E0, relocated(S_FONT_LEFT_NAME), 0);
    v1(font_left, V_FONT_28, 1);
    v1(font_left, V_FONT_200, 1);
    let tint = v0(this, V_THIS_4C);
    v1(font_left, V_FONT_170, tint);
    let shade = v0(this, V_THIS_4C);
    v1(font_left, V_FONT_17C, shade);
    v1(font_left, V_FONT_28, 1);
    v1(font_left, V_FONT_120, 1);
    v3(font_left, V_FONT_1DC, fr.f24.to_bits(), fr.f28.to_bits(), 0);
    wr32(this.wrapping_add(F_COUNT), 0);

    // Right font block.
    let s_fright = callee_cdecl!(12, u32, 0x610);
    let font_right = if s_fright != 0 {
        let first = v0(this, V_THIS_48);
        let second = v0(this, V_THIS_48);
        let name = callee_cdecl!(18, u32, relocated(S_FONT_RIGHT), second);
        callee_thiscall!(19, u32, s_fright, name, first)
    } else {
        0
    };
    wr32(this.wrapping_add(F_FONT_RIGHT), font_right);
    let fc_ans = callee_cdecl!(
        20, u32, (&fr.scratch as *const u32) as u32, 0x41
    );
    v4(font_right, V_FONT_1CC, FONT_MAGIC, fc_ans, 0, 2);
    group_font(this, F_FONT_RIGHT, F_PANEL_RIGHT, anchor, 1, 1, 0, 0);
    group_font(
        this, F_FONT_RIGHT, F_PANEL_RIGHT, anchor, 0x10, 4, 0,
        fr.f1c.to_bits(),
    );
    v1(font_right, V_FONT_1FC, 0);
    v1(font_right, V_FONT_118, 1);
    v1(font_right, V_FONT_200, 1);
    let fc_ans2 = callee_cdecl!(
        20, u32, (&fr.scratch as *const u32) as u32, 0x41
    );
    v1(font_right, V_FONT_208, fc_ans2);
    let tint = v0(this, V_THIS_4C);
    v1(font_right, V_FONT_170, tint);
    let shade = v0(this, V_THIS_4C);
    v1(font_right, V_FONT_17C, shade);
    v1(font_right, V_FONT_28, 1);
    v1(font_right, V_FONT_120, 1);
    v3(font_right, V_FONT_1DC, fr.f24.to_bits(), fr.f28.to_bits(), 0);
    v2(font_right, V_FONT_1E0, relocated(S_FONT_RIGHT_NAME), 0);
    wr32(this.wrapping_add(F_MODE), 3);

    // Center font block. Its final triple reads the two seed words that
    // the 1.5x step did not overwrite.
    let s_fcenter = callee_cdecl!(13, u32, 0x610);
    let font_center = if s_fcenter != 0 {
        let first = v0(this, V_THIS_48);
        let second = v0(this, V_THIS_48);
        let name = callee_cdecl!(18, u32, relocated(S_FONT_CENTER), second);
        callee_thiscall!(19, u32, s_fcenter, name, first)
    } else {
        0
    };
    wr32(this.wrapping_add(F_FONT_CENTER), font_center);
    let fc_ans = callee_cdecl!(
        20, u32, (&fr.scratch as *const u32) as u32, 0x41
    );
    v4(font_center, V_FONT_1CC, FONT_MAGIC, fc_ans, 0, 2);
    group_font(this, F_FONT_CENTER, F_PANEL_BOTTOM, anchor, 1, 1, 0, 0);
    v1(font_center, V_FONT_1FC, 0);
    v1(font_center, V_FONT_118, 1);
    v2(font_center, V_FONT_1E0, relocated(S_FONT_CENTER_NAME), 0);
    v1(font_center, V_FONT_28, 1);
    v1(font_center, V_FONT_200, 1);
    let tint = v0(this, V_THIS_4C);
    v1(font_center, V_FONT_170, tint);
    let shade = v0(this, V_THIS_4C);
    v1(font_center, V_FONT_17C, shade);
    v1(font_center, V_FONT_28, 1);
    v1(font_center, V_FONT_120, 1);
    v3(font_center, V_FONT_1DC, fr.f2c.to_bits(), fr.f30.to_bits(), 0);

    // Tail: publish the selection, run the four one-word hooks, then
    // describe the trailing object unless it sits at the frame anchor
    // (it never does when `this` is heap), and hand it to the registrar.
    // The registrar's answer is the leftover return value.
    wr32(this.wrapping_add(F_STATE), 8);
    wr32(
        this.wrapping_add(F_SELECTED),
        rd32(this.wrapping_add(F_FONT_LEFT)),
    );
    wr32(this.wrapping_add(F_FLAGS), 0);
    v1(this, V_TAIL_28, 1);
    v1(this, V_TAIL_24, 1);
    v1(this, V_TAIL_18, 1);
    v1(this, V_TAIL_13C, 1);
    let trailing = this.wrapping_add(0x218);
    let frame_anchor = (&fr._mid[0] as *const u32) as u32;
    if trailing != frame_anchor {
        wr32(trailing, 0);
        wr32(trailing.wrapping_add(4), relocated(TAIL_MAGIC));
    }
    callee_cdecl!(21, u32, trailing)
}

/// Select-menu builder.
///
/// Asks the object's slot 0x140 whether the menu is already built; a
/// nonzero low byte returns that answer at once. Otherwise computes the
/// menu layout in single precision from two seeded words, one float-stack
/// answer, the display dimensions (each chosen from a global pair by a
/// helper answer) and three seeded globals, applying a 1.3x scale when
/// the small-display mark is set and one of two constant rows by layout
/// selector; builds four texture panels (each: allocate, create through
/// slot 0x48, configure, two or four record groups through slots 0x114
/// or 0x104, close through slot 0x120) and three fonts (each: allocate,
/// create through two slot-0x48 answers, configure through slot 0x1CC,
/// one or two record groups shared with a sibling panel, then the
/// per-font slot sequence); publishes the selection, runs the four
/// one-word tail hooks, describes the trailing object past the menu
/// fields and hands it to the registrar, whose answer is the return.
/// Operand order matches the original throughout; the one
/// integer-to-float step converts a scalar, whose low lane rounds
/// identically to the original's vector conversion.
export!(thiscall, rw_00E49C80(this: u32) -> u32 {
    run(this, false)
});

// Mutant: the small-display test is inverted, so the 1.3x scale applies
// exactly when it should not. Must fail.
export!(thiscall, mut_00E49C80(this: u32) -> u32 {
    run(this, true)
});
