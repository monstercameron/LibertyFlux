// original: 0x0069B550 rage::crAnimChannelStaticQuaternion::map

/// Maps a static Quaternion channel's slot for the current session.
///
/// `this` is the channel object (the stack word is part of the virtual
/// signature and ignored). The object is stamped with the class vtable.
/// When the thread-local session value (at thread-block `+4`) and the
/// slot at `[this+8]` are both non-null, the slot is looked up (callee 1,
/// thiscall: session head, slot address); unless the lookup reports
/// missing (-1), the slot is rebased by the relocator's answer (callee 2,
/// thiscall: session block, old slot). Any missing piece clears the slot
/// instead. Returns `this` in `eax`.
///
/// Original: 0x0069B550 (thiscall, one ignored stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069B550(this: u32, _ignored: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const SESS_OFF: u32 = 4;
        const SLOT_OFF: u32 = 8;
        const VTABLE: u32 = 0xFE3C3C;
        const MISSING: u32 = 0xFFFFFFFF;
        const LOOKUP: u32 = 1;
        const RELOC: u32 = 2;
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
        let sess = rd32(lf_checker_rt::tls_slot(TLS_SLOT) + SESS_OFF);
        let slot = rd32(this + SLOT_OFF);
        if sess == 0 || slot == 0 {
            wr32(this + SLOT_OFF, 0);
            return this;
        }
        let head = rd32(sess);
        let hit = lf_checker_rt::callee_thiscall!(LOOKUP, u32, head, this + SLOT_OFF);
        if hit == MISSING {
            wr32(this + SLOT_OFF, 0);
            return this;
        }
        let d = lf_checker_rt::callee_thiscall!(RELOC, u32, sess, slot);
        wr32(this + SLOT_OFF, slot.wrapping_add(d));
        this
    }
});
