// original: 0x00D8AC40 CRenderPhaseTreeImposters::vf8
//
// Specification (renderer phase tree imposter setup). The function looks up a
// context from its receiver, reads the context's index into a table of rows,
// and selects a leaf from the row: the primary leaf when present, otherwise the
// alternate one. A zero context or a zero leaf is an early return. With a leaf
// present, the function computes a bounding sphere radius and a centre from the
// row's minimum and maximum extents (single precision, original operand order)
// and stores radius and centre in the context. The word after the centre is
// never stored by the function before it is read back; it takes the value the
// checker's stack fill gives an unwritten word (zero), see UNWRITTEN_WORD below.
//
// After the context update the function builds small records through the
// allocator and the constructors, passes them to scripted callees, and runs an
// eight-step loop. Each step takes one returned float triple, scales it by the
// centre and a constant, normalises the resulting vector and feeds a second
// returned triple through the same normalisation, then builds two records and
// their vtable tails. A final record and call close the body. The cookie check
// runs after the body in every case except the omitted-cookie mutant.
//
// Locals hold what the original keeps in its frame: each named value is the one
// the frame slot held when the original read it. Scratch records that the
// original passes by address are local arrays with named words; the callees see
// only their addresses, and the checker skips those pointer arguments.

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, relocated};

// Image constants (relocated reads from the mapped image).
const TABLE_BASE: u32 = 0x0129_5CD8;
const COOKIE_VA: u32 = 0x0105_7FB4;
const COUNTER_VA: u32 = 0x0103_27A0;
const OBJ_GLOBAL_VA: u32 = 0x0118_D7F0;
const VT_CTOR_FIRST: u32 = 0x00E7_E048;
const VT_RECORD: u32 = 0x00E8_668C;
const FN_D8B610: u32 = 0x00D8_B610;
const DATA_932F30: u32 = 0x0093_2F30;
const HALF_VA: u32 = 0x00FE_8830;
const RADIUS_SCALE_VA: u32 = 0x0105_6E00;
const C1_VA: u32 = 0x00FE_88E8;
const SCALE_04_VA: u32 = 0x0105_6E04;
const SCALE_08_VA: u32 = 0x0105_6E08;
const INIT_FLOAT_A_VA: u32 = 0x0105_6E0C;
const INIT_FLOAT_B_VA: u32 = 0x0105_6E10;
const LIFT_VA: u32 = 0x00FE_8A24;
const SCALE_D94_VA: u32 = 0x00FE_8D94;
const SIGN_VA: u32 = 0x00FE_8FA0;

// The float 1.0 as pushed by the original (literal bit pattern, not a read).
const ONE_BITS: u32 = 0x3F80_0000;

// Callee ids (one checker stub per id).
const C_LOOKUP: u32 = 1;
const C_COOKIE: u32 = 2;
const C_CTOR_A: u32 = 3;
const C_CTOR_B: u32 = 4;
const C_SETUP_B: u32 = 5;
const C_RELEASE: u32 = 6;
const C_CTOR_C: u32 = 7;
const C_FINISH: u32 = 8;
const C_PUSH_TRIPLE: u32 = 9;
const C_ALLOC_24: u32 = 24;
const C_ALLOC_25: u32 = 25;
const C_ALLOC_26: u32 = 26;
const C_ALLOC_27: u32 = 27;
const C_ALLOC_28: u32 = 28;
const C_ALLOC_29: u32 = 29;
const C_ALLOC_30: u32 = 30;
const C_ALLOC_31: u32 = 31;
const C_ALLOC_32: u32 = 32;
const C_ALLOC_33: u32 = 33;
const C_ALLOC_34: u32 = 34;
const C_ALLOC_35: u32 = 35;
const C_SUB_11: u32 = 11;
const C_SUB_12: u32 = 12;
const C_BLOCK_13: u32 = 13;
const C_D62050: u32 = 14;
const C_CTOR_15: u32 = 15;
const C_FILL_16: u32 = 16;
const C_CTOR_17: u32 = 17;
const C_CTOR_18: u32 = 18;
const C_LINK_19: u32 = 19;
const C_NORM_20: u32 = 20;
const C_NORM_21: u32 = 21;
const C_TAIL_22: u32 = 22;
const C_VCALL: u32 = 23;

