// original: 0x00acc3f0 audio_voice_update
/// Audio voice level/envelope/phase update.
///
/// Each tick: snapshots the current level, mixes a fresh sample from the
/// voice's hook (resolved through the key lookup callee, then a virtual
/// call) with the global gain, and integrates the level towards the mix
/// with a dead-zone clamp. A second envelope decays towards zero at one of
/// two rates, and the phase advances and wraps into [-pi, pi].
///
/// `key` selects the voice record (forwarded to the lookup callee);
/// `dt` is the tick step. Returns nothing meaningful: the original leaves
/// flag-test residue in EAX on one path and callee residue on the other,
/// so the contract checks no return channel.
mod fn_00acc3f0_rt {
/// Call the virtual sample hook at vtable slot +0x24 on a lookup result.
///
/// The original loads the target through the object's vtable and calls it as
/// thiscall/0, taking the float result from the x87 stack. Both sides land on
/// the same checker-planted stub; an `f32` return type reads ST0 directly.
#[inline(always)]
unsafe fn virt_sample(obj: u32) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(0x24) as *const u32);
    let hook: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(target as usize);
    hook(obj)
}
/// Call the virtual sample hook at vtable slot +0x24 on a lookup result.
///
/// The original loads the target through the object's vtable and calls it as
/// thiscall/0, taking the float result from the x87 stack. Both sides land on
/// the same checker-planted stub; an `f32` return type reads ST0 directly.
#[inline(always)]
unsafe fn virt_sample(obj: u32) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(0x24) as *const u32);
    let hook: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(target as usize);
    hook(obj)
}

/// Absolute value through the sign-bit mask, as the original's `andps` does.
#[inline(always)]
fn f32_abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFF_FFFF)
}
/// Call the virtual sample hook at vtable slot +0x24 on a lookup result.
///
/// The original loads the target through the object's vtable and calls it as
/// thiscall/0, taking the float result from the x87 stack. Both sides land on
/// the same checker-planted stub; an `f32` return type reads ST0 directly.
#[inline(always)]
unsafe fn virt_sample(obj: u32) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(0x24) as *const u32);
    let hook: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(target as usize);
    hook(obj)
}

/// Absolute value through the sign-bit mask, as the original's `andps` does.
#[inline(always)]
fn f32_abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFF_FFFF)
}

/// Negation through the sign-bit flip, as the original's `xorps` with -0.0.
#[inline(always)]
fn f32_neg_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

// ---------------------------------------------------------------------------
// Bit-exact SSE scalar ops with FORCED operand order.
//
// A Rust `a * b` usually compiles to `mulss` with `a` in the destination, but
// the register allocator is free to swap the operands (it prefers whichever
// value is already in a register). For ordinary values that is harmless, but
// when both operands are NaNs with different payloads the destination's NaN
// wins, so a swap changes the result bits. These helpers implement the Intel
// rule explicitly (destination NaN wins, quieted; else source NaN; else the
// hardware op, whose NaN-free inputs make its own operand order irrelevant)
// so the emitted operand order no longer matters.
// ---------------------------------------------------------------------------

#[inline(always)]
fn is_nan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000 && (b & 0x007F_FFFF) != 0
}

/// Quiet a NaN the way SSE does (set the Q-bit; already-quiet NaNs unchanged).
#[inline(always)]
fn quiet_bits(b: u32) -> u32 {
    b | 0x0040_0000
}

/// `mulss dest, src` with `dest` forced as the NaN winner.
#[inline(always)]
fn mul_ss(dest: f32, src: f32) -> f32 {
    let db = dest.to_bits();
    let sb = src.to_bits();
    if is_nan_bits(db) {
        return f32::from_bits(quiet_bits(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet_bits(sb));
    }
    dest * src
}
/// Call the virtual sample hook at vtable slot +0x24 on a lookup result.
///
/// The original loads the target through the object's vtable and calls it as
/// thiscall/0, taking the float result from the x87 stack. Both sides land on
/// the same checker-planted stub; an `f32` return type reads ST0 directly.
#[inline(always)]
unsafe fn virt_sample(obj: u32) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(0x24) as *const u32);
    let hook: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(target as usize);
    hook(obj)
}

/// Absolute value through the sign-bit mask, as the original's `andps` does.
#[inline(always)]
fn f32_abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFF_FFFF)
}

/// Negation through the sign-bit flip, as the original's `xorps` with -0.0.
#[inline(always)]
fn f32_neg_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

