// original: 0x00accca0 audio_voice_xfade_update
mod fn_00accca0_rt {
/// Call a planted virtual hook with one stack argument.
#[inline(always)]
unsafe fn virt_call1(obj: u32, slot: usize, arg: u32) -> u32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(slot) as *const u32);
    let hook: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
    hook(obj, arg)
}
#[inline(always)]
unsafe fn rd_f(obj: *const u8, off: usize) -> f32 {
    *(obj.add(off) as *const f32)
}
#[inline(always)]
unsafe fn wr_f(obj: *mut u8, off: usize, v: f32) {
    *(obj.add(off) as *mut f32) = v;
}
#[inline(always)]
unsafe fn rd_32(obj: *const u8, off: usize) -> u32 {
    *(obj.add(off) as *const u32)
}
#[inline(always)]
unsafe fn wr_32(obj: *mut u8, off: usize, v: u32) {
    *(obj.add(off) as *mut u32) = v;
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
/// `subss dest, src` with `dest` forced as the NaN winner.
#[inline(always)]
fn sub_ss(dest: f32, src: f32) -> f32 {
    let db = dest.to_bits();
    let sb = src.to_bits();
    if is_nan_bits(db) {
        return f32::from_bits(quiet_bits(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet_bits(sb));
    }
    dest - src
}
/// `divss dest, src` with `dest` forced as the NaN winner.
#[inline(always)]
fn div_ss(dest: f32, src: f32) -> f32 {
    let db = dest.to_bits();
    let sb = src.to_bits();
    if is_nan_bits(db) {
        return f32::from_bits(quiet_bits(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet_bits(sb));
    }
    dest / src
}
/// `comiss a, b; ja keep-b; else a`: NaN on either side yields `a`.
///
/// Same guarding as `max_ss`: a bare `if a > b { b } else { a }` could be
/// folded into `minss`/`minnum`, which forward the wrong NaN.
#[inline(always)]
fn min_ss(a: f32, b: f32) -> f32 {
    if is_nan_bits(a.to_bits()) || is_nan_bits(b.to_bits()) {
        return a;
    }
    if a > b {
        b
    } else {
        a
    }
}
/// `comiss a, b; ja keep-a; else b`: NaN on either side yields `b`.
///
/// Written with explicit NaN guards so LLVM cannot fold it into a `maxss`
/// or `maxnum` with different NaN forwarding; the fallback runs only on
/// NaN-free inputs, where any lowering agrees.
#[inline(always)]
fn max_ss(a: f32, b: f32) -> f32 {
    if is_nan_bits(a.to_bits()) || is_nan_bits(b.to_bits()) {
        return b;
    }
    if a > b {
        a
    } else {
        b
    }
}
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
use self::fn_00accca0_rt::*;

/// Audio voice crossfade update through the global manager.
///
/// Builds a blend factor from the fade position (zero past the end, a
/// scaled span at/below zero, a ratio-clamped ramp in between), adds a
/// flag-selected tuning offset, clamps the mixed level, then resolves the
/// output handle twice through the global audio manager's virtual hook
/// and mixes the final sample with the global crossfade constant.
/// Returns the resolved handle, or the input record when the voice is
/// parked (in which case it also flags the record).
export!(thiscall, rw_00accca0(obj: *mut u8, arg0: u32) -> u32 {
    unsafe {
        // Blend factor from the fade position: zero past the end, a scaled
        // span below it, and a ratio-clamped ramp in between.
        let pos = rd_f(obj as *const u8, 0x160);
        let blend = if !(1000.0f32 > pos) {
            0.0f32
        } else if !(0.0f32 >= pos) {
            let ratio = min_ss(div_ss(rd_f(obj as *const u8, 0x70), rd_f(obj as *const u8, 0x20)), 1.0);
            let shaped = mul_ss(sub_ss(1.0, mul_ss(pos, f32::from_bits(0x3A83_126F))), ratio);
            mul_ss(
                sub_ss(rd_f(obj as *const u8, 8), rd_f(obj as *const u8, 0xC)),
                max_ss(0.0, shaped),
            )
        } else {
            sub_ss(rd_f(obj as *const u8, 8), rd_f(obj as *const u8, 0xC))
        };
        // Flag-selected tuning offset: none, 0.03, or 0.05.
        let flags = rd_32(obj as *const u8, 0x164);
        let blend = if flags & 0x0C00_0000 != 0 {
            let tune = if (flags >> 27) & 1 == 1 {
                f32::from_bits(0x3D4C_CCCD)
            } else {
                f32::from_bits(0x3CF5_C28F)
            };
            add_ss(tune, blend)
        } else {
            blend
        };
        // Clamp the mixed level and resolve the output handle twice through
        // the global audio manager.
        let mixed = min_ss(
            add_ss(rd_f(obj as *const u8, 0x1C), blend),
            rd_f(obj as *const u8, 0x70),
        );
        let key = rd_32(obj as *const u8, 0xD8);
        wr_f(obj, 0x78, mixed);
        let mgr = *global::<u32>(0x18B8968);
        let got = virt_call1(mgr, 0x14, key);
        wr_32(obj, 0xE0, rd_32(got as *const u8, 0x18));
        let got2 = virt_call1(mgr, 0x14, key);
        let k = *global::<f32>(0x12DDE98);
        let out = add_ss(mul_ss(add_ss(rd_f(got2 as *const u8, 0x1C), 1.0), k), sub_ss(1.0, k));
        wr_f(obj, 0xE4, out);
        if rd_32(obj as *const u8, 0xD0) == 0 {
            got2
        } else {
            *((arg0.wrapping_add(0xF17)) as *mut u8) |= 0x40;
            arg0
        }
    }
});
