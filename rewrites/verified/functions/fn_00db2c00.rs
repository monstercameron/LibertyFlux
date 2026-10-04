// original: 0x00db2c00 UITexture::vf87 (merged symbol)

/// Rebuild one UI slot's derived values: optionally refresh it through the
/// sibling update routine, then recompute its four bounds and submit them.
///
/// `this` is the texture object. The active slot comes from the index global
/// (times 68 bytes): a cleared enable byte at slot `+0x130` or a null anchor
/// at `+0x1d4` ends the call with no work. Otherwise, when the slot's dirty
/// word at `+0x128` is set, the sibling update routine runs with the slot's
/// flag byte (`+0x12f`, as 0/1); if the object's state byte at `+0x231` came
/// back changed, a formatted value is stored at slot `+0x120`. A nonzero word
/// at slot `+0x124` is then handed to the use callee with the anchor.
///
/// Four bounds are computed into a frame array, preset to
/// (+1e6, -1e6, -1e6, +1e6): the byte at slot `+0x12e` picks the direct form
/// (each bound is a slot float from `+0xf0`/`+0xf4`/`+0xf8`/`+0xfc` scaled by
/// one of two dimension globals, chosen per bound by a polled bit) or the
/// halved form (the same with the `+0xf8`/`+0xfc` floats first halved by the
/// half global). Two polled picks (for ids 8 and 7) and the slot bytes at
/// `+0x12c`/`+0x12d` (as 0/1) go to the apply callee; the anchor decides
/// between the three-pointer submit (slot words at `+0x120` and `+0x110` plus
/// the bounds array) and the two-pointer submit (slot word plus bounds);
/// after a flush call the two picks are applied again and the enable byte is
/// cleared.
///
/// Edge cases: the sibling routine reports the state change by writing
/// through the object pointer; the bounds array and the format cell travel as
/// frame addresses, so they are snapshotted, not compared, as call arguments.
/// Early exits leave the original's return register untouched (the rewrite
/// returns 0 there; the return value is unchecked).
///
/// Original: 0x00db2c00 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00db2c00(this: u32) -> u32 {
    unsafe {
        const ANCHOR: u32 = 0x1d4;
        const STATE: u32 = 0x231;
        const SLOT_STRIDE: u32 = 68;
        const SIBLING: u32 = 1;
        const FORMAT: u32 = 2;
        const USE: u32 = 3;
        const POLL: u32 = 4;
        const PICK: u32 = 5;
        const APPLY: u32 = 6;
        const TOUCH: u32 = 7;
        const SUBMIT3: u32 = 8;
        const SUBMIT2: u32 = 9;
        const FLUSH: u32 = 10;
        const INDEX_GLOBAL: u32 = 0x017a65ac;
        const DIM_A0: u32 = 0x0105c880;
        const DIM_A1: u32 = 0x0105c87c;
        const DIM_B0: u32 = 0x0105c884;
        const DIM_B1: u32 = 0x0105c888;
        const HALF_GLOBAL: u32 = 0x00fe8830;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        #[inline(always)]
        unsafe fn dim(va: u32) -> f32 {
            unsafe { (rd32(lf_checker_rt::relocated(va)) as i32) as f32 }
        }

        if rd32(this + ANCHOR) == 0 {
            return 0;
        }
        let idx = rd32(lf_checker_rt::relocated(INDEX_GLOBAL));
        let base = this.wrapping_add(idx.wrapping_mul(17).wrapping_mul(4));
        if rd8(base + 0x130) == 0 {
            return 0;
        }
        if rd32(base + 0x128) != 0 {
            let flag = if rd8(base + 0x12f) != 0 { 1u32 } else { 0u32 };
            let before = rd8(this + STATE);
            lf_checker_rt::callee_thiscall!(SIBLING, u32, this, flag);
            if before != rd8(this + STATE) {
                let mut cell: u32 = 0;
                lf_checker_rt::callee_cdecl!(FORMAT, u32, &mut cell as *mut u32 as u32, 0x42);
                wr32(base + 0x120, (cell & 0xffffff) | 0x9b000000);
            }
        }
        let word = rd32(base + 0x124);
        if word != 0 {
            lf_checker_rt::callee_thiscall!(USE, u32, rd32(this + ANCHOR), word);
        }
        let mut arr = [
            f32::from_bits(0x49742400),
            f32::from_bits(0xc9742400),
            f32::from_bits(0xc9742400),
            f32::from_bits(0x49742400),
        ];
        let f0 = rdf(base + 0xf0);
        let f4 = rdf(base + 0xf4);
        let f8 = rdf(base + 0xf8);
        let fc = rdf(base + 0xfc);
        if rd8(base + 0x12e) != 0 {
            let a0: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c0 = if a0 as u8 != 0 { dim(DIM_B1) } else { dim(DIM_B0) };
            arr[0] = mul(c0, f0);
            let a1: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c1 = if a1 as u8 != 0 { dim(DIM_B1) } else { dim(DIM_B0) };
            arr[2] = mul(add(f8, f0), c1);
            let a2: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c2 = if a2 as u8 != 0 { dim(DIM_A1) } else { dim(DIM_A0) };
            arr[1] = mul(c2, f4);
            let a3: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c3 = if a3 as u8 != 0 { dim(DIM_A1) } else { dim(DIM_A0) };
            arr[3] = mul(add(fc, f4), c3);
        } else {
            let half = f32::from_bits(rd32(lf_checker_rt::relocated(HALF_GLOBAL)));
            let a0: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c0 = if a0 as u8 != 0 { dim(DIM_B1) } else { dim(DIM_B0) };
            arr[0] = mul(sub(f0, mul(f8, half)), c0);
            let a1: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c1 = if a1 as u8 != 0 { dim(DIM_B1) } else { dim(DIM_B0) };
            arr[2] = mul(add(mul(f8, half), f0), c1);
            let a2: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c2 = if a2 as u8 != 0 { dim(DIM_A1) } else { dim(DIM_A0) };
            arr[1] = mul(sub(f4, mul(fc, half)), c2);
            let a3: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
            let c3 = if a3 as u8 != 0 { dim(DIM_A1) } else { dim(DIM_A0) };
            arr[3] = mul(add(mul(fc, half), f4), c3);
        }
        let r0: u32 = lf_checker_rt::callee_cdecl!(PICK, u32, 8);
        let bl = if r0 != 0 { 1u32 } else { 0u32 };
        let r1: u32 = lf_checker_rt::callee_cdecl!(PICK, u32, 7);
        let bh = if r1 != 0 { 1u32 } else { 0u32 };
        let b12c = if rd8(base + 0x12c) != 0 { 1u32 } else { 0u32 };
        lf_checker_rt::callee_cdecl!(APPLY, u32, 8, b12c);
        let b12d = if rd8(base + 0x12d) != 0 { 1u32 } else { 0u32 };
        lf_checker_rt::callee_cdecl!(APPLY, u32, 7, b12d);
        let anchor = rd32(this + ANCHOR);
        if rd32(anchor) != 0 {
            lf_checker_rt::callee_thiscall!(TOUCH, u32, anchor);
            lf_checker_rt::callee_cdecl!(
                SUBMIT3,
                u32,
                arr.as_mut_ptr() as u32,
                base.wrapping_add(0x110),
                base.wrapping_add(0x120)
            );
        } else {
            lf_checker_rt::callee_thiscall!(TOUCH, u32, anchor);
            lf_checker_rt::callee_cdecl!(
                SUBMIT2,
                u32,
                arr.as_mut_ptr() as u32,
                base.wrapping_add(0x120)
            );
        }
        lf_checker_rt::callee_cdecl!(FLUSH, u32,);
        lf_checker_rt::callee_cdecl!(APPLY, u32, 8, bl);
        let ret: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, 7, bh);
        wr8(base + 0x130, 0);
        ret
    }
});

