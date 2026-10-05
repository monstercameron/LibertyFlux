// original: 0x00d3c120 dummytask_helper_a (proposed)

/// Wander-task movement update: steers the task at `task` toward its target
/// using the mover state in `arg_a` and the target descriptor in `arg_b`.
/// The third stack argument is not read.
///
/// Layout: `task+0x10` is an optional position source (null selects the
/// task's own stored vector at `+0x20`), `task+0x38` a slot handle or zero.
/// `arg_a+0` is a small element count, `arg_a+0xc` a mode flag, `arg_a+0x18`
/// a timer, `arg_a+0x28` an array of 16-byte elements. `arg_b+0x20` points at
/// the target position vector (`+0x30`/`+0x34`/`+0x38`).
///
/// Behaviour: after notifying the base object, the function resolves or
/// releases the slot handle, asks the mover for its current vector, then
/// either ticks the timer down (mode set) or measures the planar or spatial
/// distance to the target (chosen by comparing a scaled counter against a
/// threshold) and reports it. A trailing loop adds one to the first lane of
/// each array element, two state words are reset, and a final mover call
/// runs whose result is returned. Two global words are read (a float seed
/// and a state word) and several read-only float constants.
///
/// Float order throughout is the original's; the two float comparisons that
/// treat unordered as taken/not-taken are written to match exactly.
///
/// Original: 0x00d3c120 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d3c120(task: u32, arg_a: u32, arg_b: u32, _arg_c: u32) -> u32 {
    unsafe {
        const T_SRC: u32 = 0x10;
        const T_VEC: u32 = 0x20;
        const T_HANDLE: u32 = 0x38;
        const A_COUNT: u32 = 0x00;
        const A_STATE: u32 = 0x04;
        const A_WORD: u32 = 0x08;
        const A_MODE: u32 = 0x0c;
        const A_VEC: u32 = 0x10;
        const A_TIMER: u32 = 0x18;
        const A_ARR: u32 = 0x28;
        const B_OBJ: u32 = 0x20;
        const G_SEED: u32 = 0x01054a30;
        const G_WORD: u32 = 0x011735b4;
        const K_TICK: u32 = 0x00fe878c;
        const K_SCALE: u32 = 0x00fe8684;
        const K_THRESH: u32 = 0x00fe879c;
        const K_ONE: u32 = 0x00fe88e8;
        const K_ALT: u32 = 0x00fe8b80;
        const FLAT_ARG: u32 = 0x42700000;

        const C_BASE: u32 = 1;
        const C_PROBE: u32 = 2;
        const C_ACQUIRE: u32 = 3;
        const C_ALIVE: u32 = 4;
        const C_RELEASE: u32 = 5;
        const C_MOVER: u32 = 6;
        const C_TICK: u32 = 7;
        const C_COUNTER: u32 = 8;
        const C_REPORT2: u32 = 9;
        const C_REPORT3: u32 = 10;
        const C_FINISH: u32 = 11;

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
        /// Resolve the current position vector: the source object's vector
        /// (through its indirect pointer when present, else its inline one),
        /// or the task's stored vector when there is no source. The vector is
        /// also stored back to the task. Pure 32-bit copies.
        #[inline(always)]
        unsafe fn load_vec(task: u32) -> [u32; 4] {
            unsafe {
                let src = rd32(task + T_SRC);
                if src == 0 {
                    [rd32(task + T_VEC), rd32(task + T_VEC + 4),
                     rd32(task + T_VEC + 8), rd32(task + T_VEC + 12)]
                } else {
                    let ind = rd32(src + 0x20);
                    let base = if ind == 0 { src + 0x10 } else { ind + 0x30 };
                    let v = [rd32(base), rd32(base + 4), rd32(base + 8), rd32(base + 12)];
                    wr32(task + T_VEC, v[0]);
                    wr32(task + T_VEC + 4, v[1]);
                    wr32(task + T_VEC + 8, v[2]);
                    wr32(task + T_VEC + 12, v[3]);
                    v
                }
            }
        }

        let obj = rd32(arg_b + B_OBJ);
        let seed = rd32(lf_checker_rt::global::<u32>(G_SEED) as u32);
        lf_checker_rt::callee_thiscall!(C_BASE, u32, task);
        let mover = arg_a;
        // Scratch word reused by the final call: the seed, still intact
        // (the measure block's writes land above it).
        if rd32(mover + A_MODE) == 0 {
            if rd32(task + T_HANDLE) == 0 {
                let r: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, arg_b);
                if (r as u8) != 0 {
                    let mut vec = load_vec(task);
                    let h: u32 = lf_checker_rt::callee_cdecl!(
                        C_ACQUIRE,
                        u32,
                        arg_b,
                        vec.as_mut_ptr() as u32
                    );
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
            seed
        );
        if (r as u8) != 0 {
            if rd32(mover + A_MODE) != 0 {
                lf_checker_rt::callee_thiscall!(C_TICK, u32, mover);
                wrf(mover + A_TIMER, fsub(rdf(mover + A_TIMER), rdc(K_TICK)));
            } else {
                let tx = rdf(obj + 0x30);
                let ty = rdf(obj + 0x34);
                let tz = rdf(obj + 0x38);
                let vec = load_vec(task);
                let n: u32 = lf_checker_rt::callee_cdecl!(C_COUNTER, u32,);
                let prod = fmul((n as i32) as f32, rdc(K_SCALE));
                if !(rdc(K_THRESH) > prod) {
                    let dx = fsub(tx, f32::from_bits(vec[0]));
                    let dy = fsub(ty, f32::from_bits(vec[1]));
                    let dz = fsub(tz, f32::from_bits(vec[2]));
                    let len2 = fadd(
                        fadd(fmul(dx, dx), fmul(dy, dy)),
                        fmul(dz, dz),
                    );
                    let v = fadd(len2.sqrt(), rdc(K_ALT));
                    let (mut sa, mut sb) = (0u32, 0u32);
                    lf_checker_rt::callee_cdecl!(
                        C_REPORT3,
                        u32,
                        &mut sb as *mut u32 as u32,
                        &mut sa as *mut u32 as u32,
                        v.to_bits(),
                        0x10,
                        mover,
                        mover + A_VEC
                    );
                } else {
                    let dx = fsub(tx, f32::from_bits(vec[0]));
                    let dy = fsub(ty, f32::from_bits(vec[1]));
                    let len2 = fadd(fmul(dx, dx), fmul(dy, dy));
                    // The original tests the squared length against zero
                    // through an unordered-aware compare: only an exact zero
                    // takes the flat path, NaN goes through the root.
                    let s = if len2 == 0.0 {
                        0.0f32
                    } else {
                        fdiv(rdc(K_ONE), len2.sqrt())
                    };
                    let _dirx = fmul(dx, s);
                    let _diry = fmul(s, dy);
                    let _dirz = fmul(s, 0.0);
                    let (mut sa, mut sb) = (0u32, 0u32);
                    lf_checker_rt::callee_cdecl!(
                        C_REPORT2,
                        u32,
                        &mut sb as *mut u32 as u32,
                        &mut sa as *mut u32 as u32,
                        FLAT_ARG,
                        0x10,
                        mover,
                        mover + A_VEC
                    );
                }
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
            seed
        )
    }
});
