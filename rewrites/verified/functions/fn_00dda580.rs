// original: 0x00dda580 uimontageclip_prepare_slot_secondary (proposed)

/// Ensure the secondary slot object exists, refresh its transform, then
/// dispatch the given selector and record the outcome.
///
/// `this` is the clip; `sel` is a small integer selector. When the slot word
/// at `+SLOT` is null it is created: a `NEW_SIZE`-byte block is allocated
/// (callee 2, cdecl), two chained queries run against this clip's own virtual
/// slot `QUERY` (thiscall, no stack arguments; the first answer is forwarded
/// as the second argument of the construct call below), a lookup runs
/// (callee 4, cdecl, two words), the block is constructed (callee 5, thiscall:
/// ECX = block, two stack words), and it is registered (callee 6, thiscall:
/// ECX = block, four stack words = finder result, `STR_REG`, an out word
/// pre-filled with -1, -1) and stored into `+SLOT`. A null allocation
/// short-circuits the chain (the later load faults on both sides).
///
/// The fresh slot is then set up: its virtual slot `SETUP` is called with
/// (40.0, 40.0); the transform service (callee 8, thiscall: ECX = scratch
/// buffer, two stack words = `SEL`, 0) answers 24 bytes that are passed by
/// value, prefixed with `K_STRUCT`, to the part at `+PART` through its
/// virtual slot `APPLY` (thiscall, seven stack words); the slot's virtual
/// slot `COMMIT` is called with (`K_TAG`, apply result); the release helper
/// (callee 11) runs; the constant 2 is stored at slot `+TAG_OFF`; and the
/// slot's virtual slot `FINISH` is called with this clip's own `PLAIN` slot
/// result (virtual slot `PLAIN` on this, no stack arguments).
///
/// Dispatch: `sel` 0..4 selects one of the `STR_DISP_*` strings for the
/// dispatch call (callee 14, thiscall: ECX = slot, two stack words = finder
/// result, string); any other value skips it. The sibling refresh (callee 15,
/// thiscall: ECX = this, one stack word = whether the byte at `+FLAG_OFF` is
/// non-zero) runs next, then this clip's virtual slot `PROBE` feeds the
/// resolver (callee 17, thiscall: ECX = the shared object at `SHARED`, one
/// stack word). A null resolver answer, or a resolver object whose virtual
/// slot 0 disagrees with the hash helper (callee 19, cdecl, `STR_HASH`), ends
/// dispatch. Otherwise this clip's `PLAIN` result is passed to the ranker
/// (callee 20, thiscall: ECX = resolver object): a null ranker answer stores
/// `sel` directly, a non-null one ends dispatch. Ending dispatch stores 0
/// when `sel` is 2, maps 3 to 1, and otherwise stores `sel`, all at
/// `+STORE_OFF`.
///
/// The finder (callee 1, cdecl, `STR_FIND`) runs first; its answer is only
/// ever forwarded to later calls.
///
/// Original: 0x00dda580 (thiscall, one stack word, the callee pops 4 bytes, no return value).
lf_checker_rt::export!(thiscall, rw_00dda580(this: u32, sel: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x300;
        const PART: u32 = 0x1e0;
        const FLAG_OFF: u32 = 0x318;
        const STORE_OFF: u32 = 0x310;
        const TAG_OFF: u32 = 0x1d8;
        const QUERY: u32 = 0x48;
        const PLAIN: u32 = 0x4c;
        const PROBE: u32 = 0x54;
        const SETUP: u32 = 0x80;
        const APPLY: u32 = 0x4c;
        const COMMIT: u32 = 0x104;
        const FINISH: u32 = 0x194;
        const NEW_SIZE: u32 = 0x25c;
        const FORTY: u32 = 0x42200000;
        const SEL: u32 = 0x41500000;
        const K_STRUCT: u32 = 2;
        const K_TAG: u32 = 8;
        const SHARED: u32 = 0x01981a4c;
        const STR_FIND: u32 = 0x00efbb14;
        const STR_LOOKUP: u32 = 0x00efb5f8;
        const STR_REG: u32 = 0x00efb608;
        const STR_D0: u32 = 0x00efb614;
        const STR_D2: u32 = 0x00efb620;
        const STR_D1: u32 = 0x00efb630;
        const STR_D3: u32 = 0x00efb640;
        const STR_D4: u32 = 0x00efb654;
        const STR_HASH: u32 = 0x00efb668;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vcall0(child: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(child) + slot) as usize);
                f(child)
            }
        }

        let find: u32 =
            lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(STR_FIND));
        if rd32(this + SLOT) == 0 {
            let fresh: u32 = lf_checker_rt::callee_cdecl!(2, u32, NEW_SIZE);
            let obj = if fresh == 0 {
                0
            } else {
                let r1 = vcall0(this, QUERY);
                let r2 = vcall0(this, QUERY);
                let r3: u32 = lf_checker_rt::callee_cdecl!(
                    4, u32, lf_checker_rt::relocated(STR_LOOKUP), r2);
                lf_checker_rt::callee_thiscall!(5, u32, fresh, r3, r1)
            };
            wr32(this + SLOT, obj);
            let mut outw = [0xffffffffu32; 1];
            lf_checker_rt::callee_thiscall!(
                6, u32, obj, find, lf_checker_rt::relocated(STR_REG),
                outw.as_mut_ptr() as u32, 0xffffffff);
            let o = rd32(this + SLOT);
            let f_setup: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(o) + SETUP) as usize);
            f_setup(o, FORTY, FORTY);
            let mut scratch = [0u32; 8];
            let buf = scratch.as_mut_ptr() as u32;
            let src: u32 =
                lf_checker_rt::callee_thiscall!(8, u32, buf, SEL, 0);
            let w = [rd32(src), rd32(src + 4), rd32(src + 8),
                     rd32(src + 12), rd32(src + 16), rd32(src + 20)];
            let part = rd32(this + PART);
            let f_apply: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(part) + APPLY) as usize);
            let applied = f_apply(part, K_STRUCT, w[0], w[1], w[2], w[3], w[4], w[5]);
            let f_commit: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(o) + COMMIT) as usize);
            f_commit(o, K_TAG, applied);
            lf_checker_rt::callee_thiscall!(11, u32, buf);
            wr32(o + TAG_OFF, 2);
            let plain = vcall0(this, PLAIN);
            let f_finish: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(o) + FINISH) as usize);
            f_finish(o, plain);
        }
        let disp = match sel {
            0 => STR_D0,
            2 => STR_D2,
            1 => STR_D1,
            3 => STR_D3,
            4 => STR_D4,
            _ => 0,
        };
        if disp != 0 {
            lf_checker_rt::callee_thiscall!(
                14, u32, rd32(this + SLOT), find, lf_checker_rt::relocated(disp));
        }
        let flagged = ((this + FLAG_OFF) as *const u8).read() != 0;
        lf_checker_rt::callee_thiscall!(15, u32, this, flagged as u32);
        let probe = vcall0(this, PROBE);
        let resolved: u32 = lf_checker_rt::callee_thiscall!(
            17, u32, lf_checker_rt::relocated(SHARED), probe);
        if resolved != 0 {
            let v0 = vcall0(resolved, 0);
            let h: u32 = lf_checker_rt::callee_cdecl!(
                19, u32, lf_checker_rt::relocated(STR_HASH));
            if v0 == h {
                let plain = vcall0(this, PLAIN);
                let rank: u32 =
                    lf_checker_rt::callee_thiscall!(20, u32, resolved, plain);
                if rank == 0 {
                    wr32(this + STORE_OFF, sel);
                    return 0;
                }
            }
        }
        if sel == 2 {
            wr32(this + STORE_OFF, 0);
        } else {
            wr32(this + STORE_OFF, if sel == 3 { 1 } else { sel });
        }
        0
    }
});
