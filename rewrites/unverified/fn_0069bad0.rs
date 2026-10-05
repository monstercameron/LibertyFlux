// original: 0x0069BAD0 rage::crAnimChannelCurveFloat::create

/// Creates a fresh crAnimChannelCurveFloat channel.
///
/// A `SIZE`-byte object is allocated through the thread allocator reached
/// as `tls[0] -> [+8] -> vtable[+8]` (callee 1, thiscall: allocator, size,
/// `0x10`, `0`); a null answer yields null. Otherwise the object is
/// initialized (vtable stamp, kind magic at `+4`, zeros at +8, +0xC) and its
/// pointer is returned in `eax`.
///
/// Original: 0x0069BAD0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0069BAD0() -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const OBJ_SIZE: u32 = 0x18;
        const ALLOC_HINT: u32 = 0x10;
        const VTABLE: u32 = 0xFE3E54;
        const KIND_OFF: u32 = 4;
        const KIND_MAGIC: u32 = 0x500;
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
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let heap_obj = rd32(tls_base + ALLOC_OBJ_OFF);
        let vtable = rd32(heap_obj);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + ALLOC_SLOT) as usize) };
        let fresh = alloc(heap_obj, OBJ_SIZE, ALLOC_HINT, 0);
        if fresh == 0 {
            return 0;
        }
        wr32(fresh + KIND_OFF, KIND_MAGIC);
        wr32(fresh, lf_checker_rt::relocated(VTABLE));
        wr32(fresh + 8, 0);
        wr32(fresh + 0x0C, 0);
        fresh
    }
});