// Layout of the context, the row and the leaf (byte offsets).
const CTX_INDEX: u32 = 0x00;
const CTX_SUB_WORD: u32 = 0x0C;
const CTX_RADIUS: u32 = 0x10;
const CTX_CENTRE_X: u32 = 0x20;
const CTX_CENTRE_Y: u32 = 0x24;
const CTX_CENTRE_Z: u32 = 0x28;
const CTX_UNWRITTEN: u32 = 0x2C;
const ROW_PRIMARY: u32 = 0x08;
const ROW_ALTERNATE: u32 = 0x0C;
const ROW_MIN_X: u32 = 0x20;
const ROW_MIN_Y: u32 = 0x24;
const ROW_MIN_Z: u32 = 0x28;
const ROW_MAX_X: u32 = 0x30;
const ROW_MAX_Y: u32 = 0x34;
const ROW_MAX_Z: u32 = 0x38;
const PRIMARY_LEAF: u32 = 0xB4;
const ALTERNATE_LEAF: u32 = 0x00;
const WRONG_PRIMARY_LEAF: u32 = 0xB8;
const WRONG_ALTERNATE_LEAF: u32 = 0x04;
const RECORD_MIX: u32 = 0x04;

// The word the original reads from its own frame before any store and then
// writes into the context. No callee before that read can write it (the only
// call before it takes no pointer), so its value is the checker's stack fill.
const UNWRITTEN_WORD: u32 = 0;

// Mutant selector: one behavioural change per wrong version.
const KIND_NONE: u8 = 0;
const KIND_WRONG_CX: u8 = 1;
const KIND_OMIT_COOKIE: u8 = 2;
const KIND_WRONG_LEAF: u8 = 3;
const KIND_NEIGHBOUR_GLOBAL: u8 = 4;

// Scratch record size in words (the constructors fill it; the stubs do not read it).
const REC_WORDS: usize = 16;

#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn w32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn rf(a: u32) -> f32 {
    f32::from_bits(unsafe { r32(a) })
}
#[inline(always)]
unsafe fn wf(a: u32, v: f32) {
    unsafe { w32(a, v.to_bits()) }
}
#[inline(always)]
unsafe fn gu(va: u32) -> u32 {
    unsafe { r32(relocated(va)) }
}
#[inline(always)]
unsafe fn gf(va: u32) -> f32 {
    unsafe { rf(relocated(va)) }
}

/// Address of a local, as the original would pass a pointer to a frame slot.
#[inline(always)]
fn addr<T>(p: &mut T) -> u32 {
    (p as *mut T as usize) as u32
}

/// Sign flip with the sign-mask constant; its low dword is read from the image.
#[inline(always)]
fn neg(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ unsafe { gu(SIGN_VA) })
}

/// Reciprocal square root scaled by c1, as the original's normalisation does:
/// zero (ordered) gives zero, anything else gives c1 / sqrt(v), NaN included.
#[inline(always)]
fn inv_norm(v: f32, c1: f32) -> f32 {
    if v == 0.0 { 0.0 } else { c1 / v.sqrt() }
}

/// Mod-16 adjustment the original applies to the vtable call result.
#[inline(always)]
fn mod16_fix(a: u32) -> u32 {
    let a = a & 0x8000_000F;
    if (a as i32) < 0 {
        (a.wrapping_sub(1) | 0xFFFF_FFF0).wrapping_add(1)
    } else {
        a
    }
}

/// Two vtable calls on the record and the pad mix that follows them.
unsafe fn vtable_pair(record: u32) {
    unsafe {
        let first = callee_thiscall!(C_VCALL, u32, record);
        let pad_first = mod16_fix(first);
        let pad_gap = mod16_fix(0x10_u32.wrapping_sub(pad_first));
        let second = callee_thiscall!(C_VCALL, u32, record);
        let total = second.wrapping_add(pad_gap);
        let sign_bias = if (total as i32) < 0 { 0xF_u32 } else { 0 };
        let total_biased = total.wrapping_add(sign_bias);
        let shifted = ((total_biased as i32) >> 4) as u32;
        let mix_bits = shifted << 14;
        let mixed = mix_bits ^ r32(record + RECORD_MIX);
        let mask = mixed & 0x01FF_C000;
        w32(record + RECORD_MIX, r32(record + RECORD_MIX) ^ mask);
    }
}

