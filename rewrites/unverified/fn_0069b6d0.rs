// original: 0x0069B6D0 rage::crAnimChannelStaticVector3::crAnimChannelStaticVector3

/// Constructs a static Vector3 channel in place.
///
/// `this` is the object: the kind magic `0xD00` goes at `+4`, the class
/// vtable at `+0`, the slot at `+8` is cleared, then a 16-byte value block
/// is allocated through the thread allocator reached as
/// `tls[0] -> [+8] -> vtable[+8]` (callee 1, thiscall: allocator, `0x10`,
/// `0x10`, `0`) and stored into the slot (a null answer is stored as-is).
/// Returns `this` in `eax`.
///
/// Original: 0x0069B6D0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0069B6D0(this: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x10;
        const ALLOC_HINT: u32 = 0x10;
        const VTABLE: u32 = 0xFE3CEC;
        const KIND_OFF: u32 = 4;
        const KIND_MAGIC: u32 = 0xD00;
        const SLOT_OFF: u32 = 8;
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
        wr32(this + KIND_OFF, KIND_MAGIC);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr32(this + SLOT_OFF, 0);
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let tls_inner = rd32(tls_base);
        let heap_obj = rd32(tls_inner + ALLOC_OBJ_OFF);
        let vtable = rd32(heap_obj);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + ALLOC_SLOT) as usize) };
        let block = alloc(heap_obj, BLOCK_SIZE, ALLOC_HINT, 0);
        wr32(this + SLOT_OFF, block);
        this
    }
});
