// original: 0x00618820 timed_sweep_fill_rows (proposed)

/// Build one frame of a timed sweep: resolve the active target through the
/// object graph, then fill a caller-provided row buffer, or take an early
/// exit when the sweep is disabled.
///
/// `this` points to the sweep object. Float inputs live at `+0x84` (rate)
/// and `+0x90/+0x94/+0x98` (copied aside for the resolver); the object graph
/// runs `+0x2cc -> +0xc` (entry), `+0x18` (target list) and `+0x40` (enable
/// table); `+0x30c` is a select index and `+0x2c4` a state word that is both
/// read and handed to the state callee.
///
/// Behaviour in order: the resolver callee (setter 1) is called with
/// `this+0x10` and three out-buffers; its nine output words drive everything
/// below. The first three, scaled by the constant at the first float slot,
/// become the row biases. A global selector (or its fallback pair when it
/// holds -1) scaled by 3 indexes the enable table when the select index is
/// 0; a zero entry exits early returning the table base. Otherwise the
/// target slot is `base - 16 + index*16`: the done flag byte is set, the
/// slot's word at `+0xc` is tested (zero exits early returning 0 after
/// publishing the slot), and the slot pointer at `+0x8` is published before
/// the gather callee (setter 2) runs. The state callee (setter 3) runs only
/// when the state word is 0; the state word is then passed to setter 4 and a
/// fresh row buffer is requested from setter 5 with `(5, 0xa, 0x10)`.
///
/// When the buffer is non-null it is filled: `this+0x2c8` takes
/// `buffer+0xa0`, the first row takes the biases and 1.0, and nine further
/// rows are computed from `i * step0 * step1` passed through the two scalar
/// helpers (setters 6 and 7, float in and out of XMM0) combined with the
/// scaled resolver outputs, each row ending in 1.0. Setter 8 closes the fill.
/// A non-zero done flag then runs setter 9. Both published slots are cleared
/// and the last callee answer is returned.
///
/// Original: thiscall, no stack arguments. All float arithmetic is single
/// precision in the original's operation order.
lf_checker_rt::export!(thiscall, rw_00618820(this: u32) -> u32 {
    unsafe {
        const C_RESOLVER: u32 = 1;
        const C_GATHER: u32 = 2;
        const C_STATE: u32 = 3;
        const C_USE_STATE: u32 = 4;
        const C_ALLOC_ROWS: u32 = 5;
        const C_HELPER_A: u32 = 6;
        const C_HELPER_B: u32 = 7;
        const C_FINISH: u32 = 8;
        const C_DONE_HOOK: u32 = 9;
        const ONE_BITS: u32 = 0x3f800000;
        const G_SELECT: u32 = 0x0106b308;
        const G_GATE: u32 = 0x017f584c;
        const G_FALLBACK: u32 = 0x0106b30c;
        const G_FALLBACK_ALT: u32 = 0x01063b54;
        const G_FLAG: u32 = 0x017ed94b;
        const G_SLOT: u32 = 0x017f58e0;
        const G_SLOT_PTR: u32 = 0x017f58e4;
        const F_BIAS_SCALE: u32 = 0x00fe8c2c;
        const F_RATE_A: u32 = 0x00fe8830;
        const F_RATE_B: u32 = 0x00fe86c4;
        const F_RATE_C: u32 = 0x00fe8874;
        const F_STEP_A: u32 = 0x00fe8aec;
        const F_STEP_B: u32 = 0x00fe87a4;
        const O_RATE: u32 = 0x84;
        const O_GRAPH: u32 = 0x2cc;
        const O_SELECT: u32 = 0x30c;
        const O_STATE: u32 = 0x2c4;
        const O_ROW_BASE: u32 = 0x2c8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn g32(g: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(g)) }
        }
        #[inline(always)]
        unsafe fn gf(g: u32) -> f32 {
            unsafe { f32::from_bits(g32(g)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Resolver outputs: out0 feeds the biases, out1/out2 the row factors.
        let mut out0 = [0u32; 3];
        let mut out1 = [0u32; 3];
        let mut out2 = [0u32; 3];
        lf_checker_rt::callee_thiscall!(
            C_RESOLVER, u32, this.wrapping_add(0x10),
            out0.as_mut_ptr() as u32, out1.as_mut_ptr() as u32, out2.as_mut_ptr() as u32
        );
        let scale = gf(F_BIAS_SCALE);
        let bias0 = mul(f32::from_bits(out0[0]), scale);
        let bias1 = mul(f32::from_bits(out0[1]), scale);
        let bias2 = mul(f32::from_bits(out0[2]), scale);

        let mut select = rd32(this.wrapping_add(O_SELECT));
        let entry = rd32(rd32(this.wrapping_add(O_GRAPH)).wrapping_add(0xc));
        let mut sel = g32(G_SELECT);
        if sel == 0xffff_ffff {
            sel = g32(G_FALLBACK);
            if g32(G_GATE) != 0 {
                sel = g32(G_FALLBACK_ALT);
            }
        }
        if select == 0 {
            let table = rd32(entry.wrapping_add(0x40));
            let idx = sel.wrapping_add(sel.wrapping_mul(2));
            let b = ((table.wrapping_add(idx)) as *const u8).read();
            if b == 0 {
                return table;
            }
            select = b as u32;
        }
        let list = rd32(entry.wrapping_add(0x18));
        let slot = rd32(list).wrapping_sub(0x10).wrapping_add(select.wrapping_mul(16));
        ((lf_checker_rt::relocated(G_FLAG)) as *mut u8).write(1);
        let tag = ((slot.wrapping_add(0xc)) as *const u16).read();
        wr32(lf_checker_rt::relocated(G_SLOT), slot);
        if tag == 0 {
            return 0;
        }
        let slot_ptr = rd32(slot.wrapping_add(8));
        wr32(lf_checker_rt::relocated(G_SLOT_PTR), slot_ptr);
        lf_checker_rt::callee_thiscall!(
            C_GATHER, u32, slot_ptr,
            list.wrapping_add(0x10), list.wrapping_add(0x18),
            entry.wrapping_add(0x14), list.wrapping_add(8)
        );

        // Row-factor scale from the rate input.
        let rate = rdf(this.wrapping_add(O_RATE));
        let mut k = mul(rate, gf(F_RATE_A));
        k = mul(k, rate);
        k = mul(k, gf(F_RATE_B));
        k = mul(k, gf(F_RATE_C));
        let f0 = mul(f32::from_bits(out2[0]), k);
        let f1 = mul(f32::from_bits(out2[1]), k);
        let f2 = mul(f32::from_bits(out2[2]), k);
        let f3 = mul(f32::from_bits(out1[0]), k);
        let f4 = mul(f32::from_bits(out1[1]), k);
        let f5 = mul(f32::from_bits(out1[2]), k);

        let state = rd32(this.wrapping_add(O_STATE));
        if state == 0 {
            lf_checker_rt::callee_thiscall!(C_STATE, u32, this.wrapping_add(O_STATE));
        }
        lf_checker_rt::callee_cdecl!(C_USE_STATE, u32, rd32(this.wrapping_add(O_STATE)));
        let buf: u32 = lf_checker_rt::callee_cdecl!(C_ALLOC_ROWS, u32, 5, 0x0a, 0x10);
        let mut last = buf;
        if buf != 0 {
            wr32(this.wrapping_add(O_ROW_BASE), buf.wrapping_add(0xa0));
            wrf(buf, bias0);
            wrf(buf.wrapping_add(4), bias1);
            wrf(buf.wrapping_add(8), bias2);
            wr32(buf.wrapping_add(0xc), ONE_BITS);
            let step_a = gf(F_STEP_A);
            let step_b = gf(F_STEP_B);
            let mut row = buf.wrapping_add(0x10);
            let mut i = 0u32;
            while i < 9 {
                let x = mul(mul(i as f32, step_a), step_b);
                let ha = f32::from_bits(lf_checker_rt::callee_cdecl!(C_HELPER_A, u32, x.to_bits()));
                let hb = f32::from_bits(lf_checker_rt::callee_cdecl!(C_HELPER_B, u32, x.to_bits()));
                let v0 = add(add(mul(f3, hb), mul(f0, ha)), bias0);
                let v1 = add(add(mul(f4, hb), mul(f1, ha)), bias1);
                let v2 = add(add(mul(f5, hb), mul(f2, ha)), bias2);
                wrf(row, v0);
                wr32(row.wrapping_add(0xc), ONE_BITS);
                wrf(row.wrapping_add(4), v1);
                wrf(row.wrapping_add(8), v2);
                row = row.wrapping_add(0x10);
                i += 1;
            }
            last = lf_checker_rt::callee_cdecl!(C_FINISH, u32,);
        }
        if ((lf_checker_rt::relocated(G_FLAG)) as *const u8).read() != 0 {
            last = lf_checker_rt::callee_cdecl!(C_DONE_HOOK, u32,);
        }
        wr32(lf_checker_rt::relocated(G_SLOT_PTR), 0);
        wr32(lf_checker_rt::relocated(G_SLOT), 0);
        last
    }
});
