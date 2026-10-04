// original: 0x00E3E570 compose_ui_element_colors
//! Compose one UI element's color block.
//!
//! Gated on the element's signed count word at +0x30 (nothing happens when it
//! is zero or negative). Derives two adjusted floats from the inputs, resolves
//! a base color through two intercepted lookup calls, clamps a looked-up
//! intensity between 0 and a second lookup, packs the clamped byte over the
//! base color's low 24 bits, notifies the element slot selected by the flag
//! byte, and submits the nine-word block through the final intercepted call.
//! The intensity byte is the notify call's answer when it fires, otherwise the
//! low byte of the first lookup.
//!
//! Semantically void: the original leaves the last callee's answer in `eax`
//! (or the incoming `eax` on the early path); the rewrite returns the last
//! answer, or 0 on the early path.
//!
//! Verified by checker v3: 1000/1000 trials, 3 branch shapes, honesty mutant
//! (high selector off by one) fails on the call log. Lane r-b148.

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated};

/// Truncate an `f32` to `i32` exactly like x86 `cvttss2si`.
///
/// Rust's `as` saturates on overflow and maps NaN to 0; the instruction
/// returns `i32::MIN` for NaN and for magnitudes at or above 2^31 (negative
/// overflow already coincides with saturation, so only those need mapping).
#[inline(always)]
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

export!(thiscall, rw_00e3e570(this: u32, flag: u32, a: f32, b: f32) -> u32 {
    unsafe { compose_element(this, flag, a, b, 0x41) }
});

/// Shared body; `hi_sel` is the lookup selector used when the mode check passes.
unsafe fn compose_element(this: u32, flag: u32, a: f32, b: f32, hi_sel: u32) -> u32 {
    const UP_K: u32 = 0x00FE8748;
    const DN_K: u32 = 0x00FE8734;
    const MODE: u32 = 0x011D6FD4;
    const NOTIFY_FLAG: u32 = 0x01161548;
    const LO_SEL: u32 = 0x3B;
    const LOOKUP_SEL: u32 = 0x37;
    unsafe {
        let base = this as *mut u8;
        if (base.add(0x30) as *const i32).read() <= 0 {
            return 0;
        }
        let dn = a - global::<f32>(DN_K).read();
        let _up = b + global::<f32>(UP_K).read();
        let mut buf = [dn.to_bits(), b.to_bits()];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let ok = callee_cdecl!(1, u32, 0);
        let sel = if (ok as u8) != 0 && global::<u32>(MODE).read() != 2 {
            hi_sel
        } else {
            LO_SEL
        };
        let p2 = callee_cdecl!(2, u32, buf_ptr, sel);
        let argb_base = (p2 as *const u32).read();
        let p3 = callee_cdecl!(3, u32, buf_ptr, LOOKUP_SEL);
        let first = cvttss2si(f32::from_bits((p3 as *const u32).read()));
        // The level byte is whatever `al` holds here: the notify call's
        // answer when it fires, otherwise the low byte of the first lookup.
        let level_byte = if global::<u8>(NOTIFY_FLAG).read() != 0 {
            callee_thiscall!(4, u32, relocated(NOTIFY_FLAG)) as u8
        } else {
            first as u8
        };
        let p3b = callee_cdecl!(3, u32, buf_ptr, LOOKUP_SEL);
        let cap = f32::from_bits((p3b as *const u32).read());
        let level = level_byte as f32;
        // Clamp: 0 when level is negative, else level capped at `cap`.
        // `f > cap` is false for unordered inputs, matching the jbe fallthrough.
        let clamped = if 0.0f32 > level {
            0.0
        } else if level > cap {
            cap
        } else {
            level
        };
        let alpha = cvttss2si(clamped) as u8;
        let argb = ((alpha as u32) << 24) | (argb_base & 0x00FF_FFFF);
        let slot = if (flag as u8) != 0 { 10u32 } else { 11u32 };
        callee_thiscall!(5, u32, this.wrapping_add(slot * 4));
        let a_bits = a.to_bits();
        let dn_bits = dn.to_bits();
        let one = 1.0f32.to_bits();
        let mut w = [a_bits, a_bits, dn_bits, dn_bits, one, one, 0, 0, argb];
        let wp = w.as_mut_ptr();
        callee_cdecl!(
            6, u32,
            wp.add(0) as u32, wp.add(1) as u32, wp.add(2) as u32,
            wp.add(3) as u32, wp.add(4) as u32, wp.add(5) as u32,
            wp.add(6) as u32, wp.add(7) as u32, wp.add(8) as u32
        );
        callee_stdcall!(7, u32,)
    }
}
