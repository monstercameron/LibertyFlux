// original: 0x00cd3130 task_state_update

/// Resolve a task handle, then configure and steer it.
///
/// `this` is the task object; `arg0` the subject record, `arg1` a control
/// word, `arg2` a go flag and `arg3` an optional record. No return value.
///
/// Behaviour: the handle comes from the lookup callee, falling back to a
/// global allocator plus a three-call chain and a re-lookup when the first
/// lookup misses. Two virtual polls run on the handle (the second answer
/// is discarded) and a third must report the ready state or the task exits
/// through the notify callee. A control bit from `arg1` is folded into the
/// handle flags, a second three-call chain runs, and three float constants
/// selected by the subject mode are pushed through three configure callees;
/// a scaled product is stored to the subject and the optional record is
/// offered to another callee. When the subject steering bits are both set,
/// its speed vector is measured: above the bound and with the go flag set,
/// a constant-context query plus a float-returning poll feed the final
/// steer callee with the subject geometry.
///
/// Original: 0x00cd3130 (thiscall, four stack words; no return value).
#[allow(clippy::all)]
#[allow(unsafe_code)]
unsafe fn run_00cd3130(
    this: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    mutate_ready: bool,
) -> u32 {
    unsafe {
        const LOOKUP_A_CALLEE: u32 = 1;
        const LOOKUP_B_CALLEE: u32 = 2;
        const ALLOC_CALLEE: u32 = 3;
        const BIT_CALLEE: u32 = 4;
        const APPLY_CALLEE: u32 = 5;
        const FINISH_CALLEE: u32 = 6;
        const REGISTER_CALLEE: u32 = 7;
        const CONFIG_CALLEE: u32 = 8;
        const SET_A_CALLEE: u32 = 9;
        const SET_B_CALLEE: u32 = 10;
        const SET_C_CALLEE: u32 = 11;
        const OFFER_CALLEE: u32 = 12;
        const QUERY_CALLEE: u32 = 13;
        const POLL_CALLEE: u32 = 14;
        const STEER_CALLEE: u32 = 15;
        const NOTIFY_CALLEE: u32 = 16;
        const VPROBE_CALLEE: u32 = 17;
        const VCHECK_CALLEE: u32 = 18;
        const VREADY_CALLEE: u32 = 19;
        const G_SHARED: u32 = 0x167e2a0;
        const G_1A: u32 = 0x1051934;
        const G_1B: u32 = 0x10518d4;
        const G_2A: u32 = 0x105193c;
        const G_2B: u32 = 0x10518dc;
        const G_3A: u32 = 0x1051938;
        const G_3B: u32 = 0x10518d8;
        const G_M1: u32 = 0xed3b54;
        const G_M2: u32 = 0xfe8a24;
        const G_DIST: u32 = 0xfe879c;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
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
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                tgt(obj)
            }
        }
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                tgt(obj, a0)
            }
        }

        let edi = this;
        let ebx = arg0;
        let c0 = rd32(ebx.wrapping_add(0x224));
        let mut esi: u32 =
            lf_checker_rt::callee_thiscall!(LOOKUP_A_CALLEE, u32, c0.wrapping_add(0x44), 1u32);
        if esi == 0 {
            let g = g32(G_SHARED);
            esi = lf_checker_rt::callee_thiscall!(ALLOC_CALLEE, u32, g);
            let mut e = 0u32;
            if esi != 0 {
                let b: u32 = lf_checker_rt::callee_thiscall!(BIT_CALLEE, u32, edi);
                let v: u32 =
                    lf_checker_rt::callee_thiscall!(APPLY_CALLEE, u32, edi, (b as u8) as u32);
                e = lf_checker_rt::callee_thiscall!(FINISH_CALLEE, u32, esi, v);
            }
            let c1 = rd32(ebx.wrapping_add(0x224));
            let _: u32 =
                lf_checker_rt::callee_thiscall!(REGISTER_CALLEE, u32, c1.wrapping_add(0x44), e, 1u32);
            let c2 = rd32(ebx.wrapping_add(0x224));
            esi = lf_checker_rt::callee_thiscall!(LOOKUP_B_CALLEE, u32, c2.wrapping_add(0x44), 1u32);
        }
        let p30 = vcall0(esi, 0x30);
        let _: u32 = vcall1(p30, 0x10, 1);
        let ready = vcall0(esi, 0x0c);
        // MUTANT (mut_00cd3130): the ready state never matches.
        let want = if mutate_ready { 0x1b1u32 } else { 0x1b0u32 };
        if ready != want {
            let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY_CALLEE, u32, ebx);
            return 0;
        }
        let ab = ((arg1 as u8).wrapping_shl(2) ^ rd8(esi.wrapping_add(0x68))) & 4;
        wr8(esi.wrapping_add(0x68), rd8(esi.wrapping_add(0x68)) ^ ab);
        let b: u32 = lf_checker_rt::callee_thiscall!(BIT_CALLEE, u32, edi);
        let v: u32 = lf_checker_rt::callee_thiscall!(APPLY_CALLEE, u32, edi, (b as u8) as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(CONFIG_CALLEE, u32, esi, v);
        let hi = rd8(edi.wrapping_add(0xf0)) & 0x80 != 0;
        let f = if hi { gf(G_1A) } else { gf(G_1B) };
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_A_CALLEE, u32, esi, f.to_bits());
        let f = if hi { gf(G_2A) } else { gf(G_2B) };
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_B_CALLEE, u32, esi, f.to_bits());
        let f = if hi { gf(G_3A) } else { gf(G_3B) };
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_C_CALLEE, u32, esi, f.to_bits());
        let x0 = mul(gf(G_M1), gf(G_M2));
        wr32(ebx.wrapping_add(0xaa8), x0.to_bits());
        if arg3 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(OFFER_CALLEE, u32, esi, arg3);
        }
        let al = rd8(edi.wrapping_add(0xf1));
        if al & 2 == 0 || al & 1 == 0 {
            return 0;
        }
        let dx = rdf(edi.wrapping_add(0x20));
        let dy = rdf(edi.wrapping_add(0x24));
        let dz = rdf(edi.wrapping_add(0x28));
        // Operand order is the original's: (dx*dx + dy*dy) + dz*dz.
        let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let d = core::hint::black_box(d2).sqrt();
        if !(d > gf(G_DIST)) {
            return 0;
        }
        if arg2 as u8 == 0 {
            return 0;
        }
        let p: u32 = lf_checker_rt::callee_thiscall!(
            QUERY_CALLEE,
            u32,
            0x171c968u32,
            rd32(edi.wrapping_add(0xd0))
        );
        let s =
            lf_checker_rt::callee_thiscall!(POLL_CALLEE, f64, p) as f32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            STEER_CALLEE,
            u32,
            esi,
            edi.wrapping_add(0x20),
            s.to_bits()
        );
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00cd3130(
    this: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe { run_00cd3130(this, arg0, arg1, arg2, arg3, false) }
});

lf_checker_rt::export!(thiscall, mut_00cd3130(
    this: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe { run_00cd3130(this, arg0, arg1, arg2, arg3, true) }
});
