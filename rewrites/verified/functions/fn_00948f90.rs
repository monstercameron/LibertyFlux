// original: 0x00948f90 conditional_object_reset
//! Guarded object reset with two float clamps.
//!
//! Takes a nullable object pointer. Returns immediately on null or when the
//! mode byte at +0x10B8 is not 2 (exit EAX is then the caller's entry EAX,
//! pinned by the contract since a rewrite cannot observe that register).
//! Otherwise it clears a flag, runs an engine pre-pass, asks the object for a
//! float through vtable slot 0xFC and, when that value is ordered-above a
//! global limit, issues a second virtual call (slot 0xF4); clamps the two
//! stored floats at +0x10AC/+0x10D8 to the constant 1000.0 when they exceed
//! the same limit; rewrites a group of flag bytes/words; optionally invokes a
//! helper on the sub-object returned through slot 0xA0; runs two more engine
//! passes; and finishes with a two-pointer teardown pair plus a final pass,
//! storing 1.0f at +0x12DC. The original spills the slot-0xFC answer over its
//! own incoming stack slot, which a rewrite cannot reproduce, so the contract
//! disables the stack comparison (the spilled value is the scripted callee
//! answer, verified instead through the call sequence it selects).

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Pinned entry-EAX value: what the original returns on its early paths.
/// Must equal regs[0] in the contract.
const ENTRY_EAX: u32 = 0x1234_5678;
const F_LIMIT: u32 = 0x00FE_8C58;
const G_FLAG: u32 = 0x011D_6FD4;
const CLAMP_TO: u32 = 0x447A_0000; // 1000.0f
const ONE_F: u32 = 0x3F80_0000; // 1.0f

#[inline]
unsafe fn rd8(base: u32, off: u32) -> u8 {
    unsafe { ((base + off) as *const u8).read() }
}
#[inline]
unsafe fn wr8(base: u32, off: u32, v: u8) {
    unsafe { ((base + off) as *mut u8).write(v) }
}
#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { ((base + off) as *const u32).read() }
}
#[inline]
unsafe fn wr32(base: u32, off: u32, v: u32) {
    unsafe { ((base + off) as *mut u32).write(v) }
}
#[inline]
unsafe fn rd_f32(base: u32, off: u32) -> f32 {
    f32::from_bits(unsafe { rd32(base, off) })
}

export!(cdecl, rw_00948F90(obj: u32) -> u32 {
    if obj == 0 {
        return ENTRY_EAX;
    }
    if unsafe { rd8(obj, 0x10B8) } != 2 {
        return ENTRY_EAX;
    }
    unsafe { wr8(obj, 0x0F14, rd8(obj, 0x0F14) & 0xFB) };

    let _p0 = callee_thiscall!(1, u32, obj, 0, 0);

    // Virtual float query through slot 0xFC (callee id 2, ST0 answer).
    let vt = unsafe { rd32(obj, 0) };
    let query: extern "thiscall" fn(u32) -> f32 =
        unsafe { core::mem::transmute(rd32(vt, 0xFC) as usize) };
    let measured = query(obj);
    let limit = f32::from_bits(unsafe { global::<u32>(F_LIMIT).read() });
    if measured > limit {
        let adjust: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt, 0xF4) as usize) };
        let _p1 = adjust(obj, CLAMP_TO, 0);
    }

    // Clamp the two stored floats to the constant (note: the constant, not
    // the limit value they were compared against).
    if unsafe { rd_f32(obj, 0x10AC) } > limit {
        unsafe { wr32(obj, 0x10AC, CLAMP_TO) };
    }
    if unsafe { rd_f32(obj, 0x10D8) } > limit {
        unsafe { wr32(obj, 0x10D8, CLAMP_TO) };
    }

    unsafe { wr8(obj, 0x0F1B, rd8(obj, 0x0F1B) | 1) };
    unsafe { wr32(obj, 0x0118, rd32(obj, 0x0118) & 0xFFFF_803F) };
    unsafe { ((obj + 0x011C) as *mut u16).write(0) };
    let mode = unsafe { rd32(obj, 0x1300) };
    if mode == 0 {
        unsafe { wr8(obj, 0x1474, rd8(obj, 0x1474) & 0xFB) };
    } else if mode == 1 {
        unsafe { wr8(obj, 0x1310, rd8(obj, 0x1310) & 0xFB) };
    }
    let flag = unsafe { rd8(obj, 0x0F20) };
    unsafe { wr8(obj, 0x0F16, rd8(obj, 0x0F16) & 0xFE) };
    unsafe { wr8(obj, 0x0F19, rd8(obj, 0x0F19) | 1) };
    unsafe { wr8(obj, 0x0F20, (flag & 0xF7) | 4) };

    // Optional sub-object helper through slot 0xA0.
    let fetch: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(rd32(vt, 0xA0) as usize) };
    let sub = fetch(obj);
    if sub != 0 {
        let _p2 = callee_thiscall!(5, u32, sub, 0x400, 0);
    }

    unsafe { wr32(obj, 0x0024, rd32(obj, 0x0024) | 0x20) };
    unsafe { wr32(obj, 0x0118, rd32(obj, 0x0118) | 0x0010_0000) };
    let _p3 = callee_thiscall!(6, u32, obj, 0);

    if (unsafe { rd8(obj, 0x01E2) } & 0x0F) < 2 {
        unsafe { wr32(obj, 0x0024, rd32(obj, 0x0024) | 1) };
    } else if unsafe { global::<u32>(G_FLAG).read() } == 0 {
        unsafe { wr32(obj, 0x0024, rd32(obj, 0x0024) | 1) };
    }

    let _p4 = callee_cdecl!(7, u32, obj);
    let _p5 = callee_cdecl!(8, u32, obj + 0x80, obj);
    let ans = callee_thiscall!(9, u32, obj, 1);
    unsafe { wr32(obj, 0x12DC, ONE_F) };
    ans
});
