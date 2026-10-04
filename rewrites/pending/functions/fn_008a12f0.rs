// original: 0x008A12F0 audio_voice_shape (proposed)
/// Shape a voice entry's response curves (proposed name `audio_voice_shape`).
///
/// Refreshes the entry's live bounds, derives the mix weight either from
/// the live mix pointer (clamped to 0..1) or from the bank table entry by
/// probing the spatial mixer, evaluates the two response curves at the
/// weight and its complement, shapes both through the curve shaper, and
/// sets the mode flags from the weight and residual against the entry
/// threshold.
///
/// Float comparisons reproduce the original's SSE branch conditions
/// exactly, including NaN paths: a below-branch is taken for unordered
/// operands, an above-branch only for ordered-greater. The mixer's
/// stack-resident triple is verified through a call-time snapshot since
/// frame addresses legitimately differ between the sides.
lf_k2_rt::export!(thiscall, rw_008A12F0(parent: u32, ent: *mut u8) -> u32 {
    unsafe {
        let live = *(ent.add(0x54) as *const u32);
        if live != 0 {
            *(ent.add(0x48) as *mut u32) = *(live as *const u32);
        }
        let live = *(ent.add(0x58) as *const u32);
        if live != 0 {
            *(ent.add(0x4C) as *mut u32) = *(live as *const u32);
        }
        let mix_ptr = *(ent.add(0x60) as *const u32);
        let (weight, residual) = if mix_ptr != 0 {
            (clamp01_008A12F0(*(mix_ptr as *const f32)), 0.0f32)
        } else {
            mix_probe_008A12F0(parent, ent)
        };
        let f1: f32 =
            lf_k2_rt::callee_thiscall!(2, f32, ent as u32, weight.to_bits());
        let f2: f32 = lf_k2_rt::callee_thiscall!(
            2,
            f32,
            ent as u32,
            (1.0f32 - weight).to_bits()
        );
        *(ent.add(0x40) as *mut f32) =
            lf_k2_rt::callee_cdecl!(3, f32, f1.to_bits());
        *(ent.add(0x44) as *mut f32) =
            lf_k2_rt::callee_cdecl!(3, f32, f2.to_bits());
        *(ent.add(0x73) as *mut u16) = 0x0101;
        let threshold = *(ent.add(0x50) as *const f32);
        // The flag test compares the weight against the 0.0/1.0 extremes
        // and the first response against the threshold. The original's
        // lahf/test/jp idiom jumps exactly when the operands compare
        // not-equal (ordered-unequal or unordered), i.e. float !=.
        let mode = *ent.add(0x72);
        if mode == 2 {
            if weight != 0.0 {
                if weight != 1.0 {
                    return 0;
                }
                if residual > threshold {
                    *ent.add(0x73) = 0;
                }
                return 0;
            }
            if residual > threshold {
                *ent.add(0x74) = 0;
            }
            return 0;
        }
        if mode == 0 {
            if weight != 0.0 {
                if weight != 1.0 {
                    return 0;
                }
                *ent.add(0x73) = 0;
                return 0;
            }
            *ent.add(0x74) = 0;
            return 0;
        }
        0
    }
});

/// Clamp to 0..1 with the original's exact NaN behaviour: NaN passes
/// through because both below-branches are taken for unordered operands.
fn clamp01_008A12F0(v: f32) -> f32 {
    if v.is_nan() {
        return v;
    }
    if !(0.0f32 < v) {
        return 0.0;
    }
    if v < 1.0 {
        return v;
    }
    1.0
}

/// An SSE below-branch condition: taken when ordered-less or unordered.
fn comiss_below_008A12F0(a: f32, b: f32) -> bool {
    a < b || a.is_nan() || b.is_nan()
}

/// Derive the mix weight by probing the spatial mixer with the bank
/// table triple, interpolating the response between the live bounds,
/// and the residual extreme carried alongside the weight.
fn mix_probe_008A12F0(parent: u32, ent: *mut u8) -> (f32, f32) {
    unsafe {
        let stride = *lf_k2_rt::global::<u32>(0x115D968);
        let table = *lf_k2_rt::global::<u32>(0x115D988);
        let bank = *((parent + 0x40) as *const u8);
        let row = *((table
            .wrapping_add((bank as u32).wrapping_mul(0x6F40))
            .wrapping_add(0x6F14)) as *const u32);
        let idx = *(ent.add(0x70) as *const u16);
        let entry = row.wrapping_add((idx as u32).wrapping_mul(stride));
        let lane = (*((entry + 0xE7) as *const u8) & 7) as u32;
        let taps = (*lf_k2_rt::global::<u32>(0x115F80C + lane * 8))
            .wrapping_add(1)
            .wrapping_mul(2);
        let base = entry.wrapping_add(taps.wrapping_mul(8));
        let mut triple = [
            *(base as *const f32),
            *((base + 4) as *const f32),
            *((base + 8) as *const f32),
        ];
        let r: f32 =
            lf_k2_rt::callee_stdcall!(1, f32, triple.as_mut_ptr() as u32);
        let lo = *(ent.add(0x48) as *const f32);
        let hi = *(ent.add(0x4C) as *const f32);
        let weight = if !comiss_below_008A12F0(lo, r) {
            0.0
        } else if !comiss_below_008A12F0(r, hi) {
            1.0
        } else {
            (r - lo) / (hi - lo)
        };
        let under = lo - r;
        let over = r - hi;
        (weight, if under > over { under } else { over })
    }
}
