// original: 0x00650430 ptx_effect_evaluator_exit_path

// Specification.
//
// Evaluates one particle-effect rule set. The object `this` holds a scratch
// flag byte (+0x69C) and a float (+0x698); `a1` is the effect object (a
// table pointer at +0x260, the payload at +0x78, scale words at +0x1A0,
// +0x1E0, +0x1E4, +0x1E8, +0x1EC); `a2` is the rule cursor, whose dwords
// hold the per-record counts, exit words and scale words, and whose record
// pointer and float fields feed the clamp arithmetic. `a3` is an optional
// second object: callee 20 runs only when it is nonzero.
//
// Steps, in call order:
//  1. Read the global record byte `dl` and the table float for it.
//  2. Call callee 10 (cdecl, five words) for the header answer H.
//  3. When the scratch flag is nonzero, call callee 14 with the float; its
//     answer becomes the result of the early exit.
//  4. If the cursor's exit word for `dl` is zero, call callee 13 (the
//     security cookie check) and return the answer (`this`, or callee 14's
//     answer when it ran).
//  5. Otherwise the divisor is 3 when the scratch flag is set and 1 when it
//     is not (2 * flag + 1), and the clamp quotient is 0x97B / divisor
//     (signed). Loop over the outer passes: each takes the smaller, signed,
//     of the cursor count less the accumulator and the quotient; calls the
//     sizer (callee 11), allocates (callee 12) and publishes the allocation
//     and its scaled block to the two globals.
//  6. A null allocation calls callee 13 and returns 0.
//  7. Otherwise the inner loop runs once per count, each pass on the record
//     at T + 0x100 + dl*96 (T the table pointer, which is the exit word on
//     the first pass and T+8 afterwards). A null table pointer takes the
//     memset path: callee 24 runs once per divisor on the allocation's
//     blocks. Each pass calls callee 16 (the function table), scales the
//     four output words, branches on the header answer's flag bit 2 (callee
//     17 writes two words, then they are combined with the table and the
//     scales), calls callee 18 and callee 19, scales the products, runs the
//     record search and callees 20 (twice, when a3 is nonzero) and 21,
//     clamps the cursor's floats, calls callee 22 (square root) and callee 23
//     with ten words, and advances T.
//  8. After the outer pass the accumulator takes the count; callee 15 runs
//     when the allocation cell is nonzero; the outer loop repeats while the
//     last exit word is nonzero.
//  9. Call callee 13 and return the accumulated count.
//
// Float arithmetic is single precision, operands in the order the original
// uses them (see mul_ss, add_ss, sub_ss). Callee 17 takes its third word as
// T+0x5C in production; the register-transport form (the checker's copy of
// this body, export rw_00650430_xmm3) passes the table float there instead,
// so the stock checker can load XMM3 from a stack word.
//
// Narrowings are listed in the verification result, not here. In particular
// callee 18 is entered with a float in XMM2 (the record float, or a frame
// word when object flag bit 15 is set) that this body does not model.

use core::hint::black_box;

// Globals.
const G_OBJ: u32 = 0x01BB_6678;
const G_WR0: u32 = 0x018B_7534;
const G_WR1: u32 = 0x018B_7538;
const G_SIZER_ARG: u32 = 0x018B_753C;
const G_TABLE_F: u32 = 0x0110_EF20; // out-block constant of callee 16 (VT_OUT_BLOCK)
const G_SCALE_BLOCK: u32 = 0x0110_EF10; // per-lane scale of the callee 16 out block
const G_TABLE_VAL: u32 = 0x018B_7588; // float table indexed by a byte
const G_C17_SCALE: u32 = 0x00FE_870C;
const G_B_SCALE: u32 = 0x00FE_86E8;
const G_C_CLAMP: u32 = 0x00FE_88E8;
const G_C_SCALE_E: u32 = 0x00FE_8C08;
const G_C_D: u32 = 0x00FE_8728;
const G_C_FLAG3: u32 = 0x0111_011C;
const G_C_ONE: u32 = 0x00FE_88E8;
const TAG_STD: u32 = 0x0115_0A84;
const TAG_BASE: u32 = 0x0115_0AA8;
const COOKIE_CHECK_CALLEE: u32 = 13;

