// original: 0x00c2e0b0 audfire_voice_process (proposed name)
// Full per-voice fire-audio process step.
//
// Resolves the voice record for `arg0` (falling back to the slot default),
// runs the direction triple through a sampled matrix stage and a two-vector
// mix stage, accumulates mode-selected clamped contributions into the emitter
// state, applies a scaled decay through a shaping call, folds the result back
// into the emitter position, and publishes the outcome to the voice record in
// a mode-dependent layout. `arg1` and `arg2` are mix vectors, `arg3` and
// `arg4` are decay parameters. Returns nothing meaningful.
export!(thiscall, rw_00c2e0b0(
    this_ptr: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
) -> u32 {
    unsafe {
        let vt = lu(arg0);
        let slot_fetch: u32 = lu(vt + 0xa0);
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot_fetch as usize);
        let mut picked = fetch(arg0);
        if picked != 0 {
            picked = fetch(arg0);
            let slot_fin: u32 = lu(lu(picked) + 0xe0);
            let finish: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot_fin as usize);
            picked = finish(picked);
        } else {
            picked = lu(arg0 + 0x100);
        }
        if picked == 0 {
            return 0;
        }
        let w = load_i16(arg0 + 0x2e) as i32;
        let cx = load_i16(this_ptr) as i32;
        let g1 = lu((relocated(0x01295cd8) as i32 + w * 4) as u32);
        let v = lu(lu(g1 + 0xcc).wrapping_add((cx * 4) as u32)) as i32;
        if v == -1 {
            return 0;
        }
        let fd = callee_thiscall!(3, u32, arg0, v as u32);
        let m = lu(arg0 + 0x20);
        let m0 = load_f(m);
        let m4 = load_f(m + 4);
        let m8 = load_f(m + 8);
        let m10 = load_f(m + 0x10);
        let m14 = load_f(m + 0x14);
        let m18 = load_f(m + 0x18);
        let m20 = load_f(m + 0x20);
        let m24 = load_f(m + 0x24);
        let m28 = load_f(m + 0x28);
        let f30 = load_f(fd + 0x30);
        let f34 = load_f(fd + 0x34);
        let f38 = load_f(fd + 0x38);
        let mut p6 = fmul_first(m10, f34);
        let mut p2 = fmul_first(m14, f34);
        let mut p1 = fmul_first(m18, f34);
        p6 = fadd_first(p6, fmul_first(m0, f30));
        p6 = fadd_first(p6, fmul_first(m20, f38));
        p2 = fadd_first(p2, fmul_first(m4, f30));
        p2 = fadd_first(p2, fmul_first(m24, f38));
        p1 = fadd_first(p1, fmul_first(m8, f30));
        p1 = fadd_first(p1, fmul_first(m28, f38));
        // The sampler takes an in-vector and fills an out-vector; both travel
        // as frame pointers, so the contract skips the addresses and snapshots
        // the pointed-to words instead.
        let pin = [p6.to_bits(), p2.to_bits(), p1.to_bits(), 0u32];
        let mut pout = [0u32; 4];
        callee_thiscall!(
            4,
            u32,
            arg0,
            pout.as_mut_ptr() as u32,
            pin.as_ptr() as u32,
            0,
            0
        );
        let q0 = f32::from_bits(pin[0]);
        let q1 = f32::from_bits(pin[1]);
        let q2 = f32::from_bits(pin[2]);
        let r0 = f32::from_bits(pout[0]);
        let r1 = f32::from_bits(pout[1]);
        let r2 = f32::from_bits(pout[2]);
        let a10 = load_f(arg1);
        let a14 = load_f(arg1 + 4);
        let a18 = load_f(arg1 + 8);
        let b0 = load_f(arg2);
        let b4 = load_f(arg2 + 4);
        let b8 = load_f(arg2 + 8);
        let mut x4 = fmul_first(q0, b8);
        let mut x1 = fmul_first(q0, b4);
        let mut x3 = fmul_first(q2, b0);
        let mut x6 = fmul_first(q2, b4);
        let mut x0 = fmul_first(q1, b8);
        let mut x7 = fmul_first(q1, b0);
        x4 = fsub_first(x4, x3);
        x7 = fsub_first(x7, x1);
        let xv1 = fadd_first(a14, x4);
        x6 = fsub_first(x6, x0);
        let xv0 = fadd_first(a10, x6);
        let xv2 = fadd_first(a18, x7);
        let mut y3 = fsub_first(r1, xv1);
        let mut y1 = fsub_first(r2, xv2);
        let mut y4 = fsub_first(r0, xv0);
        let gc0 = load_f(relocated(0x011735c0));
        y3 = fmul_first(y3, gc0);
        y1 = fmul_first(y1, gc0);
        y4 = fmul_first(y4, gc0);
        let mut t6 = fmul_first(m4, y3);
        let mut t2 = fmul_first(m14, y3);
        let mut t5 = fmul_first(m24, y3);
        t6 = fadd_first(t6, fmul_first(y4, m0));
        t6 = fadd_first(t6, fmul_first(m8, y1));
        t2 = fadd_first(t2, fmul_first(m10, y4));
        t2 = fadd_first(t2, fmul_first(m18, y1));
        t5 = fadd_first(t5, fmul_first(y4, m20));
        t5 = fadd_first(t5, fmul_first(m28, y1));
        let mode = lu(this_ptr + 4) as i32;
        let sm = load_f(this_ptr + 8);
        let sign = lu(relocated(0x00fe8fa0));
        let neg = |x: f32| f32::from_bits(x.to_bits() ^ sign);
        let lim = load_f(relocated(0x010484c0));
        let nlim = neg(lim);
        let gbc = load_f(relocated(0x011735bc));
        // Symmetric clamp: min(max(-lim, v), lim), with NaN passing through.
        let clamp = |v: f32| {
            let d = if nlim > v { nlim } else { v };
            if d > lim { lim } else { d }
        };
        // Shared second-stage clamp over t2 into state word 0x28.
        let mut s3 = sm;
        let mut run_shared = false;
        match mode {
            4 | 5 => {
                let s = if mode == 5 { sm } else { neg(sm) };
                let c = clamp(t2);
                let acc = fadd_first(
                    fmul_first(fmul_first(gbc, s), c),
                    load_f(this_ptr + 0x20),
                );
                store_f(this_ptr + 0x20, acc);
                let c2 = clamp(t6);
                let acc2 = fadd_first(
                    fmul_first(fmul_first(gbc, s), c2),
                    load_f(this_ptr + 0x24),
                );
                store_f(this_ptr + 0x24, acc2);
            }
            2 | 3 => {
                s3 = if mode == 3 { sm } else { neg(sm) };
                let c = clamp(t5);
                let acc = fadd_first(
                    fmul_first(fmul_first(gbc, s3), c),
                    load_f(this_ptr + 0x20),
                );
                store_f(this_ptr + 0x20, acc);
                run_shared = true;
            }
            0 | 1 => {
                s3 = if mode == 1 { sm } else { neg(sm) };
                let c = clamp(t5);
                let acc = fadd_first(
                    fmul_first(fmul_first(gbc, s3), c),
                    load_f(this_ptr + 0x24),
                );
                store_f(this_ptr + 0x24, acc);
                run_shared = true;
            }
            _ => {}
        }
        if run_shared {
            let c = clamp(t2);
            let acc = fadd_first(
                fmul_first(fmul_first(gbc, s3), c),
                load_f(this_ptr + 0x28),
            );
            store_f(this_ptr + 0x28, acc);
        }
        let a3 = f32::from_bits(arg3);
        let a4 = f32::from_bits(arg4);
        let k = load_f(relocated(0x00fe8d94));
        let (scale, calarg) = if a3 > k && a4 > k {
            (a3, a4)
        } else if mode == 4 {
            (
                load_f(relocated(0x010484c8)),
                load_f(relocated(0x010484c4)),
            )
        } else {
            (
                load_f(relocated(0x010484bc)),
                load_f(relocated(0x010484b8)),
            )
        };
        let d1 = fmul_first(fmul_first(load_f(this_ptr + 0x10), scale), gbc);
        let d2 = fmul_first(fmul_first(load_f(this_ptr + 0x14), scale), gbc);
        let d3 = fmul_first(fmul_first(load_f(this_ptr + 0x18), scale), gbc);
        let n20 = fsub_first(load_f(this_ptr + 0x20), d1);
        store_f(this_ptr + 0x20, n20);
        let n24 = fsub_first(load_f(this_ptr + 0x24), d2);
        store_f(this_ptr + 0x24, n24);
        let n28 = fsub_first(load_f(this_ptr + 0x28), d3);
        store_f(this_ptr + 0x28, n28);
        let ans = f32::from_bits(callee_cdecl!(5, u32, calarg.to_bits()));
        store_f(this_ptr + 0x20, fmul_first(n20, ans));
        store_f(this_ptr + 0x24, fmul_first(n24, ans));
        store_f(this_ptr + 0x28, fmul_first(n28, ans));
        let c0 = fmul_first(load_f(this_ptr + 0x20), gbc);
        let c1 = fmul_first(gbc, load_f(this_ptr + 0x24));
        let c2 = fmul_first(gbc, load_f(this_ptr + 0x28));
        let n10 = fadd_first(load_f(this_ptr + 0x10), c0);
        store_f(this_ptr + 0x10, n10);
        let n14 = fadd_first(load_f(this_ptr + 0x14), c1);
        store_f(this_ptr + 0x14, n14);
        let n18 = fadd_first(load_f(this_ptr + 0x18), c2);
        store_f(this_ptr + 0x18, n18);
        match mode {
            4 | 5 => {
                store_f(fd + 0x24, n10);
                store_u(fd + 0x20, n14.to_bits());
            }
            2 | 3 => {
                store_u(fd + 0x10, n18.to_bits());
                store_u(fd + 0x18, n10.to_bits());
            }
            0 | 1 => {
                let hk = load_f(relocated(0x00fe8d7c));
                let mut t0 = fmul_first(load_f(fd + 0x34), load_f(fd + 0x30));
                t0 = fmul_first(t0, load_f(this_ptr + 0x18));
                if t0 > 0.0 {
                    let u = load_f(this_ptr + 0x28) * hk;
                    store_u(this_ptr + 0x18, 0);
                    store_f(this_ptr + 0x28, u);
                }
                let t0b = fmul_first(load_f(this_ptr + 0x14), load_f(fd + 0x30));
                if t0b < 0.0 {
                    let u = load_f(this_ptr + 0x24) * hk;
                    store_u(this_ptr + 0x14, 0);
                    store_f(this_ptr + 0x24, u);
