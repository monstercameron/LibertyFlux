// original: 0x008adb80 audio_filter_state_update
use lf_checker_rt::{callee_thiscall, export, global};

#[inline(always)]
unsafe fn g32(va: u32) -> u32 {
    *global::<u32>(va)
}

#[inline(always)]
unsafe fn g16(va: u32) -> u16 {
    *global::<u16>(va)
}

#[inline(always)]
unsafe fn g64(lo_bytes: u32, tab: u32) -> u64 {
    *global::<u64>(tab.wrapping_add(lo_bytes))
}

#[inline(always)]
unsafe fn s32(va: u32, v: u32) {
    *global::<u32>(va) = v;
}

#[inline(always)]
unsafe fn h32(base: u32, disp: i32) -> u32 {
    ((base.wrapping_add(disp as u32)) as *const u32).read_unaligned()
}

#[inline(always)]
unsafe fn h16(base: u32, disp: i32) -> u16 {
    ((base.wrapping_add(disp as u32)) as *const u16).read_unaligned()
}

#[inline(always)]
unsafe fn hs32(base: u32, disp: i32, v: u32) {
    ((base.wrapping_add(disp as u32)) as *mut u32).write_unaligned(v);
}

#[inline(always)]
unsafe fn h32sx(base: u32, idx: u32, scale: u32, disp: i32) -> u32 {
    h32(base.wrapping_add(idx.wrapping_mul(scale)), disp)
}

#[inline(always)]
fn bits(x: f32) -> u32 {
    core::hint::black_box(x).to_bits()
}

/// Pinned single-precision operand: the barrier stops the optimiser from
/// swapping the operands of a binary float operation (load folding picks
/// either side as the memory source), which would change which non-number
/// payload survives when both operands are non-numbers.
#[inline(always)]
fn fb(v: u32) -> f32 {
    core::hint::black_box(f32::from_bits(v))
}

#[inline(always)]
fn bb64(x: f64) -> f64 {
    core::hint::black_box(x)
}

/// One lane of a packed single-precision square root: quiet propagation for
/// non-numbers, signed zero kept, negatives to the negative quiet default.
#[inline(always)]
fn sqrt_lane(v: u32) -> u32 {
    let x = f32::from_bits(v);
    if x.is_nan() {
        v | 0x0040_0000
    } else if x == 0.0 {
        v
    } else if x < 0.0 {
        0xFFC0_0000
    } else {
        bits(x.sqrt())
    }
}

/// Truncating single-precision to 64-bit integer conversion: out of range
/// (including non-numbers and infinities) yields the indefinite minimum.
#[inline(always)]
fn trunc_f32_to_i64(v: u32) -> i64 {
    let x = f32::from_bits(v);
    if x.is_nan() || x >= 9.223372036854776e18 || x < -9.223372036854776e18 {
        i64::MIN
    } else {
        x as i64
    }
}

// ---------------------------------------------------------------------------
// 0x008ADB80 audio_filter_state_update
// ---------------------------------------------------------------------------