// Object and cursor offsets.
const A1_TABLE: u32 = 0x260;
const A1_PAYLOAD: u32 = 0x78;
const A1_SCALE: u32 = 0x1E8;
const A1_SCALE_B: u32 = 0x1E4;
const A1_SCALE_C: u32 = 0x1A0;
const A1_K: u32 = 0x1E0;
const A1_SCALE_D: u32 = 0x1EC;
const A1_REC_WORD0: u32 = 0x180;
const A1_REC_WORD1: u32 = 0x184;
const A1_REC_WORD2: u32 = 0x188;
const A2_OBJECT: u32 = 0xE0;
const A2_SCALE: u32 = 0xF4;
const A2_LIVE_COUNT: u32 = 0x14;
const A2_EC: u32 = 0xEC;
const A2_CLAMP_X: u32 = 0x60;
const A2_CLAMP_Y: u32 = 0x70;
const THIS_SCRATCH_FLAG: u32 = 0x69C;
const THIS_SCRATCH_FLOAT: u32 = 0x698;
const THIS_FLAGS: u32 = 0x11C;
const THIS_CHILD: u32 = 0x188;
const THIS_COUNT_A: u32 = 0x4C0;
const THIS_RECS_A: u32 = 0x4BC;
const THIS_COUNT_B: u32 = 0x4F4;
const THIS_RECS_B: u32 = 0x4F0;
const THIS_PARAM_1CC: u32 = 0x4CC;
const THIS_PARAM_500: u32 = 0x500;
const THIS_BYTE_4D5: u32 = 0x4D5;
const THIS_BYTE_509: u32 = 0x509;
const THIS_FN_678: u32 = 0x678;
const CLAMP_QUOTIENT: i32 = 0x97B;
const RECORD_STRIDE: u32 = 48;
const RECORD_BLOCK_STRIDE: u32 = 96;
const ALLOC_SCALE: u32 = 0x288;
const ALLOC_TAG: u32 = 0x6C;

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn grd32(rva: u32) -> u32 {
    unsafe { rd32(lf_checker_rt::relocated(rva)) }
}

#[inline(always)]
unsafe fn gwr32(rva: u32, v: u32) {
    unsafe { wr32(lf_checker_rt::relocated(rva), v) }
}

/// Single-precision multiply, destination operand first (black_box pins the order).
#[inline(always)]
fn mul_ss(dst_bits: u32, src_bits: u32) -> u32 {
    let a = black_box(f32::from_bits(dst_bits));
    let b = black_box(f32::from_bits(src_bits));
    black_box(a * b).to_bits()
}

/// Single-precision add, destination operand first.
#[inline(always)]
fn add_ss(dst_bits: u32, src_bits: u32) -> u32 {
    let a = black_box(f32::from_bits(dst_bits));
    let b = black_box(f32::from_bits(src_bits));
    black_box(a + b).to_bits()
}

/// Single-precision subtract, destination operand first.
#[inline(always)]
fn sub_ss(dst_bits: u32, src_bits: u32) -> u32 {
    let a = black_box(f32::from_bits(dst_bits));
    let b = black_box(f32::from_bits(src_bits));
    black_box(a - b).to_bits()
}

/// Truncating float-to-int conversion as the x86 instruction does it: NaN and
/// out-of-range values give the indefinite integer 0x80000000.
#[inline(always)]
fn cvt_trunc_i32(bits: u32) -> u32 {
    let f = f32::from_bits(bits);
    if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
        0x8000_0000
    } else {
        (f as i32) as u32
    }
}

/// Record search over the records at `base` (48 bytes each): the index (from
/// one) of the first record whose first float is not below `key`, else
/// `count`. A count of one or less gives one.
fn record_search(base: u32, count: u32, key: u32) -> u32 {
    if count <= 1 {
        return 1;
    }
    let key_f = f32::from_bits(key);
    let mut i: u32 = 1;
    while i < count {
        let first = unsafe { rd32(base.wrapping_add(i.wrapping_mul(RECORD_STRIDE))) };
        if f32::from_bits(first) >= key_f {
            return i;
        }
        i += 1;
    }
    count
}