// ---------------------------------------------------------------------------
// Bit-exact SSE scalar ops with FORCED operand order.
//
// A Rust `a * b` usually compiles to `mulss` with `a` in the destination, but
// the register allocator is free to swap the operands (it prefers whichever
// value is already in a register). For ordinary values that is harmless, but
// when both operands are NaNs with different payloads the destination's NaN
// wins, so a swap changes the result bits. These helpers implement the Intel
// rule explicitly (destination NaN wins, quieted; else source NaN; else the
// hardware op, whose NaN-free inputs make its own operand order irrelevant)
// so the emitted operand order no longer matters.
// ---------------------------------------------------------------------------

#[inline(always)]
fn is_nan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000 && (b & 0x007F_FFFF) != 0
}

/// Quiet a NaN the way SSE does (set the Q-bit; already-quiet NaNs unchanged).
#[inline(always)]
fn quiet_bits(b: u32) -> u32 {
    b | 0x0040_0000
}

/// `mulss dest, src` with `dest` forced as the NaN winner.
#[inline(always)]
fn mul_ss(dest: f32, src: f32) -> f32 {
    let db = dest.to_bits();
    let sb = src.to_bits();
    if is_nan_bits(db) {
        return f32::from_bits(quiet_bits(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet_bits(sb));
    }
    dest * src
}

/// `addss dest, src` with `dest` forced as the NaN winner.
#[inline(always)]
fn add_ss(dest: f32, src: f32) -> f32 {
    let db = dest.to_bits();
    let sb = src.to_bits();
    if is_nan_bits(db) {
        return f32::from_bits(quiet_bits(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet_bits(sb));
    }
    dest + src
}
/// Call the virtual sample hook at vtable slot +0x24 on a lookup result.
///
/// The original loads the target through the object's vtable and calls it as
/// thiscall/0, taking the float result from the x87 stack. Both sides land on
/// the same checker-planted stub; an `f32` return type reads ST0 directly.
#[inline(always)]
unsafe fn virt_sample(obj: u32) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(0x24) as *const u32);
    let hook: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(target as usize);
    hook(obj)
}

/// Absolute value through the sign-bit mask, as the original's `andps` does.
#[inline(always)]
fn f32_abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFF_FFFF)
}

/// Negation through the sign-bit flip, as the original's `xorps` with -0.0.
#[inline(always)]
fn f32_neg_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

// ---------------------------------------------------------------------------
// Bit-exact SSE scalar ops with FORCED operand order.
//
// A Rust `a * b` usually compiles to `mulss` with `a` in the destination, but
// the register allocator is free to swap the operands (it prefers whichever
// value is already in a register). For ordinary values that is harmless, but
// when both operands are NaNs with different payloads the destination's NaN
// wins, so a swap changes the result bits. These helpers implement the Intel
// rule explicitly (destination NaN wins, quieted; else source NaN; else the
// hardware op, whose NaN-free inputs make its own operand order irrelevant)
// so the emitted operand order no longer matters.
// ---------------------------------------------------------------------------

#[inline(always)]
fn is_nan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000 && (b & 0x007F_FFFF) != 0
}
/// Call the virtual sample hook at vtable slot +0x24 on a lookup result.
///
/// The original loads the target through the object's vtable and calls it as
/// thiscall/0, taking the float result from the x87 stack. Both sides land on
/// the same checker-planted stub; an `f32` return type reads ST0 directly.
#[inline(always)]
unsafe fn virt_sample(obj: u32) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(0x24) as *const u32);
    let hook: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(target as usize);
    hook(obj)
}

/// Absolute value through the sign-bit mask, as the original's `andps` does.
#[inline(always)]
fn f32_abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFF_FFFF)
}

/// Negation through the sign-bit flip, as the original's `xorps` with -0.0.
#[inline(always)]
fn f32_neg_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

// ---------------------------------------------------------------------------
// Bit-exact SSE scalar ops with FORCED operand order.
//
// A Rust `a * b` usually compiles to `mulss` with `a` in the destination, but
// the register allocator is free to swap the operands (it prefers whichever
// value is already in a register). For ordinary values that is harmless, but
// when both operands are NaNs with different payloads the destination's NaN
// wins, so a swap changes the result bits. These helpers implement the Intel
// rule explicitly (destination NaN wins, quieted; else source NaN; else the
// hardware op, whose NaN-free inputs make its own operand order irrelevant)
// so the emitted operand order no longer matters.
// ---------------------------------------------------------------------------

#[inline(always)]
fn is_nan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000 && (b & 0x007F_FFFF) != 0
}

/// Quiet a NaN the way SSE does (set the Q-bit; already-quiet NaNs unchanged).
#[inline(always)]
fn quiet_bits(b: u32) -> u32 {
    b | 0x0040_0000
}
}
use self::fn_00acc3f0_rt::*;

