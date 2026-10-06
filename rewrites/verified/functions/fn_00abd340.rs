// original: 0x00ABD340 ui_slot_pair_setup (proposed)

/// Build two UI slot objects, tweak their tag words, and chain to the shared
/// slot registrar.
///
/// Arguments: none (cdecl/0, plain frame teardown). Ends in a direct jump
/// to the registrar, which also takes no arguments and returns nothing the
/// caller reads; the rewrite therefore forwards to the intercepted tail
/// stand-in and returns its answer. The export itself is intentionally a
/// frameless two-call sequence (body, then tail stand-in): the tail stub
/// discards its caller's return address, so it must be entered with no
/// live frame. The real work sits in the nested body below.
///
/// Algorithm. Allocate object one (cdecl helper, size `0x0c`) and run its
/// two-word constructor (thiscall, tag `2`) unless allocation failed, in
/// which case the null object faults on its first virtual call, exactly as
/// the original does. Read the virtual slot at `+8` twice through the
/// object's table; each answer is reduced modulo 16 by the signed
/// remainder idiom (`and 0x8000000f` with sign fix-up, i.e. `as i32 % 16`),
/// combined as `((second + ((16 - first) % 16)) / 16) << 14` (signed
/// division rounding toward zero), xored with the tag word at
/// `+4`, masked to bits `0x01FFC000` and xored back into the tag word.
/// Then run three fixed singleton helpers (two thiscalls on a constant
/// object address, one bare call), allocate object two (size `0x10`) and,
/// unless that failed, stamp its real table pointer and fields
/// (`0xAC6EF0`, `1.0f`) while folding the global counter at `0x10327A0`
/// into its tag word and incrementing the counter. Repeat the double
/// virtual call and tag tweak on object two (through the real table, whose
/// slot the contract plants with the stand-in), run the finaliser (cdecl:
/// constant `0xAC7050`, pointer to a zero scratch word), and tail-chain.
///
/// Edge cases: a null allocation on the first object faults both sides
/// identically at the first virtual load; the second object's init is
/// skipped on null but its virtual calls are unconditional, matching the
/// original's shape. Negative virtual answers exercise the signed
/// division, which rounds toward zero for negative sums; an unsigned
/// `/ 16` differs there and is the proof's wrong version. The `% 16`
/// reductions are also signed, but their signedness provably cancels:
/// each remainder feeds only `(16 - r) % 16`, which equals `(16 - (r&15))`
/// `& 15` for every input, so flipping it is behaviorally silent (both
/// variants compiled to identical code); it is not mutated.
///
/// Original: 0x00ABD340 (cdecl, no stack arguments; terminal jump).
lf_checker_rt::export!(cdecl, rw_00abd340() -> u32 {
    // Never inline the body into the export: the export must stay a
    // frameless call-call sequence so the tail stub's return discipline
    // (discard return address, the callee pops 0 bytes) lands back at the trampoline.
    #[inline(never)]
    unsafe fn body() {
        unsafe {
            const ALLOC: u32 = 1;
            const CTOR: u32 = 2;
            const VSLOT: u32 = 3;
            const SING_A: u32 = 4;
            const SING_B: u32 = 5;
            const BARE: u32 = 6;
            const FINAL: u32 = 7;
            const SINGLETON: u32 = 0x01510B10;
            const VTMP: u32 = 0x00E7E048;
            const VREAL: u32 = 0x00EA5B5C;
            const FIELD8: u32 = 0x00AC6EF0;
            const FINALIST: u32 = 0x00AC7050;
            const COUNTER: u32 = 0x010327A0;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn wr32(a: u32, v: u32) {
                unsafe { (a as *mut u32).write_unaligned(v) }
            }
            #[inline(always)]
            fn mod16(v: u32) -> u32 {
                (v as i32 % 16) as u32
            }
            #[inline(always)]
            fn div16_shl14(v: u32) -> u32 {
                (v as i32 / 16).wrapping_shl(14) as u32
            }

            // Object one: allocate, construct, double virtual call, tag tweak.
            let o1: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, 0x0Cu32, 0u32);
            let edi: u32 = if o1 == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(CTOR, u32, o1, 2u32)
            };
            let vt = rd32(edi);
            let tgt = rd32(vt.wrapping_add(8));
            let gate: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
            let a1 = gate(edi);
            let r1 = mod16(a1);
            let ebx = 0x10u32;
            let r2 = mod16(ebx.wrapping_sub(r1));
            let a2 = gate(edi);
            let q = div16_shl14(a2.wrapping_add(r2));
            let e4 = rd32(edi.wrapping_add(4));
            wr32(edi.wrapping_add(4), e4 ^ ((q ^ e4) & 0x01FF_C000));

            // Singleton helpers and the bare call.
            let c0 = lf_checker_rt::relocated(SINGLETON);
            let _: u32 = lf_checker_rt::callee_thiscall!(SING_A, u32, c0);
            let _: u32 = lf_checker_rt::callee_thiscall!(SING_B, u32, c0, 0u32, 0u32);
            let _: u32 = lf_checker_rt::callee_cdecl!(BARE, u32,);

            // Object two: allocate, stamp, double virtual call, tag tweak.
            let o2: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, 0x10u32, 0u32);
            let esi = o2;
            if esi != 0 {
                let mut c = rd32(esi.wrapping_add(4));
                wr32(esi, lf_checker_rt::relocated(VTMP));
                c ^= lf_checker_rt::global::<u32>(COUNTER).read_unaligned();
                c &= 0x3FFF;
                wr32(esi.wrapping_add(4), rd32(esi.wrapping_add(4)) ^ c);
                let ctr = lf_checker_rt::global::<u32>(COUNTER);
                ctr.write_unaligned(ctr.read_unaligned().wrapping_add(1));
                wr32(esi, lf_checker_rt::relocated(VREAL));
                wr32(esi.wrapping_add(8), lf_checker_rt::relocated(FIELD8));
                wr32(esi.wrapping_add(0x0C), 0x3F80_0000);
            }
            let vt2 = rd32(esi);
            let tgt2 = rd32(vt2.wrapping_add(8));
            let gate2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt2 as usize);
            let a3 = gate2(esi);
            let r3 = mod16(a3);
            let r4 = mod16(ebx.wrapping_sub(r3));
            let a4 = gate2(esi);
            let q2 = div16_shl14(a4.wrapping_add(r4));
            let s4 = rd32(esi.wrapping_add(4));
            wr32(esi.wrapping_add(4), s4 ^ ((q2 ^ s4) & 0x01FF_C000));

            // Finaliser over a zero scratch word.
            let mut slot = 0u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                FINAL,
                u32,
                lf_checker_rt::relocated(FINALIST),
                &mut slot as *mut u32 as u32
            );
        }
    }
    unsafe { body() };
    // Real call, in non-tail position on purpose (the compare below blocks
    // the tail-call optimisation): the tail stand-in discards exactly one
    // stack word (this call's return address) and returns to the
    // trampoline, so this export must be frameless at the call. The magic
    // below never occurs (it is not in the tail script), so this returns
    // the stand-in's answer unchanged. Verified by disassembling this
    // export (call/call/compare shape, no pushes); re-check after any
    // toolchain change.
    let r = lf_checker_rt::callee_cdecl!(8, u32,);
    if r == 0xDEAD_BEEF { 0 } else { r }
});
