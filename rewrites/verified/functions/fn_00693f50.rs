// original: 0x00693F50 rage::crCreatureComponentBlendShapes::vf0

//
// Deleting destructor: runs the intercepted destructor on the object,
// then frees it through the thread-local allocator (tls slot 0 -> [+8] ->
// vtable slot +0xc) when the low bit of the flag argument is set and the
// object is non-null. Returns the object pointer. (The null-object path
// is untestable: ECX always holds a live pointer in the harness.)
//
// Original: 0x00693F50 (thiscall, one stack argument, callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00693F50(obj: u32, flag: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const DTOR: u32 = 1;
        const FREE_SLOT: u32 = 0x0C;
        lf_checker_rt::callee_thiscall!(DTOR, u32, obj);
        if flag & 1 != 0 && obj != 0 {
            let heap_obj = rd32(lf_checker_rt::tls_slot(0).wrapping_add(8));
            let vtable = rd32(heap_obj);
            let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(FREE_SLOT)) as usize);
            free_mem(heap_obj, obj);
        }
        obj
    }
});
