// original: 0x00E3F060 blend_param_single_store (proposed)

/// Evaluate one object's float parameters and store a single result.
///
/// `this` points to an object holding three selectable parameter floats
/// (`+0x314/+0x318/+0x31c`), addends (`+0x328`, `+0x340`), a mode byte
/// (`+0x381`) and a flag byte (`+0x395`). `a2` receives one float; `a4`
/// selects the parameter (`0`, `1`, anything else); `a1`, `a3` and `a5` are
/// integers combined into a lookup key, and the numeric value of `a2` itself
/// feeds that key as well.
///
/// Behaviour: a lookup callee is asked about the key
/// `a3*0x2b0 + 0x1c + a1 + ((a5<<4) - a2)*4` (with the registry object
/// constant in ECX), and a second callee turns its answer into the float
/// `f` (x87 ST0). A half-scaled pick `x2 = pick*0.5` (the 0.5 lives in a
/// global float table) is formed; a main value `x1` starts from the selected
/// parameter (`p0`, `p1+p0`, or `(a4-1) as float * p2` plus `p0`, or plus
/// `p1+p0` when the flag byte is set) plus `[this+0x328]`. The first result
/// `x1 - (f*0.5 + x2)` is stored to `[a2]`. The mode byte then decides the
/// final store: mode 1 stores `x1 - pick + [this+0x340]`, mode 2 stores
/// `x1 - ([this+0x340] + f)`, any other mode re-stores the first result.
/// Returns `a2`. The float operation order is the original's.
///
/// Original: 0x00E3F060 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00E3F060(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const KEY_STRIDE: i32 = 0x2b0;
        const KEY_BIAS: i32 = 0x1c;
        const OFF_P0: u32 = 0x314;
        const OFF_P1: u32 = 0x318;
        const OFF_P2: u32 = 0x31c;
        const OFF_SUM: u32 = 0x328;
        const OFF_ALT: u32 = 0x340;
        const OFF_MODE: u32 = 0x381;
        const OFF_FLAG: u32 = 0x395;
        const REGISTRY: u32 = 0x116bff0;
        const HALF_TABLE: u32 = 0xfe8830;
        const CALLEE_LOOKUP: u32 = 1;
        const CALLEE_FLOAT: u32 = 2;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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

        // Lookup key; a2 contributes its address value as an integer.
        let key = (a3 as i32)
            .wrapping_mul(KEY_STRIDE)
            .wrapping_add(KEY_BIAS)
            .wrapping_add(a1 as i32)
            .wrapping_add(
                ((a5 as i32).wrapping_shl(4).wrapping_sub(a2 as i32))
                    .wrapping_mul(4),
            ) as u32;
        let found: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_LOOKUP,
            u32,
            lf_checker_rt::relocated(REGISTRY),
            key,
            1
        );
        let f: f32 = lf_checker_rt::callee_stdcall!(CALLEE_FLOAT, f32, found);

        let sel = a4;
        let half = rdf(lf_checker_rt::relocated(HALF_TABLE));
        let pick = if sel == 0 {
            rdf(this.wrapping_add(OFF_P0))
        } else if sel == 1 {
            rdf(this.wrapping_add(OFF_P1))
        } else {
            rdf(this.wrapping_add(OFF_P2))
        };
        let scaled = mul(pick, half);
        let mut main = if sel == 0 {
            rdf(this.wrapping_add(OFF_P0))
        } else if sel == 1 {
            add(
                rdf(this.wrapping_add(OFF_P1)),
                rdf(this.wrapping_add(OFF_P0)),
            )
        } else {
            let t = mul(
                sel.wrapping_sub(1) as i32 as f32,
                rdf(this.wrapping_add(OFF_P2)),
            );
            if rd8(this.wrapping_add(OFF_FLAG)) == 0 {
                add(t, rdf(this.wrapping_add(OFF_P0)))
            } else {
                add(
                    t,
                    add(
                        rdf(this.wrapping_add(OFF_P1)),
                        rdf(this.wrapping_add(OFF_P0)),
                    ),
                )
            }
        };
        main = add(main, rdf(this.wrapping_add(OFF_SUM)));

        let first = sub(main, add(mul(f, half), scaled));
        wrf(a2, first);
        // The mode byte is sign-extended and compared against 1 then 2.
        let mode = rd8(this.wrapping_add(OFF_MODE)) as i8;
        if mode == 1 {
            main = sub(main, pick);
            wrf(a2, add(main, rdf(this.wrapping_add(OFF_ALT))));
        } else if mode == 2 {
            wrf(a2, sub(main, add(rdf(this.wrapping_add(OFF_ALT)), f)));
        } else {
            wrf(a2, first);
        }
        a2
    }
});
