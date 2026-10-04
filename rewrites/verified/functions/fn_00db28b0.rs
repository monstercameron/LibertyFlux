// original: 0x00db28b0 ui_slot_ensure_bound (proposed)

/// Bind a UI slot on demand: tear down when asked to release, otherwise
/// resolve the slot's target through a chain of lookups and keep it alive.
///
/// `this` is the UI object and the low byte of `arg0` selects release (zero)
/// versus bind. Release with the bound flag at `+0x231` clear does nothing;
/// otherwise the anchor at `+0x1d4` is detached when present, the flag is
/// cleared, and a formatted value is stored at `+0x1e0`. Bind with the flag
/// already set, or with the ready byte at `+0x232` clear, does nothing;
/// otherwise two notify callees run (the second with a table pointer picked
/// by the index at `+0x22c`). Index 2 additionally resolves a target: a probe
/// object is opened, virtual slot `+0x54` and the probe chain are polled and
/// their answers compared against freshly resolved values down up to three
/// levels, and the winning handle lands in `ebx` (-1 when every level
/// disagreed). A saved pointer plus a non-negative handle then opens the
/// target and its out-pointer is collected; without index 2, or when that
/// produced nothing, the out-pointer is collected through the ready byte
/// instead. A null out-pointer ends the call after the tail callee; otherwise
/// a five-word descriptor (the last word 4) is submitted with it, the scope
/// and step callees run, the handle from the first submit call is rebound,
/// the out-pointer's reference count at `+0x20` is dropped (freeing it at
/// zero), the bound flag is set, and a formatted value is stored at `+0x1e0`
/// before the tail callee.
///
/// Edge cases: the out-pointer cell is the caller's own argument slot on the
/// original side, which the rewrite cannot reproduce, so the stack comparison
/// is off and the collected words are snapshotted instead. Early exits leave
/// the original's return register untouched (the rewrite returns 0 there; the
/// return value is unchecked). The handle comparison is signed: only a
/// negative handle skips the target open.
///
/// Original: 0x00db28b0 (thiscall, one stack argument, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00db28b0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x231;
        const READY: u32 = 0x232;
        const ANCHOR: u32 = 0x1d4;
        const SLOT_VALUE: u32 = 0x1e0;
        const EXTRA: u32 = 0x258;
        const INDEX: u32 = 0x22c;
        const TABLE: u32 = 0x01057690;
        const OBJ: u32 = 0x01981a4c;
        const SCOPE: u32 = 0x017a6658;
        const G_SLOT: u32 = 0x018b6c8c;
        const G_SUBMIT: u32 = 0x017f5630;

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
        unsafe fn vthis(this: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this) + slot) as usize);
                f(this)
            }
        }
        #[inline(always)]
        unsafe fn vobj(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }

        let obj = lf_checker_rt::relocated(OBJ);
        if arg0 & 0xff == 0 {
            if rd8(this + FLAG) == 0 {
                return 0;
            }
            if rd32(this + ANCHOR) != 0 {
                let mut s1: u32 = 0;
                lf_checker_rt::callee_thiscall!(
                    15u32,
                    u32,
                    &mut s1 as *mut u32 as u32,
                    lf_checker_rt::relocated(SCOPE)
                );
                lf_checker_rt::callee_thiscall!(23u32, u32, rd32(this + ANCHOR), 0);
                let mut s2: u32 = 0;
                let f2 = &mut s2 as *mut u32 as u32;
                lf_checker_rt::callee_thiscall!(17u32, u32, f2);
                lf_checker_rt::callee_thiscall!(21u32, u32, f2);
            }
            let mut cell = arg0;
            lf_checker_rt::callee_cdecl!(20u32, u32, &mut cell as *mut u32 as u32, 0x42);
            wr8(this + FLAG, 0);
            wr32(this + SLOT_VALUE, cell);
            // No tail call on this path: the original returns directly.
            return 0;
        }
        if rd8(this + FLAG) != 0 {
            return 0;
        }
        if rd8(this + READY) == 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(1u32, u32, rd32(this + EXTRA));
        let idx = rd32(this + INDEX);
        let tval = rd32(lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)));
        lf_checker_rt::callee_cdecl!(2u32, u32, tval, 0);
        let mut outcell: u32 = 0;
        let mut saved: u32 = 0;
        if idx == 2 {
            let a: u32 = lf_checker_rt::callee_thiscall!(
                3u32,
                u32,
                obj,
                lf_checker_rt::relocated(0x00ef201cu32)
            );
            saved = rd32(a.wrapping_add(0x1f8));
            let first: u32 = vthis(this, 0x54);
            let ebp: u32 = lf_checker_rt::callee_thiscall!(5u32, u32, obj, first);
            let r1: u32 = vobj(ebp, 0);
            let c1: u32 =
                lf_checker_rt::callee_cdecl!(8u32, u32, lf_checker_rt::relocated(0x00ef2030u32));
            let ebx: u32;
            if r1 == c1 {
                let v54: u32 = vobj(ebp, 0x54);
                let e2: u32 = lf_checker_rt::callee_thiscall!(5u32, u32, obj, v54);
                let r2: u32 = vobj(e2, 0);
                let c2: u32 = lf_checker_rt::callee_cdecl!(
                    8u32,
                    u32,
                    lf_checker_rt::relocated(0x00ef2040u32)
                );
                if r2 == c2 {
                    ebx = rd32(rd32(lf_checker_rt::relocated(G_SLOT)).wrapping_add(0x208));
                } else {
                    ebx = lf_checker_rt::callee_thiscall!(9u32, u32, a, ebp);
                }
            } else {
                let r3: u32 = vobj(ebp, 0);
                let c3: u32 = lf_checker_rt::callee_cdecl!(
                    8u32,
                    u32,
                    lf_checker_rt::relocated(0x00ef2050u32)
                );
                if r3 == c3 {
                    let a2: u32 = lf_checker_rt::callee_thiscall!(
                        3u32,
                        u32,
                        obj,
                        lf_checker_rt::relocated(0x00ef205cu32)
                    );
                    let v54b: u32 = vobj(ebp, 0x54);
                    let e3: u32 = lf_checker_rt::callee_thiscall!(5u32, u32, obj, v54b);
                    ebx = lf_checker_rt::callee_thiscall!(10u32, u32, a2, e3);
                } else {
                    ebx = 0xffff_ffff;
                }
            }
            if saved != 0 && (ebx as i32) >= 0 {
                let h: u32 = lf_checker_rt::callee_thiscall!(11u32, u32, saved, ebx);
                if h != 0 {
                    lf_checker_rt::callee_cdecl!(
                        12u32,
                        u32,
                        rd32(h.wrapping_add(0x48)),
                        rd32(h.wrapping_add(0x4c)),
                        &mut outcell as *mut u32 as u32
                    );
                }
            }
        }
        let mut edx = outcell;
        if !(idx == 2 && edx != 0) {
            lf_checker_rt::callee_cdecl!(
                13u32,
                u32,
                this.wrapping_add(READY),
                &mut outcell as *mut u32 as u32
            );
            edx = outcell;
            if edx == 0 {
                return lf_checker_rt::callee_cdecl!(22u32, u32,);
            }
        }
        let g = rd32(lf_checker_rt::relocated(G_SUBMIT));
        let mut desc = [0u32, 0, 0, 4, 0];
        let bound: u32 = lf_checker_rt::callee_thiscall!(14u32, u32, g, edx, desc.as_mut_ptr() as u32);
        let mut s3: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            15u32,
            u32,
            &mut s3 as *mut u32 as u32,
            lf_checker_rt::relocated(SCOPE)
        );
        lf_checker_rt::callee_thiscall!(16u32, u32, this, bound);
        let mut s4: u32 = 0;
        lf_checker_rt::callee_thiscall!(17u32, u32, &mut s4 as *mut u32 as u32);
        let outptr = outcell;
        let rc = rd32(outptr.wrapping_add(0x20)).wrapping_sub(1);
        wr32(outptr.wrapping_add(0x20), rc);
        if rc == 0 {
            lf_checker_rt::callee_thiscall!(18u32, u32, outptr);
            lf_checker_rt::callee_cdecl!(19u32, u32, outptr);
        }
        // The cell reuses the stack slot that held `saved` on the index-2
        // path; elsewhere it is untouched (zero) fill.
        let mut cell2: u32 = saved;
        lf_checker_rt::callee_cdecl!(20u32, u32, &mut cell2 as *mut u32 as u32, 0);
        wr8(this + FLAG, 1);
        wr32(this + SLOT_VALUE, cell2);
        let mut s5: u32 = 0;
        lf_checker_rt::callee_thiscall!(21u32, u32, &mut s5 as *mut u32 as u32);
        lf_checker_rt::callee_cdecl!(22u32, u32,)
    }
});

