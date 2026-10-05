// original: 0x0093B860 stream_heavy_update (proposed)

/// Fold a request into the streaming totals past four gates.
///
/// Needs the enable flag set, bit 0 of `(X ^ Y) & X`, class answer
/// `0x49` and a nonzero request; each failing gate returns the
/// call-site address (unchecked: this contract sets ret none). Then
/// divides `marker + count + request` by the count, keeping the
/// remainder as the new marker (the count is never zero here: a zero
/// divisor traps differently on each side), syncs, applies the
/// remainder to the fixed engine object, fetches table row 5, stamps
/// the four marker globals, truncates the fetched second word toward
/// zero exactly like the x87 store, and adds the live value into the
/// total global.
lf_checker_rt::export!(cdecl, rw_0093b860(request: u32) -> u32 {
    unsafe {
        const CLASS: u32 = 1;
        const COUNT: u32 = 2;
        const SYNC: u32 = 3;
        const APPLY: u32 = 4;
        const FETCH: u32 = 5;
        const LIVE: u32 = 6;
        const ENABLE: u32 = 0x11609F6;
        const MIX_A: u32 = 0x18B7A88;
        const MIX_B: u32 = 0x18B7A84;
        const MARKER: u32 = 0x11A4EFC;
        const ENGINE: u32 = 0x12845D0;
        const ROW: u32 = 5;
        const FLAG_A: u32 = 0x11A4F04;
        const FLAG_B: u32 = 0x1036EF0;
        const MODE: u32 = 0x11A4EF4;
        const FLAG_C: u32 = 0x1036EF8;
        const TOTAL: u32 = 0x11A4F00;
        let g = |va: u32| -> u32 {
            lf_checker_rt::global::<u32>(va).read_unaligned()
        };
        // NOTE: early outs return the call-site address, which no safe
        // rewrite can reproduce; the contract leaves ret unchecked.
        if lf_checker_rt::global::<u8>(ENABLE).read() == 0 {
            return 0;
        }
        if (g(MIX_A) ^ g(MIX_B)) & g(MIX_A) & 1 == 0 {
            return 0;
        }
        let cls: u32 = lf_checker_rt::callee_cdecl!(CLASS, u32, 0, 0xFFFF_FFFF);
        if cls != 0x49 {
            return 0;
        }
        if request == 0 {
            return 0;
        }
        let t: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32,);
        let n = g(MARKER).wrapping_add(t).wrapping_add(request);
        // t is never zero under this contract (division traps differ).
        let (_q, r) = (n / t, n % t);
        lf_checker_rt::global::<u32>(MARKER).write_unaligned(r);
        let _: u32 = lf_checker_rt::callee_cdecl!(SYNC, u32,);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            APPLY,
            u32,
            lf_checker_rt::relocated(ENGINE),
            r
        );
        let mut slot = [0u32; 2];
        let got: u32 = lf_checker_rt::callee_cdecl!(
            FETCH,
            u32,
            &mut slot as *mut u32 as u32,
            ROW
        );
        lf_checker_rt::global::<u8>(FLAG_A).write(0);
        lf_checker_rt::global::<u8>(FLAG_B).write(1);
        lf_checker_rt::global::<u32>(MODE).write_unaligned(4);
        lf_checker_rt::global::<u32>(FLAG_C).write_unaligned(0);
        let f = f32::from_bits(((got + 4) as *const u32).read_unaligned());
        const TWO63: f32 = 9223372036854775808.0;
        let whole = if f.is_nan() || f >= TWO63 || f < -TWO63 {
            0x8000_0000_0000_0000u64 as u32
        } else {
            f as i64 as u64 as u32
        };
        let live: u32 = lf_checker_rt::callee_cdecl!(LIVE, u32,);
        let sum = live.wrapping_add(whole);
        lf_checker_rt::global::<u32>(TOTAL).write_unaligned(sum);
        sum
    }
});
