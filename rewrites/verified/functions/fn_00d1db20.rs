// original: 0x00d1db20 cover_aim_blend (proposed)
/// Blend a cover-aim angle from a ped, a direction and scripted samplers.
///
/// `ped` is the ped, `dir` three floats, `out` a flag byte this function
/// sets. Callee 1 (cdecl, no stack args; it takes two f64 values in XMM0/1
/// which the checker cannot observe, and answers an f64 in XMM0 whose high
/// half the checker cannot deliver, so the high half stays pinned while the
/// low half varies) supplies the base angle narrowed to f32.
///
/// The tag class (`[tag]>>5&3`, where `tag` is `ped+0xd68`) routes first:
/// class 1 returns base+pi/2 with flag 0, class 0 returns base-pi/2 with
/// flag 1. Otherwise a self-tag check (bytes at `+0x218`/`+0x219` plus
/// whether the tag pointer equals `ped+0xe70`) decides between the tail and
/// the main path.
///
/// The main path samples callee 2 (thiscall, out-pointer plus 0) into three
/// words, scales the direction by 10, adds the sample, and runs callee 3
/// (cdecl, two out-pointers plus four zeros) whose answer must be non-null
/// with low 3 bits off 1. Callee 4 (thiscall on that answer, two
/// out-pointers) refreshes the buffers; callees 5 and 6 (cdecl, pi/2 in
/// XMM0, pinned f64 answers like callee 1's low half) supply scalars that
/// rotate the direction. Three gates (an absolute dot under 0.707, a sum of
/// squares under 9, a second dot over 0.707, all strict, NaN failing shut)
/// guard the final split on the callee-3 answer's tag class: class 1 with a
/// positive rotated projection returns base+pi/2 with flag 0, class 0 with a
/// negative one returns base-pi/2 with flag 1. Every other route reaches the
/// tail, which dots the position at `ped+0x20` with the direction and
/// returns base-pi/2 (flag 1) for a strictly negative dot, base+pi/2
/// (flag 0) otherwise. The float result returns in ST0.
/// Original: 0x00d1db20 (cdecl, three stack args).
lf_checker_rt::export!(cdecl, rw_00d1db20(ped: u32, dir: u32, out: u32) -> f32 {
    unsafe {
        const PED_TAG: u32 = 0xd68;
        const PED_POS: u32 = 0x20;
        const SELF_TAG: u32 = 0xe70;
        const FLAG_A: u32 = 0x218;
        const FLAG_B: u32 = 0x219;
        const HALF_PI_BITS: u32 = 0x00fe8978;
        const F64_HI: u32 = 0x3ff00000;
        const BASE: u32 = 1;
        const SAMPLE: u32 = 2;
        const SCAN: u32 = 3;
        const REFRESH: u32 = 4;
        const FIRST_F64: u32 = 5;
        const SECOND_F64: u32 = 6;
        const PROBE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn tail(ped: u32, dir: u32, out: u32, base: f32, half_pi: f32) -> f32 {
            unsafe {
                let v0 = rdf(dir);
                let v1 = rdf(dir + 4);
                let v2 = rdf(dir + 8);
                let p = rd32(ped + PED_POS);
                let s_a = mul(rdf(p), v0);
                let s_b = mul(rdf(p + 4), v1);
                let s_c = add(s_b, s_a);
                let s_d = mul(rdf(p + 8), v2);
                let dot = add(s_c, s_d);
                if dot < 0.0 {
                    (out as *mut u8).write(1);
                    sub(base, half_pi)
                } else {
                    (out as *mut u8).write(0);
                    add(base, half_pi)
                }
            }
        }

        let half_pi = f32::from_bits(rd32(lf_checker_rt::relocated(HALF_PI_BITS)));
        let lo: u32 = lf_checker_rt::callee_cdecl!(BASE, u32,);
        let base = core::hint::black_box(f64::from_bits(((F64_HI as u64) << 32) | lo as u64)) as f32;
        let self_tagged = if rd8(ped + FLAG_A) != 0 {
            false
        } else if rd8(ped + FLAG_B) == 0 {
            false
        } else {
            ped.wrapping_add(SELF_TAG) == rd32(ped + PED_TAG)
        };
        let tagp = rd32(ped + PED_TAG);
        let cls = (rd32(tagp) >> 5) & 3;
        if cls == 1 {
            (out as *mut u8).write(0);
            return add(base, half_pi);
        }
        if cls == 0 {
            (out as *mut u8).write(1);
            return sub(base, half_pi);
        }
        if !self_tagged {
            return tail(ped, dir, out, base, half_pi);
        }
        let mut sample = [0u32; 3];
        lf_checker_rt::callee_thiscall!(SAMPLE, u32, tagp, sample.as_mut_ptr() as u32, 0);
        let v0 = rdf(dir);
        let v1 = rdf(dir + 4);
        let v2 = rdf(dir + 8);
        let ten = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8b08)));
        let t3 = add(mul(v0, ten), f32::from_bits(sample[0]));
        let t2 = add(mul(v1, ten), f32::from_bits(sample[1]));
        let t0 = add(f32::from_bits(sample[2]), mul(v2, ten));
        let mut buf = [0u32; 9];
        buf[0] = t3.to_bits();
        buf[1] = t2.to_bits();
        buf[2] = t0.to_bits();
        let mut scan_out = [0u32; 3];
        let found: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32, scan_out.as_mut_ptr() as u32, buf.as_mut_ptr() as u32, 0, 0, 0, 0);
        if found == 0 || (rd8(found) & 7) == 1 {
            return tail(ped, dir, out, base, half_pi);
        }
        let mut probe_out = [0u32; 3];
        lf_checker_rt::callee_thiscall!(SAMPLE, u32, found, probe_out.as_mut_ptr() as u32, 0);
        let mut aux = [0u32; 3];
        let under: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, found, aux.as_mut_ptr() as u32, buf.as_mut_ptr() as u32);
        let r5: u64 = lf_checker_rt::callee_cdecl!(FIRST_F64, u64,);
        let s1 = core::hint::black_box(f64::from_bits(r5)) as f32;
        let r6: u64 = lf_checker_rt::callee_cdecl!(SECOND_F64, u64,);
        let s2 = core::hint::black_box(f64::from_bits(r6)) as f32;
        let t_a = mul(v1, s1);
        let t_b = mul(v0, s2);
        let rx = sub(t_b, t_a);
        let t_c = mul(v0, s1);
        let t_d = mul(v1, s2);
        let ry = add(t_d, t_c);
        let d0 = sub(f32::from_bits(buf[4]), f32::from_bits(scan_out[0]));
        let d1 = sub(f32::from_bits(buf[5]), f32::from_bits(scan_out[1]));
        let mut slot50 = d0.to_bits();
        let mut slot54 = d1.to_bits();
        lf_checker_rt::callee_thiscall!(PROBE, u32, &mut slot50 as *mut u32 as u32);
        let slot50f = f32::from_bits(slot50);
        let slot54f = f32::from_bits(slot54);
        let z7 = mul(v2, 0.0);
        let t_e = mul(slot54f, v1);
        let t_f = mul(slot50f, v0);
        let mut gate = add(add(t_e, t_f), z7);
        if gate < 0.0 {
            gate = -gate;
        }
        let c707 = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8878)));
        if !(gate < c707) {
            return tail(ped, dir, out, base, half_pi);
        }
        let q0 = sub(f32::from_bits(buf[6]), f32::from_bits(scan_out[2]));
        let t_h = mul(d0, d0);
        let t_i = mul(d1, d1);
        let t_j = mul(q0, q0);
        let sum = add(add(t_h, t_i), t_j);
        let c9 = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8b00)));
        if !(sum < c9) {
            return tail(ped, dir, out, base, half_pi);
        }
        let u1 = mul(rdf(under), v0);
        let u0 = mul(rdf(under + 4), v1);
        let u0 = add(u0, u1);
        let u1b = mul(rdf(under + 8), v2);
        let dot3 = add(u0, u1b);
        if !(dot3 > c707) {
            return tail(ped, dir, out, base, half_pi);
        }
        let cls2 = (rd32(found) >> 5) & 3;
        let w = add(add(mul(slot54f, ry), mul(slot50f, rx)), z7);
        if cls2 == 1 {
            if w > 0.0 {
                (out as *mut u8).write(0);
                return add(base, half_pi);
            }
            return tail(ped, dir, out, base, half_pi);
        }
        if cls2 != 0 {
            return tail(ped, dir, out, base, half_pi);
        }
        let z = add(add(mul(d1, ry), mul(d0, rx)), z7);
        if z < 0.0 {
            (out as *mut u8).write(1);
            sub(base, half_pi)
        } else {
            tail(ped, dir, out, base, half_pi)
        }
    }
});
