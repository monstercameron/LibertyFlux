// original: 0x00690410 anim_blend_inputs_shared_index (proposed)

/// Blend several animation inputs into a target animation, one target track at
/// a time, finding the matching track by a merge-join on the first input only
/// and reusing its position for every other input.
///
/// `this` points to the target track set (tracks array at `+0x0c`, 16-bit
/// count at `+0x10`). `count` inputs are given as pointers in `inputs`, each
/// another track set whose tracks are sorted by key (byte at `+5` shifted left
/// 16, or-ed with the 16-bit word at `+6`) and laid out alike. `weights` holds
/// one float per input; `callback`, when non-null, is an object whose virtual
/// slot at `+0x10` may adjust the per-track weights.
///
/// Weights: the leftover `1 - sum(weights)` (zero when the sum is not below
/// 1, or lies in (0.999, 1)) seeds a running total. For each target track the
/// weights are copied and one shared cursor is advanced through the first
/// input to the first track whose key is not below the target's (an input
/// whose key is already above it ends the search for the track altogether);
/// an exact match is taken for the first input, and every later input
/// contributes the track at the position just before the cursor. A picked
/// track whose skip flag (bit 0x10) is set is dropped with its weight, as is
/// a missing one. Nothing is done for the track if no input contributed, or
/// if the first input has no pick.
/// The callback then sees (key high byte, key low word, count, weights). The
/// weights are converted to cumulative blend factors w / (leftover + running
/// sum), clamped to [0, 1], with a running total below 0.001 resetting to 0.
///
/// Each picked input track is then blended into the target track by the
/// target's kind (low nibble of the flag byte): 1 is a quaternion (copy when
/// the target's flag bit 0x10 is set, otherwise sign-aligned linear blend and
/// renormalise through the square-root callee), 0 is a three-float linear
/// blend with snap-to-source at 0.999, 2 is a one-float blend that also
/// carries the source's flag, anything else copies one float once the factor
/// reaches 0.5. The float operation order is the original's.
///
/// Original: 0x00690410 (thiscall, six stack words; the third and fourth are
/// not read).
lf_checker_rt::export!(thiscall, rw_00690410(this: u32, count: u32, inputs: u32, callback: u32, _a3: u32, _a4: u32, weights: u32) -> u32 {
    unsafe {
        const SET_TRACKS: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const TRACK_FLAGS: u32 = 0x04;
        const TRACK_KEY_HI: u32 = 0x05;
        const TRACK_KEY_LO: u32 = 0x06;
        const TRACK_VALUE: u32 = 0x10;
        const SKIP_FLAG: u8 = 0x10;
        const VTABLE_SLOT_WEIGHT_HOOK: u32 = 0x10;
        const ONE: f32 = 1.0;
        const NEAR_ONE: f32 = f32::from_bits(0x3f7f_be77); // 0.999
        const HALF: f32 = 0.5;
        const EPS: f32 = f32::from_bits(0x3a83_126f); // 0.001
        const SQRT_CALLEE: u32 = 1;
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        #[inline(always)]
        unsafe fn key(track: u32) -> u32 {
            unsafe { (rd8(track + TRACK_KEY_HI) as u32) << 16 | rd16(track + TRACK_KEY_LO) }
        }

        /// Sum of the weights as the original's vectorised loop forms it: for
        /// eight or more entries, two four-lane accumulators over blocks of
        /// eight, folded lane-wise then pairwise; the remainder is added in order.
        unsafe fn weight_sum(w: u32, n: u32) -> f32 {
            unsafe {
                let mut idx = 0u32;
                let mut s = 0.0f32;
                if n >= 8 {
                    let limit = n - (n & 7);
                    let mut lo = [0.0f32; 4];
                    let mut hi = [0.0f32; 4];
                    while idx < limit {
                        for l in 0..4u32 {
                            lo[l as usize] = add(lo[l as usize], rdf(w + (idx + l) * 4));
                            hi[l as usize] = add(hi[l as usize], rdf(w + (idx + 4 + l) * 4));
                        }
                        idx += 8;
                    }
                    let v = [
                        add(lo[0], hi[0]),
                        add(lo[1], hi[1]),
                        add(lo[2], hi[2]),
                        add(lo[3], hi[3]),
                    ];
                    let x0 = add(v[2], v[0]);
                    let x1 = add(v[3], v[1]);
                    s = add(x0, x1);
                }
                while idx < n {
                    s = add(s, rdf(w + idx * 4));
                    idx += 1;
                }
                s
            }
        }

        /// Blend one picked source track `r` into target track `d` with factor `t`.
        unsafe fn blend(d: u32, r: u32, t: f32) {
            unsafe {
                let cl = rd8(d + TRACK_FLAGS);
                match cl & 0x0f {
                    1 => {
                        if cl & SKIP_FLAG != 0 {
                            for off in [0u32, 4, 8, 12] {
                                wr32(d + TRACK_VALUE + off, rd32(r + TRACK_VALUE + off));
                            }
                            ((d + TRACK_FLAGS) as *mut u8).write(cl & 0xef);
                            return;
                        }
                        let (x, y, z) = (rdf(d + 0x10), rdf(d + 0x14), rdf(d + 0x18));
                        let w = rdf(d + 0x1c);
                        let (rx, ry, rz, rw) = (rdf(r + 0x10), rdf(r + 0x14), rdf(r + 0x18), rdf(r + 0x1c));
                        let p_x = mul(x, rx);
                        let p_y = mul(y, ry);
                        let mut dot = add(p_y, p_x);
                        dot = add(dot, mul(z, rz));
                        dot = add(dot, mul(rw, w));
                        if 0.0 > dot {
                            wrf(d + 0x10, neg(x));
                            wrf(d + 0x14, neg(y));
                            wrf(d + 0x18, neg(z));
                            wrf(d + 0x1c, neg(w));
                        }
                        let s = sub(ONE, t);
                        let a = mul(rx, t);
                        let b = mul(rdf(d + 0x10), s);
                        let nx = add(b, a);
                        let c = mul(rdf(d + 0x14), s);
                        wrf(d + 0x10, nx);
                        let ny = add(mul(ry, t), c);
                        let e = mul(rdf(d + 0x18), s);
                        let sw = mul(s, rdf(d + 0x1c));
                        wrf(d + 0x14, ny);
                        let nz = add(mul(rz, t), e);
                        wrf(d + 0x18, nz);
                        let nw = add(mul(rw, t), sw);
                        wrf(d + 0x1c, nw);
                        let n2 = add(add(add(mul(nx, nx), mul(ny, ny)), mul(nz, nz)), mul(nw, nw));
                        let inv = if n2 == 0.0 {
                            0.0f32
                        } else {
                            let root: f32 = lf_checker_rt::callee_cdecl!(SQRT_CALLEE, f32, n2.to_bits());
                            core::hint::black_box(ONE) / core::hint::black_box(root)
                        };
                        wrf(d + 0x10, mul(nx, inv));
                        wrf(d + 0x14, mul(ny, inv));
                        wrf(d + 0x18, mul(nz, inv));
                        wrf(d + 0x1c, mul(inv, nw));
                    }
                    0 => {
                        if !(t > EPS) {
                            return;
                        }
                        if NEAR_ONE > t && cl & SKIP_FLAG == 0 {
                            for off in [0x10u32, 0x14, 0x18] {
                                let cur = rdf(d + off);
                                let v = add(mul(sub(rdf(r + off), cur), t), cur);
                                wrf(d + off, v);
                            }
                        } else {
                            for off in [0u32, 4, 8, 12] {
                                wr32(d + TRACK_VALUE + off, rd32(r + TRACK_VALUE + off));
                            }
                        }
                        ((d + TRACK_FLAGS) as *mut u8).write(rd8(d + TRACK_FLAGS) & 0xef);
                    }
                    2 => {
                        let mut flags = cl;
                        let lerped = NEAR_ONE > t && cl & SKIP_FLAG == 0;
                        if lerped {
                            let cur = rdf(d + 0x10);
                            let v = add(mul(sub(rdf(r + 0x10), cur), t), cur);
                            wrf(d + 0x10, v);
                        } else {
                            if !(t > EPS) {
                                return;
                            }
                            wr32(d + 0x10, rd32(r + 0x10));
                        }
                        if rd8(r + TRACK_FLAGS) & SKIP_FLAG != 0 {
                            flags |= SKIP_FLAG;
                        } else {
                            flags &= 0xef;
                        }
                        ((d + TRACK_FLAGS) as *mut u8).write(flags);
                    }
                    _ => {
                        if t < HALF {
                            return;
                        }
                        wr32(d + 0x10, rd32(r + 0x10));
                        ((d + TRACK_FLAGS) as *mut u8).write(cl & 0xef);
                    }
                }
            }
        }

        // Leftover weight seeding the running total.
        let n = count;
        let leftover = if (n as i32) <= 0 {
            ONE
        } else {
            let sum = weight_sum(weights, n);
            if ONE > sum && sum <= NEAR_ONE {
                sub(ONE, sum)
            } else {
                0.0
            }
        };

        let len = n as usize;
        let mut picked: Vec<u32> = vec![0; len];
        let mut cursor: Vec<u32> = vec![0; len];
        let mut factors: Vec<f32> = vec![0.0; len];
        let set = rd32(this);
        let targets = rd16(set + SET_COUNT) as i32;
        for i in 0..targets.max(0) as u32 {
            let target = rd32(rd32(rd32(this) + SET_TRACKS) + i * 4);
            for k in 0..len {
                factors[k] = rdf(weights + k as u32 * 4);
                picked[k] = 0;
            }
            if n == 0 {
                continue;
            }
            let mut nothing_to_blend = true;
            let mut k = 0usize;
            'inputs: while k < len {
                let input = rd32(inputs + k as u32 * 4);
                if k == 0 {
                    // Only the first input is searched; its cursor is shared.
                    let input_count = rd16(input + SET_COUNT) as i32;
                    if (cursor[0] as i32) >= input_count {
                        break 'inputs;
                    }
                    let want = key(target);
                    loop {
                        let cand = rd32(rd32(input + SET_TRACKS) + cursor[0] * 4);
                        let have = key(cand);
                        if have == want {
                            picked[0] = cand;
                            cursor[0] += 1;
                            break;
                        }
                        if have > want {
                            break 'inputs;
                        }
                        cursor[0] += 1;
                        if !((cursor[0] as i32) < input_count) {
                            break;
                        }
                    }
                } else {
                    // Later inputs contribute the track just before the cursor.
                    picked[k] = rd32(rd32(input + SET_TRACKS).wrapping_add(cursor[0].wrapping_mul(4)).wrapping_sub(4));
                }
                let found = picked[k];
                if found != 0 {
                    if rd8(found + TRACK_FLAGS) & SKIP_FLAG == 0 {
                        nothing_to_blend = false;
                        k += 1;
                        continue;
                    }
                    picked[k] = 0;
                }
                factors[k] = 0.0;
                k += 1;
            }
            if nothing_to_blend {
                continue;
            }
            if callback != 0 {
                let hook: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(callback) + VTABLE_SLOT_WEIGHT_HOOK) as usize);
                hook(
                    callback,
                    rd8(target + TRACK_KEY_HI) as u32,
                    rd16(target + TRACK_KEY_LO),
                    n,
                    factors.as_mut_ptr() as u32,
                );
            }
            // Convert to cumulative blend factors.
            let mut running = leftover;
            for k in 0..len {
                let w = factors[k];
                running = add(running, w);
                if !(running >= EPS) {
                    factors[k] = 0.0;
                    running = 0.0;
                    continue;
                }
                let q = core::hint::black_box(w) / core::hint::black_box(running);
                factors[k] = if 0.0 > q {
                    0.0
                } else if q > ONE {
                    ONE
                } else {
                    q
                };
            }
            // Without a pick for the first input nothing is blended; a later
            // input with a positive factor always has a pick.
            if picked[0] == 0 {
                continue;
            }
            for k in 0..len {
                let source = picked[k];
                let t = factors[k];
                if !(t > 0.0) {
                    continue;
                }
                blend(target, source, t);
            }
        }
        0
    }
});


