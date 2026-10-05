// original: 0x00696420 rage::crAnimChannelRawQuaternion::clone

/// Clones a crAnimChannelRawQuaternion channel: allocates a fresh object and copies into it.
///
/// `this` is the source channel. A fresh object of `SIZE` bytes is allocated
/// through the thread allocator reached as `tls[0] -> [+8] -> vtable[+8]`
/// (callee 1, thiscall: allocator, size, `0x10`, `0`); a null answer yields
/// null. Otherwise the source is duplicated into it through the class
/// copy routine (callee 2, thiscall: fresh object, source) and the copy
/// routine's answer is returned in `eax`.
///
/// Original: 0x00696420 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00696420(this: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const OBJ_SIZE: u32 = 0x10;
        const ALLOC_HINT: u32 = 0x10;
        const COPY: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        lf_checker_rt::callee_thiscall!(COPY, u32, fresh, this)
    }
});
