// original: 0x006963D0 rage::crAnimChannelRawVector3::dtor

/// Destroys a crAnimChannelRawVector3 channel, freeing its keys and itself when asked.
///
/// `this` is the object and the stack argument carries flags. The object is
/// stamped with the class vtable; when the key count (unsigned word at
/// `+0xE`) is non-zero and the key pointer at `+8` is non-null, the keys
/// are released. The object is then stamped with the base vtable and, when
/// bit 0 of the flags is set, released itself. Both releases go through
/// the thread allocator reached as `tls[0] -> [+8] -> vtable[+0xC]`
/// (callee 1, thiscall: allocator, block). Returns `this` in `eax`.
///
/// Original: 0x006963D0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_006963D0(this: u32, flags: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        const CLASS_VTABLE: u32 = 0xFE39C4;
        const BASE_VTABLE: u32 = 0xFE3A74;
        const COUNT_OFF: u32 = 0x0E;
        const KEYS_OFF: u32 = 8;
        const FREE_FLAG: u32 = 1;
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
        wr32(this, lf_checker_rt::relocated(CLASS_VTABLE));
        if rd16(this + COUNT_OFF) != 0 {
            let keys = rd32(this + KEYS_OFF);
            if keys != 0 {
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let tls_inner = rd32(tls_base);
        let heap_obj = rd32(tls_inner + ALLOC_OBJ_OFF);
        let vtable = rd32(heap_obj);
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
                free(heap_obj, keys);
            }
        }
        wr32(this, lf_checker_rt::relocated(BASE_VTABLE));
        if flags & FREE_FLAG != 0 {
            let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
            let tls_inner = rd32(tls_base);
            let heap_obj = rd32(tls_inner + ALLOC_OBJ_OFF);
            let vtable = rd32(heap_obj);
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
            free(heap_obj, this);
        }
        this
    }
});
