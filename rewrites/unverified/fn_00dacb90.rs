// original: 0x00dacb90 hassle_ped_steer_update (proposed)
//
// thiscall/1: update the steering accumulator block at [this+0x60..0x70]
// from the linked task objects, then store the squared planar distance to
// the target point.
//
// Layout (all offsets in bytes). `this` (+0x14: link A; +0x20/+0x24: two
// scale floats; +0x40: flag byte selecting the long blend path; +0x60:
// accumulator dword + two floats + dword; +0x70: output). A (+0x20: link B,
// may be null; +0x28: mode word; +0x00: vtable on the mode-dispatched path).
// B (+0x00/0x04/0x08: vector V; +0x10/0x14/0x18: vector W; +0x30..0x3c:
// fallback record). S, the stack arg (+0x20: link T; +0x30/+0x34 on T: the
// target point).
//
// Algorithm. Pick a 16-byte record (at B+0x30 when B is non-null, else at
// A+0x10) into +0x60..0x6c. On the long path (flag set): when B is null,
// run the C1/C2 init pair (thiscall/0 then thiscall/1 with ecx=A+0x10,
// which fills B); then blend V scaled by +0x20 and W scaled by +0x24 into
// the accumulators, in the original's SSE order. On the short path just
// add +0x20/+0x24 into +0x60/+0x64. When mode bits (([A+0x28]>>6)&0xf)
// are 2 or 3, scale a vtable-fetched vector by two absolute constants
// (a game .data word and a .rdata constant, read through relocated
// addresses) and add it in. Tail: store (dx*dx+dy*dy) of accumulator vs
// T's point into +0x70. Returns T (whatever eax holds at the end).
//
// The second C1/C2 pair is dead: its guard re-reads a value that was
// already dereferenced, so it is always non-null there. Kept faithfully;
// it never fires.
/// Inner implementation; `t0_force` overrides the t0 absolute read when set
/// (used only by the lane's second wrong-version export, not shipped).
unsafe fn b90_inner(this: u32, s: u32, t0_force: Option<u32>) -> u32 {
    unsafe {
        const LINK_A: u32 = 0x14;
        const LINK_B: u32 = 0x20;
        const MODE_WORD: u32 = 0x28;
        const SCALE_0: u32 = 0x20;
        const SCALE_1: u32 = 0x24;
        const FLAG: u32 = 0x40;
        const ACC0: u32 = 0x60;
        const ACC1: u32 = 0x64;
        const ACC2: u32 = 0x68;
        const ACC3: u32 = 0x6c;
        const OUT_DIST2: u32 = 0x70;
        const REC_OFF: u32 = 0x30;
        const ALT_OFF: u32 = 0x10;
        const VT_SLOT: u32 = 0xec;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const ABS_T0: u32 = 0x011735bc;
        const ABS_K: u32 = 0x00fe8b68;

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

        let esi = this;
        let a = rd32(esi + LINK_A);
        let b = rd32(a + LINK_B);
        let rec = if b != 0 {
            b.wrapping_add(REC_OFF)
        } else {
            a.wrapping_add(ALT_OFF)
        };
        wr32(esi + ACC0, rd32(rec));
        wrf(esi + ACC1, rdf(rec + 4));
        wrf(esi + ACC2, rdf(rec + 8));
        wr32(esi + ACC3, rd32(rec + 0x0c));

        if (esi as *const u8).add(FLAG as usize).read() != 0 {
            let edi = a;
            if rd32(edi + LINK_B) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(C1, u32, edi);
                let arg = rd32(edi + LINK_B);
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(C2, u32, edi.wrapping_add(0x10), arg);
            }
            let eax0 = rd32(edi + LINK_B);
            let edi2 = rd32(esi + LINK_A);
            let (mut x6, v1_saved, mut x7) = (rdf(eax0), rdf(eax0 + 4), rdf(eax0 + 8));
            let mut x1 = v1_saved;
            if rd32(edi2 + LINK_B) == 0 {
                // Dead in practice (eax0 above would have faulted first).
                let _: u32 = lf_checker_rt::callee_thiscall!(C1, u32, edi2);
                let arg = rd32(edi2 + LINK_B);
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(C2, u32, edi2.wrapping_add(0x10), arg);
            }
            let x0 = rdf(esi + SCALE_0);
            let mut x2 = rdf(esi + ACC1);
            let eb = rd32(edi2 + LINK_B);
            let (mut x3, mut x4, mut x5) = (rdf(eb + 0x10), rdf(eb + 0x14), rdf(eb + 0x18));
            x1 = mul(x1, x0);
            x6 = mul(x6, x0);
            x2 = add(x2, x1);
            x1 = rdf(esi + ACC2);
            x6 = add(x6, rdf(esi + ACC0));
            x7 = mul(x7, x0);
            wrf(esi + ACC1, x2);
            x1 = add(x1, x7);
            wrf(esi + ACC0, x6);
            wrf(esi + ACC2, x1);
            let x0b = rdf(esi + SCALE_1);
            x3 = mul(x3, x0b);
            x4 = mul(x4, x0b);
            x5 = mul(x5, x0b);
            x6 = add(x6, x3);
            x2 = add(x2, x4);
            x1 = add(x1, x5);
            wrf(esi + ACC0, x6);
            wrf(esi + ACC1, x2);
            wrf(esi + ACC2, x1);
        } else {
            wrf(esi + ACC0, add(rdf(esi + SCALE_0), rdf(esi + ACC0)));
            wrf(esi + ACC1, add(rdf(esi + SCALE_1), rdf(esi + ACC1)));
        }

        let ecx = rd32(esi + LINK_A);
        let mode = (rd32(ecx + MODE_WORD) >> 6) & 0xf;
        if mode == 2 || mode == 3 {
            // Abs-blocked branch: honest implementation through relocated
            // reads, for the gap probe. Never runs in the passing slice.
            let vt = rd32(ecx);
            let t0 = match t0_force {
                Some(b) => f32::from_bits(b),
                None => rdf(lf_checker_rt::relocated(ABS_T0)),
            };
            let target: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt + VT_SLOT) as usize);
            let mut frame_slot: u32 = 0;
            let got = target(ecx, core::ptr::addr_of_mut!(frame_slot) as u32);
            let k = rdf(lf_checker_rt::relocated(ABS_K));
            let mut y1 = mul(rdf(got + 4), k);
            let mut y3 = mul(rdf(got), k);
            let mut y2 = mul(rdf(got + 8), k);
            y1 = mul(y1, t0);
            y3 = mul(y3, t0);
            y2 = mul(y2, t0);
            wrf(esi + ACC1, add(rdf(esi + ACC1), y1));
            wrf(esi + ACC2, add(rdf(esi + ACC2), y2));
            wrf(esi + ACC0, add(y3, rdf(esi + ACC0)));
        }

        let t = rd32(s + LINK_B);
        let mut d1 = sub(rdf(esi + ACC0), rdf(t + 0x30));
        let mut d0 = sub(rdf(esi + ACC1), rdf(t + 0x34));
        d1 = mul(d1, d1);
        d0 = mul(d0, d0);
        wrf(esi + OUT_DIST2, add(d0, d1));
        t
    }
}

lf_checker_rt::export!(thiscall, rw_00dacb90(this: u32, s: u32) -> u32 {
    unsafe { b90_inner(this, s, None) }
});
