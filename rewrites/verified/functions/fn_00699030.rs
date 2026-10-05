// original: 0x00699030 rage::crAnimChannelDeltaFloat::vf14

/// Compresses one delta-float animation channel: finds the value range,
/// picks a quantization step, quantizes every sample, delta-codes the
/// integers, then packs the deltas and residuals into bit streams.
///
/// Arguments (thiscall): `this` is the channel; `samples` points at the
/// input floats; `count` is how many; `stridep1` is one more than the
/// float stride between consecutive samples; `scalebits` is a float scale.
/// Returns 1 on success, 0 when the input is degenerate (`count < 3`,
/// `scale` exactly zero, or the step computation overflows).
///
/// The range (max, min) over the strided samples seeds from the most
/// negative/positive floats; max wins ties and NaN handling follows the
/// original's compare-and-assign exactly. The step helper (callee 1,
/// cdecl: the range/scale ratio as a double, f64 answer on the x87 stack)
/// picks a quantum: at most 1.0 it clamps to 1.0, below 2^30 it is used
/// as-is, otherwise the function gives up. The kept step's truncated bit
/// length is the value width; samples quantize to
/// `min_u32(truncate((x - min) / newrange + 0.5), mask)` through the
/// allocator helper (callee 2, thiscall: scratch block, word count).
///
/// The quantized values pack into the first bit stream (callee 3 sets it
/// up from width and word count; each value's low bits go in whole-word
/// or split across two words). A second quantum rescales the residuals:
/// each sample's reconstruction error re-quantizes with round-half-away
/// (via the thread allocator, callee 7, twice: delta array then residual
/// array), consecutive differences are stored, and the cheapest of six
/// bit widths (4, 8, 16, 32, 64, 128, first-minimum-wins on ties) packs
/// the output stream (callee 5 sets it up; callee 6 decodes each entry
/// through a frame slot; every 32nd entry packs the slot word). The
/// residual builder (callee 4, thiscall on the `this+0x20` object with a
/// two-word frame block) links the residual array in. All four heap
/// blocks are released through the thread-local free helper (callee 8)
/// when their guard words say so. Unsigned-int to float conversions use
/// Rust's exact rounding, matching the original's int-double-float
/// dance bit for bit; x87 truncate/convert edge cases (NaN, overflow to
/// the indefinite) are reproduced explicitly.
///
/// Original: 0x00699030 (thiscall, four stack words, callee pops 16).
/// Note: the inventory lists 1655 bytes, but the function runs 19 bytes
/// further (a fourth free call, epilogue, the callee pops 16 bytes); true size 1674.
lf_checker_rt::export!(thiscall, rw_00699030(this: u32, samples: u32, count: u32, stridep1: u32, scalebits: u32) -> u32 {
    unsafe {
        const STEP_HELPER: u32 = 1;
        const ALLOC_HELPER: u32 = 2;
        const STREAM_SETUP: u32 = 3;
        const RESIDUAL_LINK: u32 = 4;
        const OUT_SETUP: u32 = 5;
        const ENTRY_DECODE: u32 = 6;
        const TLS_ALLOC: u32 = 7;
        const TLS_FREE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// x87 `fistp qword` with truncation, low 32 bits of the result.
        /// NaN and out-of-i64-range inputs store the indefinite (low word 0).
        #[inline(always)]
        fn fistp_low(v: f32) -> u32 {
            const TWO63: f32 = 9.223372036854776e18;
            if v.is_nan() || v >= TWO63 || v < -TWO63 {
                0
            } else {
                (v.trunc() as i64) as u32
            }
        }
        /// x86 `cvttss2si`: truncate toward zero; NaN and overflow give MIN.
        #[inline(always)]
        fn cvtt(v: f32) -> u32 {
            const TWO31: f32 = 2147483648.0;
            if v.is_nan() || v >= TWO31 || v < -TWO31 {
                0x8000_0000
            } else {
                (v.trunc() as i32) as u32
            }
        }
        /// Bit length as the original's inc/shift loop computes it
        /// (0 stays 0 here; callers add the leading one where it applies).
        #[inline(always)]
        fn bitlen(mut v: u32) -> u32 {
            let mut n = 0u32;
            if v != 0 {
                loop {
                    n += 1;
                    v >>= 1;
                    if v == 0 {
                        break;
                    }
                }
            }
            n
        }
        /// The thread allocator object from TLS slot 0.
        #[inline(always)]
        unsafe fn tls_heap() -> u32 {
            unsafe {
                let slots = lf_checker_rt::tls_slot(0);
                rd32(slots + 8)
            }
        }
        /// Allocate `bytes` through the thread allocator (thiscall slot +8:
        /// object, size, 0x10, 0).
        #[inline(always)]
        unsafe fn tls_alloc(heap_obj: u32, bytes: u32) -> u32 {
            unsafe {
                let vtable = rd32(heap_obj);
                let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable + 8) as usize);
                alloc(heap_obj, bytes, 0x10, 0)
            }
        }
        /// Release through the thread free helper (thiscall slot +0xc).
        #[inline(always)]
        unsafe fn tls_free(heap_obj: u32, ptr: u32) {
            unsafe {
                let vtable = rd32(heap_obj);
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable + 12) as usize);
                free(heap_obj, ptr);
            }
        }
        /// Pack one value's masked bits into a bit array, whole-word when
        /// the field fits, split across two words otherwise.
        #[inline(always)]
        unsafe fn pack_bits(arr: u32, word: u32, width: u32, field: u32, data: u32) {
            unsafe {
                let c3 = 32u32.wrapping_sub(width);
                let m = 0xFFFF_FFFFu32 >> (c3 & 31);
                let f2 = field & 31;
                let sv = data & m;
                if f2 <= c3 {
                    let addr = arr.wrapping_add(word * 4);
                    let mut d = m << f2;
                    d = !d;
                    d &= rd32(addr);
                    d |= sv << f2;
                    wr32(addr, d);
                } else {
                    let addr = arr.wrapping_add(word * 4);
                    let mut d = 0xFFFF_FFFFu32 << f2;
                    d = !d;
                    d &= rd32(addr);
                    d |= sv << f2;
                    wr32(addr, d);
                    let c4 = (32u32.wrapping_sub(f2)) & 31;
                    let addr2 = arr.wrapping_add(word * 4).wrapping_add(4);
                    let mut e = !(0xFFFF_FFFFu32 >> c4);
                    e &= rd32(addr2);
                    e |= sv >> c4;
                    wr32(addr2, e);
                }
            }
        }

        if (count as i32) < 3 {
            return 0;
        }
        let scale = f32::from_bits(scalebits);
        // ucomiss+lahf/test/jnp: only exact +0/-0 takes the early exit.
        if scale == 0.0 {
            return 0;
        }
        let stride = stridep1.wrapping_add(1);

        // Range over the strided samples.
        let mut xmax = *lf_checker_rt::global::<f32>(0x00FE8E1C);
        let mut xmin = *lf_checker_rt::global::<f32>(0x00FE8D18);
        if (count as i32) > 0 {
            let step = stride.wrapping_mul(4);
            let mut p = samples;
            let mut k = 0u32;
            while k < count {
                let x = rdf(p);
                if !(xmax > x) {
                    xmax = x;
                }
                if !(x > xmin) {
                    xmin = x;
                }
                p = p.wrapping_add(step);
                k += 1;
            }
        }
        let range = sub(xmax, xmin);
        wrf(this + 0x34, xmin);
        wrf(this + 0x2c, range);
        wrf(this + 0x30, range);

        // Step helper: double in, double out (x87).
        let ratio = div(range, scale);
        let rbits = (ratio as f64).to_bits();
        let dans: f64 = lf_checker_rt::callee_cdecl!(
            STEP_HELPER,
            f64,
            rbits as u32,
            (rbits >> 32) as u32
        );
        let q = dans as f32;
        let one = *lf_checker_rt::global::<f32>(0x00FE88E8);
        let step: f32;
        if !(q > one) {
            step = 1.0;
        } else {
            let big = *lf_checker_rt::global::<f32>(0x00FE8D00);
            if q < big {
                step = q;
            } else {
                return 0;
            }
        }

        let nwords = (count >> 5) + ((count & 0x1f != 0) as u32);
        let mut qblock = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            ALLOC_HELPER,
            u32,
            qblock.as_mut_ptr() as u32,
            nwords
        );
        let qarr = qblock[0];

        let nbits = bitlen(fistp_low(step));
        let mask = 0xFFFF_FFFFu32 >> ((32u32.wrapping_sub(nbits)) & 31);
        let fmask = mask as f32;
        let newrange = div(rdf(this + 0x30), fmask);
        wrf(this + 0x30, newrange);
        let kinv = div(one, newrange);
        let half = *lf_checker_rt::global::<f32>(0x00FE8830);

        // Quantize.
        if (nwords as i32) > 0 {
            let step128 = stride.wrapping_mul(128);
            let mut qp = samples;
            let mut k = 0u32;
            while k < nwords {
                let mut v = rdf(qp);
                v = sub(v, rdf(this + 0x34));
                v = mul(v, kinv);
                v = add(v, half);
                let mut qi = fistp_low(v);
                if qi > mask {
                    qi = mask;
                }
                wr32(qarr.wrapping_add(k * 4), qi);
                qp = qp.wrapping_add(step128);
                k += 1;
            }
        }

        // First bit stream.
        let cobj = this + 8;
        let _: u32 = lf_checker_rt::callee_thiscall!(STREAM_SETUP, u32, cobj, nbits, nwords);
        let nmemb = rd32(cobj + 4);
        let carr = rd32(cobj);
        if (nwords as i32) > 0 {
            let mut i2 = 0u32;
            while i2 < nwords {
                let prod = i2.wrapping_mul(nmemb);
                let w = prod >> 5;
                let v = rd32(qarr.wrapping_add(i2 * 4));
                pack_bits(carr, w, nmemb, prod, v);
                i2 += 1;
            }
        }

        // Second quantum from the value width.
        let q2a = 1u32 << (nbits & 31);
        let nr2 = div(rdf(this + 0x2c), q2a as f32);
        let k2 = div(one, nr2);
        wrf(this + 0x2c, nr2);

        let heap_obj = tls_heap();
        let darr: u32;
        if count != 0 {
            darr = tls_alloc(heap_obj, count.wrapping_mul(4));
        } else {
            darr = 0;
        }

        // Delta re-quantize.
        if (count as i32) > 0 {
            let step4 = stride.wrapping_mul(4);
            let mut dp = samples;
            let mut j = 0u32;
            while j < count {
                let qv = rd32(qarr.wrapping_add((j >> 5) * 4));
                let e0 = mul(qv as f32, rdf(this + 0x30));
                let e1 = add(e0, rdf(this + 0x34));
                let x = rdf(dp);
                let mut d = sub(x, e1);
                d = mul(d, k2);
                if d > 0.0 {
                    d = add(d, half);
                } else {
                    d = sub(d, half);
                }
                wr32(darr.wrapping_add(j * 4), cvtt(d));
                dp = dp.wrapping_add(step4);
                j += 1;
            }
        }

        // Residuals: consecutive differences, skipping every 32nd.
        let resn = count.wrapping_sub(nwords);
        let rarr: u32;
        if resn != 0 {
            rarr = tls_alloc(heap_obj, resn.wrapping_mul(4));
        } else {
            rarr = 0;
        }
        if count != 0 {
            let mut rp = rarr;
            let mut j = 0u32;
            while j < count {
                let k = j & 0x1f;
                if k != 0 {
                    let prev = if k == 1 {
                        0
                    } else {
                        rd32(darr.wrapping_add(j * 4).wrapping_sub(4))
                    };
                    let cur = rd32(darr.wrapping_add(j * 4));
                    wr32(rp, cur.wrapping_sub(prev));
                    rp = rp.wrapping_add(4);
                }
                j += 1;
            }
        }

        // Cheapest of six bit widths, first-minimum-wins.
        let mut bestcost = 0xFFFF_FFFFu32;
        let mut bestshift = 0u32;
        let mut b2 = 4u32;
        for _ in 0..6 {
            // Note the trailing decrement: the shift is bitlen(b2) - 1.
            let c2 = bitlen(b2) - 1;
            let mut cost = 0u32;
            if (resn as i32) > 0 {
                let mut ri = 0u32;
                while (ri as i32) < (resn as i32) {
                    let r = rd32(rarr.wrapping_add(ri * 4));
                    let ar = if (r as i32) < 0 { 0u32.wrapping_sub(r) } else { r };
                    let t = ar >> (c2 & 31);
                    cost = cost.wrapping_add(1).wrapping_add(c2).wrapping_add(t);
                    if r != 0 {
                        cost = cost.wrapping_add(1);
                    }
                    ri += 1;
                }
            }
            if cost < bestcost {
                bestcost = cost;
                bestshift = b2;
            }
            b2 = b2.rotate_left(1);
        }

        // Output stream object and shift byte.
        let fobj = this + 0x20;
        ((fobj + 8) as *mut u8).write(0);
        let sh_b = if bestshift == 0 { 1 } else { bitlen(bestshift) };
        let prestored = rd32(fobj);
        ((fobj + 8) as *mut u8).write((sh_b - 1) as u8);
        if prestored != 0 {
            tls_free(heap_obj, prestored);
        }

        // Link the residual array in.
        let si16 = (resn & 0xFFFF) as u16;
        let mut dblock = [0u32; 2];
        dblock[0] = rarr;
        dblock[1] = (si16 as u32) | ((si16 as u32) << 16);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RESIDUAL_LINK,
            u32,
            fobj,
            dblock.as_mut_ptr() as u32
        );

        // Output stream setup from the cost bit length.
        let eobj = this + 0x14;
        let costbits = bitlen(bestcost);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            OUT_SETUP,
            u32,
            eobj,
            costbits,
            nwords.wrapping_sub(1)
        );

        // Output entries: decode each, pack every 32nd.
        let width = rd32(eobj + 4);
        let earr = rd32(eobj);
        let mut slot = 0u32;
        let mut i2 = 0u32;
        while i2 < count {
            let k = i2 & 0x1f;
            if k != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    ENTRY_DECODE,
                    u32,
                    fobj,
                    core::ptr::addr_of_mut!(slot) as u32
                );
            } else if i2 != 0 {
                let prod = ((i2 >> 5).wrapping_sub(1)).wrapping_mul(width);
                pack_bits(earr, prod >> 5, width, prod, slot);
            }
            i2 += 1;
        }

        // Release the heap blocks whose guards say so.
        if (resn & 0xFFFF) != 0 && rarr != 0 {
            tls_free(heap_obj, rarr);
        }
        if (count & 0xFFFF) != 0 && darr != 0 {
            tls_free(heap_obj, darr);
        }
        // Guard is the upper word of the helper block's second word.
        let guard_hi =
            unsafe { ((qblock.as_mut_ptr() as *const u8).add(6) as *const u16).read_unaligned() };
        if guard_hi != 0 && qarr != 0 {
            tls_free(heap_obj, qarr);
        }
        1
    }
});