/// Constructor tail: stores the first vtable, mixes the counter into word one,
/// bumps the counter and stores the final vtable. The mutant moves the counter
/// to the neighbouring dword.
unsafe fn ctor_tail(obj: u32, vtable: u32, kind: u8) {
    unsafe {
        let counter_va = COUNTER_VA + if kind == KIND_NEIGHBOUR_GLOBAL { 4 } else { 0 };
        let counter = relocated(counter_va) as *mut u32;
        let word_one = r32(obj + RECORD_MIX);
        w32(obj, relocated(VT_CTOR_FIRST));
        let mix = (word_one ^ counter.read_unaligned()) & 0x3FFF;
        w32(obj + RECORD_MIX, r32(obj + RECORD_MIX) ^ mix);
        counter.write_unaligned(counter.read_unaligned().wrapping_add(1));
        w32(obj, vtable);
    }
}

/// Child block for a record that has just been allocated: two helper values,
/// one read from the context, then the block builder on the record.
unsafe fn build_block(ctx: u32, record: u32) -> u32 {
    unsafe {
        let helper_a = callee_cdecl!(C_SUB_11, u32, 0);
        let ctx_word = r32(ctx + CTX_SUB_WORD);
        let helper_b = callee_cdecl!(C_SUB_12, u32, ctx_word);
        callee_thiscall!(C_BLOCK_13, u32, record, 0, helper_b, helper_a, 0)
    }
}

