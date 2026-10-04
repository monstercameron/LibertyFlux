// original: 0x00e4ddf0 CRenderPhaseRadar::vf8

/// Radar render-phase pass: gate on global switches, build and emit a
/// sequence of phase objects, then tail to the next phase.
///
/// `this` (ecx) is ignored; no stack arguments. All control flow is driven
/// by callee answers and game globals; the final eax (the last callee
/// answer on the taken path) is returned.
///
/// Behaviour, main path: if gate A (callee 0, low byte) is false, return.
/// Otherwise allocate a 12-byte tagged node (callee 1), stamp it (vtable
/// pre/post markers, a key-mixed tag at `+4` from the counter global KEY,
/// slot pointer at `+8`), emit it (callee 2), then check the path switch:
/// when set, run the alternate path (below). The main path continues only
/// while the flag bytes permit, looks up an object (callee 3) whose flag
/// byte at `+0x1c4` must have neither of the low two bits set, optionally
/// consults gate C (callee 4), then emits a fixed chain of constructed
/// objects: a 16-byte node via ctor D (callee 5, zero slot), 12/20-byte
/// nodes via ctors E/F (callees 6/7, F taking the two float globals by
/// pointer), each emitted right after construction; null allocations emit
/// null. Two mode dwords (D1/D2) select sub-chains: a middle block (callees
/// 8/9, another F node, callees 10/11 under further switches, callee 12)
/// and a tail block (two more E nodes), after which the function tails to
/// callee 13. The alternate path emits two tagged nodes with different
/// slots around calls to callees 14/8/9/12/13/15 and returns the last emit
/// answer.
///
/// Edge cases: a null allocation skips its construction but still emits;
/// every gate combination exits early with the last answer so far; the two
/// float globals are only moved, never computed on.
lf_checker_rt::export!(thiscall, rw_00e4ddf0(_this: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x0103_27A0;
        const G_PATH_B: u32 = 0x0116_09F6;
        const G_RUN: u32 = 0x011E_61C6;
        const G_HOLD1: u32 = 0x011D_B23D;
        const G_HOLD2: u32 = 0x0118_DC40;
        const G_GATE: u32 = 0x0103_44D9;
        const G_EXTRA: u32 = 0x011D_B23E;
        const D_MODE0: u32 = 0x0116_0C8C;
        const D_MODE1: u32 = 0x0116_0C9C;
        const D_KIND: u32 = 0x011D_6FD4;
        const F_LO: u32 = 0x0103_44CC;
        const F_HI: u32 = 0x0103_44C8;
        const B_THIS: u32 = 0x0103_E498;
        const B_FLAG_OFF: u32 = 0x1C4;
        const VT_PRE: u32 = 0x00E7_E048;
        const VT_POST: u32 = 0x00E7_E080;
        const SLOT_MAIN: u32 = 0x00E4_E1B0;
        const SLOT_ALT: u32 = 0x00E4_E1F0;
        const SLOT_AUX: u32 = 0x0043_2BE0;
        const CODE_D: u32 = 0x0043_2B70;
        const CODE_E0: u32 = 0x0090_2E60;
        const CODE_E1: u32 = 0x0090_3060;
        const CODE_E2: u32 = 0x0043_2BE0;
        const CODE_E3: u32 = 0x0090_2F70;
        const CODE_F0: u32 = 0x0090_36C0;
        const CODE_F1: u32 = 0x0090_3750;
        const TAG_MASK: u32 = 0x3FFF;

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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(va) as *const u8).read() }
        }
        /// Stamp a fresh 12-byte node: pre-marker, key-mixed tag, bump the
        /// key counter, post-marker, slot pointer. Preserves eax.
        #[inline(always)]
        unsafe fn tag(p: u32, slot: u32) {
            unsafe {
                let c = rd32(p + 4);
                wr32(p, lf_checker_rt::relocated(VT_PRE));
                let m = (c ^ g32(KEY)) & TAG_MASK;
                wr32(p + 4, rd32(p + 4) ^ m);
                wr32(
                    lf_checker_rt::global::<u32>(KEY) as u32,
                    g32(KEY).wrapping_add(1),
                );
                wr32(p, lf_checker_rt::relocated(VT_POST));
                wr32(p + 8, lf_checker_rt::relocated(slot));
            }
        }
        #[inline(always)]
        unsafe fn emit(v: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(2u32, u32, v) }
        }
        #[inline(always)]
        unsafe fn new_obj(size: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(1u32, u32, size, 0) }
        }

        let mut last = lf_checker_rt::callee_cdecl!(0u32, u32,);
        if (last as u8) == 0 {
            return last;
        }
        let p1 = new_obj(0x0c);
        if p1 != 0 {
            tag(p1, SLOT_MAIN);
        }
        last = emit(p1);
        if g8(G_PATH_B) != 0 {
            last = lf_checker_rt::callee_cdecl!(14u32, u32,);
            last = lf_checker_rt::callee_cdecl!(8u32, u32,);
            last = lf_checker_rt::callee_cdecl!(9u32, u32,);
            last = lf_checker_rt::callee_cdecl!(12u32, u32, 1);
            let pb = new_obj(0x0c);
            if pb != 0 {
                tag(pb, SLOT_ALT);
            }
            last = emit(pb);
            last = lf_checker_rt::callee_cdecl!(13u32, u32,);
            last = lf_checker_rt::callee_cdecl!(15u32, u32,);
            let pc = new_obj(0x0c);
            if pc != 0 {
                tag(pc, SLOT_AUX);
            }
            last = emit(pc);
            return last;
        }
        if g8(G_RUN) == 0 {
            return last;
        }
        if g8(G_HOLD1) != 0 {
            return last;
        }
        if g8(G_HOLD2) != 0 {
            return last;
        }
        last = lf_checker_rt::callee_thiscall!(3u32, u32, lf_checker_rt::relocated(B_THIS));
        if last != 0 {
            // The flag byte is loaded into al, so a rejection exits with
            // the lookup pointer's low byte replaced by the flag value.
            let flag = rd8(last + B_FLAG_OFF);
            if flag & 3 != 0 {
                return (last & 0xFFFF_FF00) | flag as u32;
            }
        }
        if g8(G_GATE) != 0 {
            last = lf_checker_rt::callee_cdecl!(4u32, u32,);
            if (last as u8) == 0 {
                return last;
            }
        }
        let p2 = new_obj(0x10);
        if p2 != 0 {
            let slot = 0u32;
            last = lf_checker_rt::callee_thiscall!(
                5u32,
                u32,
                p2,
                lf_checker_rt::relocated(CODE_D),
                &slot as *const u32 as u32
            );
        }
        last = emit(if p2 != 0 { last } else { 0 });
        if g32(D_MODE0) != 0 || g32(D_MODE1) != 0 {
            let p3 = new_obj(0x0c);
            if p3 != 0 {
                last = lf_checker_rt::callee_thiscall!(6u32, u32, p3, lf_checker_rt::relocated(CODE_E0));
            }
            last = emit(if p3 != 0 { last } else { 0 });
            let f0 = g32(F_LO);
            let f1 = g32(F_HI);
            let p4 = new_obj(0x14);
            if p4 != 0 {
                last = lf_checker_rt::callee_thiscall!(
                    7u32,
                    u32,
                    p4,
                    lf_checker_rt::relocated(CODE_F0),
                    &f1 as *const u32 as u32,
                    &f0 as *const u32 as u32
                );
            }
            last = emit(if p4 != 0 { last } else { 0 });
            let p5 = new_obj(0x0c);
            if p5 != 0 {
                last = lf_checker_rt::callee_thiscall!(6u32, u32, p5, lf_checker_rt::relocated(CODE_E1));
            }
            last = emit(if p5 != 0 { last } else { 0 });
            let p6 = new_obj(0x14);
            if p6 != 0 {
                last = lf_checker_rt::callee_thiscall!(
                    7u32,
                    u32,
                    p6,
                    lf_checker_rt::relocated(CODE_F1),
                    &f1 as *const u32 as u32,
                    &f0 as *const u32 as u32
                );
            }
            last = emit(if p6 != 0 { last } else { 0 });
            if g32(D_MODE1) != 0 {
                last = lf_checker_rt::callee_cdecl!(8u32, u32,);
                last = lf_checker_rt::callee_cdecl!(9u32, u32,);
                let p7 = new_obj(0x14);
                if p7 != 0 {
                    last = lf_checker_rt::callee_thiscall!(
                        7u32,
                        u32,
                        p7,
                        lf_checker_rt::relocated(CODE_F1),
                        &f1 as *const u32 as u32,
                        &f0 as *const u32 as u32
                    );
                }
                last = emit(if p7 != 0 { last } else { 0 });
                if g32(D_MODE1) != 2 {
                    last = lf_checker_rt::callee_cdecl!(10u32, u32,);
                    if g32(D_KIND) == 2 && g8(G_EXTRA) != 0 {
                        last = lf_checker_rt::callee_cdecl!(11u32, u32,);
                    }
                }
                last = lf_checker_rt::callee_cdecl!(12u32, u32, 0);
            }
        }
        let p8 = new_obj(0x0c);
        if p8 != 0 {
            last = lf_checker_rt::callee_thiscall!(6u32, u32, p8, lf_checker_rt::relocated(CODE_E2));
        }
        last = emit(if p8 != 0 { last } else { 0 });
        if g32(D_MODE0) == 0 && g32(D_MODE1) == 0 {
            return last;
        }
        let p9 = new_obj(0x0c);
        if p9 != 0 {
            last = lf_checker_rt::callee_thiscall!(6u32, u32, p9, lf_checker_rt::relocated(CODE_E3));
        }
        last = emit(if p9 != 0 { last } else { 0 });
        if g32(D_MODE1) == 0 {
            return last;
        }
        last = lf_checker_rt::callee_cdecl!(13u32, u32,);
        last
    }
});
