// original: 0x00997600 audio_mix_voice_frame (proposed)

/// Mix one audio voice frame into the 10 output slots of `out`.
///
/// `this` is the voice object (only used to derive callee object pointers;
/// never dereferenced). `out` points at the destination struct: the mix
/// input is the float at `+0x28`, and the results land at `+0x04`, `+0x08`,
/// `+0x0c`, `+0x10`, `+0x14`, `+0x18`, `+0x1c`, `+0x20`, `+0x24`, `+0x34`.
///
/// Behaviour: a 14-word scratch struct is initialised by callee 1, filled by
/// callee 2 (function 0x997e00, intercepted), then combined here. The input
/// goes through seven `map` calls (callee 3, thiscall, one float arg, float
/// in ST0), four `peek` calls (callee 4, one float arg left on the stack for
/// the next call to pop, float in ST0) and six `gen` calls (callee 5,
/// thiscall, constant 0 arg, float in ST0). Two of the map results are
/// truncated to int for the `ebx`/`edi` terms. The tail is pure SSE: every
/// output is `scratch * t2 + other * t1` with `t1` the first map result and
/// `t2 = K1 - t1`, where `K1`, `K2`, `K3` are float constants read from the
/// original's read-only data through unrelocated absolute addresses (file
/// VAs 0xFE88E8, 0xFE8628, 0xFE8DF8). The first map call takes the global
/// map object whose address is the unrelocated immediate 0x1283898, passed
/// through opaquely to the intercepted callee. Integer outputs use x86
/// truncate-toward-zero with the 0x80000000 out-of-range/NaN result, not
/// Rust saturation. Returns the `+0x08` word.
///
/// Original: 0x00997600 (thiscall, one stack word). Unverifiable on the
/// stock v5 worker: the original faults reading the unrelocated constants.
/// Written for a re-run with read-only data shadowing.
lf_checker_rt::export!(thiscall, rw_00997600(this: u32, out: u32) -> u32 {
    unsafe {
        const INIT_CALLEE: u32 = 1;
        const FILL_CALLEE: u32 = 2;
        const MAP_CALLEE: u32 = 3;
        const PEEK_CALLEE: u32 = 4;
        const GEN_CALLEE: u32 = 5;
        const GLOBAL_MAP: u32 = 0x0128_3898;
        const K1_VA: u32 = 0x00FE_88E8;
        const K2_VA: u32 = 0x00FE_8628;
        const K3_VA: u32 = 0x00FE_8DF8;
        const IN_OFF: u32 = 0x28;

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
        /// x86 CVTTSS2SI: truncate toward zero; NaN or out of the i32 range
        /// (at or beyond 2^31 either side) gives 0x80000000.
        #[inline(always)]
        fn cvtt(v: f32) -> i32 {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            }
        }
        #[inline(always)]
        unsafe fn map(obj: u32, x: f32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> f32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(MAP_CALLEE) as usize,
                );
                f(obj, x.to_bits())
            }
        }
        #[inline(always)]
        unsafe fn peek(x: f32) -> f32 {
            unsafe {
                let f: extern "cdecl" fn(u32) -> f32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(PEEK_CALLEE) as usize,
                );
                f(x.to_bits())
            }
        }
        #[inline(always)]
        unsafe fn gen(obj: u32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> f32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(GEN_CALLEE) as usize,
                );
                f(obj, 0)
            }
        }

        unsafe {
            let mut s = [0u32; 14];
            let s_ptr = s.as_mut_ptr() as u32;
            lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, s_ptr);
            lf_checker_rt::callee_thiscall!(FILL_CALLEE, u32, this, s_ptr);

            let inp = rdf(out + IN_OFF);
            let t1 = map(GLOBAL_MAP, inp);
            let k1 = f32::from_bits(g32(K1_VA));
            let t2 = sub(k1, t1);

            let y1 = peek(map(this + 0x280, inp));
            let y2 = peek(map(this + 0x2a8, inp));
            let y3 = peek(map(this + 0x2f8, inp));
            let y4 = peek(map(this + 0x320, inp));
            let ebx = cvtt(map(this + 0x1b8, inp));
            let edi = cvtt(map(this + 0x2d0, inp));

            let acc_a = add(add(gen(this + 0x780), gen(this + 0x5f0)), y1);
            let acc_b = add(add(gen(this + 0x6e0), gen(this + 0x5f0)), y2);
            let acc_c = add(gen(this + 0x730), y3);
            let r6 = gen(this + 0x640);
            let q1 = add(r6, y4);

            let k2 = f32::from_bits(g32(K2_VA));
            let k3 = f32::from_bits(g32(K3_VA));
            let k2t1 = mul(t1, k2);
            let k3t1 = mul(t1, k3);
            let s24 = f32::from_bits(s[0x24 / 4]);
            let s12 = f32::from_bits(s[0x0c / 4]);
            let s04 = f32::from_bits(s[0x04 / 4]);
            let s20 = f32::from_bits(s[0x20 / 4]);
            let s16 = f32::from_bits(s[0x10 / 4]);
            let s18 = f32::from_bits(s[0x18 / 4]);
            let s34 = f32::from_bits(s[0x34 / 4]);

            wrf(out + 0x24, add(mul(s24, t2), k2t1));
            wrf(out + 0x0c, add(mul(s12, t2), mul(acc_a, t1)));
            wrf(out + 0x04, add(mul(s04, t2), mul(acc_b, t1)));
            wrf(out + 0x20, add(mul(s20, t2), mul(acc_c, t1)));
            wrf(out + 0x10, add(mul(s16, t2), mul(q1, t1)));
            wrf(out + 0x18, add(mul(s18, t2), k3t1));
            wrf(out + 0x34, add(mul(s34, t2), k3t1));

            let fs14 = (s[0x14 / 4] as i32) as f32;
            let fs1c = (s[0x1c / 4] as i32) as f32;
            let fs08 = (s[0x08 / 4] as i32) as f32;
            let o1c = cvtt(add(mul(fs1c, t2), k2t1));
            wr32(out + 0x1c, o1c as u32);
            let o14 = cvtt(add(mul(fs14, t2), mul(ebx as f32, t1)));
            wr32(out + 0x14, o14 as u32);
            let o08 = cvtt(add(mul(fs08, t2), mul(edi as f32, t1)));
            wr32(out + 0x08, o08 as u32);
            o08 as u32
        }
    }
});
