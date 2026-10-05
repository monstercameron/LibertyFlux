// original: 0x00691D80 audio_blend_pass (proposed)

/// Weight-sum animation blend pass over one voice bank (original 0x00691D80).
///
/// `this` points at the bank (table at `+0x0c`, 16-bit object count `m` at
/// `+0x10`). The single float array `weights` plays two roles: first its
/// entries are summed with a vectorised reduction (two four-lane SSE
/// accumulators over blocks of eight, folded lane-wise then pairwise, the
/// remainder added in order), and the sum seeds a leftover `1 - sum` (zero
/// when the sum is not below 1, or lies in `(0.999, 1)`, or is NaN).
///
/// For each of the `m` objects: the `helper` object's data-table slot at
/// `+0x0c` is asked for a scale (thiscall with the object's key bytes and
/// a frame slot preset to `1.0`; the callee answers in AL, only the low
/// byte tested, and writes the scale back). A zero AL skips the object.
/// Exactly `1.0` copies `n` words from `weights` into scratch, anything
/// else (NaN included) multiplies. Every slot then searches its own
/// candidate table from its own persistent cursor (compared SIGNED against
/// its bound) for the 24-bit key matching the object, advancing past
/// smaller keys (unsigned) and sitting out past larger ones or an
/// overrun; a leaf whose flag byte (`+4`) has bit `0x10` zeroes both its
/// words. With no live leaf the object is skipped, else the `vcall`
/// object (when non-null) observes the samples through its virtual slot
/// at `+0x10` and may rewrite sample 0.
///
/// The samples are then converted to cumulative blend factors `w /
/// (leftover + running sum)`, clamped to `[0, 1]`, with a running total
/// below `0.001` (NaN included) zeroing the slot and resetting. Each live
/// slot blends into the object by the object's kind (low nibble of the
/// flag byte): 1 is a quaternion (copy when the flag bit `0x10` is set,
/// otherwise sign-aligned linear blend and renormalise through the
/// square-root callee, skipped with a zero factor when the squared norm
/// is exactly `0.0`), 0 is a three-float blend with snap-to-source at
/// `0.999` (slots at or below `0.001` do nothing), 2 is a one-float blend
/// that also carries the source's flag, anything else copies one float
/// once the factor reaches `0.5` (NaN factors do nothing: the original
/// branches with `jb`, matched here with a negated `>=`).
///
/// The float operation order is the original's throughout, including the
/// per-component order quirks of the quaternion blend (the x and w lanes
/// add source-first, the y and z lanes add target-first). The count `m`
/// is compared signed but comes from a 16-bit load; `n` is an unsigned
/// word count in `4..=16` by contract (below 4 the snapshot reads stale
/// stack; negative counts hang the original and are excluded).
///
/// Scratch: three `n`-word alloca buffers (samples, leaves, cursors),
/// mirrored by fixed stack arrays. EAX on exit carries stale and
/// accidental values (partial low-byte writes, helper answers) and is not
/// compared (`ret: none`).
///
/// Original: 0x00691D80 (thiscall, six stack words; the fifth is not read).
#[allow(clippy::too_many_arguments)]
fn audio_blend_pass(
    this_ptr: u32,
    n: u32,
    keys: u32,
    vcall: u32,
    helper: u32,
    weights: u32,
) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x0c;
        const COUNT_OFF: u32 = 0x10;
        const FLAG_OFF: u32 = 0x04;
        const KEY_HI_OFF: u32 = 0x05;
        const KEY_LO_OFF: u32 = 0x06;
        const SKIP_FLAG: u8 = 0x10;
        const VTABLE_SLOT_OBSERVE: u32 = 0x10;
        const DTABLE_SLOT_SCALE: u32 = 0x0c;
        const ONE: f32 = 1.0;
        const NEAR_ONE: f32 = f32::from_bits(0x3f7f_be77); // 0.999
        const EPS: f32 = f32::from_bits(0x3a83_126f); // 0.001
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn key24(obj: u32) -> u32 {
            unsafe {
                ((rd8(obj.wrapping_add(KEY_HI_OFF)) as u32) << 16)
                    | rd16(obj.wrapping_add(KEY_LO_OFF))
            }
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
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        /// Sum of the weights as the original's vectorised loop forms it.
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

        /// Blend one picked source leaf `r` into object `d` with factor `t`.
        unsafe fn blend(d: u32, r: u32, t: f32) {
            unsafe {
                const VALUE: u32 = 0x10;
                const SKIP: u8 = 0x10;
                const O: f32 = 1.0;
                const NO: f32 = f32::from_bits(0x3f7f_be77);
                const E: f32 = f32::from_bits(0x3a83_126f);
                const H: f32 = 0.5;
                const SQ: u32 = 5;
                #[inline(always)]
                fn m(a: f32, b: f32) -> f32 {
                    core::hint::black_box(a) * core::hint::black_box(b)
                }
                #[inline(always)]
                fn a(a: f32, b: f32) -> f32 {
                    core::hint::black_box(a) + core::hint::black_box(b)
                }
                #[inline(always)]
                fn s(a: f32, b: f32) -> f32 {
                    core::hint::black_box(a) - core::hint::black_box(b)
                }
                let cl = rd8(d.wrapping_add(FLAG_OFF));
                match cl & 0x0f {
                    1 => {
                        if cl & SKIP != 0 {
                            for off in [0u32, 4, 8, 12] {
                                let v = rd32(r.wrapping_add(VALUE + off));
                                (d.wrapping_add(VALUE + off) as *mut u32)
                                    .write_unaligned(v);
                            }
                            ((d + FLAG_OFF) as *mut u8).write(cl & 0xef);
                            return;
                        }
                        let (x, y, z) = (rdf(d + 0x10), rdf(d + 0x14), rdf(d + 0x18));
                        let wv = rdf(d + 0x1c);
                        let (rx, ry, rz, rw) =
                            (rdf(r + 0x10), rdf(r + 0x14), rdf(r + 0x18), rdf(r + 0x1c));
                        // dot = ((x*rx + y*ry) + z*rz) + rw*w, this order.
                        let mut dot = a(m(x, rx), m(y, ry));
                        dot = a(dot, m(z, rz));
                        dot = a(dot, m(rw, wv));
                        if 0.0 > dot {
                            wrf(d + 0x10, neg(x));
                            wrf(d + 0x14, neg(y));
                            wrf(d + 0x18, neg(z));
                            wrf(d + 0x1c, neg(wv));
                        }
                        let sc = s(O, t);
                        // x and w add source-first, y and z target-first.
                        let nx = a(m(rdf(d + 0x10), sc), m(rx, t));
                        wrf(d + 0x10, nx);
                        let ny = a(m(ry, t), m(rdf(d + 0x14), sc));
                        wrf(d + 0x14, ny);
                        let nz = a(m(rz, t), m(rdf(d + 0x18), sc));
                        wrf(d + 0x18, nz);
                        let nw = a(m(rdf(d + 0x1c), sc), m(rw, t));
                        wrf(d + 0x1c, nw);
                        let n2 = a(a(a(m(nx, nx), m(ny, ny)), m(nz, nz)), m(nw, nw));
                        let inv = if n2 == 0.0 {
                            0.0f32
                        } else {
                            let root: f32 =
                                lf_checker_rt::callee_cdecl!(SQ, f32, n2.to_bits());
                            core::hint::black_box(O) / core::hint::black_box(root)
                        };
                        wrf(d + 0x10, m(nx, inv));
                        wrf(d + 0x14, m(ny, inv));
                        wrf(d + 0x18, m(nz, inv));
                        wrf(d + 0x1c, m(nw, inv));
                    }
                    0 => {
                        if !(t > E) {
                            return;
                        }
                        if NO > t && cl & SKIP == 0 {
                            for off in [0x10u32, 0x14, 0x18] {
                                let cur = rdf(d + off);
                                let v = a(m(s(rdf(r + off), cur), t), cur);
                                wrf(d + off, v);
                            }
                        } else {
                            for off in [0u32, 4, 8, 12] {
                                let v = rd32(r.wrapping_add(VALUE + off));
                                (d.wrapping_add(VALUE + off) as *mut u32)
                                    .write_unaligned(v);
                            }
                        }
                        ((d + FLAG_OFF) as *mut u8).write(rd8(d + FLAG_OFF) & 0xef);
                    }
                    2 => {
                        let mut flags = cl;
                        let lerped = NO > t && cl & SKIP == 0;
                        if lerped {
                            let cur = rdf(d + 0x10);
                            let v = a(m(s(rdf(r + 0x10), cur), t), cur);
                            wrf(d + 0x10, v);
                        } else {
                            if !(t > E) {
                                return;
                            }
                            let v = rd32(r + 0x10);
                            (d.wrapping_add(0x10) as *mut u32).write_unaligned(v);
                        }
                        if rd8(r + FLAG_OFF) & SKIP != 0 {
                            flags |= SKIP;
                        } else {
                            flags &= 0xef;
                        }
                        ((d + FLAG_OFF) as *mut u8).write(flags);
                    }
                    _ => {
                        // The original branches with `jb`: NaN skips too.
                        if !(t >= H) {
                            return;
                        }
                        let v = rd32(r + 0x10);
                        (d.wrapping_add(0x10) as *mut u32).write_unaligned(v);
                        ((d + FLAG_OFF) as *mut u8).write(cl & 0xef);
                    }
                }
            }
        }

        let count = n as usize;
        let mut samples = [0u32; 16];
        let mut leaves = [0u32; 16];
        let mut cursor = [0u32; 16];

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

        let vt = rd32(this_ptr);
        let bank = rd32(vt.wrapping_add(TABLE_OFF));
        let m = rd16(vt.wrapping_add(COUNT_OFF));
        if m == 0 {
            return 0;
        }
        let mut i = 0u32;
        while i < m {
            let obj = rd32(bank.wrapping_add(i.wrapping_mul(4)));
            let b5 = rd8(obj.wrapping_add(KEY_HI_OFF)) as u32;
            let w6 = rd16(obj.wrapping_add(KEY_LO_OFF));
            let mut scale = ONE;
            let dt_slot = rd32(rd32(helper).wrapping_add(DTABLE_SLOT_SCALE));
            let ask: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(dt_slot as usize);
            let ans = ask(helper, b5, w6, core::ptr::addr_of_mut!(scale) as u32);
            if ans as u8 == 0 {
                i += 1;
                continue;
            }
            let scale_now = scale;
            if scale_now == ONE {
                let src_words =
                    core::slice::from_raw_parts(weights as *const u32, count);
                samples[..count].copy_from_slice(src_words);
            } else {
                let mut k = 0usize;
                while k < count {
                    let v = rdf(weights.wrapping_add((k as u32).wrapping_mul(4)));
                    samples[k] = mul(v, scale_now).to_bits();
                    k += 1;
                }
            }
            leaves[..count].fill(0);
            let mut live = false;
            let mut s = 0usize;
            while s < count {
                let key = rd32(keys.wrapping_add((s as u32).wrapping_mul(4)));
                let bound = rd16(key.wrapping_add(COUNT_OFF)) as i32;
                if (cursor[s] as i32) < bound {
                    let cands = rd32(key.wrapping_add(TABLE_OFF));
                    let want = key24(obj);
                    loop {
                        let cand =
                            rd32(cands.wrapping_add(cursor[s].wrapping_mul(4)));
                        let got = key24(cand);
                        if got == want {
                            leaves[s] = cand;
                            cursor[s] = cursor[s].wrapping_add(1);
                            break;
                        }
                        if got > want {
                            break;
                        }
                        cursor[s] = cursor[s].wrapping_add(1);
                        if (cursor[s] as i32) >= bound {
                            break;
                        }
                    }
                }
                let loaded = leaves[s];
                if loaded == 0 {
                    samples[s] = 0;
                } else if rd8(loaded.wrapping_add(FLAG_OFF)) & SKIP_FLAG != 0 {
                    leaves[s] = 0;
                    samples[s] = 0;
                } else {
                    live = true;
                }
                s += 1;
            }
            if !live {
                i += 1;
                continue;
            }
            if vcall != 0 {
                let slot = rd32(rd32(vcall).wrapping_add(VTABLE_SLOT_OBSERVE));
                let observe: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                // The stub may rewrite sample 0 through the pointer.
                let _ = observe(vcall, b5, w6, n, samples.as_mut_ptr() as u32);
            }
            let mut running = leftover;
            let mut k = 0usize;
            while k < count {
                let wv = f32::from_bits(samples[k]);
                running = add(running, wv);
                if !(running >= EPS) {
                    samples[k] = 0;
                    running = 0.0;
                    k += 1;
                    continue;
                }
                let q = div(wv, running);
                samples[k] = (if 0.0 > q {
                    0.0
                } else if q > ONE {
                    ONE
                } else {
                    q
                })
                .to_bits();
                k += 1;
            }
            let mut s = 0usize;
            while s < count {
                let entry = leaves[s];
                if entry != 0 {
                    let t = f32::from_bits(samples[s]);
                    if t > 0.0 {
                        blend(obj, entry, t);
                    }
                }
                s += 1;
            }
            i += 1;
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00691D80(this_ptr: u32, n: u32, keys: u32, vcall: u32, helper: u32, _u: u32, weights: u32) -> u32 {
    audio_blend_pass(this_ptr, n, keys, vcall, helper, weights)
});
