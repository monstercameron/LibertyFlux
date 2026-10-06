// original: 0x00997e00 audio_compute_listener_gains (proposed)

/// Compute one frame of listener-relative gains into the output record.
///
/// `this` is the audio channel object; only address-derived sub-object
/// pointers are formed from it (never dereferenced here) plus one state
/// block pointer read at `+0x820`. `outp` is the output record: its word at
/// `+0x2c` is read as a float input, and twelve words are written
/// (`+0x00`, `+0x04`, `+0x08`, `+0x0c`, `+0x10`, `+0x14`, `+0x18`, `+0x1c`,
/// `+0x20`, `+0x24`, `+0x30`, `+0x34`).
///
/// Behaviour: the input gain is loaded from the state block and stored to
/// `+0x24`. It is then run through the shared equalizer (callee 1, file
/// address 0x8acef0) eight times with different sub-objects; three of those
/// results feed nothing (the original issues the calls and drops the values,
/// which still consumes one scripted answer each), the other five land at
/// `+0x0c`, `+0x04`, `+0x20`, `+0x10` and `+0x00`/`+0x18`. Three counter
/// reads (callee 2, file address 0xdfa130) land at `+0x08`, `+0x1c` and
/// `+0x14`. A vtable call through the state block (slot `+0xfc`, stubbed as
/// callee 5) yields a scale candidate; a flag byte in writable game data
/// selects it or an alternate gain from writable game data. The selected
/// value is multiplied by the read-only scale constant (0.001), clamped to
/// the read-only ceiling (1.0) with a lower bound of 0.0 (NaN passes through
/// as NaN, matching the original's ordered comparisons), stored to `+0x30`,
/// and run through the equalizer once more and then the shaper (callee 3,
/// file address 0x890030) into `+0x34`. The record's own `+0x2c` word is run
/// through the equalizer into a temporary. Six mixing calls (callee 4, file
/// address 0x8ab800), each passed a zero argument, are accumulated in order
/// onto `+0x0c`, `+0x04`, `+0x20` and `+0x10`, and the last temporary is
/// added onto `+0x18`. The returned value is the last mixing answer's bits,
/// which is what the original leaves in eax.
///
/// All floating-point arithmetic uses the original's operand order (pinned
/// with `black_box`); results match bit for bit.
///
/// Original: 0x00997e00 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00997e00(this: u32, outp: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x820;
        const GAIN_OFF: u32 = 0x1ec0;
        const VT_SLOT: u32 = 0xfc;
        const K_SCALE: u32 = 0x00fe86b4;
        const K_MAX: u32 = 0x00fe88e8;
        const G_ALT: u32 = 0x01038c1c;
        const G_FLAG: u32 = 0x012832aa;

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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn eq(sub: u32, bits: u32) -> f32 {
            unsafe { lf_checker_rt::callee_thiscall!(1, f32, sub, bits) }
        }
        #[inline(always)]
        unsafe fn mix(sub: u32) -> f32 {
            unsafe { lf_checker_rt::callee_thiscall!(4, f32, sub, 0u32) }
        }

        let inner = rd32(this.wrapping_add(STATE_OFF));
        let g_in = rdf(inner.wrapping_add(GAIN_OFF));
        let gbits = g_in.to_bits();
        wrf(outp.wrapping_add(0x24), g_in);

        let r_a = eq(this.wrapping_add(0xc8), gbits);
        wrf(outp.wrapping_add(0x0c), r_a);
        let r_b = eq(this.wrapping_add(0xc8), gbits);
        wrf(outp.wrapping_add(0x04), r_b);
        let _drop1 = eq(this.wrapping_add(0x140), gbits);
        let d0: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        wr32(outp.wrapping_add(0x08), d0);
        let r_e = eq(this.wrapping_add(0xf0), gbits);
        wrf(outp.wrapping_add(0x20), r_e);
        let r_f = eq(this.wrapping_add(0x118), gbits);
        wrf(outp.wrapping_add(0x10), r_f);
        wrf(outp.wrapping_add(0x00), eq(this.wrapping_add(0x168), gbits));
        let r_h = eq(this.wrapping_add(0x1e0), gbits);
        wrf(outp.wrapping_add(0x18), r_h);

        // Indirect call through the state block's table, slot +0xfc, with
        // the state block as the receiver, exactly like the original.
        let vtab = rd32(inner);
        let tgt = rd32(vtab.wrapping_add(VT_SLOT));
        let f: unsafe extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(tgt as usize) };
        let s18: f32 = unsafe { f(inner) };

        let flag: u8 = unsafe {
            ((lf_checker_rt::relocated(G_FLAG)) as *const u8).read_unaligned()
        };
        let x2: f32 = if flag == 0 {
            s18
        } else {
            rdf(lf_checker_rt::relocated(G_ALT))
        };
        let scale = rdf(lf_checker_rt::relocated(K_SCALE));
        let maxv = rdf(lf_checker_rt::relocated(K_MAX));
        let x0 = mul(x2, scale);
        // Ordered clamp matching the original's two ordered comparisons:
        // below zero yields zero, above the ceiling yields the ceiling,
        // anything else (including NaN) passes through.
        let x1: f32 = if 0.0f32 > x0 {
            0.0
        } else if x0 > maxv {
            maxv
        } else {
            x0
        };
        wrf(outp.wrapping_add(0x30), x1);

        let j: f32 = eq(this.wrapping_add(0x370), x2.to_bits());
        let k: f32 = lf_checker_rt::callee_cdecl!(3, f32, j.to_bits());
        wrf(outp.wrapping_add(0x34), k);

        let _drop2 = eq(this.wrapping_add(0x190), gbits);
        let d1: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        wr32(outp.wrapping_add(0x1c), d1);
        let _drop3 = eq(this.wrapping_add(0x1b8), gbits);
        let d2: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        wr32(outp.wrapping_add(0x14), d2);

        let p: f32 = eq(this.wrapping_add(0xa0), rd32(outp.wrapping_add(0x2c)));

        let q = mix(this.wrapping_add(0x780));
        let r = mix(this.wrapping_add(0x5f0));
        wrf(outp.wrapping_add(0x0c), add(add(q, r), r_a));
        let s = mix(this.wrapping_add(0x6e0));
        let t = mix(this.wrapping_add(0x5f0));
        wrf(outp.wrapping_add(0x04), add(add(s, t), r_b));
        let u = mix(this.wrapping_add(0x730));
        wrf(outp.wrapping_add(0x20), add(u, r_e));
        let v = mix(this.wrapping_add(0x640));
        wrf(outp.wrapping_add(0x10), add(v, r_f));
        wrf(outp.wrapping_add(0x18), add(r_h, p));
        v.to_bits()
    }
});
