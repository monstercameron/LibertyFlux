// original: 0x005fbcc0 euphoria_streamref_init

/// Initialise a streamable reference that entered the active state.
///
/// `this` points at the reference (`+0` head word, `+0x4` context word, `+0x8`
/// state flags, `+0xc` owned-list head). `arg` is an opaque word threaded
/// into the emit call and the call frame below.
///
/// When the masked state (`[this+0x8] & 0x30000000`, an unsigned mask) is not
/// `0x20000000` the function does nothing and returns the masked state. On
/// the active path it initialises the context through a helper, bumps a live
/// counter, parks a four-word frame (`arg`, the thread slot's saved word, the
/// helper answer, zero) in the thread slot, and emits the reference through
/// the registry's data-table slot with (`head`, `&frame`): the registry-held
/// path cleans as thiscall, the fallback path as cdecl, matching the two
/// cleanups the original has for its two call sites. It then runs the plain
/// emit helper, marks the state word, walks the owned list calling virtual
/// slot 0 on each node, and restores the thread slot and the counter. Returns
/// the last list-hook answer, or the emit answer when the list is empty.
///
/// Original: 0x005FBCC0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005fbcc0(this: u32, arg: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x0;
        const CTX_OFF: u32 = 0x4;
        const FLAGS_OFF: u32 = 0x8;
        const STATE_MASK: u32 = 0x30000000;
        const STATE_ACTIVE: u32 = 0x20000000;
        const OWNER_OFF: u32 = 0xc;
        const NEXT_OFF: u32 = 0x4;
        const REG_OBJ: u32 = 0x0110E960;
        const REG_SLOT: u32 = 0x0110E964;
        const COUNTER: u32 = 0x018B7A50;
        const TLS_SLOT: usize = 0;
        const TLS_SAVED_OFF: u32 = 0x4;
        const INIT_CALLEE: u32 = 1;
        const EMIT_CALLEE: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let masked = rd32(this.wrapping_add(FLAGS_OFF)) & STATE_MASK;
        if masked != STATE_ACTIVE {
            return masked;
        }
        let ctx = rd32(this.wrapping_add(CTX_OFF));
        let answer: u32 = lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, ctx);
        let reg = rd32(lf_checker_rt::relocated(REG_OBJ));
        let ctr = lf_checker_rt::global::<u32>(COUNTER);
        ctr.write_unaligned(ctr.read_unaligned().wrapping_add(1));
        let tls = lf_checker_rt::tls_slot(TLS_SLOT);
        let saved = rd32(tls.wrapping_add(TLS_SAVED_OFF));
        let mut frame = [arg, saved, answer, 0u32];
        let sptr = frame.as_mut_ptr() as u32;
        wr32(tls.wrapping_add(TLS_SAVED_OFF), sptr);
        let head = rd32(this.wrapping_add(HEAD_OFF));
        let target = rd32(lf_checker_rt::relocated(REG_SLOT));
        if reg != 0 {
            let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(reg, head, sptr);
        } else {
            let f: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(head, sptr);
        }
        let mut out: u32 = lf_checker_rt::callee_cdecl!(EMIT_CALLEE, u32, arg);
        wr32(this.wrapping_add(FLAGS_OFF),
             rd32(this.wrapping_add(FLAGS_OFF)) | STATE_MASK);
        let mut node = rd32(this.wrapping_add(OWNER_OFF));
        while node != 0 {
            let hook: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(node) as usize);
            out = hook(node);
            node = rd32(node.wrapping_add(NEXT_OFF));
        }
        wr32(tls.wrapping_add(TLS_SAVED_OFF), saved);
        let ctr = lf_checker_rt::global::<u32>(COUNTER);
        ctr.write_unaligned(ctr.read_unaligned().wrapping_sub(1));
        out
    }

});
