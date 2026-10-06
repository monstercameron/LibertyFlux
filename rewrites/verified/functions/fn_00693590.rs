// original: 0x00693590 rage::crCreatureComponent::vf0

//
// Deleting destructor: stores the component vtable, then frees the object
// through the thread-local allocator (tls slot 0 -> [+8] ->
// vtable slot +0xc) when the low bit of the flag argument is set.
// Returns the object pointer.
//
// Original: 0x00693590 (thiscall, one stack argument, callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00693590(obj: u32, flag: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        // The vtable immediate has a relocation entry: the original stores
        // the relocated address, so the rewrite must too.
        const VTABLE_FILE_VA: u32 = 0xFE38AC;
        const FREE_SLOT: u32 = 0x0C;
        wr32(obj, lf_checker_rt::relocated(VTABLE_FILE_VA));
        if flag & 1 != 0 {
            let heap_obj = rd32(lf_checker_rt::tls_slot(0).wrapping_add(8));
            let vtable = rd32(heap_obj);
            let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(FREE_SLOT)) as usize);
            free_mem(heap_obj, obj);
        }
        obj
    }
});
