// original: 0x00B39840 ped_task_slot_probe (proposed)

/// Probe one task slot: either take the next table entry and test it, or run
/// a randomised region probe.
///
/// `out16` receives 16 bytes (a table entry on the table path, probe output
/// on the random path) and `out8` one byte; `cell` points at four floats;
/// `f3`..`f7` are float bits forwarded to the callees.
///
/// Behaviour: a mode flag global selects the path. On the table path the
/// counter global must satisfy `0 <= counter < limit` (signed); the entry
/// `table + counter*16` (a word, two floats, a word) is copied to `out16`
/// bitwise, the auxiliary byte `auxtable[counter]` is stored to `out8`, and
/// the counter is incremented (even when the check fails, in which case the
/// function returns 0 without calling). Otherwise the range callee is
/// invoked stdcall-style with (`out16`, the indexed word
/// `idxtable[[idx]*4]`, `f5`, `f6`, `f3`, `f4`, 2.0) and the function returns
/// whether its low byte was nonzero. On the random path a scripted random
/// word is reduced to 16 bits, converted to float, multiplied by 2^-11 then
/// by -7.0 (in that order), truncated toward zero with cvttss2si semantics
/// (NaN and out-of-range yield 0x80000000), and subtracted from 1 to form a
/// count; the region callee is invoked cdecl-style with (`cell`, `f3`..`f7`,
/// count, `out16`) and the function returns whether its low byte was
/// nonzero.
///
/// Original: 0x00B39840 (cdecl, eight stack words, returns the flag in al).
lf_checker_rt::export!(cdecl, rw_00B39840(out16: u32, out8: u32, cell: u32, f3: u32, f4: u32, f5: u32, f6: u32, f7: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x1045937;
        const TABLE: u32 = 0x1662540;
        const AUXTABLE: u32 = 0x1662640;
        const COUNTER: u32 = 0x1662650;
        const LIMIT: u32 = 0x1662654;
        const IDXTABLE: u32 = 0x118D818;
        const SCALE_A: u32 = 0xFE8680; // 2^-11, measured from the file
        const SCALE_B: u32 = 0xEA99C0; // -7.0, measured from the file
        const ENTRY_STRIDE: u32 = 16;
        const TWO_BITS: u32 = 0x4000_0000;
        const RANGE_CALLEE: u32 = 1;
        const RAND_CALLEE: u32 = 2;
        const REGION_CALLEE: u32 = 3;

        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate toward zero with cvttss2si semantics: NaN, infinities
        /// and out-of-range values yield 0x80000000, not a saturation.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let flag: u8 = ((lf_checker_rt::relocated(FLAG)) as *const u8).read();
        if flag != 0 {
            let counter = lf_checker_rt::global::<u32>(COUNTER).read() as i32;
            let limit = lf_checker_rt::global::<u32>(LIMIT).read() as i32;
            let mut ok = false;
            if counter >= 0 && counter < limit {
                let base = lf_checker_rt::relocated(TABLE)
                    .wrapping_add((counter as u32).wrapping_mul(ENTRY_STRIDE));
                for w in 0..4u32 {
                    ((out16.wrapping_add(w * 4)) as *mut u32)
                        .write_unaligned(rd32(base.wrapping_add(w * 4)));
                }
                let aux: u8 = ((lf_checker_rt::relocated(AUXTABLE)
                    .wrapping_add(counter as u32)) as *const u8)
                    .read();
                ((out8) as *mut u8).write(aux);
                ok = true;
            }
            lf_checker_rt::global::<u32>(COUNTER)
                .write((counter as u32).wrapping_add(1));
            if !ok {
                return 0;
            }
            let idx = lf_checker_rt::global::<u32>(IDXTABLE).read();
            let tabval = rd32(lf_checker_rt::relocated(IDXTABLE)
                .wrapping_add(idx.wrapping_mul(4)));
            let r: u32 = lf_checker_rt::callee_stdcall!(
                RANGE_CALLEE, u32, out16, tabval, f5, f6, f3, f4, TWO_BITS
            );
            return ((r as u8) != 0) as u32;
        }
        let rand: u32 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
        let scaled = fmul(
            fmul(
                ((rand & 0xffff) as f32),
                f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_A))),
            ),
            f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_B))),
        );
        let count = 1u32.wrapping_sub(cvtt(scaled) as u32);
        let r: u32 = lf_checker_rt::callee_cdecl!(
            REGION_CALLEE, u32, cell, f3, f4, f5, f6, f7, count, out16
        );
        ((r as u8) != 0) as u32
    }
});
