// original: 0x00ca7370 CEventHandler::vf13
/// Vet the event's auxiliary object and either build a directed response
/// through the float-gated allocator path or run the deep position-based
/// path.
///
/// `this` is the handler (thiscall: `this` in ecx, `ev` plus two stack
/// words, the third of which must be nonzero; the callee pops 0xc bytes): `+0x04` is the
/// owner (whose `+0x224` is the inner object), `+0x0c` takes the response
/// (cleared up front). `ev+0x12` is a signed word filter (1 exits),
/// `ev+0x14` points at the auxiliary object, `ev+0x40` selects the deep
/// path when zero, `ev+0x0c` is the float gate.
///
/// Head: id 1 vets the inner object; a nonzero global flag and a non-null
/// answer are required. Id 2 (slot `+0x28` of that answer) is tested on
/// its low byte; a zero low byte falls back to byte `0x219` of the owner
/// (zero returns the owner). Id 3 (slot `+0x30`) yields the working
/// object. Three derived bytes steer the rest: `dl` (byte `0x22b` of the
/// auxiliary object, unsigned above-zero test), `cl` (bit 2 of word
/// `0x24`), `al` (bit `0x13` of word `0x40` past the runtime table entry
/// selected by the signed word at `0x2e`; the table is empty in the image
/// and fabricated by the contract, an out-of-range index faulting exactly
/// like the original). When all three pass and the working object is
/// non-null, id 4 (slot `+0x18`) gates a vector block: id 5 (slot `+0x1c`)
/// fills a 3-word frame vector, id 6 combines it with the saved
/// auxiliary pointer, two float words and six zero words, two id 7/8 calls
/// must agree (else id 9 runs and id 10's low byte becomes `dh`).
///
/// Middle: with a nonzero `ev+0x40`, zero `dl`, set `cl` or set `dh` all
/// exit returning the event; otherwise two id 11 calls run and id 12
/// (slot `+0x0c`) equal to `0x13d` exits returning the event, while any
/// other outcome reaches the float gate (`ev+0x0c` above 1.0 proceeds; a
/// null id 13 allocation stores zero and both build through id 14 and
/// finish through id 15, whose answer is returned).
///
/// Deep (`ev+0x40` zero): a shifted bit of `[owner+0xa80]+0x50` exits with
/// the shifted value when set; id 4 with a zero low byte exits with its
/// full answer; set `dl` with clear `dh` exits with the id 4 answer. Past
/// an optional flag-gated recheck of `cl`/`al` (either set returns the
/// owner), id 5 fills another frame vector, id 17 allocates (null faults
/// on the flag write exactly like the original), id 18 builds with the
/// saved words and the vector, flag bits `0x10` (and `0x40` when `dl` is
/// set) are set, and id 19 finishes at `this+0x34`, its answer returned.
///
/// The original's stack realignment and security cookie are frame
/// artifacts, not behaviour: the rewrite uses a normal frame and still
/// invokes the cookie-check stub (id 16, preserving registers) at both
/// sites so the call logs match. All multi-word comparisons are equality
/// or unsigned; nothing here is signedness-sensitive except the table
/// index, which is a signed word.
lf_checker_rt::export!(thiscall, rw_00ca7370(this: u32, ev: u32, _a1: u32, arg2: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const RESPONSE: u32 = 0x0c;
        const WORD12: u32 = 0x12;
        const AUX: u32 = 0x14;
        const GATE40: u32 = 0x40;
        const FLOAT0C: u32 = 0x0c;
        const FLAG: u32 = 0x0103_CD60;
        const TABLE: u32 = 0x0129_5CD8;
        const FCONST: u32 = 0x00E9_D494;
        const GLIMIT: f32 = 1.0;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const VET: u32 = 1;
        const V28: u32 = 2;
        const V30: u32 = 3;
        const V18: u32 = 4;
        const V1C: u32 = 5;
        const COMB: u32 = 6;
        const C90A: u32 = 7;
        const C90B: u32 = 8;
        const M40: u32 = 9;
        const B13: u32 = 10;
        const VET2: u32 = 11;
        const V0C: u32 = 12;
        const ALLOC1: u32 = 13;
        const BDF: u32 = 14;
        const B81: u32 = 15;
        const COOKIE: u32 = 16;
        const ALLOC2: u32 = 17;
        const DC8: u32 = 18;
        const M69: u32 = 19;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }

        wr32(this + RESPONSE, 0);
        if arg2 == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ev;
        }
        let owner = rd32(this + OWNER);
        let inner44 = rd32(owner + INNER).wrapping_add(0x44);
        let s0: u32 = lf_checker_rt::callee_thiscall!(VET, u32, inner44);
        if rd8(lf_checker_rt::relocated(FLAG)) == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return s0;
        }
        if s0 == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0;
        }
        if (vcall0(s0, 0x28) & 0xFF) == 0 && rd8(owner + 0x219) == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return owner;
        }
        let s1 = vcall0(s0, 0x30);
        let w12e = rd16(ev + WORD12) as i16 as i32 as u32;
        if w12e == 1 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ev;
        }
        let a14 = rd32(ev + AUX);
        let dl: u32 = if rd8(a14 + 0x22b) > 0 { 1 } else { 0 };
        let clv = (rd32(a14 + 0x24) >> 2) & 0xFFFF_FF01;
        let idx = rd16(a14 + 0x2e) as i16 as i32;
        let entry = rd32(
            lf_checker_rt::relocated(TABLE).wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let alv = (rd32(entry.wrapping_add(0x40)) >> 0x13) & 0xFFFF_FF01;
        let mut dh: u32 = 0;
        let mut buf30 = [0u32; 3];
        if dl != 0 && (clv & 0xFF) == 0 && (alv & 0xFF) == 0 && s1 != 0 {
            if (vcall0(s1, 0x18) & 0xFF) != 0 {
                let mut vec = [0u32; 3];
                let f5: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(s1) + 0x1c) as usize);
                f5(s1, vec.as_mut_ptr() as u32);
                let opos = rd32(owner + 0x20);
                let f38 = rdf(opos + 0x38);
                let fconst = rdf(lf_checker_rt::relocated(FCONST));
                let mut buf50 = [0u32; 4];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    COMB, u32, buf50.as_mut_ptr() as u32, a14,
                    f32::to_bits(f38), f32::to_bits(fconst), 0u32, 0u32, 0u32, 0u32,
                    0u32, 0u32
                );
                let r1: u32 = lf_checker_rt::callee_thiscall!(
                    C90A, u32, buf50.as_mut_ptr() as u32, opos.wrapping_add(0x30)
                );
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    C90B, u32, buf50.as_mut_ptr() as u32, vec.as_mut_ptr() as u32
                );
                if r1 != r2 {
                    let px = rdf(opos + 0x30);
                    let py = rdf(opos + 0x34);
                    let pz = rdf(opos + 0x38);
                    buf30 = [
                        f32::to_bits(fsub(f32::from_bits(vec[0]), px)),
                        f32::to_bits(fsub(f32::from_bits(vec[1]), py)),
                        f32::to_bits(fsub(f32::from_bits(vec[2]), pz)),
                    ];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        M40, u32, buf30.as_mut_ptr() as u32
                    );
                    let b: u32 = lf_checker_rt::callee_thiscall!(
                        B13, u32, a14, opos.wrapping_add(0x30), buf30.as_mut_ptr() as u32
                    );
                    dh = b & 0xFF;
                }
            }
        }
        if rd8(ev + GATE40) == 0 {
            let w = rd32(owner + 0xa80);
            let sh = rd32(w + 0x50) >> 0x0d;
            if (sh & 1) != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                return sh;
            }
            let v18: u32 = vcall0(s1, 0x18);
            if (v18 & 0xFF) == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                return v18;
            }
            if dl != 0 && dh == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                return v18;
            }
            if rd32(owner + 0x29c) & 0x800 != 0 {
                if (clv & 0xFF) != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                    return owner;
                }
                if (alv & 0xFF) != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                    return owner;
                }
            }
            let f5: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(s1) + 0x1c) as usize);
            f5(s1, buf30.as_mut_ptr() as u32);
            let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC2, u32, global);
            let r: u32 = if obj == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    DC8, u32, obj, w12e, buf30.as_mut_ptr() as u32, a14, 0u32
                )
            };
            wr32(r + 0x9c, rd32(r + 0x9c) | 0x10);
            if dl != 0 {
                wr32(r + 0x9c, rd32(r + 0x9c) | 0x40);
            }
            let g: u32 = lf_checker_rt::callee_thiscall!(
                M69, u32, this.wrapping_add(0x34), r
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return g;
        }
        if dl == 0 || (clv & 0xFF) != 0 || dh != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ev;
        }
        let g1: u32 = lf_checker_rt::callee_thiscall!(VET2, u32, inner44, 4u32);
        if g1 != 0 {
            let g2: u32 = lf_checker_rt::callee_thiscall!(VET2, u32, inner44, 4u32);
            if vcall0(g2, 0x0c) == 0x13d {
                let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                return ev;
            }
        }
        if !(rdf(ev + FLOAT0C) > GLIMIT) {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ev;
        }
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC1, u32, global);
        let r: u32 = if obj == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(BDF, u32, obj, a14)
        };
        let f: u32 = lf_checker_rt::callee_thiscall!(B81, u32, inner44, r, 4u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        f
    }
});
