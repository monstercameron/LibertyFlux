// original: 0x00cd9860 randomised_timer_select (proposed)

/// Randomised timer-gated selection (thiscall, one stack word, no return value).
///
/// `this` is the record: byte `+0x28` arms it (zero returns at once), byte
/// `+0x29` refreshes word `+0x20` from the image word when set (then clears
/// itself), and words `+0x24`/`+0x20` form an expiry sum that must not exceed
/// the image word (signed, wrapping), else it returns. `+0x18` holds a
/// fallback word. `p0` is a subject record, only ever addressed (`+0xBB0`
/// for the gate request, which also overwrites the incoming argument slot —
/// dead after the callee-popped return, so the rewrite leaves it and the
/// proof runs with the stack check off).
///
/// The gate request on `p0+0xBB0` must answer zero. Then five \$RAND calls
/// (each masked to 16 bits, converted exactly to float, multiplied by
/// 2^-15 and a second image constant in that order, truncated toward zero —
/// all results stay far inside i32 range and finite, so a plain conversion
/// matches) drive the selection: the first, scaled by 100, must stay below
/// 75; the subject is probed; the second, also scaled by 100, selects
/// either the fallback word (below 80) or a counted pick (a count request
/// on probe+8, decremented, through a two-word combine, then a select
/// request whose answer equal to `p0` falls back to the fallback word). A
/// null pick takes the slow path. Otherwise a two-word combine of half and
/// whole of `0x1F40` (pick equal to the fallback) or `0xBB8` sets the
/// accumulator; the third and fourth \$RAND results, scaled by -750 and
/// -300, set two budgets of 500 minus the scaled value, whose minimum and
/// first value join a ten-word issue request (a literal 1 first, then the
/// budgets, 0, 0, 0x4B5, accumulator, pick, 0, 0xEDADE4), with the gate
/// address read back from the overwritten argument slot as its object.
/// Only after the issue request, an accumulator of -1 takes the slow path
/// instead of committing.
///
/// The slow path scales the fifth \$RAND result by -7000 into an accumulator
/// of 1000 minus it. Committing writes the image word to `+0x20`, the
/// accumulator to `+0x24` and 1 to `+0x28`.
///
/// Original: 0x00cd9860 (thiscall, one stack word, void).
lf_checker_rt::export!(thiscall, rw_00cd9860(this: u32, p0: u32) -> u32 {
    unsafe {
        const ARM_OFF: u32 = 0x28;
        const REFRESH_OFF: u32 = 0x29;
        const STAMP_OFF: u32 = 0x20;
        const ACC_OFF: u32 = 0x24;
        const FALLBACK_OFF: u32 = 0x18;
        const GATE_DISP: u32 = 0xbb0;
        const IMAGE_WORD: u32 = 0x011735b4;
        const INV_2P15: u32 = 0x00fe8680;
        const SCALE_FAST: u32 = 0x00fe8bb0;
        const SCALE_B1: u32 = 0x00fe8dfc;
        const SCALE_B2: u32 = 0x00ed7ca8;
        const SCALE_SLOW: u32 = 0x00edafe0;
        const V1_THRESH: i32 = 0x4b;
        const V2_THRESH: i32 = 0x50;
        const CALLEE_GATE: u32 = 1;
        const CALLEE_R1: u32 = 2;
        const CALLEE_PROBE: u32 = 3;
        const CALLEE_R2: u32 = 4;
        const CALLEE_COUNT: u32 = 5;
        const CALLEE_PICK: u32 = 6;
        const CALLEE_SELECT: u32 = 7;
        const CALLEE_COMBINE: u32 = 8;
        const CALLEE_R3: u32 = 9;
        const CALLEE_R4: u32 = 10;
        const CALLEE_ISSUE: u32 = 11;
        const CALLEE_R5: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// One scaled \$RAND draw: low 16 bits, exactly converted, times
        /// 2^-15 then the given constant, truncated toward zero.
        #[inline(always)]
        unsafe fn draw(id: u32, scale_va: u32) -> i32 {
            unsafe {
                let r: u32 = lf_checker_rt::callee_cdecl!(id, u32,);
                let x = ((r & 0xffff) as f32);
                let k1 = f32::from_bits(lf_checker_rt::global::<u32>(INV_2P15).read());
                let k2 = f32::from_bits(lf_checker_rt::global::<u32>(scale_va).read());
                mul(mul(x, k1), k2) as i32
            }
        }

        if ((this + ARM_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let image = lf_checker_rt::global::<u32>(IMAGE_WORD).read();
        if ((this + REFRESH_OFF) as *const u8).read() != 0 {
            wr32(this + STAMP_OFF, image);
            ((this + REFRESH_OFF) as *mut u8).write(0);
        }
        let sum = rd32(this + ACC_OFF).wrapping_add(rd32(this + STAMP_OFF));
        // setle after cmp: signed less-or-equal.
        if (sum as i32) > (image as i32) {
            return 0;
        }
        let gate_obj = p0.wrapping_add(GATE_DISP);
        let g0: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GATE, u32, gate_obj);
        if g0 != 0 {
            return 0;
        }
        let v1 = draw(CALLEE_R1, SCALE_FAST);
        let mut acc: u32;
        if v1 < V1_THRESH {
            let t: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, p0);
            let v2 = draw(CALLEE_R2, SCALE_FAST);
            let fallback = rd32(this + FALLBACK_OFF);
            let pick = if v2 < V2_THRESH {
                fallback
            } else {
                let base = t.wrapping_add(8);
                let n: u32 = lf_checker_rt::callee_thiscall!(CALLEE_COUNT, u32, base);
                let c: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PICK, u32, 0u32, n.wrapping_sub(1));
                let s: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SELECT, u32, base, c);
                if s == p0 {
                    fallback
                } else {
                    s
                }
            };
            if pick == 0 {
                let v5 = draw(CALLEE_R5, SCALE_SLOW);
                acc = (0x3e8i32 - v5) as u32;
            } else {
                let whole = if pick == fallback { 0x1f40u32 } else { 0x0bb8u32 };
                let half = (whole as i32 / 2) as u32;
                let c: u32 = lf_checker_rt::callee_cdecl!(CALLEE_COMBINE, u32, half, whole);
                let v3 = draw(CALLEE_R3, SCALE_B1);
                let b1 = 0x1f4i32 - v3;
                let v4 = draw(CALLEE_R4, SCALE_B2);
                let b2 = 0x1f4i32 - v4;
                let lo = if b2 < b1 { b2 } else { b1 };
                // Ten words: a literal 1 is pushed first, then the budgets,
                // zeros, 0x4b5, accumulator, pick, zero, 0xedade4. The issue
                // object is read back from the overwritten argument slot,
                // which still holds the gate address from the function entry.
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_ISSUE, u32, gate_obj, lf_checker_rt::relocated(0xedade4), 0u32,
                    pick, c, 0x4b5u32, 0u32, 0u32, b1 as u32, lo as u32, 1u32
                );
                // The -1 check runs after the issue request, not before it.
                if c == 0xffff_ffff {
                    let v5 = draw(CALLEE_R5, SCALE_SLOW);
                    acc = (0x3e8i32 - v5) as u32;
                } else {
                    acc = c;
                }
            }
        } else {
            let v5 = draw(CALLEE_R5, SCALE_SLOW);
            acc = (0x3e8i32 - v5) as u32;
        }
        let image = lf_checker_rt::global::<u32>(IMAGE_WORD).read();
        wr32(this + STAMP_OFF, image);
        wr32(this + ACC_OFF, acc);
        ((this + ARM_OFF) as *mut u8).write(1);
        0
    }
});
