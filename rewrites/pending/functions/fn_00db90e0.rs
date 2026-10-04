// original: 0x00db90e0 UIMusicClip::vf97
//! Rewrite of UIMusicClip::vf97 (audio UI music clip tick).
//!
//! Samples two objects' virtual float hooks, blends the samples with a shared
//! scale factor, reduces the blends with ordered min/max idioms, clamps two
//! runtime-scaled counters, and dispatches on the results: when every
//! comparison is ordered-greater, calls one of two sibling virtuals selected
//! by a flag byte; otherwise returns the last sample's bits (the value the
//! final hook call left in EAX, observed identically on both sides).
//!
//! Float comparisons replicate `comiss`+`jcc` flag semantics explicitly
//! instead of `f32::min/max`, which differ on NaN (unordered comparisons
//! keep the old value here). Two mirror idioms exist because the original
//! compares in both operand orders; mixing them up fails verification.

use lf_k2_rt::{callee_thiscall, export, global, relocated};

/// A virtual sample hook: thiscall with no stack args, float result on ST0.
type VSample = extern "thiscall" fn(u32) -> f32;

/// Call the sample hook at vtable `slot` on `obj`, like the original.
#[inline(always)]
unsafe fn vsample(obj: u32, slot: usize) -> f32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(slot) as *const u32);
    let f: VSample = core::mem::transmute(target as usize);
    f(obj)
}

/// The `comiss new,old` + `jbe keep` idiom: keep `old` unless `new` is
/// ordered-greater. NaN on either side keeps `old` (unordered jumps).
#[inline(always)]
fn unless_greater(new: f32, old: f32) -> f32 {
    if new > old {
        new
    } else {
        old
    }
}

/// The mirrored `comiss old,new` + `jbe keep` idiom: keep `old` unless `old`
/// itself is ordered-greater than `new`. NaN on either side keeps `old`.
#[inline(always)]
fn unless_old_greater(old: f32, new: f32) -> f32 {
    if old > new {
        new
    } else {
        old
    }
}

export!(thiscall, rw_rb57_90e0(this: u32, _arg: u32) -> u32 {
    unsafe {
        let k = *global::<f32>(0xfe8830);
        let a = vsample(this, 0xc8);
        let b = vsample(this, 0xb8);
        let s10 = a - b * k;
        let c = vsample(this, 0xb8);
        let held0 = c * k;
        let d = vsample(this, 0xc8);
        let s18 = d + held0;
        let e = vsample(this, 0xd0);
        let f = vsample(this, 0xc0);
        let s20 = e - f * k;
        let g = vsample(this, 0xc0);
        let held1 = g * k;
        let h = vsample(this, 0xd0);
        let scratch = h + held1;
        let other: u32 =
            callee_thiscall!(1, u32, relocated(0x1981a4c), relocated(0xef34bc));
        let i = vsample(this, 0xc8);
        let j = vsample(other, 0xb8);
        let s14 = i - j * k;
        let kk = vsample(other, 0xb8);
        let held2 = kk * k;
        let l = vsample(this, 0xc8);
        let s1c = l + held2;
        let m = vsample(this, 0xd0);
        let n = vsample(other, 0xc0);
        let s24 = m - n * k;
        let o = vsample(other, 0xc0);
        let held3 = o * k;
        let p = vsample(this, 0xd0);
        let p_bits = p.to_bits();
        let x7 = unless_greater(s14, s10);
        let x6 = unless_old_greater(s18, s1c);
        let x5 = unless_greater(s24, s20);
        let y = p + held3;
        let x4 = unless_old_greater(scratch, y);
        let cap = *global::<f32>(0xfe88e8);
        let mut x0 = (*global::<i32>(0x18b7a8c) as f32) * *global::<f32>(0x17accf0);
        x0 = unless_greater(0.0, x0);
        if x0 > cap {
            x0 = cap;
        }
        let w = (*global::<i32>(0x18b7a80) as f32) * *global::<f32>(0x17acce8);
        let x1 = if 0.0 > w {
            0.0
        } else if w > cap {
            cap
        } else {
            w
        };
        if x1 > x7 && x6 > x1 && x0 > x5 && x4 > x0 {
            let flag = *((this as *const u8).add(0x212));
            if flag == 0 {
                callee_thiscall!(3, u32, this)
            } else {
                callee_thiscall!(2, u32, this)
            }
        } else {
            p_bits
        }
    }
});
