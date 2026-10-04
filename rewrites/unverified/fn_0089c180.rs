// original: 0x0089C180 audio_loop_gain_update
//! Loop gain update: builds two base levels from integer counters plus
//! optional float trims, asks a mixer callee for a combined level, clamps it
//! against unity gain, publishes the scale through a setter, decays a stored
//! level, then folds two truncated float trims into counters for a final
//! randomized pick whose result is added to the incoming argument.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

/// x86 `cvttss2si` + low-16-bits: NaN and out-of-range inputs yield
/// 0x80000000, whose low half is 0. (Rust `as` saturates instead, so the
/// range is gated explicitly; in-range values truncate toward zero either way.)
fn trunc16(x: f32) -> u32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0
    } else {
        (x as i32) as u16 as u32
    }
}

fn opt_trim(p: u32) -> f32 {
    if p == 0 {
        0.0
    } else {
        unsafe { *(p as *const f32) }
    }
}

fn opt_trunc(p: u32) -> u32 {
    if p == 0 {
        0
    } else {
        trunc16(unsafe { *(p as *const f32) })
    }
}

export!(thiscall, rw_0089C180(this: u32, arg0: u32) -> u32 {
    unsafe {
        let one = *global::<f32>(0xFE88E8);
        let w = |off: usize| -> u32 { *((this as *const u8).add(off) as *const u32) };
        // Base levels: integer counters converted exactly, plus optional trims.
        // Operand order mirrors the original (trim + base) for NaN-bit safety.
        let v1 = opt_trim(w(0xC4)) + w(0xB4) as f32;
        let v2 = opt_trim(w(0xC8)) + w(0xB8) as f32;
        let m: f32 = callee_cdecl!(1, f32, v1.to_bits(), v2.to_bits());
        // comiss+jbe: taken unless m > 1, including NaN. `m > one` matches it.
        let scale = if m > one { one / m } else { one };
        callee_thiscall!(2, u32, this.wrapping_add(0xDC), scale.to_bits(), scale.to_bits());

        let d4 = *((this as *const u8).add(0xD4) as *const f32);
        *((this as *mut u8).add(0xD4) as *mut f32) = one - d4;

        let lo = w(0xBC).wrapping_add(opt_trunc(w(0xCC)));
        let hi = w(0xC0).wrapping_add(opt_trunc(w(0xD0)));
        let r: u32 = callee_cdecl!(3, u32, lo, hi);
        let out = r.wrapping_add(arg0);
        *((this as *mut u8).add(0xB0) as *mut u32) = out;
        out
    }
});
