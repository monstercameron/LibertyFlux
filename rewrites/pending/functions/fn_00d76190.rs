// original: 0x00d76190 SubmitScaledQuad
//! Scaled-quad submit.
//!
//! `this` points at the owner's object; `corner` points at two f32 base
//! coordinates and `scale_pair` at two f32 scale factors. Looks an object
//! up through a manager global (null exits), takes the ratio of two queried
//! floats, scales the pair by it, programs two render states, and unless
//! the linked word at `this+4` is zero, combines the corners with the
//! scaled pair and submits the quad. Returns the last helper answer taken
//! (0 on the null path). The original spills one intermediate over its own
//! incoming stack slot; the rewrite threads the value directly.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

const D_LOOKUP: u32 = 1;
const D_GET_A: u32 = 2;
const D_GET_B: u32 = 3;
const D_SETSTATE: u32 = 4;
const D_UPDATE: u32 = 5;
const D_FINAL: u32 = 6;

export!(thiscall, rw_00d76190(this: u32, corner: u32, scale_pair: u32) -> u32 {
    unsafe {
        let bias = *global::<f32>(0x00FE8AE0);
        let gain = *global::<f32>(0x00FE8830);
        let manager = *global::<u32>(0x0103E49C);
        let obj = callee_thiscall!(D_LOOKUP, u32, manager, 0x25u32, 0u32);
        if obj == 0 {
            return 0;
        }
        let first: f32 = callee_thiscall!(D_GET_A, f32, obj);
        let second: f32 = callee_thiscall!(D_GET_B, f32, obj);
        let ratio = first / second;
        let s0 = *(scale_pair as *const f32);
        let s1 = *((scale_pair + 4) as *const f32);
        // Operand order matches the original exactly, so NaN payloads and
        // rounding propagate bit-identically.
        let t1 = ratio * s0 + bias;
        let t2 = s1 * ratio + bias;
        callee_cdecl!(D_SETSTATE, u32, 0x0au32, 0u32);
        let state_answer = callee_cdecl!(D_SETSTATE, u32, 2u32, 0u32);
        if *((this + 4) as *const u32) == 0 {
            return state_answer;
        }
        callee_thiscall!(D_UPDATE, u32, this + 4);
        let u1 = s0 * gain;
        let u0 = s1 * gain;
        let v2 = *(corner as *const f32) + u1;
        let v1 = *((corner + 4) as *const f32) + u0;
        let mut s5 = t1 * gain;
        let mut s6 = t2 * gain;
        let w4 = v2 - s5;
        s5 += v2;
        let w2 = v1 - s6;
        s6 += v1;
        let one = 1.0f32.to_bits();
        // The original points this argument at its own second-arg slot after
        // storing a marker const there; the address is skipped and the
        // marker word compared (see contract).
        let marker = 0xffff8000u32;
        let marker_ptr = &marker as *const u32 as u32;
        callee_cdecl!(
            D_FINAL, u32,
            w4.to_bits(), w2.to_bits(), s5.to_bits(), s6.to_bits(),
            0u32, 0u32, 0u32, one, one, marker_ptr, 0u32
        )
    }
});
