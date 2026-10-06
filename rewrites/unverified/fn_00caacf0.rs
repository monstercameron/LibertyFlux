// original: 0x00CAACF0 ped_task_dispatch (proposed)

/// Dispatch a ped task update to the current target, then run the tail gate.
///
/// `a0` (null selects an immediate return), `a1` (may be null) and `a2` are
/// the three arguments (plain cdecl). The function keeps no state of its
/// own; everything it learns goes straight into calls. It returns nothing
/// meaningful (`eax` is incidental on every path), so the contract does not
/// compare it.
///
/// Behaviour: it first remembers `a1` in its own incoming argument slot
/// when `([a1 + 0x28] & 0x3c0) == 0xc0`, else zero (the rewrite keeps this
/// in a local; the value is observed later as the second argument of the
/// last call). Then it picks one of three blocks:
///
/// - When `a1` is live with `(([a1+0x28] >> 6) & 0xf) == 3`, the byte at
///   `a1 + 0x219` set and `a1 != a0`: it bumps a counter through callee 1's
///   answer (`[[ans + 0x228] + 0x54c]`, wrapping `+10`), probes callee 2
///   and reports through callee 3 with the inverted answer bit, then runs
///   callee 4.
/// - When that field equals 2 and callee 6 answers exactly `a1` (and the
///   byte at `a0 + 0x219` is clear): it runs the probe/report pair only
///   when a global word equals 2, then always emits three marker calls to
///   callee 5 (ids `0x101`, `0x1b0`, `0x10f`, each pushed with a 1.0 word
///   the callee does not pop).
/// - Otherwise it works from `ebp = [a0 + 0x1e4]` (null, or a timestamp
///   gap of `0x3e8` or more unsigned, falls into a plain two-word report
///   through callee 7): the same counter/probe/report/run sequence when
///   `([ebp+0x28] & 0x3c0) == 0xc0` with its `0x219` byte set, else the
///   three marker calls when the masked field equals `0x80`, callee 6
///   answers exactly `ebp` and `a0 + 0x219` is clear.
///
/// The tail (skipped when `a0 + 0x219` is clear) recomputes the timestamp
/// gap and the `0x80` mask into `al`, calls callee 8 with `0x31` or `a2`,
/// and unless the byte at `a0 + 0x218` is set calls callee 9 with that same
/// word and the remembered `a1`/zero. All timestamp compares are unsigned
/// (`jae`); the remaining compares are equalities, null checks and bit
/// tests.
///
/// Original: 0x00CAACF0 (cdecl, three stack words, caller cleans).
lf_checker_rt::export!(cdecl, rw_00CAACF0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const KIND_MASK: u32 = 0x3c0;
        const KIND_ACTIVE: u32 = 0xc0;
        const KIND_IDLE: u32 = 0x80;
        const STAMP_GAP: u32 = 0x3e8;
        const G_MODE: u32 = 0x011D6FD4;
        const G_STAMP: u32 = 0x011735B4;
        const MARKER_A: u32 = 0x101;
        const MARKER_B: u32 = 0x1b0;
        const MARKER_C: u32 = 0x10f;
        const TAIL_CODE: u32 = 0x31;
        const BUMP: u32 = 10;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        const C5: u32 = 5;
        const C6: u32 = 6;
        const C7: u32 = 7;
        const C8: u32 = 8;
        const C9: u32 = 9;

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
        unsafe fn g32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn bump_counter() {
            unsafe {
                let p: u32 = lf_checker_rt::callee_cdecl!(C1, u32, 0u32);
                let slot = rd32(p + 0x228) + 0x54c;
                wr32(slot, rd32(slot).wrapping_add(BUMP));
            }
        }
        #[inline(always)]
        unsafe fn probe_report(this: u32, a0: u32, ebx: u32, a1: u32) {
            unsafe {
                let ans: u32 = lf_checker_rt::callee_thiscall!(C2, u32, this, this);
                let cl = ((ans & 0xFF) == 0) as u32;
                lf_checker_rt::callee_cdecl!(C3, u32, a0, ebx, a1, cl);
            }
        }
        #[inline(always)]
        unsafe fn markers() {
            unsafe {
                // The original pushes a 1.0 word beside each id that the
                // callee leaves for the caller; only the id is compared.
                lf_checker_rt::callee_stdcall!(C5, u32, MARKER_A);
                lf_checker_rt::callee_stdcall!(C5, u32, MARKER_B);
                lf_checker_rt::callee_stdcall!(C5, u32, MARKER_C);
            }
        }
        /// The `[a0 + 0x1e4]` block shared by all three fallthrough paths.
        #[inline(always)]
        unsafe fn block_b(a0: u32, ebx: u32, a1: u32) {
            unsafe {
                let ebp = rd32(a0 + 0x1e4);
                if ebp == 0 {
                    lf_checker_rt::callee_cdecl!(C7, u32, a0, ebx);
                    return;
                }
                let gap = g32(G_STAMP).wrapping_sub(rd32(a0 + 0x1e8));
                if gap >= STAMP_GAP {
                    lf_checker_rt::callee_cdecl!(C7, u32, a0, ebx);
                } else if (rd32(ebp + 0x28) & KIND_MASK) == KIND_ACTIVE
                    && rd8(ebp + 0x219) != 0
                {
                    bump_counter();
                    probe_report(a1, a0, ebx, a1);
                    lf_checker_rt::callee_cdecl!(C4, u32,);
                } else if gap < STAMP_GAP && (rd32(ebp + 0x28) & KIND_MASK) == KIND_IDLE {
                    let q2: u32 = lf_checker_rt::callee_cdecl!(C6, u32, 0u32, 0u32);
                    if q2 == ebp && rd8(a0 + 0x219) == 0 {
                        markers();
                    } else if q2 != ebp {
                        lf_checker_rt::callee_cdecl!(C7, u32, a0, ebx);
                    }
                } else {
                    lf_checker_rt::callee_cdecl!(C7, u32, a0, ebx);
                }
            }
        }

        if a0 == 0 {
            return 0;
        }
        let mut ebx = a2;
        // Remembered a1 (the original smashes its own arg slot; the value
        // resurfaces as callee 9's second argument).
        let saved = if a1 != 0 && (rd32(a1 + 0x28) & KIND_MASK) == KIND_ACTIVE {
            a1
        } else {
            0
        };
        if a1 != 0 {
            let t = (rd32(a1 + 0x28) >> 6) & 0x0f;
            if t == 3 && rd8(a1 + 0x219) != 0 && a1 != a0 {
                bump_counter();
                probe_report(a1, a0, ebx, a1);
                lf_checker_rt::callee_cdecl!(C4, u32,);
            } else if t == 2 {
                let q: u32 = lf_checker_rt::callee_cdecl!(C6, u32, 0u32, 0u32);
                if q == a1 && rd8(a0 + 0x219) == 0 {
                    if g32(G_MODE) == 2 {
                        probe_report(a1, a0, ebx, a1);
                    }
                    markers();
                } else if q != a1 {
                    block_b(a0, ebx, a1);
                }
                // q == a1 with the byte set skips straight to the tail.
            } else {
                block_b(a0, ebx, a1);
            }
        } else {
            block_b(a0, ebx, a1);
        }

        if rd8(a0 + 0x219) == 0 {
            return 0;
        }
        let ebp = rd32(a0 + 0x1e4);
        let mut al = false;
        if ebp != 0 {
            let gap = g32(G_STAMP).wrapping_sub(rd32(a0 + 0x1e8));
            if gap < STAMP_GAP && (rd32(ebp + 0x28) & KIND_MASK) == KIND_IDLE {
                al = true;
            }
        }
        if al {
            ebx = TAIL_CODE;
        }
        lf_checker_rt::callee_cdecl!(C8, u32, ebx);
        if rd8(a0 + 0x218) != 0 {
            return 0;
        }
        if rd8(a0 + 0x219) == 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(C9, u32, ebx, saved);
        0
    }
});