export!(thiscall, rw_00acc3f0(obj: *mut u8, key: u32, dt: f32) -> u32 {
    unsafe {
        // Snapshot the current level, then scale the voice gain by a fresh
        // hook sample and the global gain.
        let level = *(obj.add(0x80) as *const f32);
        *(obj.add(0x84) as *mut f32) = level;
        let gain = *global::<f32>(0x103F354);
        let found: u32 = callee_thiscall!(1, u32, key);
        let base = *(obj.add(0x140) as *const f32) * virt_sample(found) * gain;

        let flags = *(obj.add(0x164) as *const u32);
        // Bit 1 selects the live-aux mix; otherwise the seeded mix runs.
        let (mix, scale, eax_out) = if (flags >> 1) & 1 == 1 {
            let found2: u32 = callee_thiscall!(1, u32, key);
            let aux = *(obj.add(0x14) as *const u32);
            let m = mul_ss(
                mul_ss(
                    mul_ss(
                        *((aux.wrapping_add(0x8C)) as *const f32),
                        *(obj.add(0x24) as *const f32),
                    ),
                    virt_sample(found2),
                ),
                gain,
            );
            (m, 100.0f32, flags >> 1)
        } else {
            *(obj.add(0xB0) as *mut u32) = 0;
            *(obj.add(0xB4) as *mut u32) = 0;
            *(obj.add(0xB8) as *mut u32) = 0;
            // Bit 15 selects the seed: 2.0 or 0.1.
            let seed = if (flags >> 15) & 1 == 1 {
                2.0f32
            } else {
                f32::from_bits(0x3DCC_CCCD)
            };
            let found3: u32 = callee_thiscall!(1, u32, key);
            let cand = mul_ss(
                mul_ss(*(obj.add(0x158) as *const f32), virt_sample(found3)),
                gain,
            );
            // comiss seed, cand; ja keep seed.
            let m = if seed > cand { seed } else { cand };
            (m, 10.0f32, flags >> 15)
        };

        // Merge the two mix terms and integrate the level with a dead-zone:
        // overshoot clamps the level to zero, otherwise it moves towards the
        // mix by the scaled amount.
        let mut mix = add_ss(mix, base);
        let current = *(obj.add(0x80) as *const f32);
        let mut push = scale * mix;
        *(obj.add(0x130) as *mut u32) = 0;
        *(obj.add(0x134) as *mut u32) = 0;
        push *= dt;
        *(obj.add(0x138) as *mut u32) = 0;
        *(obj.add(0x148) as *mut u32) = 0;
        // comiss push, |current|; jbe integrate.
        if push > f32_abs_bits(current) {
            *(obj.add(0x80) as *mut f32) = 0.0;
        } else {
            mix *= dt;
            mix = mul_ss(mix, scale);
            // comiss current, 0.0; jbe add-path.
            if current > 0.0 {
                *(obj.add(0x80) as *mut f32) = current - mix;
            } else {
                *(obj.add(0x80) as *mut f32) = mix + current;
            }
        }

        // Decay the auxiliary envelope towards zero at 50/12 units per tick,
        // switching rate 30 units above the current value.
        let mut env = *(obj.add(0x8C) as *const f32);
        let over = env - 30.0f32;
        // comiss over, 0.0; jb slow-path. The slow rate lives in a relocated
        // slot, so it is read from the mapped image, not hard-coded.
        let rate = if over >= 0.0 {
            50.0f32
        } else {
            *global::<f32>(0xFE8B0C)
        };
        env -= rate * dt;
        // comiss 0.0, env; ja keep zero (clamp at zero).
        let env = if 0.0f32 > env { 0.0f32 } else { env };

        // Advance the phase and wrap it into [-pi, pi].
        let fresh = *(obj.add(0x80) as *const f32);
        let mut phase = mul_ss(dt, fresh);
        *(obj.add(0x8C) as *mut f32) = env;
        phase += *(obj.add(0x7C) as *const f32);
        *(obj.add(0x7C) as *mut f32) = phase;
        // comiss phase, pi; jbe neg-wrap.
        if phase > core::f32::consts::PI {
            phase -= core::f32::consts::TAU;
            *(obj.add(0x7C) as *mut f32) = phase;
        } else if -core::f32::consts::PI > phase {
            // comiss -pi, phase; jbe done (add 2pi only when -pi > phase).
            phase += core::f32::consts::TAU;
            *(obj.add(0x7C) as *mut f32) = phase;
        }
        // EAX holds the shifted flag word the branch tests left behind.
        eax_out
    }
});