/// The body. `XMM3_TRANSPORT` is false in production form. It is true only for
/// the checker's transport export, which passes the table float as callee
/// 17's third stack word (the stock stub loads XMM3 from that word); the
/// production call passes T+0x5C there.
#[inline(never)]
unsafe fn evaluate<const XMM3_TRANSPORT: bool>(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        // 1. Global record byte and the table float read before callee 10.
        let g = grd32(G_OBJ);
        let dl = rd8(g.wrapping_add(2)) as u32;
        let table_f = rd32(
            rd32(a1.wrapping_add(A1_TABLE))
                .wrapping_add(dl.wrapping_mul(12))
                .wrapping_add(0x28C),
        );

        // 2. Header answer.
        let head: u32 = lf_checker_rt::callee_cdecl!(
            10,
            u32,
            rd32(a1.wrapping_add(A1_PAYLOAD)),
            0,
            lf_checker_rt::relocated(TAG_BASE),
            lf_checker_rt::relocated(TAG_STD),
            0
        );

        // 3. Scratch flag; the float-setup answer is the early-exit result.
        let scratch_flag = rd8(this.wrapping_add(THIS_SCRATCH_FLAG));
        let mut answer = this;
        if scratch_flag != 0 {
            let float_bits = rd32(this.wrapping_add(THIS_SCRATCH_FLOAT));
            answer = lf_checker_rt::callee_cdecl!(14, u32, float_bits);
        }

        // 4. Early exit when the cursor's exit word is zero.
        let exit_word = rd32(a2.wrapping_add(dl.wrapping_mul(4)).wrapping_add(4));
        if exit_word == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK_CALLEE, u32,);
            return answer;
        }

        // 5. Divisor and clamp quotient.
        let divisor = ((scratch_flag != 0) as u32).wrapping_mul(2).wrapping_add(1);
        let quot = (CLAMP_QUOTIENT).wrapping_div(divisor as i32) as u32;

        let mut acc: u32 = 0;
        let mut t = exit_word; // table pointer for the next pass
        // Words the original leaves in two frame blocks from one pass to the next:
        // the call-18 block holds the last products (callee 18 reads it at entry,
        // as its snapshot shows), and the call-19 block holds what callee 19 last
        // wrote. The stack is zero before the first pass.
        let mut block18_carry: [u32; 4] = [0; 4];
        let mut block19_carry: [u32; 4] = [0; 4];
        loop {
            // Outer pass: count and allocation.
            let live = rd32(a2.wrapping_add(dl.wrapping_mul(4)).wrapping_add(A2_LIVE_COUNT))
                .wrapping_sub(acc);
            let m = if (live as i32) < (quot as i32) { live } else { quot };
            let esi = divisor.wrapping_mul(m);
            let _size: u32 = lf_checker_rt::callee_thiscall!(
                11,
                u32,
                m,
                grd32(G_SIZER_ARG)
            );
            let alloc_arg = esi.wrapping_mul(6);
            let got: u32 =
                lf_checker_rt::callee_cdecl!(12, u32, 3, alloc_arg, ALLOC_TAG);
            let scaled = esi.wrapping_mul(ALLOC_SCALE);
            gwr32(G_WR1, got);
            gwr32(G_WR0, scaled.wrapping_add(got));

            // 6. Null allocation: cookie check, result 0.
            if got == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK_CALLEE, u32,);
                return 0;
            }

            // 7. Inner loop over the count.
            let mut counter = (m as i32).wrapping_sub(1);
            let mut memset_path = false;
            if counter >= 0 {
                loop {
                    if t == 0 {
                        memset_path = true;
                        break;
                    }
                    let rec = t.wrapping_add(0x100).wrapping_add(dl.wrapping_mul(RECORD_BLOCK_STRIDE));
                    let obj = rd32(a2.wrapping_add(A2_OBJECT));

                    // Callee 16 (function table): out block, then lane scaling.
                    let mut out_block: [u32; 4] = [
                        grd32(G_TABLE_F),
                        grd32(G_TABLE_F + 4),
                        grd32(G_TABLE_F + 8),
                        grd32(G_TABLE_F + 12),
                    ];
                    let float_74 = rd32(t.wrapping_add(0x74));
                    let _vcall: u32 = lf_checker_rt::callee_thiscall!(
                        16,
                        u32,
                        obj,
                        out_block.as_mut_ptr() as u32,
                        float_74,
                        t.wrapping_add(0x5C),
                        a3
                    );
                    let scale_lane = [
                        grd32(G_SCALE_BLOCK),
                        grd32(G_SCALE_BLOCK + 4),
                        grd32(G_SCALE_BLOCK + 8),
                        grd32(G_SCALE_BLOCK + 12),
                    ];
                    for lane in 0..4 {
                        out_block[lane] = mul_ss(out_block[lane], scale_lane[lane]);
                    }

                    // Header flag bit 2 selects callee 17. The branch combines
                    // its two out-block words with the table and the scales.
                    let w = rd32(head.wrapping_add(0x100));
                    let bit2 = w != 0 && (rd8(w.wrapping_add(0x10)) & 4) != 0;
                    let r17: u32 = if bit2 {
                        let mut blk17: [u32; 2] = [0; 2];
                        let c17_third = if XMM3_TRANSPORT { table_f } else { t.wrapping_add(0x5C) };
                        let _c17: u32 = lf_checker_rt::callee_stdcall!(
                            17,
                            u32,
                            a1,
                            blk17.as_mut_ptr() as u32,
                            c17_third
                        );
                        let table_k = rd8(a1.wrapping_add(A1_K)) as u32;
                        let table_v = grd32(G_TABLE_VAL.wrapping_add(table_k.wrapping_mul(4)));
                        let c_g = grd32(G_C17_SCALE);
                        let scale_a2 = mul_ss(rd32(a2.wrapping_add(A2_SCALE)), c_g);
                        let diff17 = sub_ss(blk17[1], blk17[0]);
                        let x1 = add_ss(mul_ss(diff17, table_v), blk17[0]);
                        let x2 = mul_ss(x1, rd32(a1.wrapping_add(A1_SCALE_D)));
                        mul_ss(x2, scale_a2)
                    } else {
                        mul_ss(rd32(a1.wrapping_add(A1_SCALE)), rd32(a2.wrapping_add(A2_SCALE)))
                    };

                    // Callee 18 on the child object, with the flag word's bit 23.
                    let flags = rd32(this.wrapping_add(THIS_FLAGS));
                    let r50 = rd32(rec.wrapping_add(0x50));
                    let flag = (flags >> 23) & 1;
                    let mut block18: [u32; 4] = block18_carry;
                    let _c18: u32 = lf_checker_rt::callee_thiscall!(
                        18,
                        u32,
                        this.wrapping_add(THIS_CHILD),
                        block18.as_mut_ptr() as u32,
                        flag,
                        t.wrapping_add(0x10),
                        a3
                    );

                    // Four products of callee 18's words with the bytes of the cursor's +0xEC word.
                    let c_scale = grd32(G_B_SCALE);
                    let p_ec = rd32(a2.wrapping_add(A2_EC));
                    let b0 = p_ec & 0xFF;
                    let b1 = (p_ec >> 8) & 0xFF;
                    let b2 = (p_ec >> 16) & 0xFF;
                    let b3 = p_ec >> 24;
                    let scale_b2 = mul_ss(mul_ss((b2 as i32 as f32).to_bits(), c_scale), block18[0]);
                    let scale_b1 = mul_ss(mul_ss((b1 as i32 as f32).to_bits(), c_scale), block18[1]);
                    let scale_b0 = mul_ss(mul_ss((b0 as i32 as f32).to_bits(), c_scale), block18[2]);
                    let scale_b3 = mul_ss(mul_ss((b3 as i32 as f32).to_bits(), c_scale), block18[3]);
                    let scale_a1 = mul_ss(
                        mul_ss(rd32(a1.wrapping_add(A1_SCALE_B)), rd32(a1.wrapping_add(A1_SCALE_C))),
                        scale_b3,
                    );

                    // Callee 19 on the header answer.
                    let mut block19: [u32; 4] = block19_carry;
                    let _c19: u32 = lf_checker_rt::callee_thiscall!(
                        19,
                        u32,
                        head,
                        a1,
                        block19.as_mut_ptr() as u32,
                        t.wrapping_add(0x5C)
                    );
                    let w0 = mul_ss(block19[0], scale_b2);
                    let w1 = mul_ss(block19[1], scale_b1);
                    let w2 = mul_ss(block19[2], scale_b0);
                    let w3 = mul_ss(block19[3], scale_a1);
                    block19_carry = block19;

                    // Object flag bit 3 scales the four products by the record words.
                    let (wv0, wv1, wv2, wv3) = if (flags >> 3) & 1 != 0 {
                        let c_f = grd32(G_C_FLAG3);
                        (
                            mul_ss(rd32(a1.wrapping_add(A1_REC_WORD0)), w0),
                            mul_ss(rd32(a1.wrapping_add(A1_REC_WORD1)), w1),
                            mul_ss(rd32(a1.wrapping_add(A1_REC_WORD2)), w2),
                            mul_ss(c_f, w3),
                        )
                    } else {
                        (w0, w1, w2, w3)
                    };
                    // The four products are stored back into the call-18 block for the next pass.
                    block18_carry = [wv0, wv1, wv2, wv3];

                    // Record search on the first record list.
                    let count = (rd8(this.wrapping_add(THIS_COUNT_A)) as u32)
                        | ((rd8(this.wrapping_add(THIS_COUNT_A + 1)) as u32) << 8);
                    let rec_idx = record_search(rd32(this.wrapping_add(THIS_RECS_A)), count, r50);
                    let rec_b = rd32(this.wrapping_add(THIS_RECS_A))
                        .wrapping_add(rec_idx.wrapping_sub(1).wrapping_mul(RECORD_STRIDE));
                    let dlt = sub_ss(r50, rd32(rec_b));
                    let x0 = add_ss(mul_ss(dlt, rd32(rec_b.wrapping_add(0x20))), rd32(rec_b.wrapping_add(0x10)));
                    let x1 = add_ss(mul_ss(rd32(rec_b.wrapping_add(0x24)), dlt), rd32(rec_b.wrapping_add(0x14)));
                    let x2 = add_ss(mul_ss(rd32(rec_b.wrapping_add(0x28)), dlt), rd32(rec_b.wrapping_add(0x18)));
                    let x3 = add_ss(mul_ss(rd32(rec_b.wrapping_add(0x2C)), dlt), rd32(rec_b.wrapping_add(0x1C)));

                    // Callee 20, first call (only when a3 is nonzero).
                    let mut block20: [u32; 4] = [x0, x1, x2, x3];
                    if a3 != 0 {
                        let _c20: u32 = lf_checker_rt::callee_thiscall!(
                            20,
                            u32,
                            a3,
                            3,
                            block20.as_mut_ptr() as u32,
                            rd32(this.wrapping_add(THIS_PARAM_1CC)),
                            r50,
                            t.wrapping_add(0x5C)
                        );
                    }

                    // Blends of the callee 20 block with the table entries.
                    let edx_t10 = t.wrapping_add(0x10);
                    let cl_flag = rd8(this.wrapping_add(THIS_BYTE_4D5));
                    let idx_8c = rd8(edx_t10.wrapping_add(0x8C)) as u32;
                    let c_b = grd32(G_C_ONE);
                    let g_8c = grd32(G_TABLE_VAL.wrapping_add(idx_8c.wrapping_mul(4)));
                    let xmm4 = if cl_flag != 0 { sub_ss(c_b, g_8c) } else { g_8c };
                    let idx_8d = rd8(edx_t10.wrapping_add(0x8D)) as u32;
                    let g_8d = grd32(G_TABLE_VAL.wrapping_add(idx_8d.wrapping_mul(4)));
                    let xmm0_b = if cl_flag != 0 { sub_ss(c_b, g_8d) } else { g_8d };
                    let b0_20 = block20[0];
                    let b1_20 = block20[1];
                    let b2_20 = block20[2];
                    let b3_20 = block20[3];
                    let xmm1_c = add_ss(mul_ss(sub_ss(b2_20, b0_20), xmm4), b0_20);
                    let xmm2_c = add_ss(mul_ss(sub_ss(b3_20, b1_20), xmm0_b), b1_20);
                    let scale_v1 = r17;
                    let p6 = mul_ss(mul_ss(out_block[0], xmm1_c), scale_v1);
                    let p7 = mul_ss(mul_ss(out_block[1], xmm2_c), scale_v1);

                    // Second record search and callee 20, second call.
                    let count2 = (rd8(this.wrapping_add(THIS_COUNT_B)) as u32)
                        | ((rd8(this.wrapping_add(THIS_COUNT_B + 1)) as u32) << 8);
                    let rec_idx2 = record_search(rd32(this.wrapping_add(THIS_RECS_B)), count2, r50);
                    let rec_b2 = rd32(this.wrapping_add(THIS_RECS_B))
                        .wrapping_add(rec_idx2.wrapping_sub(1).wrapping_mul(RECORD_STRIDE));
                    let d2 = sub_ss(r50, rd32(rec_b2));
                    let y0 = add_ss(mul_ss(rd32(rec_b2.wrapping_add(0x20)), d2), rd32(rec_b2.wrapping_add(0x10)));
                    let y1 = add_ss(mul_ss(rd32(rec_b2.wrapping_add(0x24)), d2), rd32(rec_b2.wrapping_add(0x14)));
                    let y2 = add_ss(mul_ss(rd32(rec_b2.wrapping_add(0x28)), d2), rd32(rec_b2.wrapping_add(0x18)));
                    let y3 = add_ss(mul_ss(rd32(rec_b2.wrapping_add(0x2C)), d2), rd32(rec_b2.wrapping_add(0x1C)));
                    let mut block20b: [u32; 4] = [y0, y1, y2, y3];
                    if a3 != 0 {
                        let _c20b: u32 = lf_checker_rt::callee_thiscall!(
                            20,
                            u32,
                            a3,
                            3,
                            block20b.as_mut_ptr() as u32,
                            rd32(this.wrapping_add(THIS_PARAM_500)),
                            r50,
                            t.wrapping_add(0x5C)
                        );
                    }
                    // The original reloads Y0 and Y1 from the block after the call.
                    let (y0p, y1p) = if a3 != 0 { (block20b[0], block20b[1]) } else { (y0, y1) };

                    // Callee 21 on the child object's table.
                    let idx_8e = rd8(t.wrapping_add(0x10).wrapping_add(0x8E)) as u32;
                    let g_8e = grd32(G_TABLE_VAL.wrapping_add(idx_8e.wrapping_mul(4)));
                    let sel21 = if rd8(this.wrapping_add(THIS_BYTE_509)) != 0 { sub_ss(c_b, g_8e) } else { g_8e };
                    let c_d = grd32(G_C_D);
                    let t94 = rd32(t.wrapping_add(0x10).wrapping_add(0x84));
                    let r10 = rd32(rec.wrapping_add(0x10));
                    let r14 = rd32(rec.wrapping_add(0x14));
                    let r18 = rd32(rec.wrapping_add(0x18));
                    let r58 = rd32(rec.wrapping_add(0x58));
                    let pre_blend = add_ss(mul_ss(sub_ss(y1p, y0p), sel21), y0p);
                    let scaled_blend = mul_ss(pre_blend, c_d);
                    let blend = add_ss(add_ss(scaled_blend, t94), r58);
                    // The larger of p6 and p7 (ties and unordered take p7).
                    let m21 = if f32::from_bits(p6) > f32::from_bits(p7) { p6 } else { p7 };
                    // Clamps store into the cursor only when the stored word is larger.
                    let x_a = sub_ss(r10, m21);
                    if f32::from_bits(rd32(a2.wrapping_add(A2_CLAMP_X))) > f32::from_bits(x_a) {
                        wr32(a2.wrapping_add(A2_CLAMP_X), x_a);
                    }
                    let x_b = sub_ss(r14, m21);
                    if f32::from_bits(rd32(a2.wrapping_add(A2_CLAMP_X + 4))) > f32::from_bits(x_b) {
                        wr32(a2.wrapping_add(A2_CLAMP_X + 4), x_b);
                    }
                    let x_c = sub_ss(r18, m21);
                    if f32::from_bits(rd32(a2.wrapping_add(A2_CLAMP_X + 8))) > f32::from_bits(x_c) {
                        wr32(a2.wrapping_add(A2_CLAMP_X + 8), x_c);
                    }
                    let y_a = add_ss(r10, m21);
                    if f32::from_bits(y_a) > f32::from_bits(rd32(a2.wrapping_add(A2_CLAMP_Y))) {
                        wr32(a2.wrapping_add(A2_CLAMP_Y), y_a);
                    }
                    let y_b = add_ss(r14, m21);
                    if f32::from_bits(y_b) > f32::from_bits(rd32(a2.wrapping_add(A2_CLAMP_Y + 4))) {
                        wr32(a2.wrapping_add(A2_CLAMP_Y + 4), y_b);
                    }
                    let y_c = add_ss(r18, m21);
                    if f32::from_bits(y_c) > f32::from_bits(rd32(a2.wrapping_add(A2_CLAMP_Y + 8))) {
                        wr32(a2.wrapping_add(A2_CLAMP_Y + 8), y_c);
                    }

                    // Callee 21 on this+0x678: an eight-word block, zeroed first.
                    let mut blk21: [u32; 8] = [0; 8];
                    let _c21: u32 = lf_checker_rt::callee_thiscall!(
                        21,
                        u32,
                        this.wrapping_add(THIS_FN_678),
                        blk21.as_mut_ptr() as u32
                    );

                    // Callee 22: square root of the sum of the squares of the record's
                    // floats at +0x30, +0x34 and +0x38.
                    let r30 = rd32(rec.wrapping_add(0x30));
                    let r34 = rd32(rec.wrapping_add(0x34));
                    let r38 = rd32(rec.wrapping_add(0x38));
                    let sq_a = mul_ss(r34, r34);
                    let sq_b = mul_ss(r30, r30);
                    let sum_ab = add_ss(sq_a, sq_b);
                    let sq_c = mul_ss(r38, r38);
                    let sum22 = add_ss(sum_ab, sq_c);
                    let sqrt22: f32 = lf_checker_rt::callee_cdecl!(22, f32, sum22);
                    let d_f0 = sqrt22.to_bits();

                    // Four clamped words packed as bytes: B3, B0, B1, B2 (high to low).
                    let c_clamp_hi = grd32(G_C_CLAMP);
                    let c_scale_e = grd32(G_C_SCALE_E);
                    let clamp_unit = |v: u32| -> u32 {
                        let lo = if f32::from_bits(0) > f32::from_bits(v) { 0 } else { v };
                        if f32::from_bits(lo) > f32::from_bits(c_clamp_hi) { c_clamp_hi } else { lo }
                    };
                    let byte_of = |v: u32| -> u32 { cvt_trunc_i32(mul_ss(clamp_unit(v), c_scale_e)) & 0xFF };
                    let b3 = byte_of(wv3);
                    let b0 = byte_of(wv0);
                    let b1 = byte_of(wv1);
                    let b2 = byte_of(wv2);
                    let packed = (b3 << 24) | (b0 << 16) | (b1 << 8) | b2;

                    // Callee 23: ten stack words. Word 4 is the blend with its sign flipped.
                    let flag_bit27 = ((flags >> 27) & 1) as u8;
                    let neg_z = blend ^ 0x8000_0000;
                    let _c23: u32 = lf_checker_rt::callee_cdecl!(
                        23,
                        u32,
                        r14,
                        r18,
                        p6,
                        p7,
                        neg_z,
                        packed,
                        d_f0,
                        r50,
                        blk21.as_mut_ptr() as u32,
                        flag_bit27 as u32
                    );

                    // Advance the table pointer: T+8 is the next exit word.
                    t = rd32(t.wrapping_add(8).wrapping_add(dl.wrapping_mul(4)));
                    counter = counter.wrapping_sub(1);
                    if counter < 0 {
                        break;
                    }
                }
            }

            // Memset path: callee 24 once per divisor on the allocation's blocks.
            if memset_path {
                let mut block_p = grd32(G_WR1);
                let mut off_p = 0u32;
                let mut passes = divisor;
                loop {
                    if block_p != 0 {
                        let _c24: u32 = lf_checker_rt::callee_cdecl!(24, u32, block_p, 0, off_p);
                        block_p = block_p.wrapping_add(off_p);
                    }
                    off_p = off_p.wrapping_add(ALLOC_SCALE);
                    passes = passes.wrapping_sub(1);
                    if passes == 0 {
                        break;
                    }
                }
                gwr32(G_WR1, block_p);
            }

            // Accumulator; callee 15 when the allocation cell is nonzero.
            acc = acc.wrapping_add(m);
            if grd32(G_WR1) != 0 {
                let _c15: u32 = lf_checker_rt::callee_cdecl!(15, u32,);
            }
            if t == 0 {
                break;
            }
        }

        let _c13: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK_CALLEE, u32,);
        acc
    }
}

// Production export: the rewrite of the function as the original calls it.
lf_checker_rt::export!(thiscall, rw_00650430(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe { evaluate::<false>(this, a1, a2, a3) }
});

// Checker transport export (same body; see evaluate). Not a behaviour change.
lf_checker_rt::export!(thiscall, rw_00650430_xmm3(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe { evaluate::<true>(this, a1, a2, a3) }
});
