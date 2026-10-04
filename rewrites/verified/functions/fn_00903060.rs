// original: 0x00903060 input_device_poll_scale (proposed)
//
// Poll an input device and push scaled values to the input applier.
//
// With no arguments: poke the device twice, fetch the device object, and
// return through the tail callee when there is none. Otherwise resolve four
// input codes through the shared input getter (three more when the selector
// global is non-zero), read the device's raw value through its virtual slot
// at `+0xFC`, and normalise it: clamp below by zero, divide by the span
// between the limit word at `+0xA84` (clamped below by the read-only constant
// `K`) and `K`, then scale by `K`. The level word at `+0xB84` (clamped below
// by `K`) and the globals block select up to eight applier calls carrying the
// resolved codes and the scaled values; each float gate is a strict
// greater-than-zero, so NaN takes the quiet path exactly like the original's
// branch-if-below-or-equal. Returns the tail callee's answer.
//
// Original: 0x00903060 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00903060() -> u32 {
    unsafe {
        const POKE1: u32 = 1;
        const POKE2: u32 = 2;
        const FETCH0: u32 = 3;
        const GETDEV: u32 = 4;
        const GETTER: u32 = 5;
        const APPLY: u32 = 6;
        const RELEASE: u32 = 7;
        const TAIL: u32 = 8;
        const VIRT: u32 = 9;
        const VT_SLOT: u32 = 0xFC;
        const SIGN_SRC: u32 = 0x0106_B310;
        const SELECTOR: u32 = 0x011D_6FD4;
        const LEVEL_ON: u32 = 0x0118_F4D0;
        const G0A: u32 = 0x0118_F4C8;
        const G1A: u32 = 0x0118_F4CC;
        const G2A: u32 = 0x0118_F4D4;
        const G3A: u32 = 0x0118_F4D8;
        const KADDR: u32 = 0x00FE_8BB0;
        const LIM_OFF: u32 = 0xA84;
        const LVL_OFF: u32 = 0xB84;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd(a)) }
        }
        #[inline(always)]
        unsafe fn code(id: u32, slot: &mut u32) -> u32 {
            unsafe {
                let ret: u32 = lf_checker_rt::callee_cdecl!(GETTER, u32, slot as *mut u32 as u32, id, 0xFF);
                rd(ret)
            }
        }

        let _: u32 = lf_checker_rt::callee_cdecl!(POKE1, u32, 0);
        let sign = rd(lf_checker_rt::global::<u32>(SIGN_SRC) as u32) as i32;
        let pick: u32 = if sign < 0 { sign as u32 } else { 0 };
        let _: u32 = lf_checker_rt::callee_cdecl!(POKE2, u32, 1, pick);
        let _: u32 = lf_checker_rt::callee_cdecl!(POKE2, u32, 0x0A, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(FETCH0, u32,);
        let dev: u32 = lf_checker_rt::callee_cdecl!(GETDEV, u32,);
        if dev == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32,);
            let tail: u32 = lf_checker_rt::callee_cdecl!(TAIL, u32,);
            return tail;
        }
        let mut scratch: u32 = 0;
        let edi: u32 = code(0x10, &mut scratch);
        let mut tmp1: u32 = code(0x07, &mut scratch);
        scratch = 0;
        let esi: u32 = code(0x04, &mut scratch);
        scratch = 0;
        let tmp2: u32 = code(0x13, &mut scratch);
        if rd(lf_checker_rt::global::<u32>(SELECTOR) as u32) != 0 {
            let edi2: u32 = code(0x17, &mut tmp1);
            let ans6: u32 = code(0x18, &mut tmp1);
            scratch = 0;
            let esi2: u32 = code(0x19, &mut scratch);
            return poll_apply_rest(dev, edi2, esi2, ans6, tmp2);
        }
        poll_apply_rest(dev, edi, esi, tmp1, tmp2)
    }
});

// Second half of 0x00903060: normalise the device value and run the applier
// calls. Split so the two code-resolution prefixes share it.
unsafe fn poll_apply_rest(dev: u32, edi: u32, esi: u32, l1: u32, tmp2: u32) -> u32 {
    unsafe {
        const APPLY: u32 = 6;
        const RELEASE: u32 = 7;
        const TAIL: u32 = 8;
        const VT_SLOT: u32 = 0xFC;
        const LEVEL_ON: u32 = 0x0118_F4D0;
        const G0A: u32 = 0x0118_F4C8;
        const G1A: u32 = 0x0118_F4CC;
        const G2A: u32 = 0x0118_F4D4;
        const G3A: u32 = 0x0118_F4D8;
        const KADDR: u32 = 0x00FE_8BB0;
        const LIM_OFF: u32 = 0xA84;
        const LVL_OFF: u32 = 0xB84;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd(a)) }
        }

        let slot_fn: u32 = rd(rd(dev) + VT_SLOT);
        let raw: f32 = {
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(slot_fn as usize);
            f(dev)
        };
        let k = rdf(lf_checker_rt::global::<u32>(KADDR) as u32);
        let mut norm = sub(raw, k);
        if 0.0 > norm {
            norm = 0.0;
        }
        let mut span = sub(rdf(dev + LIM_OFF), k);
        if k > span {
            span = k;
        }
        norm = div(norm, span);
        let lvl = rdf(dev + LVL_OFF);
        norm = mul(norm, k);
        let mut level = lvl;
        if !(k > lvl) {
            level = k;
        }
        let g0 = rdf(lf_checker_rt::global::<u32>(G0A) as u32);
        if (rd(lf_checker_rt::global::<u32>(LEVEL_ON) as u32) as i32) > 0 {
            if g0 > 0.0 {
                let v = add(g0, norm);
                let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 1, v.to_bits(), esi, 1, 0);
            }
            let g1 = rdf(lf_checker_rt::global::<u32>(G1A) as u32);
            if g1 > 0.0 {
                let v = add(g1, level);
                let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 0, v.to_bits(), esi, 1, 0);
            }
        }
        if level > 0.0 && g0 > 0.0 {
            let v = add(g0, norm);
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 1, v.to_bits(), esi, 0, 0);
        } else {
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 1, norm.to_bits(), edi, 0, 0);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 0, level.to_bits(), l1, 0, 0);
        if level > 0.0 && g0 > 0.0 {
            let v = add(g0, norm);
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 1, v.to_bits(), esi, 1, 0);
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 1, norm.to_bits(), edi, 1, 0);
        }
        let g2 = rdf(lf_checker_rt::global::<u32>(G2A) as u32);
        if g2 > 0.0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 1, g2.to_bits(), tmp2, 0, 1);
        }
        let g3 = rdf(lf_checker_rt::global::<u32>(G3A) as u32);
        if g3 > 0.0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 0, g3.to_bits(), tmp2, 0, 1);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32,);
        lf_checker_rt::callee_cdecl!(TAIL, u32,)
    }
}
