// original: 0x00E3EEB0 blend_param_pair_store (proposed)

/// Evaluate one object's float parameters and store two result pairs.
///
/// `this` points to an object holding a row base (`+0x50`), three selectable
/// parameter floats (`+0x314/+0x318/+0x31c`), addends (`+0x328`, `+0x32c`,
/// `+0x340`, `+0x344`, `+0x348`, `+0x35c`, `+0x33c`), a mode byte (`+0x359`)
/// and a flag byte (`+0x395`). `a1` and `a2` receive two floats each, `a3`
/// selects the parameter (`0`, `1`, anything else), and `a4` is a row index.
///
/// Behaviour: a lookup callee is asked about the row
/// (`[this+0x50] + a4*0x7c + 0x40`, with the registry object constant in
/// ECX), and a second callee turns its answer into the float `f` (x87 ST0).
/// A base value `x2 = ([this+0x348] + f) + [this+0x35c]` and a half-scaled
/// pick `x3 = pick*0.5` (the 0.5 lives in a global float table) are formed;
/// a main value `x1` starts from the selected parameter (`p0`, `p1+p0`, or
/// `(a3-1) as float * p2` plus `p0`, or plus `p1+p0` when the flag byte is
/// set) plus `[this+0x328]`. The mode byte then decides the stores: mode 1
/// stores `x1 - pick + [this+0x340]`, mode 2 stores `x1 - (x2*0.5 +
/// [this+0x340])`, any other mode stores `x1 - (x2*0.5 + x3)`. In every
/// case `[a2+4] = [this+0x344] + [this+0x32c]`, `[a1] = [a2] + [this+0x348]
/// + [this+0x35c]` and `[a1+4] = [this+0x33c] + [this+0x32c]`. Returns `a1`.
/// The float operation order is the original's.
///
/// Original: 0x00E3EEB0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00E3EEB0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: i32 = 0x7c;
        const ROW_BIAS: i32 = 0x40;
        const OFF_ROW_BASE: u32 = 0x50;
        const OFF_P0: u32 = 0x314;
        const OFF_P1: u32 = 0x318;
        const OFF_P2: u32 = 0x31c;
        const OFF_SUM: u32 = 0x328;
        const OFF_BIAS: u32 = 0x32c;
        const OFF_ALT: u32 = 0x340;
        const OFF_ALT_B: u32 = 0x344;
        const OFF_ACCUM: u32 = 0x348;
        const OFF_TAIL: u32 = 0x35c;
        const OFF_LAST: u32 = 0x33c;
        const OFF_MODE: u32 = 0x359;
        const OFF_FLAG: u32 = 0x395;
        const REGISTRY: u32 = 0x116bff0;
        const HALF_TABLE: u32 = 0xfe8830;
        const CALLEE_LOOKUP: u32 = 1;
        const CALLEE_FLOAT: u32 = 2;

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

        // Row pointer for the lookup callee; the index is consumed before
        // the float result reuses its stack slot.
        let row = rd32(this.wrapping_add(OFF_ROW_BASE)).wrapping_add(
            (a4 as i32)
                .wrapping_mul(ROW_STRIDE)
                .wrapping_add(ROW_BIAS) as u32,
        );
        let found: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_LOOKUP,
            u32,
            lf_checker_rt::relocated(REGISTRY),
            row,
            1
        );
        let f: f32 = lf_checker_rt::callee_stdcall!(CALLEE_FLOAT, f32, found);

        let sel = a3;
        let accum = rdf(this.wrapping_add(OFF_ACCUM));
        let half = rdf(lf_checker_rt::relocated(HALF_TABLE));
        let pick = if sel == 0 {
            rdf(this.wrapping_add(OFF_P0))
        } else if sel == 1 {
            rdf(this.wrapping_add(OFF_P1))
        } else {
            rdf(this.wrapping_add(OFF_P2))
        };
        let mut base = add(accum, f);
        let scaled = mul(pick, half);
        base = add(base, rdf(this.wrapping_add(OFF_TAIL)));
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

        // The mode byte is sign-extended and compared against 1 then 2.
        let mode = rd8(this.wrapping_add(OFF_MODE)) as i8;
        if mode == 1 {
            main = sub(main, pick);
            main = add(main, rdf(this.wrapping_add(OFF_ALT)));
        } else {
            // This half-scale runs for every mode except 1.
            base = mul(base, half);
            if mode == 2 {
                base = add(base, rdf(this.wrapping_add(OFF_ALT)));
            } else {
                base = add(base, scaled);
            }
            main = sub(main, base);
        }
        // Stores in the original's order (matters if a1 aliases a2).
        wrf(a2, main);
        wrf(
            a2.wrapping_add(4),
            add(
                rdf(this.wrapping_add(OFF_ALT_B)),
                rdf(this.wrapping_add(OFF_BIAS)),
            ),
        );
        wrf(
            a1,
            add(add(main, accum), rdf(this.wrapping_add(OFF_TAIL))),
        );
        wrf(
            a1.wrapping_add(4),
            add(
                rdf(this.wrapping_add(OFF_LAST)),
                rdf(this.wrapping_add(OFF_BIAS)),
            ),
        );
        a1
    }
});
