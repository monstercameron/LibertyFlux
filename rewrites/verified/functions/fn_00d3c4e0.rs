// original: 0x00d3c4e0 dummytask_helper_b (proposed)

/// Wander-task steering update: drives the mover state in `arg_a` toward the
/// target descriptor in `arg_b` for the task at `task`. The third stack
/// argument is not read.
///
/// Layout: `task+0x1c` is a slot handle or zero. `arg_a+0` is a small element
/// count, `arg_a+0xc` a mode flag, `arg_a+0x28` an array of 16-byte elements.
/// `arg_b+0x20` points at the target vector block (`+0x10` inline vector,
/// `+0x30` position vector).
///
/// Behaviour: after resolving or releasing the slot handle and polling the
/// mover, the function either runs a tick callback (mode set) or measures:
/// a scaled counter picks a table-driven blend of two runtime sine/cosine
/// table entries with the current direction (the table index comes from an
/// x87 truncation to 64 bits, low byte kept), the blended direction is
/// normalised unless it is exactly zero, and the result is reported with the
/// mover state. A trailing loop adds one to the first lane of each array
/// element, two state words are reset, and a final mover call runs whose
/// result is returned. One global word and the two tables are read; several
/// read-only float constants scale the counters.
///
/// The two null-object helper calls on the short-count path are dead: the
/// prologue already dereferenced the object, so any trial reaching them has
/// a non-null object. They are documented, not called.
///
/// Float order throughout is the original's; the zero tests treat an exact
/// zero (positive or negative) as zero and NaN as non-zero, matching the
/// original's unordered-aware compares.
///
/// Original: 0x00d3c4e0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d3c4e0(task: u32, arg_a: u32, arg_b: u32, _arg_c: u32) -> u32 {
    unsafe {
        const T_HANDLE: u32 = 0x1c;
        const A_COUNT: u32 = 0x00;
        const A_STATE: u32 = 0x04;
        const A_WORD: u32 = 0x08;
        const A_MODE: u32 = 0x0c;
        const A_VEC: u32 = 0x10;
        const A_ARR: u32 = 0x28;
        const B_OBJ: u32 = 0x20;
        const G_WORD: u32 = 0x011735b4;
        const TAB_A: u32 = 0x016cf7c8;
        const TAB_B: u32 = 0x016cf3c8;
        const K_Q: u32 = 0x00fe8684;
        const K_C1: u32 = 0x00fe8b0c;
        const K_C0: u32 = 0x00fe8afc;
        const K_M: u32 = 0x00fe87dc;
        const K_TAU: u32 = 0x00fe8aec;
        const K_TAB: u32 = 0x00eb1174;
        const K_ONE: u32 = 0x00fe88e8;
        const ONE_BITS: u32 = 0x3f800000;

        const C_PROBE: u32 = 1;
        const C_ACQUIRE: u32 = 2;
        const C_ALIVE: u32 = 3;
        const C_RELEASE: u32 = 4;
        const C_MOVER: u32 = 5;
        const C_TICK: u32 = 6;
        const C_COUNTER: u32 = 7;
        const C_REPORT: u32 = 8;
        const C_FINISH: u32 = 9;

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
        unsafe fn rdc(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// The original's x87 chop conversion to 64 bits, low byte kept: a
        /// truncation; a NaN, infinity or out-of-range value stores the
        /// indefinite, whose low byte is zero.
        #[inline(always)]
        fn chop_index(v: f32) -> u32 {
            if !v.is_finite() || v >= 9.223372e18 || v <= -9.223372e18 {
                0
            } else {
                (v as i64) as u32 & 0xff
            }
        }

        let mover = arg_a;
        if rd32(mover + A_MODE) == 0 {
            if rd32(task + T_HANDLE) == 0 {
                let r: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, arg_b);
                if (r as u8) != 0 {
                    let h: u32 = lf_checker_rt::callee_cdecl!(C_ACQUIRE, u32, arg_b);
                    wr32(task + T_HANDLE, h);
                }
            } else {
                let h = rd32(task + T_HANDLE);
                if h != 0 {
                    let r: u32 = lf_checker_rt::callee_cdecl!(C_ALIVE, u32, h);
                    if (r as u8) != 0 {
                        lf_checker_rt::callee_cdecl!(C_RELEASE, u32, h, arg_b);
                        wr32(task + T_HANDLE, 0);
                    }
                }
            }
        }
        let mut slot = 0u32;
        let r: u32 = lf_checker_rt::callee_thiscall!(
            C_MOVER,
            u32,
            mover,
            &mut slot as *mut u32 as u32,
            ONE_BITS
        );
        if (r as u8) != 0 {
            if rd32(mover + A_MODE) != 0 {
                lf_checker_rt::callee_thiscall!(C_TICK, u32, mover);
            } else {
                let obj = rd32(arg_b + B_OBJ);
                let n1: u32 = lf_checker_rt::callee_cdecl!(C_COUNTER, u32,);
                let scaled = fadd(
                    fmul(fmul((n1 as i32) as f32, rdc(K_Q)), rdc(K_C1)),
                    rdc(K_C0),
                );
                let count = rd32(mover + A_COUNT);
                let vec: [f32; 3];
                if count > 1 {
                    let e = count.wrapping_mul(2);
                    let b0 = mover.wrapping_add(e.wrapping_mul(8));
                    let d0 = fsub(rdf(b0), rdf(b0.wrapping_sub(0x10)));
                    let d1 = fsub(rdf(b0 + 4), rdf(b0.wrapping_sub(0x0c)));
                    let len2 = fadd(fmul(d1, d1), fmul(d0, d0));
                    let s = if len2 == 0.0 {
                        0.0f32
                    } else {
                        fdiv(rdc(K_ONE), len2.sqrt())
                    };
                    vec = [fmul(d0, s), fmul(d1, s), fmul(s, 0.0)];
                } else {
                    // Null here would have faulted the prologue loads, so
                    // the original's two null-object calls below are dead.
                    vec = [rdf(obj + 0x10), rdf(obj + 0x14), rdf(obj + 0x18)];
                }
                let n2: u32 = lf_checker_rt::callee_cdecl!(C_COUNTER, u32,);
                let mix = fmul(fmul((n2 as i32) as f32, rdc(K_Q)), rdc(K_M));
                let n3: u32 = lf_checker_rt::callee_cdecl!(C_COUNTER, u32,);
                let tf = fmul(
                    fmul(fmul((n3 as i32) as f32, rdc(K_Q)), rdc(K_TAU)),
                    rdc(K_TAB),
                );
                let idx = chop_index(tf);
                let tbase_a = lf_checker_rt::relocated(TAB_A);
                let tbase_b = lf_checker_rt::relocated(TAB_B);
                let v0 = fadd(fmul(rdf(tbase_a + idx * 4), mix), vec[0]);
                let v1 = fadd(fmul(rdf(tbase_b + idx * 4), mix), vec[1]);
                let v2 = vec[2];
                let len2 = fadd(fadd(fmul(v1, v1), fmul(v0, v0)), fmul(v2, v2));
                let s = if len2 == 0.0 {
                    0.0f32
                } else {
                    fdiv(rdc(K_ONE), len2.sqrt())
                };
                let _dirx = fmul(s, v0);
                let _diry = fmul(s, v1);
                let _dirz = fmul(s, v2);
                let (mut sa, mut sb) = (0u32, 0u32);
                lf_checker_rt::callee_cdecl!(
                    C_REPORT,
                    u32,
                    &mut sb as *mut u32 as u32,
                    &mut sa as *mut u32 as u32,
                    scaled.to_bits(),
                    0x10,
                    mover,
                    mover + A_VEC
                );
                let count = rd32(mover + A_COUNT);
                if count > 1 {
                    let base = mover + A_ARR;
                    let one = rdc(K_ONE);
                    let mut i = 1u32;
                    while i < count {
                        let p = base.wrapping_add((i - 1).wrapping_mul(0x10));
                        wrf(p, fadd(rdf(p), one));
                        i = i.wrapping_add(1);
                    }
                }
            }
            wr32(mover + A_STATE, 0);
            wr32(mover + A_WORD, rd32(lf_checker_rt::global::<u32>(G_WORD) as u32));
        }
        let mut slot2 = 0u32;
        lf_checker_rt::callee_thiscall!(
            C_FINISH,
            u32,
            mover,
            &mut slot2 as *mut u32 as u32,
            ONE_BITS
        )
    }
});
