// original: 0x005d6690 table_column_redistribute (proposed)
//
// Redistribute one float column of a table node from two source columns.
//
// `this` is a table node: three float arrays for the indexed rows live at
// `+0xec` (low), `+0xf0` (high) and `+0xf4` (output, written), each holding
// the signed row count at `+0xfc`; a second pair of arrays at `+0xe8` and
// `+0xf8` holds the signed count at `+0x100`. Scalar parameters live at
// `+0x80`, `+0x8c`, `+0xbc`, `+0xc0` (floats) and `+0x20` (float, rewritten).
//
// The function sums the low column into `sum_lo` and the high column into
// `sum_hi` (four-wide vector blocks folded lane-wise, then a scalar tail in
// index order), walks two `+0x8` chains from `this` for the node whose
// `+0x20` (first walk) or `+0x24` (second walk) differs from -1.0 or whose
// type at `+0x14` differs from 0x12 (a null end of chain faults, as in the
// original), and builds a target from the found node's `+0x20`, `+0xb0`
// and `+0xb4` words minus terms in the row count and the scalar parameters.
// The second walk's result is discarded by the original; only its faults
// are observable, so this rewrite performs the same reads.
//
// Five output paths follow from ordered floating comparisons against the
// target (`jb` after `comiss` is taken exactly when the ordered `>=` is
// false, i.e. also for NaN): sum_lo already at or above the target copies
// the low column plus a bias; otherwise, with the target at or above
// sum_hi, a non-positive `+0x20` copies the high column plus the bias, a
// zero sum_hi spreads the target evenly over the rows, and otherwise the
// high column is scaled by target/sum_hi; with the target strictly between
// the sums each row is interpolated between the columns. Every row of the
// output is then raised by `+0x80 + +0x8c`, and `+0x20` accumulates the
// output plus `+0xc0` twice per row starting from `+0xc0`. Finally the
// second array pair is copied element-wise. Counts are SIGNED throughout:
// a non-positive count skips its loop. All float operations run in the
// original's operand order, pinned with black_box for bit-exactness.
//
// Original: 0x005d6690 (thiscall, no stack arguments, returns garbage that
// is deterministic: the last copied word when the second count is positive,
// the output-array pointer when only the row count is positive, else the
// `+0xc0` word).
lf_checker_rt::export!(thiscall, rw_005d6690(this: u32) -> u32 {
    unsafe {
        const NEG_ONE: f32 = -1.0;
        const TWO: f32 = 2.0;
        const NODE_TYPE: u32 = 0x12;
        const ARR_LO: u32 = 0xec;
        const ARR_HI: u32 = 0xf0;
        const ARR_OUT: u32 = 0xf4;
        const ARR_D: u32 = 0xe8;
        const ARR_E: u32 = 0xf8;
        const N1: u32 = 0xfc;
        const N2: u32 = 0x100;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let n1 = rd32(this + N1) as i32;
        // Column sums: vector blocks of four folded lane-wise, scalar tail.
        let mut sum_lo = 0.0f32;
        let mut sum_hi = 0.0f32;
        let mut i = 0i32;
        if n1 > 0 {
            let a = rd32(this + ARR_LO);
            let b = rd32(this + ARR_HI);
            if n1 >= 4 {
                let mut lo = [0.0f32; 4];
                let mut hi = [0.0f32; 4];
                let limit = n1 - (n1 & 3);
                while i < limit {
                    for l in 0..4usize {
                        lo[l] = add(lo[l], rdf(a + ((i + l as i32) as u32).wrapping_mul(4)));
                        hi[l] = add(hi[l], rdf(b + ((i + l as i32) as u32).wrapping_mul(4)));
                    }
                    i += 4;
                }
                // movhlps/addps/shufps(0xf5)/addss fold of each accumulator.
                let t0 = add(lo[0], lo[2]);
                let t1 = add(lo[1], lo[3]);
                sum_lo = add(t0, t1);
                let u0 = add(hi[0], hi[2]);
                let u1 = add(hi[1], hi[3]);
                sum_hi = add(u0, u1);
            }
            while i < n1 {
                sum_lo = add(sum_lo, rdf(a + (i as u32).wrapping_mul(4)));
                sum_hi = add(sum_hi, rdf(b + (i as u32).wrapping_mul(4)));
                i += 1;
            }
        }
        // First chain walk: node whose +0x20 != -1.0 (ordered) or type != 0x12.
        let mut node = this;
        loop {
            let v = unsafe { f32::from_bits(((node + 0x20) as *const u32).read_volatile()) };
            if v != NEG_ONE {
                let t = ((node + 0x14) as *const u32).read_volatile();
                if t == NODE_TYPE {
                    node = ((node + 0x08) as *const u32).read_volatile();
                    if node == 0 {
                        break;
                    }
                    continue;
                }
            }
            break;
        }
        let w = add(
            unsafe { f32::from_bits(((node + 0xb4) as *const u32).read_volatile()) },
            unsafe { f32::from_bits(((node + 0xb0) as *const u32).read_volatile()) },
        );
        let n20 = unsafe { f32::from_bits(((node + 0x20) as *const u32).read_volatile()) };
        let mut target = sub(n20, w);
        // Second chain walk on +0x24; the original discards the result, so
        // only the reads (and their faults) are reproduced, volatily so the
        // compiler cannot drop them as dead.
        let mut node2 = this;
        loop {
            let v = unsafe { f32::from_bits(((node2 + 0x24) as *const u32).read_volatile()) };
            if v != NEG_ONE {
                let t = ((node2 + 0x14) as *const u32).read_volatile();
                if t == NODE_TYPE {
                    node2 = ((node2 + 0x08) as *const u32).read_volatile();
                    if node2 == 0 {
                        break;
                    }
                    continue;
                }
            }
            break;
        }
        core::hint::black_box(node2);
        // Target from the found node, the row count and the scalars.
        let n1f = n1 as f32;
        let n1p1 = n1.wrapping_add(1) as f32;
        target = sub(target, mul(n1p1, rdf(this + 0xc0)));
        target = sub(target, mul(add(rdf(this + 0x80), rdf(this + 0x8c)), n1f));
        target = sub(target, mul(mul(n1f, rdf(this + 0xbc)), TWO));
        let bias = mul(rdf(this + 0xbc), TWO);
        let out = rd32(this + ARR_OUT);
        if !(sum_lo >= target) {
            if !(target >= sum_hi) {
                // Between the sums: interpolate each row.
                let f = div(sub(target, sum_lo), sub(sum_hi, sum_lo));
                let mut k = 0i32;
                while k < n1 {
                    let k1 = (k as u32).wrapping_add(1).wrapping_mul(4);
                    let lo = rdf(rd32(this + ARR_LO).wrapping_add(k1).wrapping_sub(4));
                    let mut v = rdf(rd32(this + ARR_HI).wrapping_add(k1).wrapping_sub(4));
                    v = add(mul(sub(v, lo), f), lo);
                    v = add(v, mul(rdf(this + 0xbc), TWO));
                    wrf(out.wrapping_add(k1).wrapping_sub(4), v);
                    k += 1;
                }
            } else if !(rdf(this + 0x20) > 0.0) {
                // Non-positive +0x20: copy the high column plus bias.
                let mut k = 0i32;
                while k < n1 {
                    let k1 = (k as u32).wrapping_add(1).wrapping_mul(4);
                    let v = add(
                        rdf(rd32(this + ARR_HI).wrapping_add(k1).wrapping_sub(4)),
                        mul(rdf(this + 0xbc), TWO),
                    );
                    wrf(out.wrapping_add(k1).wrapping_sub(4), v);
                    k += 1;
                }
            } else if sum_hi != 0.0 {
                // Nonzero sum_hi: scale the high column by target/sum_hi.
                let f = div(target, sum_hi);
                let mut k = 0i32;
                while k < n1 {
                    let k1 = (k as u32).wrapping_add(1).wrapping_mul(4);
                    let mut v = rdf(rd32(this + ARR_HI).wrapping_add((k as u32).wrapping_mul(4)));
                    v = add(mul(v, f), mul(rdf(this + 0xbc), TWO));
                    wrf(out.wrapping_add(k1).wrapping_sub(4), v);
                    k += 1;
                }
            } else {
                // Zero sum_hi: spread the target evenly.
                let mut k = 0i32;
                while k < rd32(this + N1) as i32 {
                    let per = div(target, n1 as f32);
                    let k1 = (k as u32).wrapping_add(1).wrapping_mul(4);
                    let v = add(per, mul(rdf(this + 0xbc), TWO));
                    wrf(rd32(this + ARR_OUT).wrapping_add(k1).wrapping_sub(4), v);
                    k += 1;
                }
            }
        } else {
            // sum_lo already covers the target: copy low column plus bias.
            let mut k = 0i32;
            while k < n1 {
                let k1 = (k as u32).wrapping_add(1).wrapping_mul(4);
                let v = add(
                    rdf(rd32(this + ARR_LO).wrapping_add(k1).wrapping_sub(4)),
                    bias,
                );
                wrf(out.wrapping_add(k1).wrapping_sub(4), v);
                k += 1;
            }
        }
        // Raise every output row and accumulate +0x20.
        let c0 = rd32(this + 0xc0);
        wr32(this + 0x20, c0);
        let mut ret_eax = c0;
        let mut k = 0i32;
        while k < rd32(this + N1) as i32 {
            let k1 = (k as u32).wrapping_add(1).wrapping_mul(4);
            let o = rd32(this + ARR_OUT);
            let v = add(add(rdf(this + 0x80), rdf(this + 0x8c)), rdf(o.wrapping_add(k1).wrapping_sub(4)));
            wrf(o.wrapping_add(k1).wrapping_sub(4), v);
            let acc = add(add(rdf(o.wrapping_add(k1).wrapping_sub(4)), rdf(this + 0xc0)), rdf(this + 0x20));
            wrf(this + 0x20, acc);
            ret_eax = o;
            k += 1;
        }
        // Copy the second array pair.
        let n2 = rd32(this + N2) as i32;
        let mut j = 0i32;
        while j < n2 {
            let v = rd32(rd32(this + ARR_D).wrapping_add((j as u32).wrapping_mul(4)));
            wr32(rd32(this + ARR_E).wrapping_add((j as u32).wrapping_mul(4)), v);
            ret_eax = v;
            j += 1;
        }
        ret_eax
    }
});
