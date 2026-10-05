// original: 0x0069B4A0 rage::crAnimChannelStaticQuaternion::copy_from

/// Copies a static Quaternion channel into this object.
///
/// `this` is the destination and the stack argument the source. The base
/// vtable is stamped, the four header bytes at `+4` are copied, then the
/// class vtable. When the thread-local session value (at thread-block
/// `+4`) and the slot at `[this+8]` are both non-null, the slot is looked
/// up (callee 1, thiscall: session head, slot address); unless the lookup
/// reports missing (-1, compared for equality), the slot is rebased by
/// the relocator's answer (callee 2, thiscall: session block, old slot).
/// Any missing piece clears the slot instead. Then 16 fresh bytes are
/// allocated through the thread allocator reached as
/// `tls[0] -> [+8] -> vtable[+8]` (callee 3, thiscall: allocator, `0x10`,
/// `0x10`, `0`; the allocator-busy flag at thread-block `+0x34` is
/// cleared around the call), the slot is overwritten with the block, and
/// the source payload at `[src+8]` is copied in as two 8-byte moves.
/// Returns `this`.
///
/// Original: 0x0069B4A0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_0069B4A0(this: u32, src: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const SESS_OFF: u32 = 4;
        const SLOT_OFF: u32 = 8;
        const BUSY_OFF: u32 = 0x34;
        const PAYLOAD_OFF: u32 = 8;
        const PAYLOAD_SIZE: u32 = 0x10;
        const ALLOC_HINT: u32 = 0x10;
        const ALLOC_SLOT: u32 = 8;
        const BASE_VTABLE: u32 = 0xFE3A74;
        const CLASS_VTABLE: u32 = 0xFE3C3C;
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
        wr32(this, BASE_VTABLE);
        unsafe {
            let b4 = ((src + 4) as *const u8).read();
            ((this + 4) as *mut u8).write(b4);
            let b5 = ((src + 5) as *const u8).read();
            ((this + 5) as *mut u8).write(b5);
            let w6 = ((src + 6) as *const u16).read_unaligned();
            ((this + 6) as *mut u16).write_unaligned(w6);
        }
        wr32(this, CLASS_VTABLE);
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let tobj = tls_base;
        let heap_obj = rd32(tobj + ALLOC_OBJ_OFF);
        let vtable = rd32(heap_obj);
        let sess = rd32(tobj + SESS_OFF);
        let slot = rd32(this + SLOT_OFF);
        if sess != 0 && slot != 0 {
            let head = rd32(sess);
            let hit = lf_checker_rt::callee_thiscall!(LOOKUP, u32, head, this + SLOT_OFF);
            if hit == MISSING {
                wr32(this + SLOT_OFF, 0);
            } else {
                let d = lf_checker_rt::callee_thiscall!(RELOC, u32, sess, slot);
                wr32(this + SLOT_OFF, slot.wrapping_add(d));
            }
        } else {
            wr32(this + SLOT_OFF, 0);
        }
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + ALLOC_SLOT) as usize) };
        unsafe { ((tobj + BUSY_OFF) as *mut u8).write(0) };
        let block = alloc(heap_obj, PAYLOAD_SIZE, ALLOC_HINT, 0);
        unsafe { ((tobj + BUSY_OFF) as *mut u8).write(1) };
        wr32(this + SLOT_OFF, block);
        let qsrc = rd32(src + PAYLOAD_OFF);
        let lo = unsafe { (qsrc as *const u64).read_unaligned() };
        unsafe { (block as *mut u64).write_unaligned(lo) };
        let hi = unsafe { ((qsrc + 8) as *const u64).read_unaligned() };
        unsafe { ((block + 8) as *mut u64).write_unaligned(hi) };
        this
    }
});
