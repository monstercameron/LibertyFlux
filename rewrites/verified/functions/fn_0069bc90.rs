// original: 0x0069BC90 rage::crAnimChannelCurveFloat::map

/// Maps a curve-float channel's elements for the current session.
///
/// `this` is the channel object and the stack argument the session block.
/// The object is stamped with the class vtable, then the element base at
/// `[this+8]`, when non-null, is rebased by the relocator's answer
/// (callee 1, thiscall: session, old base). Each of the `count` elements
/// (unsigned word at `+0xC`) whose slot address and element pointer at
/// `[slot+4]` are both non-null is looked up in the session table at
/// `[session]`: the entry count is the sum of the words at `+0`/`+2`
/// (signed `jle` exit on non-positive, unobservable versus unsigned), and
/// an entry matches when the pointer falls in `[s0, s0+base)` (both
/// unsigned `jb` compares). A match adds that entry's stored delta,
/// `[head+idx*12+8] - [head+idx*12+4]`, to the element pointer; no match
/// (or an empty table) takes the slow path, which calls the session
/// fix-up (callee 2, cdecl: `0xFC9C80`, `0`, `0`) unless the thread-local
/// gate byte at `tls[0] -> [+4] -> [+0xC]` is set, then adds zero.
/// Returns `this`.
///
/// Original: 0x0069BC90 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069BC90(this: u32, src: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const SESS_OFF: u32 = 4;
        const GATE_OFF: u32 = 0x0C;
        const BASE_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x0C;
        const ELEM_SIZE: u32 = 8;
        const PTR_OFF: u32 = 4;
        const ENTRY_BASE: u32 = 4;
        const ENTRY_SIZE: u32 = 12;
        const RANGE_OFF: u32 = 8;
        const DELTA_SUB_OFF: u32 = 4;
        const VTABLE: u32 = 0xFE3E54;
        const FIXUP: u32 = 2;
        const FIXUP_ARG: u32 = 0xFC9C80;
        const RELOC: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let sess = src;
        let base0 = rd32(this + BASE_OFF);
        if base0 != 0 {
            let d = lf_checker_rt::callee_thiscall!(RELOC, u32, sess, base0);
            wr32(this + BASE_OFF, base0.wrapping_add(d));
        }
        let count = rd16(this + COUNT_OFF);
        if count == 0 {
            return this;
        }
        let mut i: u32 = 0;
        loop {
            let ebp = rd32(this + BASE_OFF).wrapping_add(i.wrapping_mul(ELEM_SIZE));
            if ebp != 0 {
                let edi = rd32(ebp + PTR_OFF);
                if edi != 0 {
                    let t = rd32(sess);
                    let n = rd16(t).wrapping_add(rd16(t + 2));
                    let mut delta: u32 = 0;
                    let mut found = false;
                    if (n as i32) > 0 {
                        let mut edx: u32 = 0;
                        loop {
                            let e = t.wrapping_add(ENTRY_BASE).wrapping_add(edx.wrapping_mul(ENTRY_SIZE));
                            let s0 = rd32(e);
                            if edi >= s0 {
                                let s1 = rd32(e + RANGE_OFF).wrapping_add(s0);
                                if edi < s1 {
                                    let shead = rd32(sess);
                                    let f = shead.wrapping_add(edx.wrapping_mul(ENTRY_SIZE));
                                    delta = rd32(f + RANGE_OFF).wrapping_sub(rd32(f + DELTA_SUB_OFF));
                                    found = true;
                                    break;
                                }
                            }
                            edx += 1;
                            if !((edx as i32) < (n as i32)) {
                                break;
                            }
                        }
                    }
                    if !found {
                        let tobj = lf_checker_rt::tls_slot(TLS_SLOT);
                        let sess2 = rd32(tobj + SESS_OFF);
                        let gate = unsafe { ((sess2 + GATE_OFF) as *const u8).read() };
                        if gate == 0 {
                            let _ = lf_checker_rt::callee_cdecl!(FIXUP, u32, lf_checker_rt::relocated(FIXUP_ARG), 0, 0);
                        }
                    }
                    wr32(ebp + PTR_OFF, edi.wrapping_add(delta));
                }
            }
            i += 1;
            let lim = rd16(this + COUNT_OFF);
            if !((i as i32) < (lim as i32)) {
                break;
            }
        }
        this
    }
});
