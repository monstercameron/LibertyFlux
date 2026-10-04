// original: 0x00db2480 UITexture::vf85 (merged symbol)

/// Refresh a UI texture's cached layout numbers, then reshape two of them
/// according to a polled mode.
///
/// `this` points to the texture object: dword at `+0x00` is its virtual table,
/// floats at `+0x1e4`..`+0x1fc` are the cached numbers. Six virtual slots are
/// used, all called with the object in `ecx` and no stack arguments: four
/// float getters (slots `+0xc8`, `+0xd0`, `+0xb8`, `+0xc0`) whose results are
/// stored to `+0x1e4`, `+0x1e8`, `+0x1ec`, `+0x1f0`; a mode poller (slot
/// `+0x74`) returning an integer; and a scale getter (slot `+0x6c`) returning
/// a float.
///
/// After the four stores the mode is polled up to four times, stopping at the
/// first hit: `2`, `4`, `8`, `0x10` select reshape branches 1-4, anything else
/// falls through and returns the last mode. Each reshape calls the scale
/// getter and two of the float getters, then combines the three answers with
/// the cached numbers using multiply/subtract/add by the constant `0.5` and
/// one negation, writing two cached numbers (branches 1-2: `+0x1f0` and
/// `+0x1e8`; branch 3: `+0x1ec`, `+0x1e4`, `+0x1fc`; branch 4: `+0x1ec`,
/// `+0x1e4`, `+0x1f4`). Branches 3-4 also read the entry values at `+0x1f4`
/// and `+0x1fc`.
///
/// Edge cases: arbitrary float bits (including NaN and infinities) flow
/// through unchanged in shape; the arithmetic order is the original's, so
/// NaN payloads and rounding match bit for bit. The return value is the last
/// call's answer: the mode on fall-through, otherwise the last getter's bits.
///
/// Original: 0x00db2480 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00db2480(this: u32) -> u32 {
    unsafe {
        const HALF: f32 = 0.5;
        const SIGN: u32 = 0x8000_0000;
        const SLOT_SCALE: u32 = 0x6c;
        const SLOT_MODE: u32 = 0x74;
        const SLOT_F2: u32 = 0xb8;
        const SLOT_F3: u32 = 0xc0;
        const SLOT_F0: u32 = 0xc8;
        const SLOT_F1: u32 = 0xd0;
        const N0: u32 = 0x1e4;
        const N1: u32 = 0x1e8;
        const N2: u32 = 0x1ec;
        const N3: u32 = 0x1f0;
        const N4: u32 = 0x1f4;
        const N7: u32 = 0x1fc;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn get(this: u32, vt: u32, slot: u32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(this)
            }
        }
        #[inline(always)]
        unsafe fn mode(this: u32, vt: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + SLOT_MODE) as usize);
                f(this)
            }
        }

        let vt = rd32(this);
        wrf(this + N0, get(this, vt, SLOT_F0));
        wrf(this + N1, get(this, vt, SLOT_F1));
        wrf(this + N2, get(this, vt, SLOT_F2));
        wrf(this + N3, get(this, vt, SLOT_F3));
        if mode(this, vt) == 2 {
            // Branch 1: scale, second getter, fourth getter.
            let a = get(this, vt, SLOT_SCALE);
            let b = get(this, vt, SLOT_F1);
            let c = get(this, vt, SLOT_F3);
            let half_c = mul(c, HALF);
            let mut acc = sub(b, half_c);
            let scaled = mul(a, rdf(this + N3));
            acc = neg(acc);
            wrf(this + N3, scaled);
            let half_scaled = mul(scaled, HALF);
            acc = sub(acc, half_scaled);
            wrf(this + N1, acc);
            return c.to_bits();
        }
        if mode(this, vt) == 4 {
            // Branch 2: scale, fourth getter, second getter.
            let a = get(this, vt, SLOT_SCALE);
            let b = get(this, vt, SLOT_F3);
            let half_b = mul(b, HALF);
            let c = get(this, vt, SLOT_F1);
            let mut acc = mul(a, rdf(this + N3));
            let mut sum = add(c, half_b);
            wrf(this + N3, acc);
            acc = mul(acc, HALF);
            sum = sub(sum, acc);
            sum = neg(sum);
            wrf(this + N1, sum);
            return c.to_bits();
        }
        if mode(this, vt) == 8 {
            // Branch 3: scale, first getter, third getter.
            let a = get(this, vt, SLOT_SCALE);
            let b = get(this, vt, SLOT_F0);
            let c = get(this, vt, SLOT_F2);
            let half_c = mul(c, HALF);
            let base = rdf(this + N4);
            let mut acc = sub(b, half_c);
            let mut prod = mul(rdf(this + N2), a);
            wrf(this + N2, prod);
            prod = mul(prod, HALF);
            prod = add(prod, acc);
            wrf(this + N0, prod);
            acc = sub(rdf(this + N7), base);
            acc = mul(acc, a);
            acc = add(acc, base);
            wrf(this + N7, acc);
            return c.to_bits();
        }
        let m = mode(this, vt);
        if m == 0x10 {
            // Branch 4: scale, third getter, first getter.
            let a = get(this, vt, SLOT_SCALE);
            let b = get(this, vt, SLOT_F2);
            let half_b = mul(b, HALF);
            let c = get(this, vt, SLOT_F0);
            let mut acc = mul(rdf(this + N2), a);
            let mut sum = add(c, half_b);
            wrf(this + N2, acc);
            acc = mul(acc, HALF);
            sum = sub(sum, acc);
            let keep = rdf(this + N7);
            wrf(this + N0, sum);
            let mut tail = sub(keep, rdf(this + N4));
            tail = mul(tail, a);
            let unter = sub(keep, tail);
            wrf(this + N4, unter);
            return c.to_bits();
        }
        m
    }
});