/// Body of the function. Returns early on the same conditions as the original's
/// early exits; the cookie check that follows is made by `run`.
unsafe fn body(this: u32, kind: u8) {
    unsafe {
        let ctx = callee_thiscall!(C_LOOKUP, u32, this);
        if ctx == 0 {
            return;
        }
        let row_index = r32(ctx + CTX_INDEX);
        let row = r32(relocated(TABLE_BASE) + row_index.wrapping_mul(4));
        let primary = r32(row + ROW_PRIMARY);
        let leaf = if primary != 0 {
            let offset = if kind == KIND_WRONG_LEAF { WRONG_PRIMARY_LEAF } else { PRIMARY_LEAF };
            r32(primary + offset)
        } else {
            let alternate = r32(row + ROW_ALTERNATE);
            if alternate == 0 {
                return;
            }
            let offset = if kind == KIND_WRONG_LEAF { WRONG_ALTERNATE_LEAF } else { ALTERNATE_LEAF };
            r32(alternate + offset)
        };
        if leaf == 0 {
            return;
        }

        // Bounding sphere and centre (single precision, original operand order).
        let half = gf(HALF_VA);
        let min_x = rf(row + ROW_MIN_X);
        let min_y = rf(row + ROW_MIN_Y);
        let min_z = rf(row + ROW_MIN_Z);
        let max_x = rf(row + ROW_MAX_X);
        let max_y = rf(row + ROW_MAX_Y);
        let max_z = rf(row + ROW_MAX_Z);
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let dz = max_z - min_z;
        let diag_sq = ((dx * dx) + (dy * dy)) + (dz * dz);
        let radius = (diag_sq.sqrt() * half) * gf(RADIUS_SCALE_VA);
        let centre_x = if kind == KIND_WRONG_CX {
            (min_x - max_x) * half
        } else {
            (min_x + max_x) * half
        };
        let centre_y = (min_y + max_y) * half;
        let centre_z = (min_z + max_z) * half;

        w32(ctx + CTX_RADIUS, radius.to_bits());
        wf(ctx + CTX_CENTRE_X, centre_x);
        wf(ctx + CTX_CENTRE_Y, centre_y);
        wf(ctx + CTX_CENTRE_Z, centre_z);
        w32(ctx + CTX_UNWRITTEN, UNWRITTEN_WORD);

        // Two constructed records: A (the leaf-side record) and B.
        let mut rec_a = [0u32; REC_WORDS];
        callee_thiscall!(C_CTOR_A, u32, addr(&mut rec_a));
        let mut rec_b = [0u32; REC_WORDS];
        callee_thiscall!(C_CTOR_B, u32, addr(&mut rec_b));

        // Lift and scale the radius for the first push onto the global.
        let scaled_radius = gf(SCALE_08_VA) * radius;
        let lift = radius * gf(LIFT_VA);
        let lift_c1 = lift + gf(C1_VA);
        let neg_scaled = neg(scaled_radius);
        callee_thiscall!(
            C_PUSH_TRIPLE,
            u32,
            relocated(OBJ_GLOBAL_VA),
            addr(&mut rec_a),
            neg_scaled.to_bits(),
            scaled_radius.to_bits(),
            neg_scaled.to_bits(),
            scaled_radius.to_bits(),
            ONE_BITS,
            lift_c1.to_bits()
        );
        callee_thiscall!(C_SETUP_B, u32, addr(&mut rec_a), addr(&mut rec_b));

        // First allocation chain.
        let block_first = callee_cdecl!(C_ALLOC_24, u32, 0x18, 0);
        let child_first = if block_first != 0 { build_block(ctx, block_first) } else { 0 };
        callee_cdecl!(C_RELEASE, u32, child_first);

        // Second scaled triple: the radius plus the constant, scaled by zero and
        // the scale constants, subtracted from the centre.
        let radius_plus = radius + gf(C1_VA);
        let zero_scale = core::hint::black_box(0.0_f32);
        let mut zero_term = radius_plus * zero_scale;
        let scale_04 = gf(SCALE_04_VA);
        let radius_plus_scaled = radius_plus * gf(SCALE_D94_VA);
        zero_term *= scale_04;
        let delta_x = centre_x - zero_term;
        let lift_term = radius_plus_scaled * scale_04;
        let delta_y = centre_y - zero_term;
        let delta_z = centre_z - lift_term;
        let mut delta = [delta_x.to_bits(), delta_y.to_bits(), delta_z.to_bits()];
        let mut centre = [centre_x.to_bits(), centre_y.to_bits(), centre_z.to_bits()];
        let mut unit = [0u32, ONE_BITS, 0u32];
        callee_stdcall!(C_D62050, u32, addr(&mut delta), addr(&mut centre), addr(&mut unit));

        // Basis record: the constructor takes it by address.
        let mut basis = [0u32; 3];
        callee_thiscall!(C_CTOR_C, u32, addr(&mut rec_a), addr(&mut basis));

        // Third allocation: a large record, finished by the next callee.
        let block_large = callee_cdecl!(C_ALLOC_25, u32, 0x410, 0);
        let finished_large = if block_large != 0 {
            callee_thiscall!(C_CTOR_17, u32, block_large, addr(&mut rec_a))
        } else {
            0
        };
        callee_cdecl!(C_RELEASE, u32, finished_large);

        // Fourth allocation and its initialiser.
        let block_small = callee_cdecl!(C_ALLOC_26, u32, 0x18, 0);
        let init_a = gf(INIT_FLOAT_A_VA);
        let init_done = if block_small != 0 {
            callee_thiscall!(C_CTOR_15, u32, block_small, 1, 0, 1, init_a.to_bits(), 1, 0)
        } else {
            0
        };
        callee_cdecl!(C_RELEASE, u32, init_done);

        // Fifth allocation: an index record filled from the row index.
        let block_index = callee_cdecl!(C_ALLOC_27, u32, 8, 0);
        let indexed = if block_index != 0 {
            callee_thiscall!(C_FILL_16, u32, block_index, r32(ctx + CTX_INDEX))
        } else {
            0
        };
        callee_cdecl!(C_RELEASE, u32, indexed);

        // Link the record and the row pointer, with the frame word set to -1.
        let mut link_word = u32::MAX;
        let mut row_word = row;
        callee_cdecl!(C_LINK_19, u32, relocated(FN_D8B610), addr(&mut row_word), addr(&mut link_word));

        // Sixth allocation: a record with a constructor tail.
        let block_tail = callee_cdecl!(C_ALLOC_28, u32, 8, 0);
        if block_tail != 0 {
            ctor_tail(block_tail, relocated(VT_RECORD), kind);
        }
        callee_cdecl!(C_RELEASE, u32, block_tail);

        // Seventh allocation: a plane record (1.0, 1.0, 0xFF000000, zeros, and a
        // byte-sized flag of one) passed to the constructor.
        let mut plane = [ONE_BITS, ONE_BITS, 0xFF00_0000, 0, 0, 0, 1];
        let block_plane = callee_cdecl!(C_ALLOC_29, u32, 0x2C, 0);
        let plane_done = if block_plane != 0 {
            callee_thiscall!(C_CTOR_18, u32, block_plane, 0, addr(&mut plane))
        } else {
            0
        };
        callee_cdecl!(C_RELEASE, u32, plane_done);

        // Second push onto the global, using the radius and the lift again.
        let neg_radius = neg(radius);
        callee_thiscall!(
            C_PUSH_TRIPLE,
            u32,
            relocated(OBJ_GLOBAL_VA),
            addr(&mut rec_a),
            neg_radius.to_bits(),
            radius.to_bits(),
            neg_radius.to_bits(),
            radius.to_bits(),
            ONE_BITS,
            lift_c1.to_bits()
        );
        callee_thiscall!(C_SETUP_B, u32, addr(&mut rec_a), addr(&mut rec_b));

        // Second allocation chain (same helper sequence as the first).
        let block_second = callee_cdecl!(C_ALLOC_30, u32, 0x18, 0);
        let child_second = if block_second != 0 { build_block(ctx, block_second) } else { 0 };
        callee_cdecl!(C_RELEASE, u32, child_second);

        // Eight-step loop over the returned triples.
        for counter in 0..8u32 {
            let triple = callee_cdecl!(C_NORM_20, u32, addr(&mut delta), counter);
            let t0 = radius_plus;
            let x1 = (rf(triple) * t0) * scale_04;
            let x2 = (rf(triple + 4) * t0) * scale_04;
            let x3 = (rf(triple + 8) * t0) * scale_04;
            let x0 = centre_x - x1;
            let y = centre_y - x2;
            let z = centre_z - x3;
            let mut vx = x0 - centre_x;
            let mut vy = y - centre_y;
            let mut vz = z - centre_z;

            let pair = callee_cdecl!(C_NORM_21, u32, addr(&mut unit), counter);

            let mut v = vy * vy;
            let vx_sq = vx * vx;
            v += vx_sq;
            let vz_sq = vz * vz;
            v += vz_sq;
            let inv = inv_norm(v, gf(C1_VA));
            vx *= inv;
            vy *= inv;
            vz *= inv;
            let n_x = vx;
            let n_y = vy;
            let n_z = vz;

            // Second triple: the cross and dot terms of the normalised vector and
            // the returned triple, normalised again.
            let a3 = (rf(pair + 4) * n_z) - (rf(pair + 8) * n_y);
            let mut r2 = rf(pair + 8) * n_x;
            let q = rf(pair) * n_z;
            r2 -= q;
            let mut a4 = rf(pair) * n_y;
            let t = rf(pair + 4) * n_x;
            a4 -= t;
            let r2sq = r2 * r2;
            let mut v2 = r2sq + (a3 * a3);
            v2 += a4 * a4;
            let inv2 = inv_norm(v2, gf(C1_VA));
            let r2n = r2 * inv2;
            let a4n = a4 * inv2;
            let a3n = a3 * inv2;
            let m1 = (a4n * n_y) - (r2n * n_z);
            let k0 = a3n * n_z;
            let k4 = a4n * n_x;
            let k2 = r2n * n_x;
            let k3 = a3n * n_y;
            let v2b = k0 - k4;
            let v3 = k2 - k3;

            let mut basis_step = [a3n.to_bits(), r2n.to_bits(), a4n.to_bits()];
            let _ = (m1, v2b, v3);
            callee_thiscall!(C_CTOR_C, u32, addr(&mut rec_a), addr(&mut basis_step));

            let record_wide = callee_cdecl!(C_ALLOC_31, u32, 0x410, 0);
            let edi = if record_wide != 0 {
                callee_thiscall!(C_CTOR_17, u32, record_wide, addr(&mut rec_a))
            } else {
                0
            };
            vtable_pair(edi);

            let block_mid = callee_cdecl!(C_ALLOC_32, u32, 0x18, 0);
            let mid_init = if block_mid != 0 {
                if counter == 0 {
                    callee_thiscall!(C_CTOR_15, u32, block_mid, 1, 0, 1, gf(INIT_FLOAT_B_VA).to_bits(), 1, 0)
                } else {
                    callee_thiscall!(C_CTOR_15, u32, block_mid, 0, 0, 1, gf(INIT_FLOAT_B_VA).to_bits(), 1, 0)
                }
            } else {
                0
            };
            callee_cdecl!(C_RELEASE, u32, mid_init);

            let block_index_step = callee_cdecl!(C_ALLOC_33, u32, 8, 0);
            let edi_f = if block_index_step != 0 {
                callee_thiscall!(C_FILL_16, u32, block_index_step, r32(ctx + CTX_INDEX))
            } else {
                0
            };
            vtable_pair(edi_f);

            let mut step_counter = counter;
            let mut step_row = row;
            callee_cdecl!(C_LINK_19, u32, relocated(FN_D8B610), addr(&mut step_row), addr(&mut step_counter));

            let block_step_tail = callee_cdecl!(C_ALLOC_34, u32, 8, 0);
            if block_step_tail != 0 {
                ctor_tail(block_step_tail, relocated(VT_RECORD), kind);
            }
            vtable_pair(block_step_tail);
        }

        // Tail: the last record, the final call and the finish.
        let ctx_word_ptr = ctx + CTX_SUB_WORD;
        let data_932 = relocated(DATA_932F30);
        callee_cdecl!(C_TAIL_22, u32, data_932, ctx_word_ptr);
        let block_end = callee_cdecl!(C_ALLOC_35, u32, 0x2C, 0);
        let end_record = if block_end != 0 {
            callee_thiscall!(C_CTOR_18, u32, block_end, 0, addr(&mut plane))
        } else {
            0
        };
        vtable_pair(end_record);
        callee_thiscall!(C_FINISH, u32, addr(&mut rec_a));
    }
}