/// Audio filter state updater.
///
/// Runs two helpers (callees 1 and 2) that fill the work buffers, then walks
/// the persistent filter state block: each guarded section initialises its
/// state words on first use (flag bits in the shared flag word record which
/// sections have run) and every pass recomputes the stage outputs from the
/// live inputs, the stored state and the read-only coefficient tables,
/// clamping through ordered comparisons. A truncated conversion refreshes
/// the integer accumulator, and a final four-step loop folds the last stage
/// into the caller's output slots. Returns the address one past the last
/// output slot written.
///
/// Lanes and registers are modelled bit-precisely: scalar loads zero the
/// upper lanes, packed moves copy all four, comparisons snapshot their
/// operands (later instructions may overwrite them before the branch),
/// every binary float operation pins its operand order so the same
/// non-number payload survives as in the original, and the packed square
/// root, the truncating conversion and the integer to double widening
/// each match the hardware edge cases.
export!(thiscall, rw_008adb80(ecx: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let mut ecx = ecx;
        let mut frame = [0u32; 32];
        let mut eax = 0u32;
        let mut edx = 0u32;
        let mut ebx = 0u32;
        let mut esi = arg1;
        let mut edi = arg0.wrapping_add(0x70);
        let mut x0 = [0u32; 4];
        let mut x1 = [0u32; 4];
        let mut x2 = [0u32; 4];
        let mut x3 = [0u32; 4];
        let mut x4 = [0u32; 4];
        let mut x5 = [0u32; 4];
        let mut x6 = [0u32; 4];
        let mut x7 = [0u32; 4];
        let mut pd0 = 0f64;
        let mut pd1 = 0f64;
        let mut x87v = 0u32;
        let _ = ebx;
        frame[0x0A] = ecx;
        let _r1: u32 = callee_thiscall!(1, u32, ecx, frame.as_mut_ptr().add(0x1C) as u32, edi, esi);
        // the second call pushes the incoming second argument first and only
        // then reloads the saved receiver into esi/ecx for the call itself
        let _r2: u32 = callee_thiscall!(2, u32, ecx, frame.as_mut_ptr().add(0x10) as u32, edi, esi);
        esi = ecx;
        x0[0] = frame[0x1C];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x1[0] = frame[0x1D];
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x0[0] = bits(fb(x0[0]) * fb(x0[0]));
        x1[0] = bits(fb(x1[0]) * fb(x1[0]));
        edx = g32(0x0115FDC4);
        x1[0] = bits(fb(x1[0]) + fb(x0[0]));
        x0[0] = frame[0x1E];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) * fb(x0[0]));
        x1[0] = bits(fb(x1[0]) + fb(x0[0]));
        x2[0] = sqrt_lane(x1[0]);
        x2[1] = sqrt_lane(x1[1]);
        x2[2] = sqrt_lane(x1[2]);
        x2[3] = sqrt_lane(x1[3]);
        frame[0xB] = x2[0];
        let cf_a0 = (edx & 0xFF); let cf_b0 = 0x00000001u32;
        if ((cf_a0) & (cf_b0)) == 0 {
         x0[0] = g32(0x00FE8B9C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x5[0] = g32(0x00FE8B48);
         x5[1] = 0; x5[2] = 0; x5[3] = 0;
         x4[0] = g32(0x00E7CBA8);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         x3[0] = g32(0x00E7CBA4);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x6[0] = g32(0x00FE8CB0);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x7[0] = g32(0x00FE8734);
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
         x1[0] = g32(0x00E7CB80);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         frame[0x9] = x0[0];
         s32(0x0115FDB0, x0[0]);
         x0[0] = g32(0x00FE8BD0);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         edx = edx | (0x00000001u32);
         s32(0x0115FDC4, edx);
         s32(0x0115FDA4, x5[0]);
         s32(0x0115FDA8, x4[0]);
         s32(0x0115FDB4, x3[0]);
         frame[0x8] = x0[0];
         s32(0x0115FDBC, x0[0]);
         s32(0x0115FDC0, x6[0]);
         s32(0x0115FDAC, x7[0]);
         s32(0x0115FDB8, x1[0]);
        } else {
         x0[0] = g32(0x0115FDBC);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x6[0] = g32(0x0115FDC0);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x1[0] = g32(0x0115FDB8);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x3[0] = g32(0x0115FDB4);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x7[0] = g32(0x0115FDAC);
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
         x4[0] = g32(0x0115FDA8);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         x5[0] = g32(0x0115FDA4);
         x5[1] = 0; x5[2] = 0; x5[3] = 0;
         frame[0x8] = x0[0];
         x0[0] = g32(0x0115FDB0);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         frame[0x9] = x0[0];
        }
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        x0[0] = bits(fb(x0[0]) - fb(frame[0x9]));
        x1[0] = bits(fb(x1[0]) * fb(x0[0]));
        frame[0xF] = x0[0];
        x0[0] = x6[0]; x0[1] = x6[1]; x0[2] = x6[2]; x0[3] = x6[3];
        x0[0] = bits(fb(x0[0]) - fb(x3[0]));
        x1[0] = bits(fb(x1[0]) * fb(x0[0]));
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        x0[0] = bits(fb(x0[0]) - fb(x5[0]));
        x5[0] = x5[0] ^ x5[0];
        x5[1] = x5[1] ^ x5[1];
        x5[2] = x5[2] ^ x5[2];
        x5[3] = x5[3] ^ x5[3];
        x1[0] = bits(fb(x1[0]) + fb(x3[0]));
        let cf_a1 = x0[0]; let cf_b1 = x5[0];
        if f32::from_bits(cf_a1) >= f32::from_bits(cf_b1) {
         x7[0] = bits(fb(x7[0]) * fb(x0[0]));
         x3[0] = bits(fb(x3[0]) - fb(x4[0]));
         x7[0] = bits(fb(x7[0]) * fb(x3[0]));
         x7[0] = bits(fb(x7[0]) + fb(x4[0]));
        } else {
         x7[0] = x4[0]; x7[1] = x4[1]; x7[2] = x4[2]; x7[3] = x4[3];
        }
        x0[0] = frame[0xF];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        let cf_a2 = x0[0]; let cf_b2 = x5[0];
        if f32::from_bits(cf_a2) >= f32::from_bits(cf_b2) {
         x7[0] = x1[0]; x7[1] = x1[1]; x7[2] = x1[2]; x7[3] = x1[3];
        }
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        x0[0] = bits(fb(x0[0]) - fb(frame[0x8]));
        let cf_a3 = x0[0]; let cf_b3 = x5[0];
        if f32::from_bits(cf_a3) >= f32::from_bits(cf_b3) {
         x7[0] = x6[0]; x7[1] = x6[1]; x7[2] = x6[2]; x7[3] = x6[3];
        }
        x1[0] = g32(0x00FE8B5C);
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x6[0] = g32(0x00FE88E8);
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        frame[0x7] = x1[0];
        let cf_a4 = (edx & 0xFF); let cf_b4 = 0x00000002u32;
        if ((cf_a4) & (cf_b4)) == 0 {
         x0[0] = g32(0x00FE8AD8);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x3[0] = g32(0x00E7CB84);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x4[0] = g32(0x00FE8830);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         frame[0x8] = x0[0];
         s32(0x0115FDC8, x0[0]);
         x0[0] = g32(0x00E7CB9C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         frame[0x9] = x0[0];
         s32(0x0115FDD4, x0[0]);
         s32(0x0115FDE0, x1[0]);
         edx = edx | (0x00000002u32);
         x0[0] = x6[0]; x0[1] = x6[1]; x0[2] = x6[2]; x0[3] = x6[3];
         x1[0] = x3[0]; x1[1] = x3[1]; x1[2] = x3[2]; x1[3] = x3[3];
         s32(0x0115FDC4, edx);
         s32(0x0115FDCC, x5[0]);
         s32(0x0115FDD8, x4[0]);
         s32(0x0115FDE4, x0[0]);
         s32(0x0115FDD0, x3[0]);
         s32(0x0115FDDC, x1[0]);
        } else {
         x0[0] = g32(0x0115FDD4);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x1[0] = g32(0x0115FDDC);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x4[0] = g32(0x0115FDD8);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         x3[0] = g32(0x0115FDD0);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         frame[0x9] = x0[0];
         x0[0] = g32(0x0115FDC8);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         frame[0x8] = x0[0];
         x0[0] = g32(0x0115FDE4);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x6[0] = x2[0]; x6[1] = x2[1]; x6[2] = x2[2]; x6[3] = x2[3];
        x6[0] = bits(fb(x6[0]) - fb(frame[0x9]));
        x0[0] = bits(fb(x0[0]) - fb(x4[0]));
        x1[0] = bits(fb(x1[0]) * fb(x6[0]));
        frame[0xF] = x6[0];
        x1[0] = bits(fb(x1[0]) * fb(x0[0]));
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        x0[0] = bits(fb(x0[0]) - fb(frame[0x8]));
        x1[0] = bits(fb(x1[0]) + fb(x4[0]));
        let cf_a5 = x0[0]; let cf_b5 = x5[0];
        if f32::from_bits(cf_a5) >= f32::from_bits(cf_b5) {
         x4[0] = bits(fb(x4[0]) - fb(g32(0x0115FDCC)));
         x3[0] = bits(fb(x3[0]) * fb(x0[0]));
         x3[0] = bits(fb(x3[0]) * fb(x4[0]));
         x3[0] = bits(fb(x3[0]) + fb(g32(0x0115FDCC)));
         frame[0x3] = x3[0];
        } else {
         x0[0] = g32(0x0115FDCC);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x3[0] = x0[0]; x3[1] = x0[1]; x3[2] = x0[2]; x3[3] = x0[3];
         frame[0x3] = x0[0];
        }
        x0[0] = frame[0xF];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        let cf_a6 = x0[0]; let cf_b6 = x5[0];
        x6[0] = g32(0x00FE88E8);
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        if f32::from_bits(cf_a6) >= f32::from_bits(cf_b6) {
         x3[0] = x1[0]; x3[1] = x1[1]; x3[2] = x1[2]; x3[3] = x1[3];
         frame[0x3] = x3[0];
        }
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FDE0)));
        let cf_a7 = x0[0]; let cf_b7 = x5[0];
        if f32::from_bits(cf_a7) >= f32::from_bits(cf_b7) {
         x3[0] = g32(0x0115FDE4);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         frame[0x3] = x3[0];
        }
        x4[0] = frame[0x12];
        x4[1] = 0; x4[2] = 0; x4[3] = 0;
        x2[0] = frame[0x10];
        x2[1] = 0; x2[2] = 0; x2[3] = 0;
        x0[0] = x4[0]; x0[1] = x4[1]; x0[2] = x4[2]; x0[3] = x4[3];
        x1[0] = x2[0]; x1[1] = x2[1]; x1[2] = x2[2]; x1[3] = x2[3];
        x0[0] = bits(fb(x0[0]) * fb(x4[0]));
        x1[0] = bits(fb(x1[0]) * fb(x2[0]));
        frame[0xC] = x0[0];
        x0[0] = bits(fb(x0[0]) + fb(x1[0]));
        frame[0xD] = x1[0];
        let cf_a8 = x0[0]; let cf_b8 = x5[0];
        if !(f32::from_bits(cf_a8) != f32::from_bits(cf_b8)) {
         x4[0] = x5[0]; x4[1] = x5[1]; x4[2] = x5[2]; x4[3] = x5[3];
        } else {
         x0[0] = sqrt_lane(x0[0]);
         x0[1] = sqrt_lane(x0[1]);
         x0[2] = sqrt_lane(x0[2]);
         x0[3] = sqrt_lane(x0[3]);
         x4[0] = x6[0]; x4[1] = x6[1]; x4[2] = x6[2]; x4[3] = x6[3];
         x4[0] = bits(fb(x4[0]) / fb(x0[0]));
        }
        x1[0] = frame[0x12];
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x0[0] = x4[0]; x0[1] = x4[1]; x0[2] = x4[2]; x0[3] = x4[3];
        x0[0] = bits(fb(x0[0]) * fb(x5[0]));
        x2[0] = bits(fb(x2[0]) * fb(x4[0]));
        x0[0] = bits(fb(x0[0]) * fb(x5[0]));
        x1[0] = bits(fb(x1[0]) * fb(x4[0]));
        frame[0xF] = x0[0];
        x4[0] = frame[0xF];
        x4[1] = 0; x4[2] = 0; x4[3] = 0;
        x6[0] = x4[0]; x6[1] = x4[1]; x6[2] = x4[2]; x6[3] = x4[3];
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        x0[0] = bits(fb(x0[0]) * fb(g32(0x00E7CB8C)));
        x6[0] = bits(fb(x6[0]) - fb(x0[0]));
        x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
        x0[0] = bits(fb(x0[0]) * fb(g32(0x00E7CB8C)));
        frame[0xE] = x6[0];
        frame[0xF] = x0[0];
        x0[0] = bits(fb(x0[0]) + fb(x6[0]));
        x6[0] = g32(0x00FE88E8);
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        let cf_a9 = x5[0]; let cf_b9 = x0[0];
        if f32::from_bits(cf_a9) > f32::from_bits(cf_b9) {
         x0[0] = x5[0]; x0[1] = x5[1]; x0[2] = x5[2]; x0[3] = x5[3];
        }
        x2[0] = bits(fb(x2[0]) * fb(g32(0x00E7CB8C)));
        x0[0] = bits(fb(x0[0]) * fb(x0[0]));
        x2[0] = bits(fb(x2[0]) + fb(x4[0]));
        frame[0x14] = x0[0];
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        x0[0] = bits(fb(x0[0]) + fb(frame[0xF]));
        let cf_a10 = x5[0]; let cf_b10 = x0[0];
        if f32::from_bits(cf_a10) > f32::from_bits(cf_b10) {
         x0[0] = x5[0]; x0[1] = x5[1]; x0[2] = x5[2]; x0[3] = x5[3];
        }
        x1[0] = bits(fb(x1[0]) * fb(g32(0x00E7CBAC)));
        x0[0] = bits(fb(x0[0]) * fb(x0[0]));
        frame[0x15] = x0[0];
        x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
        x0[0] = bits(fb(x0[0]) + fb(frame[0xE]));
        let cf_a11 = x5[0]; let cf_b11 = x0[0];
        if f32::from_bits(cf_a11) > f32::from_bits(cf_b11) {
         x0[0] = x5[0]; x0[1] = x5[1]; x0[2] = x5[2]; x0[3] = x5[3];
        }
        x1[0] = bits(fb(x1[0]) + fb(x2[0]));
        x0[0] = bits(fb(x0[0]) * fb(x0[0]));
        let cf_a12 = x5[0]; let cf_b12 = x1[0];
        frame[0x16] = x0[0];
        if f32::from_bits(cf_a12) > f32::from_bits(cf_b12) {
         x1[0] = x5[0]; x1[1] = x5[1]; x1[2] = x5[2]; x1[3] = x5[3];
        }
        x4[0] = frame[0x11];
        x4[1] = 0; x4[2] = 0; x4[3] = 0;
        x0[0] = x4[0]; x0[1] = x4[1]; x0[2] = x4[2]; x0[3] = x4[3];
        x0[0] = bits(fb(x0[0]) * fb(x4[0]));
        x1[0] = bits(fb(x1[0]) * fb(x1[0]));
        x0[0] = bits(fb(x0[0]) + fb(frame[0xD]));
        frame[0x17] = x1[0];
        x0[0] = bits(fb(x0[0]) + fb(frame[0xC]));
        let cf_a13 = x0[0]; let cf_b13 = x5[0];
        if !(f32::from_bits(cf_a13) != f32::from_bits(cf_b13)) {
         x1[0] = x5[0]; x1[1] = x5[1]; x1[2] = x5[2]; x1[3] = x5[3];
        } else {
         x0[0] = sqrt_lane(x0[0]);
         x0[1] = sqrt_lane(x0[1]);
         x0[2] = sqrt_lane(x0[2]);
         x0[3] = sqrt_lane(x0[3]);
         x1[0] = x6[0]; x1[1] = x6[1]; x1[2] = x6[2]; x1[3] = x6[3];
         x1[0] = bits(fb(x1[0]) / fb(x0[0]));
        }
        x0[0] = frame[0x10];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) * fb(x1[0]));
        x2[0] = frame[0x12];
        x2[1] = 0; x2[2] = 0; x2[3] = 0;
        x4[0] = bits(fb(x4[0]) * fb(x1[0]));
        x0[0] = bits(fb(x0[0]) * fb(x5[0]));
        x2[0] = bits(fb(x2[0]) * fb(x1[0]));
        x0[0] = bits(fb(x0[0]) + fb(x4[0]));
        x4[0] = g32(0x00FE8F20); x4[1] = g32(0x00FE8F24); x4[2] = g32(0x00FE8F28); x4[3] = g32(0x00FE8F2C);
        edi = arg0;
        x2[0] = bits(fb(x2[0]) * fb(x5[0]));
        eax = ((h16(edi, 208) as u32 & 0xFFFF) as u16 as i16 as i32 as u32);
        x0[0] = bits(fb(x0[0]) + fb(x2[0]));
        ecx = g32(0x0115F814);
        x1[0] = x6[0]; x1[1] = x6[1]; x1[2] = x6[2]; x1[3] = x6[3];
        x2[0] = x6[0]; x2[1] = x6[1]; x2[2] = x6[2]; x2[3] = x6[3];
        x2[0] = bits(fb(x2[0]) - fb(x3[0]));
        x0[0] = x0[0] & g32(0x00FE8F80);
        x0[1] = x0[1] & g32(0x00FE8F84);
        x0[2] = x0[2] & g32(0x00FE8F88);
        x0[3] = x0[3] & g32(0x00FE8F8C);
        x1[0] = bits(fb(x1[0]) - fb(x0[0]));
        x0[0] = frame[0x14]; x0[1] = frame[0x15]; x0[2] = frame[0x16]; x0[3] = frame[0x17];
        x3[0] = bits(fb(x3[0]) * fb(h32(esi, 5972)));
        x2[0] = bits(fb(x2[0]) * fb(h32(esi, 5964)));
        eax = (eax).wrapping_add(eax.wrapping_mul(8));
        x3[0] = bits(fb(x3[0]) + fb(x2[0]));
        { let t0 = x1[0]; let t1 = x1[1]; let t2 = x1[2]; let t3 = x1[3];
          x1[0] = t0; x1[1] = t0; x1[2] = x1[0]; x1[3] = x1[0]; }
        x0[0] = bits(fb(x0[0]) * fb(x1[0]));
        x0[1] = bits(fb(x0[1]) * fb(x1[1]));
        x0[2] = bits(fb(x0[2]) * fb(x1[2]));
        x0[3] = bits(fb(x0[3]) * fb(x1[3]));
        x4[0] = bits(fb(x4[0]) - fb(x1[0]));
        x4[1] = bits(fb(x4[1]) - fb(x1[1]));
        x4[2] = bits(fb(x4[2]) - fb(x1[2]));
        x4[3] = bits(fb(x4[3]) - fb(x1[3]));
        x1[0] = frame[0x3];
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x1[0] = bits(fb(x1[0]) * fb(h32(esi, 5976)));
        x4[0] = bits(fb(x4[0]) * fb(g32(0x00E7CBB0)));
        x4[1] = bits(fb(x4[1]) * fb(g32(0x00E7CBB4)));
        x4[2] = bits(fb(x4[2]) * fb(g32(0x00E7CBB8)));
        x4[3] = bits(fb(x4[3]) * fb(g32(0x00E7CBBC)));
        x1[0] = bits(fb(x1[0]) + fb(x2[0]));
        x4[0] = bits(fb(x4[0]) + fb(x0[0]));
        x4[1] = bits(fb(x4[1]) + fb(x0[1]));
        x4[2] = bits(fb(x4[2]) + fb(x0[2]));
        x4[3] = bits(fb(x4[3]) + fb(x0[3]));
        x0[0] = h32sx(ecx, eax, 4, 24);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) * fb(h32(edi, 204)));
        x3[0] = bits(fb(x3[0]) * fb(x4[0]));
        frame[0x6] = x0[0];
        x0[0] = x4[0]; x0[1] = x4[1]; x0[2] = x4[2]; x0[3] = x4[3];
        { let t0 = x0[0]; let t1 = x0[1]; let t2 = x0[2]; let t3 = x0[3];
          x0[0] = t1; x0[1] = t1; x0[2] = x4[1]; x0[3] = x4[1]; }
        x1[0] = bits(fb(x1[0]) * fb(x0[0]));
        x0[0] = x4[0]; x0[1] = x4[1]; x0[2] = x4[2]; x0[3] = x4[3];
        { let t0 = x0[0]; let t1 = x0[1]; let t2 = x0[2]; let t3 = x0[3];
          x0[0] = t2; x0[1] = t2; x0[2] = x4[2]; x0[3] = x4[2]; }
        x3[0] = bits(fb(x3[0]) + fb(x1[0]));
        x1[0] = h32(esi, 5980);
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x1[0] = bits(fb(x1[0]) * fb(frame[0x3]));
        { let t0 = x4[0]; let t1 = x4[1]; let t2 = x4[2]; let t3 = x4[3];
          x4[0] = t3; x4[1] = t3; x4[2] = x4[3]; x4[3] = x4[3]; }
        x1[0] = bits(fb(x1[0]) + fb(x2[0]));
        x1[0] = bits(fb(x1[0]) * fb(x0[0]));
        x0[0] = h32(esi, 5984);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x3[0] = bits(fb(x3[0]) + fb(x1[0]));
        x1[0] = frame[0x3];
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x0[0] = bits(fb(x0[0]) * fb(x1[0]));
        x1[0] = bits(fb(x1[0]) * fb(g32(0x00FE87E8)));
        x0[0] = bits(fb(x0[0]) + fb(x2[0]));
        frame[0x3] = x1[0];
        x0[0] = bits(fb(x0[0]) * fb(x4[0]));
        x3[0] = bits(fb(x3[0]) + fb(x0[0]));
        x0[0] = x6[0]; x0[1] = x6[1]; x0[2] = x6[2]; x0[3] = x6[3];
        x0[0] = bits(fb(x0[0]) - fb(x3[0]));
        x3[0] = bits(fb(x3[0]) * fb(g32(0x00FE8858)));
        x0[0] = bits(fb(x0[0]) * fb(g32(0x00FE87D0)));
        x0[0] = bits(fb(x0[0]) + fb(x3[0]));
        frame[0x2] = x0[0];
        let cf_a14 = (edx & 0xFF); let cf_b14 = 0x00000004u32;
        if ((cf_a14) & (cf_b14)) == 0 {
         x1[0] = g32(0x00FE8B5C);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x3[0] = g32(0x00FE876C);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x6[0] = g32(0x00FE8B38);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         s32(0x0115FE00, x1[0]);
         x1[0] = x3[0]; x1[1] = x3[1]; x1[2] = x3[2]; x1[3] = x3[3];
         edx = edx | (0x00000004u32);
         x4[0] = x5[0]; x4[1] = x5[1]; x4[2] = x5[2]; x4[3] = x5[3];
         x0[0] = x5[0]; x0[1] = x5[1]; x0[2] = x5[2]; x0[3] = x5[3];
         s32(0x0115FDC4, edx);
         s32(0x0115FDE8, x5[0]);
         s32(0x0115FDEC, x4[0]);
         s32(0x0115FDF4, x6[0]);
         s32(0x0115FDF8, x0[0]);
         s32(0x0115FE04, x5[0]);
         s32(0x0115FDF0, x3[0]);
         frame[0x9] = x1[0];
         s32(0x0115FDFC, x1[0]);
        } else {
         x0[0] = g32(0x0115FE00);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x6[0] = g32(0x0115FDF4);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x3[0] = g32(0x0115FDF0);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x4[0] = g32(0x0115FDEC);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         frame[0x7] = x0[0];
         x0[0] = g32(0x0115FDFC);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         frame[0x9] = x0[0];
         x0[0] = g32(0x0115FDF8);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x1[0] = frame[0xB];
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x2[0] = x1[0]; x2[1] = x1[1]; x2[2] = x1[2]; x2[3] = x1[3];
        x2[0] = bits(fb(x2[0]) - fb(x6[0]));
        x6[0] = frame[0x9];
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        frame[0x8] = x2[0];
        x2[0] = g32(0x0115FE04);
        x2[1] = 0; x2[2] = 0; x2[3] = 0;
        x6[0] = bits(fb(x6[0]) * fb(frame[0x8]));
        x2[0] = bits(fb(x2[0]) - fb(x0[0]));
        x2[0] = bits(fb(x2[0]) * fb(x6[0]));
        x6[0] = x1[0]; x6[1] = x1[1]; x6[2] = x1[2]; x6[3] = x1[3];
        x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FDE8)));
        x2[0] = bits(fb(x2[0]) + fb(x0[0]));
        let cf_a15 = x6[0]; let cf_b15 = x5[0];
        if f32::from_bits(cf_a15) >= f32::from_bits(cf_b15) {
         x0[0] = bits(fb(x0[0]) - fb(x4[0]));
         x3[0] = bits(fb(x3[0]) * fb(x6[0]));
         x0[0] = bits(fb(x0[0]) * fb(x3[0]));
         x0[0] = bits(fb(x0[0]) + fb(x4[0]));
        } else {
         x0[0] = x4[0]; x0[1] = x4[1]; x0[2] = x4[2]; x0[3] = x4[3];
        }
        x3[0] = frame[0x8];
        x3[1] = 0; x3[2] = 0; x3[3] = 0;
        let cf_a16 = x3[0]; let cf_b16 = x5[0];
        x6[0] = g32(0x00FE88E8);
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        if f32::from_bits(cf_a16) >= f32::from_bits(cf_b16) {
         x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        }
        x1[0] = bits(fb(x1[0]) - fb(frame[0x7]));
        let cf_a17 = x1[0]; let cf_b17 = x5[0];
        if f32::from_bits(cf_a17) >= f32::from_bits(cf_b17) {
         x0[0] = g32(0x0115FE04);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x3[0] = h32(esi, 5964);
        x3[1] = 0; x3[2] = 0; x3[3] = 0;
        eax = ((h16(edi, 208) as u32 & 0xFFFF) as u16 as i16 as i32 as u32);
        x3[0] = bits(fb(x3[0]) * fb(x0[0]));
        x4[0] = x6[0]; x4[1] = x6[1]; x4[2] = x6[2]; x4[3] = x6[3];
        x4[0] = bits(fb(x4[0]) - fb(x0[0]));
        x0[0] = h32(esi, 6020);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        eax = (eax).wrapping_add(eax.wrapping_mul(8));
        x2[0] = h32sx(ecx, eax, 4, 20);
        x2[1] = 0; x2[2] = 0; x2[3] = 0;
        x1[0] = x4[0]; x1[1] = x4[1]; x1[2] = x4[2]; x1[3] = x4[3];
        x1[0] = bits(fb(x1[0]) * fb(h32(esi, 5972)));
        x1[0] = bits(fb(x1[0]) + fb(x3[0]));
        frame[0x18] = x1[0];
        x1[0] = bits(fb(x1[0]) * fb(g32(0x00FE87D0)));
        x1[0] = bits(fb(x1[0]) * fb(frame[0x6]));
        x1[0] = bits(fb(x1[0]) * fb(x2[0]));
        x0[0] = bits(fb(x0[0]) * fb(x1[0]));
        x1[0] = x4[0]; x1[1] = x4[1]; x1[2] = x4[2]; x1[3] = x4[3];
        x1[0] = bits(fb(x1[0]) * fb(h32(esi, 5976)));
        frame[0x14] = x0[0];
        x0[0] = h32(esi, 6024);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x1[0] = bits(fb(x1[0]) + fb(x3[0]));
        frame[0x19] = x1[0];
        x1[0] = bits(fb(x1[0]) * fb(g32(0x00FE87D0)));
        x1[0] = bits(fb(x1[0]) * fb(frame[0x6]));
        x1[0] = bits(fb(x1[0]) * fb(x2[0]));
        x0[0] = bits(fb(x0[0]) * fb(x1[0]));
        x1[0] = h32(esi, 5980);
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x1[0] = bits(fb(x1[0]) * fb(x4[0]));
        frame[0x15] = x0[0];
        x0[0] = h32(esi, 6028);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x1[0] = bits(fb(x1[0]) + fb(x3[0]));
        frame[0x1A] = x1[0];
        x1[0] = bits(fb(x1[0]) * fb(g32(0x00FE87D0)));
        x1[0] = bits(fb(x1[0]) * fb(frame[0x6]));
        x1[0] = bits(fb(x1[0]) * fb(x2[0]));
        x0[0] = bits(fb(x0[0]) * fb(x1[0]));
        x1[0] = h32(esi, 5984);
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        x1[0] = bits(fb(x1[0]) * fb(x4[0]));
        frame[0x16] = x0[0];
        x0[0] = h32(esi, 6032);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x1[0] = bits(fb(x1[0]) + fb(x3[0]));
        frame[0x1B] = x1[0];
        x1[0] = bits(fb(x1[0]) * fb(g32(0x00FE87D0)));
        x1[0] = bits(fb(x1[0]) * fb(frame[0x6]));
        frame[0x6] = x5[0];
        x1[0] = bits(fb(x1[0]) * fb(x2[0]));
        x2[0] = g32(0x00E7CBA8);
        x2[1] = 0; x2[2] = 0; x2[3] = 0;
        x0[0] = bits(fb(x0[0]) * fb(x1[0]));
        x1[0] = h32sx(ecx, eax, 4, 8);
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        frame[0x17] = x0[0];
        x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
        frame[0x9] = x0[0];
        let cf_a18 = (edx & 0xFF); let cf_b18 = 0x00000008u32;
        if ((cf_a18) & (cf_b18)) == 0 {
         x0[0] = g32(0x00FE8830);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         s32(0x0115FE0C, x2[0]);
         x2[0] = g32(0x00E7CBA0);
         x2[1] = 0; x2[2] = 0; x2[3] = 0;
         s32(0x0115FE14, x0[0]);
         x0[0] = g32(0x00FE8A24);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         edx = edx | (0x00000008u32);
         s32(0x0115FDC4, edx);
         s32(0x0115FE08, x5[0]);
         s32(0x0115FE18, x2[0]);
         s32(0x0115FE20, x6[0]);
         s32(0x0115FE24, x2[0]);
         s32(0x0115FE10, x0[0]);
         s32(0x0115FE1C, x0[0]);
        } else {
         x2[0] = g32(0x0115FE18);
         x2[1] = 0; x2[2] = 0; x2[3] = 0;
         x0[0] = g32(0x0115FE10);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        esi = h32(edi, 172);
        frame[0x8] = x0[0];
        let cf_a19 = esi; let cf_b19 = esi;
        if ((cf_a19) & (cf_b19)) != 0 {
         x3[0] = h32(esi, 8);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
         x0[0] = bits(fb(x0[0]) * fb(h32(esi, 0)));
         x4[0] = x6[0]; x4[1] = x6[1]; x4[2] = x6[2]; x4[3] = x6[3];
         x4[0] = bits(fb(x4[0]) - fb(x1[0]));
         frame[0xE] = x0[0];
         x0[0] = h32(esi, 4);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x0[0] = bits(fb(x0[0]) * fb(x1[0]));
         x4[0] = bits(fb(x4[0]) * fb(g32(0x00E7CBA8)));
         x1[0] = x6[0]; x1[1] = x6[1]; x1[2] = x6[2]; x1[3] = x6[3];
         x1[0] = bits(fb(x1[0]) - fb(x3[0]));
         x4[0] = bits(fb(x4[0]) + fb(x0[0]));
         frame[0x7] = x1[0];
         ecx = frame[0x7];
         eax = ecx;
         eax = (eax) >> (0x00000017u32);
         x1[0] = bits(fb(x1[0]) - fb(g32(0x00FE8670)));
         x0[0] = eax;
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         pd0 = (x0[0] as i32) as f64; pd1 = (x0[1] as i32) as f64;
         eax = (eax) >> (0x0000001Fu32);
         ecx = ecx & (0x007FFFFFu32);
         pd0 = bb64(bb64(pd0) + bb64(f64::from_bits(g64((eax).wrapping_mul(8) as u32, 0x00FE8F50))));
         ecx = ecx | (0x3F800000u32);
         let cf_a20 = x1[0]; let cf_b20 = x5[0];
         x0[0] = bits((bb64(pd0)) as f32);
         x0[1] = bits((bb64(pd1)) as f32);
         x0[2] = 0; x0[3] = 0;
         frame[0x9] = x4[0];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x00FE8BC4)));
         frame[0x7] = ecx;
         x0[0] = bits(fb(x0[0]) - fb(g32(0x00E78580)));
         frame[0xF] = x0[0];
         if f32::from_bits(cf_a20) >= f32::from_bits(cf_b20) {
          x0[0] = frame[0x7];
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x1[0] = x0[0]; x1[1] = x0[1]; x1[2] = x0[2]; x1[3] = x0[3];
          x1[0] = bits(fb(x1[0]) * fb(g32(0x00E78584)));
          x0[0] = bits(fb(x0[0]) * fb(x0[0]));
          x1[0] = bits(fb(x1[0]) + fb(frame[0xF]));
          x0[0] = bits(fb(x0[0]) * fb(g32(0x00E7857C)));
          x1[0] = bits(fb(x1[0]) - fb(x0[0]));
          x1[0] = bits(fb(x1[0]) * fb(g32(0x00E78578)));
          x1[0] = bits(fb(x1[0]) * fb(g32(0x00FE8B38)));
          frame[0x6] = x1[0];
         } else {
          x0[0] = g32(0x00FE8DF8);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          frame[0x6] = x0[0];
         }
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE14)));
         frame[0xF] = x0[0];
         x1[0] = x0[0]; x1[1] = x0[1]; x1[2] = x0[2]; x1[3] = x0[3];
         x0[0] = g32(0x0115FE24);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x1[0] = bits(fb(x1[0]) * fb(g32(0x0115FE1C)));
         x0[0] = bits(fb(x0[0]) - fb(x2[0]));
         x1[0] = bits(fb(x1[0]) * fb(x0[0]));
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE08)));
         x1[0] = bits(fb(x1[0]) + fb(x2[0]));
         let cf_a21 = x0[0]; let cf_b21 = x5[0];
         if f32::from_bits(cf_a21) >= f32::from_bits(cf_b21) {
          x0[0] = bits(fb(x0[0]) * fb(frame[0x8]));
          x2[0] = bits(fb(x2[0]) - fb(g32(0x0115FE0C)));
          x0[0] = bits(fb(x0[0]) * fb(x2[0]));
          x0[0] = bits(fb(x0[0]) + fb(g32(0x0115FE0C)));
         } else {
          x0[0] = g32(0x0115FE0C);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
         }
         x2[0] = frame[0xF];
         x2[1] = 0; x2[2] = 0; x2[3] = 0;
         let cf_a22 = x2[0]; let cf_b22 = x5[0];
         if f32::from_bits(cf_a22) >= f32::from_bits(cf_b22) {
          x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
         }
         x3[0] = bits(fb(x3[0]) - fb(g32(0x0115FE20)));
         let cf_a23 = x3[0]; let cf_b23 = x5[0];
         if f32::from_bits(cf_a23) >= f32::from_bits(cf_b23) {
          x0[0] = g32(0x0115FE24);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
         }
         let cf_a24 = x0[0]; let cf_b24 = x4[0];
         x1[0] = frame[0x6];
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x1[0] = bits(fb(x1[0]) + fb(frame[0xE]));
         ecx = g32(0x0115F814);
         frame[0x6] = x1[0];
         if !(f32::from_bits(cf_a24) > f32::from_bits(cf_b24)) {
          frame[0x9] = x0[0];
         }
        }
        eax = ((h16(edi, 208) as u32 & 0xFFFF) as u16 as i16 as i32 as u32);
        x1[0] = x6[0]; x1[1] = x6[1]; x1[2] = x6[2]; x1[3] = x6[3];
        eax = (eax).wrapping_add(eax.wrapping_mul(8));
        x3[0] = x5[0]; x3[1] = x5[1]; x3[2] = x5[2]; x3[3] = x5[3];
        x1[0] = bits(fb(x1[0]) - fb(h32sx(ecx, eax, 4, 12)));
        x7[0] = bits(fb(x7[0]) * fb(h32sx(ecx, eax, 4, 12)));
        x0[0] = h32sx(ecx, eax, 4, 20);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) * fb(frame[0x3]));
        x1[0] = bits(fb(x1[0]) * fb(g32(0x00E7CBA8)));
        frame[0x5] = x0[0];
        frame[0x3] = x5[0];
        x1[0] = bits(fb(x1[0]) + fb(x7[0]));
        frame[0xF] = x1[0];
        let cf_a25 = esi; let cf_b25 = esi;
        if ((cf_a25) & (cf_b25)) != 0 {
         x0[0] = h32(esi, 12);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x0[0] = bits(fb(x0[0]) * fb(h32sx(ecx, eax, 4, 16)));
         x3[0] = h32(esi, 16);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         frame[0x3] = x0[0];
        }
        x0[0] = g32(0x00FE87E4);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x4[0] = g32(0x00FE8AB8);
        x4[1] = 0; x4[2] = 0; x4[3] = 0;
        x1[0] = g32(0x00FE8880);
        x1[1] = 0; x1[2] = 0; x1[3] = 0;
        frame[0xB] = x0[0];
        frame[0xD] = x4[0];
        let cf_a26 = (edx & 0xFF); let cf_b26 = 0x00000010u32;
        if ((cf_a26) & (cf_b26)) == 0 {
         s32(0x0115FE34, x0[0]);
         x0[0] = g32(0x00FE8830);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         s32(0x0115FE40, x0[0]);
         edx = edx | (0x00000010u32);
         x2[0] = x1[0]; x2[1] = x1[1]; x2[2] = x1[2]; x2[3] = x1[3];
         x0[0] = x5[0]; x0[1] = x5[1]; x0[2] = x5[2]; x0[3] = x5[3];
         s32(0x0115FDC4, edx);
         s32(0x0115FE28, x5[0]);
         s32(0x0115FE2C, x6[0]);
         s32(0x0115FE38, x2[0]);
         s32(0x0115FE44, x0[0]);
         s32(0x0115FE30, x4[0]);
         s32(0x0115FE3C, x4[0]);
        } else {
         x2[0] = g32(0x0115FE38);
         x2[1] = 0; x2[2] = 0; x2[3] = 0;
         x0[0] = g32(0x0115FE44);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x7[0] = x3[0]; x7[1] = x3[1]; x7[2] = x3[2]; x7[3] = x3[3];
        x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FE34)));
        x0[0] = bits(fb(x0[0]) - fb(x2[0]));
        frame[0xE] = x7[0];
        x7[0] = bits(fb(x7[0]) * fb(g32(0x0115FE3C)));
        x7[0] = bits(fb(x7[0]) * fb(x0[0]));
        x7[0] = bits(fb(x7[0]) + fb(x2[0]));
        frame[0xC] = x7[0];
        x7[0] = x3[0]; x7[1] = x3[1]; x7[2] = x3[2]; x7[3] = x3[3];
        x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FE28)));
        let cf_a27 = x7[0]; let cf_b27 = x5[0];
        if f32::from_bits(cf_a27) >= f32::from_bits(cf_b27) {
         x7[0] = bits(fb(x7[0]) * fb(g32(0x0115FE30)));
         x2[0] = bits(fb(x2[0]) - fb(g32(0x0115FE2C)));
         x7[0] = bits(fb(x7[0]) * fb(x2[0]));
         x7[0] = bits(fb(x7[0]) + fb(g32(0x0115FE2C)));
        } else {
         x0[0] = g32(0x0115FE2C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x7[0] = x0[0]; x7[1] = x0[1]; x7[2] = x0[2]; x7[3] = x0[3];
        }
        x0[0] = frame[0xE];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        let cf_a28 = x0[0]; let cf_b28 = x5[0];
        if f32::from_bits(cf_a28) >= f32::from_bits(cf_b28) {
         x7[0] = frame[0xC];
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
        }
        x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
        x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE40)));
        let cf_a29 = x0[0]; let cf_b29 = x5[0];
        if f32::from_bits(cf_a29) >= f32::from_bits(cf_b29) {
         x7[0] = g32(0x0115FE44);
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
        }
        x7[0] = bits(fb(x7[0]) * fb(frame[0x3]));
        x2[0] = g32(0x00FE8830);
        x2[1] = 0; x2[2] = 0; x2[3] = 0;
        let cf_a30 = x2[0]; let cf_b30 = x3[0];
        x0[0] = g32(0x00FE888C);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        frame[0xE] = x7[0];
        frame[0xC] = x0[0];
        if f32::from_bits(cf_a30) >= f32::from_bits(cf_b30) {
         let cf_a31 = (edx & 0xFF); let cf_b31 = 0x00000020u32;
         if ((cf_a31) & (cf_b31)) == 0 {
          x0[0] = g32(0x00FE87E4);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          s32(0x0115FE54, x0[0]);
          x0[0] = g32(0x00FE8830);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          edx = edx | (0x00000020u32);
          x2[0] = x1[0]; x2[1] = x1[1]; x2[2] = x1[2]; x2[3] = x1[3];
          s32(0x0115FDC4, edx);
          s32(0x0115FE48, x5[0]);
          s32(0x0115FE4C, x5[0]);
          frame[0x8] = x1[0];
          s32(0x0115FE58, x2[0]);
          s32(0x0115FE60, x0[0]);
          s32(0x0115FE64, x6[0]);
          s32(0x0115FE50, x4[0]);
          s32(0x0115FE5C, x4[0]);
         } else {
          x2[0] = g32(0x0115FE58);
          x2[1] = 0; x2[2] = 0; x2[3] = 0;
          frame[0x8] = x2[0];
         }
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE54)));
         frame[0x7] = x0[0];
         x0[0] = bits(fb(x0[0]) * fb(g32(0x0115FE5C)));
         frame[0x4] = x0[0];
         x0[0] = g32(0x0115FE64);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x6[0] = frame[0x4];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x0[0] = bits(fb(x0[0]) - fb(x2[0]));
         x6[0] = bits(fb(x6[0]) * fb(x0[0]));
         x0[0] = x6[0];
         x0[0] = bits(fb(x0[0]) + fb(x2[0]));
         x2[0] = x3[0]; x2[1] = x3[1]; x2[2] = x3[2]; x2[3] = x3[3];
         x2[0] = bits(fb(x2[0]) - fb(g32(0x0115FE48)));
         frame[0x4] = x6[0];
         frame[0x4] = x0[0];
         let cf_a32 = x2[0]; let cf_b32 = x5[0];
         if f32::from_bits(cf_a32) >= f32::from_bits(cf_b32) {
          x0[0] = frame[0x8];
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE4C)));
          x2[0] = bits(fb(x2[0]) * fb(g32(0x0115FE50)));
          x2[0] = bits(fb(x2[0]) * fb(x0[0]));
          x0[0] = frame[0x4];
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x2[0] = bits(fb(x2[0]) + fb(g32(0x0115FE4C)));
         } else {
          x6[0] = g32(0x0115FE4C);
          x6[1] = 0; x6[2] = 0; x6[3] = 0;
          x2[0] = x6[0]; x2[1] = x6[1]; x2[2] = x6[2]; x2[3] = x6[3];
         }
         x6[0] = frame[0x7];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         let cf_a33 = x6[0]; let cf_b33 = x5[0];
         x6[0] = g32(0x00FE88E8);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         if f32::from_bits(cf_a33) >= f32::from_bits(cf_b33) {
          x2[0] = x0[0]; x2[1] = x0[1]; x2[2] = x0[2]; x2[3] = x0[3];
         }
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE60)));
         let cf_a34 = x0[0]; let cf_b34 = x5[0];
         if f32::from_bits(cf_a34) >= f32::from_bits(cf_b34) {
          x2[0] = g32(0x0115FE64);
          x2[1] = 0; x2[2] = 0; x2[3] = 0;
         }
        } else {
         let cf_a35 = (edx & 0xFF); let cf_b35 = 0x00000040u32;
         if ((cf_a35) & (cf_b35)) == 0 {
          s32(0x0115FE68, x2[0]);
          edx = edx | (0x00000040u32);
          x2[0] = x1[0]; x2[1] = x1[1]; x2[2] = x1[2]; x2[3] = x1[3];
          s32(0x0115FDC4, edx);
          s32(0x0115FE6C, x6[0]);
          s32(0x0115FE74, x0[0]);
          frame[0x8] = x1[0];
          s32(0x0115FE78, x2[0]);
          s32(0x0115FE80, x6[0]);
          s32(0x0115FE84, x5[0]);
          s32(0x0115FE70, x4[0]);
          s32(0x0115FE7C, x4[0]);
         } else {
          x2[0] = g32(0x0115FE78);
          x2[1] = 0; x2[2] = 0; x2[3] = 0;
          frame[0x8] = x2[0];
         }
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE74)));
         frame[0x7] = x0[0];
         x0[0] = bits(fb(x0[0]) * fb(g32(0x0115FE7C)));
         frame[0x4] = x0[0];
         x0[0] = g32(0x0115FE84);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x6[0] = frame[0x4];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x0[0] = bits(fb(x0[0]) - fb(x2[0]));
         x6[0] = bits(fb(x6[0]) * fb(x0[0]));
         x0[0] = x6[0];
         x0[0] = bits(fb(x0[0]) + fb(x2[0]));
         x2[0] = x3[0]; x2[1] = x3[1]; x2[2] = x3[2]; x2[3] = x3[3];
         x2[0] = bits(fb(x2[0]) - fb(g32(0x0115FE68)));
         frame[0x4] = x6[0];
         frame[0x4] = x0[0];
         let cf_a36 = x2[0]; let cf_b36 = x5[0];
         if f32::from_bits(cf_a36) >= f32::from_bits(cf_b36) {
          x0[0] = frame[0x8];
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE6C)));
          x2[0] = bits(fb(x2[0]) * fb(g32(0x0115FE70)));
          x2[0] = bits(fb(x2[0]) * fb(x0[0]));
          x0[0] = frame[0x4];
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x2[0] = bits(fb(x2[0]) + fb(g32(0x0115FE6C)));
         } else {
          x6[0] = g32(0x0115FE6C);
          x6[1] = 0; x6[2] = 0; x6[3] = 0;
          x2[0] = x6[0]; x2[1] = x6[1]; x2[2] = x6[2]; x2[3] = x6[3];
         }
         x6[0] = frame[0x7];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         let cf_a37 = x6[0]; let cf_b37 = x5[0];
         x6[0] = g32(0x00FE88E8);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         if f32::from_bits(cf_a37) >= f32::from_bits(cf_b37) {
          x2[0] = x0[0]; x2[1] = x0[1]; x2[2] = x0[2]; x2[3] = x0[3];
         }
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE80)));
         let cf_a38 = x0[0]; let cf_b38 = x5[0];
         if f32::from_bits(cf_a38) >= f32::from_bits(cf_b38) {
          x2[0] = g32(0x0115FE84);
          x2[1] = 0; x2[2] = 0; x2[3] = 0;
         }
        }
        x2[0] = bits(fb(x2[0]) * fb(frame[0x3]));
        let cf_a39 = (edx & 0xFF); let cf_b39 = (edx & 0xFF);
        if (((cf_a39) & (cf_b39)) & 0x80) == 0 {
         x0[0] = g32(0x00FE8830);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         s32(0x0115FE88, x0[0]);
         x0[0] = g32(0x00FE888C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         s32(0x0115FE94, x0[0]);
         x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
         edx = edx | (0x00000080u32);
         s32(0x0115FE90, x4[0]);
         s32(0x0115FE9C, x4[0]);
         s32(0x0115FDC4, edx);
         s32(0x0115FE8C, x5[0]);
         frame[0x8] = x0[0];
         s32(0x0115FE98, x0[0]);
         s32(0x0115FEA0, x6[0]);
         s32(0x0115FEA4, x6[0]);
         x4[0] = x1[0]; x4[1] = x1[1]; x4[2] = x1[2]; x4[3] = x1[3];
        } else {
         x4[0] = g32(0x0115FE98);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         frame[0x8] = x4[0];
        }
        x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
        x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE94)));
        frame[0x4] = x0[0];
        x0[0] = bits(fb(x0[0]) * fb(g32(0x0115FE9C)));
        frame[0x7] = x0[0];
        x0[0] = g32(0x0115FEA4);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) - fb(x4[0]));
        x4[0] = frame[0x7];
        x4[1] = 0; x4[2] = 0; x4[3] = 0;
        x4[0] = bits(fb(x4[0]) * fb(x0[0]));
        x0[0] = frame[0x8];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x4[0] = bits(fb(x4[0]) + fb(x0[0]));
        frame[0x7] = x4[0];
        x4[0] = x3[0]; x4[1] = x3[1]; x4[2] = x3[2]; x4[3] = x3[3];
        x4[0] = bits(fb(x4[0]) - fb(g32(0x0115FE88)));
        let cf_a40 = x4[0]; let cf_b40 = x5[0];
        if f32::from_bits(cf_a40) >= f32::from_bits(cf_b40) {
         x4[0] = bits(fb(x4[0]) * fb(g32(0x0115FE90)));
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FE8C)));
         x4[0] = bits(fb(x4[0]) * fb(x0[0]));
         x4[0] = bits(fb(x4[0]) + fb(g32(0x0115FE8C)));
        } else {
         x4[0] = g32(0x0115FE8C);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
        }
        x0[0] = frame[0x4];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        let cf_a41 = x0[0]; let cf_b41 = x5[0];
        if f32::from_bits(cf_a41) >= f32::from_bits(cf_b41) {
         x4[0] = frame[0x7];
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
        }
        x3[0] = bits(fb(x3[0]) - fb(g32(0x0115FEA0)));
        let cf_a42 = x3[0]; let cf_b42 = x5[0];
        if f32::from_bits(cf_a42) >= f32::from_bits(cf_b42) {
         x4[0] = g32(0x0115FEA4);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
        }
        ecx = frame[0xA];
        x0[0] = frame[0x5];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x3[0] = h32(ecx, 5932);
        x3[1] = 0; x3[2] = 0; x3[3] = 0;
        let cf_a43 = x0[0]; let cf_b43 = x3[0];
        x4[0] = bits(fb(x4[0]) * fb(frame[0x3]));
        if !(f32::from_bits(cf_a43) > f32::from_bits(cf_b43)) {
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         frame[0x5] = x3[0];
        }
        x3[0] = x6[0]; x3[1] = x6[1]; x3[2] = x6[2]; x3[3] = x6[3];
        x3[0] = bits(fb(x3[0]) - fb(x0[0]));
        frame[0x7] = x3[0];
        x3[0] = g32(0x00FE8AB8);
        x3[1] = 0; x3[2] = 0; x3[3] = 0;
        let cf_a44 = edx; let cf_b44 = 0x00000100u32;
        if ((cf_a44) & (cf_b44)) == 0 {
         x0[0] = g32(0x00FE87E4);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         s32(0x0115FEB4, x0[0]);
         x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
         s32(0x0115FEB8, x0[0]);
         x0[0] = g32(0x00FE8830);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         s32(0x0115FEC0, x0[0]);
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
         edx = edx | (0x00000100u32);
         x7[0] = x5[0]; x7[1] = x5[1]; x7[2] = x5[2]; x7[3] = x5[3];
         s32(0x0115FDC4, edx);
         s32(0x0115FEA8, x7[0]);
         s32(0x0115FEAC, x6[0]);
         frame[0x8] = x1[0];
         s32(0x0115FEC4, x5[0]);
         frame[0x3] = x0[0];
         s32(0x0115FEB0, x0[0]);
         s32(0x0115FEBC, x0[0]);
        } else {
         x0[0] = g32(0x0115FEBC);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x7[0] = g32(0x0115FEA8);
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
         frame[0x4] = x0[0];
         x0[0] = g32(0x0115FEB8);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         frame[0x8] = x0[0];
         x0[0] = g32(0x0115FEB0);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         frame[0x3] = x0[0];
         x0[0] = frame[0x4];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x5[0] = frame[0x2];
        x5[1] = 0; x5[2] = 0; x5[3] = 0;
        x5[0] = bits(fb(x5[0]) - fb(g32(0x0115FEB4)));
        x0[0] = bits(fb(x0[0]) * fb(x5[0]));
        frame[0x10] = x5[0];
        x5[0] = frame[0x2];
        x5[1] = 0; x5[2] = 0; x5[3] = 0;
        frame[0x4] = x0[0];
        x0[0] = g32(0x0115FEC4);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) - fb(frame[0x8]));
        x6[0] = frame[0x4];
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        x5[0] = bits(fb(x5[0]) - fb(x7[0]));
        x6[0] = bits(fb(x6[0]) * fb(x0[0]));
        frame[0xA] = x5[0];
        x7[0] = frame[0xA];
        x7[1] = 0; x7[2] = 0; x7[3] = 0;
        frame[0x4] = x6[0];
        x0[0] = x6[0];
        x6[0] = frame[0x8];
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        x0[0] = bits(fb(x0[0]) + fb(x6[0]));
        x5[0] = x5[0] ^ x5[0];
        x5[1] = x5[1] ^ x5[1];
        x5[2] = x5[2] ^ x5[2];
        x5[3] = x5[3] ^ x5[3];
        let cf_a45 = x7[0]; let cf_b45 = x5[0];
        frame[0x4] = x0[0];
        if f32::from_bits(cf_a45) >= f32::from_bits(cf_b45) {
         x0[0] = frame[0x3];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FEAC)));
         x0[0] = bits(fb(x0[0]) * fb(x7[0]));
         x0[0] = bits(fb(x0[0]) * fb(x6[0]));
         x0[0] = bits(fb(x0[0]) + fb(g32(0x0115FEAC)));
         frame[0x3] = x0[0];
         x0[0] = frame[0x4];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        } else {
         x6[0] = g32(0x0115FEAC);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         frame[0x3] = x6[0];
        }
        x6[0] = frame[0x10];
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        let cf_a46 = x6[0]; let cf_b46 = x5[0];
        x6[0] = g32(0x00FE88E8);
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        if f32::from_bits(cf_a46) >= f32::from_bits(cf_b46) {
         frame[0x3] = x0[0];
        }
        x0[0] = frame[0x2];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FEC0)));
        let cf_a47 = x0[0]; let cf_b47 = x5[0];
        if f32::from_bits(cf_a47) >= f32::from_bits(cf_b47) {
         x0[0] = g32(0x0115FEC4);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        } else {
         x0[0] = frame[0x3];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x0[0] = bits(fb(x0[0]) * fb(frame[0x5]));
        x7[0] = g32(0x00FE8830);
        x7[1] = 0; x7[2] = 0; x7[3] = 0;
        frame[0x3] = x0[0];
        x0[0] = frame[0x2];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        let cf_a48 = x7[0]; let cf_b48 = x0[0];
        if f32::from_bits(cf_a48) >= f32::from_bits(cf_b48) {
         let cf_a49 = edx; let cf_b49 = 0x00000200u32;
         if ((cf_a49) & (cf_b49)) == 0 {
          x0[0] = g32(0x00FE87E4);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          s32(0x0115FED4, x0[0]);
          x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
          frame[0x4] = x0[0];
          s32(0x0115FED8, x0[0]);
          x0[0] = g32(0x00FE8AB8);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          edx = edx | (0x00000200u32);
          s32(0x0115FDC4, edx);
          s32(0x0115FEC8, x5[0]);
          s32(0x0115FECC, x5[0]);
          s32(0x0115FEE0, x7[0]);
          s32(0x0115FEE4, x6[0]);
          s32(0x0115FED0, x3[0]);
          s32(0x0115FEDC, x0[0]);
         } else {
          x3[0] = g32(0x0115FED8);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
          x0[0] = g32(0x0115FEDC);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          frame[0x4] = x3[0];
          x3[0] = g32(0x0115FED4);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
          frame[0xB] = x3[0];
          x3[0] = g32(0x0115FED0);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
         x5[0] = frame[0x2];
         x5[1] = 0; x5[2] = 0; x5[3] = 0;
         x5[0] = bits(fb(x5[0]) - fb(frame[0xB]));
         x7[0] = frame[0x2];
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
         x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FEC8)));
         x0[0] = bits(fb(x0[0]) * fb(x5[0]));
         frame[0x10] = x5[0];
         x5[0] = x5[0] ^ x5[0];
         x5[1] = x5[1] ^ x5[1];
         x5[2] = x5[2] ^ x5[2];
         x5[3] = x5[3] ^ x5[3];
         let cf_a50 = x7[0]; let cf_b50 = x5[0];
         frame[0x8] = x0[0];
         x0[0] = g32(0x0115FEE4);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x0[0] = bits(fb(x0[0]) - fb(frame[0x4]));
         x6[0] = frame[0x8];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x6[0] = bits(fb(x6[0]) * fb(x0[0]));
         x0[0] = x6[0];
         frame[0x8] = x6[0];
         x6[0] = frame[0x4];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x0[0] = bits(fb(x0[0]) + fb(x6[0]));
         if f32::from_bits(cf_a50) >= f32::from_bits(cf_b50) {
          x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FECC)));
          x3[0] = bits(fb(x3[0]) * fb(x7[0]));
          x3[0] = bits(fb(x3[0]) * fb(x6[0]));
          x3[0] = bits(fb(x3[0]) + fb(g32(0x0115FECC)));
         } else {
          x3[0] = g32(0x0115FECC);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
         x6[0] = frame[0x10];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         let cf_a51 = x6[0]; let cf_b51 = x5[0];
         x6[0] = g32(0x00FE88E8);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         if f32::from_bits(cf_a51) >= f32::from_bits(cf_b51) {
          x3[0] = x0[0]; x3[1] = x0[1]; x3[2] = x0[2]; x3[3] = x0[3];
         }
         x0[0] = frame[0x2];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FEE0)));
         let cf_a52 = x0[0]; let cf_b52 = x5[0];
         if f32::from_bits(cf_a52) >= f32::from_bits(cf_b52) {
          x3[0] = g32(0x0115FEE4);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
        } else {
         let cf_a53 = edx; let cf_b53 = 0x00000400u32;
         if ((cf_a53) & (cf_b53)) == 0 {
          x0[0] = g32(0x00FE888C);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          s32(0x0115FEF4, x0[0]);
          x0[0] = g32(0x00FE8AB8);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          s32(0x0115FF04, x5[0]);
          x5[0] = x0[0]; x5[1] = x0[1]; x5[2] = x0[2]; x5[3] = x0[3];
          edx = edx | (0x00000400u32);
          x3[0] = x1[0]; x3[1] = x1[1]; x3[2] = x1[2]; x3[3] = x1[3];
          frame[0xB] = x0[0];
          s32(0x0115FEFC, x0[0]);
          x0[0] = frame[0x2];
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          s32(0x0115FDC4, edx);
          s32(0x0115FEE8, x7[0]);
          s32(0x0115FEEC, x6[0]);
          s32(0x0115FEF8, x3[0]);
          s32(0x0115FF00, x6[0]);
          s32(0x0115FEF0, x5[0]);
         } else {
          x3[0] = g32(0x0115FEFC);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
          x5[0] = g32(0x0115FEF0);
          x5[1] = 0; x5[2] = 0; x5[3] = 0;
          frame[0xB] = x3[0];
          x3[0] = g32(0x0115FEF8);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
         x6[0] = frame[0xB];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x7[0] = x0[0]; x7[1] = x0[1]; x7[2] = x0[2]; x7[3] = x0[3];
         x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FEF4)));
         x0[0] = g32(0x0115FF04);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x0[0] = bits(fb(x0[0]) - fb(x3[0]));
         frame[0xA] = x5[0];
         x5[0] = x5[0] ^ x5[0];
         x5[1] = x5[1] ^ x5[1];
         x5[2] = x5[2] ^ x5[2];
         x5[3] = x5[3] ^ x5[3];
         x6[0] = bits(fb(x6[0]) * fb(x7[0]));
         frame[0x10] = x7[0];
         x7[0] = frame[0x2];
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
         x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FEE8)));
         x0[0] = bits(fb(x0[0]) * fb(x6[0]));
         let cf_a54 = x7[0]; let cf_b54 = x5[0];
         x0[0] = bits(fb(x0[0]) + fb(x3[0]));
         if f32::from_bits(cf_a54) >= f32::from_bits(cf_b54) {
          x5[0] = frame[0xA];
          x5[1] = 0; x5[2] = 0; x5[3] = 0;
          x3[0] = bits(fb(x3[0]) - fb(g32(0x0115FEEC)));
          x5[0] = bits(fb(x5[0]) * fb(x7[0]));
          x3[0] = bits(fb(x3[0]) * fb(x5[0]));
          x5[0] = x5[0] ^ x5[0];
          x5[1] = x5[1] ^ x5[1];
          x5[2] = x5[2] ^ x5[2];
          x5[3] = x5[3] ^ x5[3];
          x3[0] = bits(fb(x3[0]) + fb(g32(0x0115FEEC)));
         } else {
          x6[0] = g32(0x0115FEEC);
          x6[1] = 0; x6[2] = 0; x6[3] = 0;
          x3[0] = x6[0]; x3[1] = x6[1]; x3[2] = x6[2]; x3[3] = x6[3];
         }
         x6[0] = frame[0x10];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         let cf_a55 = x6[0]; let cf_b55 = x5[0];
         x6[0] = g32(0x00FE88E8);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         if f32::from_bits(cf_a55) >= f32::from_bits(cf_b55) {
          x3[0] = x0[0]; x3[1] = x0[1]; x3[2] = x0[2]; x3[3] = x0[3];
         }
         x0[0] = frame[0x2];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FF00)));
         let cf_a56 = x0[0]; let cf_b56 = x5[0];
         if f32::from_bits(cf_a56) >= f32::from_bits(cf_b56) {
          x3[0] = g32(0x0115FF04);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
        }
        x3[0] = bits(fb(x3[0]) * fb(frame[0x5]));
        let cf_a57 = edx; let cf_b57 = 0x00000800u32;
        if ((cf_a57) & (cf_b57)) == 0 {
         x0[0] = g32(0x00FE8830);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         s32(0x0115FF08, x0[0]);
         x0[0] = g32(0x00FE888C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x7[0] = x6[0]; x7[1] = x6[1]; x7[2] = x6[2]; x7[3] = x6[3];
         s32(0x0115FF14, x0[0]);
         s32(0x0115FF20, x6[0]);
         x6[0] = g32(0x00FE8AB8);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         edx = edx | (0x00000800u32);
         x0[0] = x1[0]; x0[1] = x1[1]; x0[2] = x1[2]; x0[3] = x1[3];
         s32(0x0115FDC4, edx);
         s32(0x0115FF0C, x5[0]);
         s32(0x0115FF18, x0[0]);
         s32(0x0115FF24, x7[0]);
         s32(0x0115FF10, x6[0]);
         frame[0xB] = x6[0];
         s32(0x0115FF1C, x6[0]);
        } else {
         x6[0] = g32(0x0115FF14);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x0[0] = g32(0x0115FF1C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x7[0] = g32(0x0115FF24);
         x7[1] = 0; x7[2] = 0; x7[3] = 0;
         frame[0xC] = x6[0];
         x6[0] = g32(0x0115FF10);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         frame[0xB] = x0[0];
         x0[0] = g32(0x0115FF18);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         frame[0xD] = x6[0];
        }
        x5[0] = frame[0x2];
        x5[1] = 0; x5[2] = 0; x5[3] = 0;
        x5[0] = bits(fb(x5[0]) - fb(frame[0xC]));
        x6[0] = frame[0xB];
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        frame[0x10] = x5[0];
        x5[0] = x7[0]; x5[1] = x7[1]; x5[2] = x7[2]; x5[3] = x7[3];
        x7[0] = frame[0x10];
        x7[1] = 0; x7[2] = 0; x7[3] = 0;
        x5[0] = bits(fb(x5[0]) - fb(x0[0]));
        x6[0] = bits(fb(x6[0]) * fb(x7[0]));
        x5[0] = bits(fb(x5[0]) * fb(x6[0]));
        x6[0] = frame[0x2];
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FF08)));
        x5[0] = bits(fb(x5[0]) + fb(x0[0]));
        frame[0x10] = x6[0];
        frame[0xC] = x5[0];
        x5[0] = x5[0] ^ x5[0];
        x5[1] = x5[1] ^ x5[1];
        x5[2] = x5[2] ^ x5[2];
        x5[3] = x5[3] ^ x5[3];
        let cf_a58 = x6[0]; let cf_b58 = x5[0];
        if f32::from_bits(cf_a58) >= f32::from_bits(cf_b58) {
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FF0C)));
         x6[0] = frame[0xD];
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x6[0] = bits(fb(x6[0]) * fb(frame[0x10]));
         x0[0] = bits(fb(x0[0]) * fb(x6[0]));
         x0[0] = bits(fb(x0[0]) + fb(g32(0x0115FF0C)));
        } else {
         x0[0] = g32(0x0115FF0C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        let cf_a59 = x7[0]; let cf_b59 = x5[0];
        x7[0] = frame[0xE];
        x7[1] = 0; x7[2] = 0; x7[3] = 0;
        if f32::from_bits(cf_a59) >= f32::from_bits(cf_b59) {
         x0[0] = frame[0xC];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x6[0] = frame[0x2];
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FF20)));
        let cf_a60 = x6[0]; let cf_b60 = x5[0];
        x6[0] = g32(0x00FE88E8);
        x6[1] = 0; x6[2] = 0; x6[3] = 0;
        if f32::from_bits(cf_a60) >= f32::from_bits(cf_b60) {
         x0[0] = g32(0x0115FF24);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
        }
        x7[0] = bits(fb(x7[0]) * fb(frame[0x7]));
        x0[0] = bits(fb(x0[0]) * fb(frame[0x5]));
        x7[0] = bits(fb(x7[0]) + fb(frame[0x3]));
        let cf_a61 = x7[0]; let cf_b61 = x6[0];
        if f32::from_bits(cf_a61) > f32::from_bits(cf_b61) {
         x7[0] = x6[0]; x7[1] = x6[1]; x7[2] = x6[2]; x7[3] = x6[3];
        }
        x2[0] = bits(fb(x2[0]) * fb(frame[0x7]));
        x2[0] = bits(fb(x2[0]) + fb(x3[0]));
        let cf_a62 = x2[0]; let cf_b62 = x6[0];
        if f32::from_bits(cf_a62) > f32::from_bits(cf_b62) {
         x2[0] = x6[0]; x2[1] = x6[1]; x2[2] = x6[2]; x2[3] = x6[3];
        }
        x4[0] = bits(fb(x4[0]) * fb(frame[0x7]));
        x4[0] = bits(fb(x4[0]) + fb(x0[0]));
        let cf_a63 = x4[0]; let cf_b63 = x6[0];
        if f32::from_bits(cf_a63) > f32::from_bits(cf_b63) {
         x4[0] = x6[0]; x4[1] = x6[1]; x4[2] = x6[2]; x4[3] = x6[3];
        }
        eax = h32(edi, 176);
        x5[0] = frame[0xF];
        x5[1] = 0; x5[2] = 0; x5[3] = 0;
        x0[0] = eax;
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        pd0 = (x0[0] as i32) as f64; pd1 = (x0[1] as i32) as f64;
        eax = (eax) >> (0x0000001Fu32);
        pd0 = bb64(bb64(pd0) + bb64(f64::from_bits(g64((eax).wrapping_mul(8) as u32, 0x00FE8F50))));
        x3[0] = bits((bb64(pd0)) as f32);
        x3[1] = bits((bb64(pd1)) as f32);
        x3[2] = 0; x3[3] = 0;
        x0[0] = frame[0x9];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        let cf_a64 = x5[0]; let cf_b64 = x0[0];
        if !(f32::from_bits(cf_a64) > f32::from_bits(cf_b64)) {
         x0[0] = x5[0];
        }
        let cf_a65 = x3[0]; let cf_b65 = x0[0];
        if !(f32::from_bits(cf_a65) > f32::from_bits(cf_b65)) {
         x0[0] = x3[0]; x0[1] = x3[1]; x0[2] = x3[2]; x0[3] = x3[3];
        }
        x3[0] = h32(ecx, 5936);
        x3[1] = 0; x3[2] = 0; x3[3] = 0;
        let cf_a66 = x3[0]; let cf_b66 = x0[0];
        frame[0x9] = x0[0];
        x5[0] = x5[0] ^ x5[0];
        x5[1] = x5[1] ^ x5[1];
        x5[2] = x5[2] ^ x5[2];
        x5[3] = x5[3] ^ x5[3];
        if !(f32::from_bits(cf_a66) > f32::from_bits(cf_b66)) {
         frame[0x9] = x3[0];
        }
        x87v = frame[0x9];
        x0[0] = frame[0x7];
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        hs32(edi, 96, x7[0]);
        hs32(edi, 100, x2[0]);
        hs32(edi, 104, x4[0]);
        hs32(edi, 108, x0[0]);
        x0[0] = h32(edi, 168);
        x0[1] = 0; x0[2] = 0; x0[3] = 0;
        x0[0] = bits(fb(x0[0]) + fb(frame[0x6]));
        { let q = trunc_f32_to_i64(x87v); frame[0x10] = (q & 0xFFFF_FFFF) as u32; frame[0x11] = ((q >> 32) & 0xFFFF_FFFF) as u32; }
        eax = frame[0x10];
        hs32(edi, 176, eax);
        hs32(edi, 168, x0[0]);
        let cf_a67 = edx; let cf_b67 = 0x00001000u32;
        if ((cf_a67) & (cf_b67)) == 0 {
         edx = edx | (0x00001000u32);
         x4[0] = x6[0]; x4[1] = x6[1]; x4[2] = x6[2]; x4[3] = x6[3];
         x3[0] = x5[0]; x3[1] = x5[1]; x3[2] = x5[2]; x3[3] = x5[3];
         s32(0x0115FDC4, edx);
         s32(0x0115FF28, 0x00000000u32);
         s32(0x0115FF2C, x4[0]);
         s32(0x0115FF34, 0x3E800000u32);
         s32(0x0115FF38, x1[0]);
         s32(0x0115FF40, 0x3F000000u32);
         s32(0x0115FF44, x3[0]);
         s32(0x0115FF30, 0x40800000u32);
         s32(0x0115FF3C, 0x40800000u32);
        } else {
         x1[0] = g32(0x0115FF38);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x4[0] = g32(0x0115FF2C);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         x3[0] = g32(0x0115FF44);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
        }
        let cf_a68 = edx; let cf_b68 = 0x00002000u32;
        if ((cf_a68) & (cf_b68)) == 0 {
         edx = edx | (0x00002000u32);
         s32(0x0115FDC4, edx);
         s32(0x0115FF48, 0x00000000u32);
         s32(0x0115FF4C, 0x00000000u32);
         s32(0x0115FF54, 0x3E800000u32);
         s32(0x0115FF58, 0x3F35C28Fu32);
         s32(0x0115FF60, 0x3F000000u32);
         s32(0x0115FF64, 0x3F800000u32);
         s32(0x0115FF50, 0x40800000u32);
         s32(0x0115FF5C, 0x40800000u32);
        }
        let cf_a69 = edx; let cf_b69 = 0x00004000u32;
        if ((cf_a69) & (cf_b69)) == 0 {
         edx = edx | (0x00004000u32);
         s32(0x0115FDC4, edx);
         s32(0x0115FF68, 0x3F000000u32);
         s32(0x0115FF6C, 0x3F800000u32);
         s32(0x0115FF74, 0x3F400000u32);
         s32(0x0115FF78, 0x3F35C28Fu32);
         s32(0x0115FF80, 0x3F800000u32);
         s32(0x0115FF84, 0x00000000u32);
         s32(0x0115FF70, 0x40800000u32);
         s32(0x0115FF7C, 0x40800000u32);
        }
        let cf_a70 = edx; let cf_b70 = 0x00008000u32;
        if ((cf_a70) & (cf_b70)) == 0 {
         edx = edx | (0x00008000u32);
         s32(0x0115FDC4, edx);
         s32(0x0115FF88, 0x3F000000u32);
         s32(0x0115FF8C, 0x00000000u32);
         s32(0x0115FF94, 0x3F400000u32);
         s32(0x0115FF98, 0x3F35C28Fu32);
         s32(0x0115FFA0, 0x3F800000u32);
         s32(0x0115FFA4, 0x3F800000u32);
         s32(0x0115FF90, 0x40800000u32);
         s32(0x0115FF9C, 0x40800000u32);
        }
        eax = (edi).wrapping_add(0x34u32);
        ecx = ecx ^ (ecx);
        loop {
         x0[0] = frame[(0x60u32.wrapping_add(ecx) / 4) as usize];
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x2[0] = x6[0]; x2[1] = x6[1]; x2[2] = x6[2]; x2[3] = x6[3];
         x2[0] = bits(fb(x2[0]) - fb(x0[0]));
         x0[0] = bits(fb(x0[0]) * fb(g32(0x00FE8858)));
         x3[0] = bits(fb(x3[0]) - fb(x1[0]));
         x2[0] = bits(fb(x2[0]) * fb(g32(0x00FE87D0)));
         x2[0] = bits(fb(x2[0]) + fb(x0[0]));
         x0[0] = g32(0x0115FF3C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x6[0] = x2[0]; x6[1] = x2[1]; x6[2] = x2[2]; x6[3] = x2[3];
         x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FF34)));
         x7[0] = x2[0]; x7[1] = x2[1]; x7[2] = x2[2]; x7[3] = x2[3];
         x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FF28)));
         x0[0] = bits(fb(x0[0]) * fb(x6[0]));
         let cf_a71 = x7[0]; let cf_b71 = x5[0];
         x3[0] = bits(fb(x3[0]) * fb(x0[0]));
         x3[0] = bits(fb(x3[0]) + fb(x1[0]));
         if f32::from_bits(cf_a71) >= f32::from_bits(cf_b71) {
          x0[0] = g32(0x0115FF30);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x1[0] = bits(fb(x1[0]) - fb(x4[0]));
          x0[0] = bits(fb(x0[0]) * fb(x7[0]));
          x1[0] = bits(fb(x1[0]) * fb(x0[0]));
          x1[0] = bits(fb(x1[0]) + fb(x4[0]));
         } else {
          x1[0] = x4[0]; x1[1] = x4[1]; x1[2] = x4[2]; x1[3] = x4[3];
         }
         let cf_a72 = x6[0]; let cf_b72 = x5[0];
         if f32::from_bits(cf_a72) >= f32::from_bits(cf_b72) {
          x1[0] = x3[0]; x1[1] = x3[1]; x1[2] = x3[2]; x1[3] = x3[3];
         }
         x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FF40)));
         let cf_a73 = x0[0]; let cf_b73 = x5[0];
         if f32::from_bits(cf_a73) >= f32::from_bits(cf_b73) {
          x1[0] = g32(0x0115FF44);
          x1[1] = 0; x1[2] = 0; x1[3] = 0;
         }
         x1[0] = bits(fb(x1[0]) * fb(frame[(0x50u32.wrapping_add(ecx) / 4) as usize]));
         x7[0] = x2[0]; x7[1] = x2[1]; x7[2] = x2[2]; x7[3] = x2[3];
         x6[0] = x2[0]; x6[1] = x2[1]; x6[2] = x2[2]; x6[3] = x2[3];
         hs32(eax, -4, x1[0]);
         x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FF54)));
         x1[0] = g32(0x0115FF64);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x4[0] = g32(0x0115FF58);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         x0[0] = g32(0x0115FF5C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FF48)));
         x3[0] = x1[0]; x3[1] = x1[1]; x3[2] = x1[2]; x3[3] = x1[3];
         x3[0] = bits(fb(x3[0]) - fb(x4[0]));
         x0[0] = bits(fb(x0[0]) * fb(x7[0]));
         let cf_a74 = x6[0]; let cf_b74 = x5[0];
         x3[0] = bits(fb(x3[0]) * fb(x0[0]));
         x3[0] = bits(fb(x3[0]) + fb(x4[0]));
         if f32::from_bits(cf_a74) >= f32::from_bits(cf_b74) {
          x4[0] = bits(fb(x4[0]) - fb(g32(0x0115FF4C)));
          x0[0] = g32(0x0115FF50);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x0[0] = bits(fb(x0[0]) * fb(x6[0]));
          x4[0] = bits(fb(x4[0]) * fb(x0[0]));
          x4[0] = bits(fb(x4[0]) + fb(g32(0x0115FF4C)));
         } else {
          x4[0] = g32(0x0115FF4C);
          x4[1] = 0; x4[2] = 0; x4[3] = 0;
         }
         let cf_a75 = x7[0]; let cf_b75 = x5[0];
         if f32::from_bits(cf_a75) >= f32::from_bits(cf_b75) {
          x4[0] = x3[0]; x4[1] = x3[1]; x4[2] = x3[2]; x4[3] = x3[3];
         }
         x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FF60)));
         let cf_a76 = x0[0]; let cf_b76 = x5[0];
         if f32::from_bits(cf_a76) >= f32::from_bits(cf_b76) {
          x4[0] = x1[0]; x4[1] = x1[1]; x4[2] = x1[2]; x4[3] = x1[3];
         }
         x3[0] = g32(0x0115FF78);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x6[0] = g32(0x0115FF84);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
         x0[0] = g32(0x0115FF7C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x4[0] = bits(fb(x4[0]) * fb(frame[(0x50u32.wrapping_add(ecx) / 4) as usize]));
         x1[0] = x2[0]; x1[1] = x2[1]; x1[2] = x2[2]; x1[3] = x2[3];
         x1[0] = bits(fb(x1[0]) - fb(g32(0x0115FF74)));
         x6[0] = bits(fb(x6[0]) - fb(x3[0]));
         x7[0] = x2[0]; x7[1] = x2[1]; x7[2] = x2[2]; x7[3] = x2[3];
         x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FF68)));
         x0[0] = bits(fb(x0[0]) * fb(x1[0]));
         frame[0x10] = x1[0];
         let cf_a77 = x7[0]; let cf_b77 = x5[0];
         x6[0] = bits(fb(x6[0]) * fb(x0[0]));
         x6[0] = bits(fb(x6[0]) + fb(x3[0]));
         if f32::from_bits(cf_a77) >= f32::from_bits(cf_b77) {
          x3[0] = bits(fb(x3[0]) - fb(g32(0x0115FF6C)));
          x0[0] = g32(0x0115FF70);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x0[0] = bits(fb(x0[0]) * fb(x7[0]));
          x3[0] = bits(fb(x3[0]) * fb(x0[0]));
          x3[0] = bits(fb(x3[0]) + fb(g32(0x0115FF6C)));
         } else {
          x3[0] = g32(0x0115FF6C);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
         let cf_a78 = x1[0]; let cf_b78 = x5[0];
         if f32::from_bits(cf_a78) >= f32::from_bits(cf_b78) {
          x3[0] = x6[0]; x3[1] = x6[1]; x3[2] = x6[2]; x3[3] = x6[3];
         }
         x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x0115FF80)));
         let cf_a79 = x0[0]; let cf_b79 = x5[0];
         if f32::from_bits(cf_a79) >= f32::from_bits(cf_b79) {
          x3[0] = g32(0x0115FF84);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
         x0[0] = x2[0]; x0[1] = x2[1]; x0[2] = x2[2]; x0[3] = x2[3];
         x0[0] = bits(fb(x0[0]) - fb(g32(0x00FE8830)));
         let cf_a80 = x0[0]; let cf_b80 = x5[0];
         if f32::from_bits(cf_a80) >= f32::from_bits(cf_b80) {
          x3[0] = bits(fb(x3[0]) * fb(frame[(0x50u32.wrapping_add(ecx) / 4) as usize]));
         } else {
          x3[0] = x4[0]; x3[1] = x4[1]; x3[2] = x4[2]; x3[3] = x4[3];
         }
         hs32(eax, 0, x3[0]);
         x1[0] = g32(0x0115FFA4);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x3[0] = g32(0x0115FF98);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x0[0] = g32(0x0115FF9C);
         x0[1] = 0; x0[2] = 0; x0[3] = 0;
         x7[0] = x2[0]; x7[1] = x2[1]; x7[2] = x2[2]; x7[3] = x2[3];
         x7[0] = bits(fb(x7[0]) - fb(g32(0x0115FF94)));
         x4[0] = x1[0]; x4[1] = x1[1]; x4[2] = x1[2]; x4[3] = x1[3];
         x4[0] = bits(fb(x4[0]) - fb(x3[0]));
         x6[0] = x2[0]; x6[1] = x2[1]; x6[2] = x2[2]; x6[3] = x2[3];
         x6[0] = bits(fb(x6[0]) - fb(g32(0x0115FF88)));
         x0[0] = bits(fb(x0[0]) * fb(x7[0]));
         let cf_a81 = x6[0]; let cf_b81 = x5[0];
         x4[0] = bits(fb(x4[0]) * fb(x0[0]));
         x4[0] = bits(fb(x4[0]) + fb(x3[0]));
         if f32::from_bits(cf_a81) >= f32::from_bits(cf_b81) {
          x3[0] = bits(fb(x3[0]) - fb(g32(0x0115FF8C)));
          x0[0] = g32(0x0115FF90);
          x0[1] = 0; x0[2] = 0; x0[3] = 0;
          x0[0] = bits(fb(x0[0]) * fb(x6[0]));
          x3[0] = bits(fb(x3[0]) * fb(x0[0]));
          x3[0] = bits(fb(x3[0]) + fb(g32(0x0115FF8C)));
         } else {
          x3[0] = g32(0x0115FF8C);
          x3[1] = 0; x3[2] = 0; x3[3] = 0;
         }
         let cf_a82 = x7[0]; let cf_b82 = x5[0];
         if f32::from_bits(cf_a82) >= f32::from_bits(cf_b82) {
          x3[0] = x4[0]; x3[1] = x4[1]; x3[2] = x4[2]; x3[3] = x4[3];
         }
         x2[0] = bits(fb(x2[0]) - fb(g32(0x0115FFA0)));
         let cf_a83 = x2[0]; let cf_b83 = x5[0];
         if f32::from_bits(cf_a83) >= f32::from_bits(cf_b83) {
          x3[0] = x1[0]; x3[1] = x1[1]; x3[2] = x1[2]; x3[3] = x1[3];
         }
         x3[0] = bits(fb(x3[0]) * fb(frame[(0x50u32.wrapping_add(ecx) / 4) as usize]));
         ecx = (ecx).wrapping_add(0x00000004u32);
         eax = (eax).wrapping_add(0x0000000Cu32);
         hs32(eax, -8, x3[0]);
         let cf_a84 = ecx; let cf_b84 = 0x00000010u32;
         if (cf_a84) >= (cf_b84) { break; }
         x1[0] = g32(0x0115FF38);
         x1[1] = 0; x1[2] = 0; x1[3] = 0;
         x4[0] = g32(0x0115FF2C);
         x4[1] = 0; x4[2] = 0; x4[3] = 0;
         x3[0] = g32(0x0115FF44);
         x3[1] = 0; x3[2] = 0; x3[3] = 0;
         x6[0] = g32(0x00FE88E8);
         x6[1] = 0; x6[2] = 0; x6[3] = 0;
        }
        return eax;
    }
});

// ---------------------------------------------------------------------------
