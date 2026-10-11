// original: 0x0088ba50 audVoice_process_88BA50 (proposed)
/// Mix one voice's six channel levels and report the peak band.
///
/// `this` points to the voice object, `+0x4` to its parameter block. A
/// half-precision word at block `+0x1a` becomes a float gain (zero stays
/// zero, otherwise sign, exponent-plus-`0x70` and mantissa are shifted into
/// place); the block float at `+0x20` is scaled by a fixed image constant.
/// Unless inhibit bit `0x40` at block `+0x18` is set, a probe helper runs
/// (no stack arguments); its non-zero low byte enables clamping.
///
/// Each of the six levels at `this + 0x20..0x38` is then multiplied by the
/// gain and, when clamping, pulled back toward the matching block level at
/// `+0x24..` by a fixed epsilon unless already within it (unordered
/// compares take the below branch on both sides). The six are rescaled by
/// the scaled block float and reduced to the maximum against zero; mode
/// byte `1` at block `+0x6c` additionally scales the peak by a fixed image
/// constant.
///
/// Peaks at or below a fixed floor post `-10000` to the channel object at
/// `+0x90` through its slot at `+0x3c` (two stack arguments). Larger peaks
/// first run a shaper helper taking the peak in the vector register (no
/// stack arguments), then post `floor(peak * 2000)` clamped to
/// `[-10000, 0]` (`0x80000000` counts as below). The post helper's answer
/// is returned.
///
/// Original: 0x0088ba50 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088ba50(this: u32) -> u32 {
    unsafe {
        const PARAM_BLK: u32 = 0x4;
        const HALF_WORD: u32 = 0x1a;
        const INHIBIT_BYTE: u32 = 0x18;
        const INHIBIT_BIT: u8 = 0x40;
        const BLOCK_FLOAT: u32 = 0x20;
        const BLOCK_LEVELS: u32 = 0x24;
        const MODE_BYTE: u32 = 0x6c;
        const LEVELS: u32 = 0x20;
        const NLEVELS: u32 = 6;
        const CHANNEL_OBJ: u32 = 0x90;
        const CHANNEL_SLOT: u32 = 0x3c;
        const POST_FLOOR_ARG: u32 = 0xffff_d8f0; // -10000
        // The probe field (+0x18 past the object at [this]) is callee
        // id 1, loaded and called exactly like the original.
        const CAL_SHAPE: u32 = 2;
        // The channel slot (+0x3c) is callee id 3, reached through the
        // planted vtable exactly like the original.
        const K_GAIN: u32 = 0x00e7_7f90;
        const K_EPS: u32 = 0x0103_009c;
        const K_PEAK_SCALE: u32 = 0x00fe_87e8;
        const K_FLOOR: u32 = 0x00fe_8670;
        const K_HZ: u32 = 0x00e7_7f94;
        const PROBE_SLOT: u32 = 0x18;

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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
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
        /// Half word to float bits, exactly the original's shift sequence.
        #[inline(always)]
        fn half_bits(h: u16) -> u32 {
            if h == 0 {
                0
            } else {
                let mut ecx = (h as u32) & 0xffff_8000;
                let mut edx = (h as u32) >> 10;
                ecx <<= 3;
                edx &= 0x1f;
                let eax = (h as u32) & 0x3ff;
                ecx |= eax;
                edx += 0x70;
                ecx <<= 13;
                edx <<= 23;
                ecx | edx
            }
        }

        let blk = rd32(this.wrapping_add(PARAM_BLK));
        let half = (blk.wrapping_add(HALF_WORD) as *const u16).read_unaligned();
        let gain = f32::from_bits(half_bits(half));
        let frozen = (blk.wrapping_add(INHIBIT_BYTE) as *const u8).read() & INHIBIT_BIT != 0;
        let k_gain = f32::from_bits(g32(K_GAIN));
        let scaled = mul(rdf(blk.wrapping_add(BLOCK_FLOAT)), k_gain);
        let mut clamp = false;
        if !frozen {
            let o0 = rd32(this);
            let probe: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(o0.wrapping_add(PROBE_SLOT)) as usize);
            clamp = (probe(this) as u8) != 0;
        }
        let eps = f32::from_bits(g32(K_EPS));
        let mut i = 0u32;
        while i < NLEVELS {
            let la = this.wrapping_add(LEVELS).wrapping_add(i.wrapping_mul(4));
            let ba = blk.wrapping_add(BLOCK_LEVELS).wrapping_add(i.wrapping_mul(4));
            let mut x = mul(rdf(ba), gain);
            if clamp {
                let y = rdf(la);
                let d = sub(x, y);
                // `comiss; jb` is taken on below OR unordered: `!(d >= 0)`.
                let c = if !(d >= 0.0) { sub(y, eps) } else { add(eps, y) };
                let a = sub(d.abs(), eps);
                if a >= 0.0 {
                    x = c;
                }
            }
            wrf(la, x);
            i = i.wrapping_add(1);
        }
        let mode = (blk.wrapping_add(MODE_BYTE) as *const u8).read();
        let mut m = 0.0f32;
        let mut j = 0u32;
        while j < NLEVELS {
            let v = mul(rdf(this.wrapping_add(LEVELS).wrapping_add(j.wrapping_mul(4))), scaled);
            // `comiss; jbe` falls through only on strictly greater.
            if v > m {
                m = v;
            }
            j = j.wrapping_add(1);
        }
        if mode.wrapping_sub(1) == 0 {
            m = mul(m, f32::from_bits(g32(K_PEAK_SCALE)));
        }
        let floor = f32::from_bits(g32(K_FLOOR));
        let mut arg = POST_FLOOR_ARG;
        if m > floor {
            let _ = lf_checker_rt::callee_cdecl!(CAL_SHAPE, u32, m.to_bits());
            let hz = f32::from_bits(g32(K_HZ));
            let fl = mul(m, hz).floor();
            let edx = if fl.is_finite() && fl < 2147483648.0 && fl >= -2147483648.0 {
                fl as i32
            } else {
                i32::MIN
            };
            if edx >= -10000 {
                arg = if edx > 0 { 0 } else { edx as u32 };
            }
        }
        let obj = rd32(this.wrapping_add(CHANNEL_OBJ));
        let vt = rd32(obj);
        let post: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(CHANNEL_SLOT)) as usize);
        post(obj, arg)
    }
});