unsafe fn run(this: u32, kind: u8) {
    unsafe {
        body(this, kind);
        if kind != KIND_OMIT_COOKIE {
            callee_thiscall!(C_COOKIE, u32, gu(COOKIE_VA));
        }
    }
}

export!(thiscall, rw_d8ac40_full(this: u32) -> u32 {
    unsafe { run(this, KIND_NONE) };
    0
});

export!(thiscall, rw_d8ac40_stage2(this: u32) -> u32 {
    unsafe { run(this, KIND_NONE) };
    0
});

// Same-contract mutant: the x centre is computed as min minus max.
export!(thiscall, mut_d8ac40_full_wrong_cx(this: u32) -> u32 {
    unsafe { run(this, KIND_WRONG_CX) };
    0
});

export!(thiscall, mut_d8ac40_stage4_wrong_center_x(this: u32) -> u32 {
    unsafe { run(this, KIND_WRONG_CX) };
    0
});

export!(thiscall, mut_d8ac40_stage2_omit_cookie(this: u32) -> u32 {
    unsafe { run(this, KIND_OMIT_COOKIE) };
    0
});

export!(thiscall, mut_d8ac40_stage2_wrong_b4_offset(this: u32) -> u32 {
    unsafe { run(this, KIND_WRONG_LEAF) };
    0
});

// Same-contract mutant: the counter word moves to the neighbouring dword.
export!(thiscall, mut_d8ac40_full_neighbour_global(this: u32) -> u32 {
    unsafe { run(this, KIND_NEIGHBOUR_GLOBAL) };
    0
});
