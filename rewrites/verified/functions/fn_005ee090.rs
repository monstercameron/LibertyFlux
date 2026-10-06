// original: 0x005ee090 text_emit_format_runs (proposed)

/// Emit up to four formatted runs for a text box through a vertex sink.
///
/// `box_ptr` points to a style record; `f1`..`f4` are four edge floats passed
/// by value and `count` a caller count. The function first runs three setup
/// callees: callee 1 (thiscall on a relocated context, three words: a
/// relocated address and two globals, the first of which is also stored to a
/// global slot), callee 2 (thiscall on a global context, one relocated word)
/// and callee 3 (cdecl, `(1, count*2)` with wrapping doubling).
///
/// It then checks three kind words of the record against 0x14 (exact integer
/// compares): `+0x80`, `+0x74`, `+0x5c`, and `+0x80` again. Each match stores
/// a sibling word to a shared global slot (`+0x7c`, `+0x70`, `+0x58`, `+0x64`
/// in order) and emits two records through callee 4, which takes no stack
/// arguments and reads its two float inputs from XMM0/XMM1: `(f3,f1)`,
/// `(f4,f1)`, `(f4,f1)`, `(f4,f2)`, `(f4,f2)`, `(f3,f2)`, `(f3,f2)`, `(f3,f1)`
/// across the eight sites. Callee 4's answers are never read.
///
/// Finally, when a global flag is non-zero, callee 5 runs with no arguments
/// and the flag is cleared, and a second global flag is always cleared. The
/// returned EAX is the last callee answer (scripted on both sides).
///
/// Original: 0x005ee090 (cdecl, six stack words).
lf_checker_rt::export!(cdecl, rw_005ee090(box_ptr: u32, f1: u32, f2: u32, f3: u32, f4: u32, count: u32) -> u32 {
    unsafe {
        const KIND_WANT: u32 = 0x14;
        const KIND0: u32 = 0x80;
        const KIND1: u32 = 0x74;
        const KIND2: u32 = 0x5c;
        const VAL0: u32 = 0x7c;
        const VAL1: u32 = 0x70;
        const VAL2: u32 = 0x58;
        const VAL3: u32 = 0x64;
        const G_A: u32 = 0x017ed954;
        const G_B: u32 = 0x017ed8ec;
        const G_CTXT: u32 = 0x017f583c;
        const G_CTX1: u32 = 0x018dd510;
        const G_W1: u32 = 0x018dd540;
        const G_W2: u32 = 0x01110090;
        const G_SLOT: u32 = 0x01b4bc5c;
        const G_SHARED: u32 = 0x0110dfb0;
        const G_FLAG: u32 = 0x01b4f724;
        const G_FLAG2: u32 = 0x01b4f71c;
        const CALLEE_SETUP1: u32 = 1;
        const CALLEE_SETUP2: u32 = 2;
        const CALLEE_SETUP3: u32 = 3;
        const CALLEE_SINK: u32 = 4;
        const CALLEE_FLUSH: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gwr(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }

        let ga = g32(G_A);
        gwr(G_SLOT, ga);
        lf_checker_rt::callee_thiscall!(
            CALLEE_SETUP1, u32,
            lf_checker_rt::relocated(G_CTX1),
            lf_checker_rt::relocated(G_W1),
            g32(G_B),
            ga
        );
        lf_checker_rt::callee_thiscall!(
            CALLEE_SETUP2, u32,
            g32(G_CTXT),
            lf_checker_rt::relocated(G_W2)
        );
        let doubled = count.wrapping_add(count);
        let mut last: u32 = lf_checker_rt::callee_cdecl!(CALLEE_SETUP3, u32, 1, doubled);
        let emit = |xmm0: u32, xmm1: u32| -> u32 {
            lf_checker_rt::callee_cdecl!(CALLEE_SINK, u32, xmm0, xmm1)
        };
        if rd32(box_ptr.wrapping_add(KIND0)) == KIND_WANT {
            gwr(G_SHARED, rd32(box_ptr.wrapping_add(VAL0)));
            last = emit(f3, f1);
            last = emit(f4, f1);
        }
        if rd32(box_ptr.wrapping_add(KIND1)) == KIND_WANT {
            gwr(G_SHARED, rd32(box_ptr.wrapping_add(VAL1)));
            last = emit(f4, f1);
            last = emit(f4, f2);
        }
        if rd32(box_ptr.wrapping_add(KIND2)) == KIND_WANT {
            gwr(G_SHARED, rd32(box_ptr.wrapping_add(VAL2)));
            last = emit(f4, f2);
            last = emit(f3, f2);
        }
        // Either way XMM0 holds f3 for the last pair.
        if rd32(box_ptr.wrapping_add(KIND0)) == KIND_WANT {
            gwr(G_SHARED, rd32(box_ptr.wrapping_add(VAL3)));
            last = emit(f3, f2);
            last = emit(f3, f1);
        }
        if g32(G_FLAG) != 0 {
            last = lf_checker_rt::callee_cdecl!(CALLEE_FLUSH, u32,);
            gwr(G_FLAG, 0);
        }
        gwr(G_FLAG2, 0);
        last
    }
});
