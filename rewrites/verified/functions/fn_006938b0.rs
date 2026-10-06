// original: 0x006938B0 comp_array_dispose (proposed)

//
// Runs the intercepted teardown helper on `obj`, then frees the array at
// [obj] through the thread-local allocator (tls slot 0 -> [+8] ->
// vtable slot +0xc) when the word at [obj+6] is nonzero and the array is
// non-null. No return value.
//
// Original: 0x006938B0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_006938B0(obj: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const TEARDOWN: u32 = 1;
        const FREE_SLOT: u32 = 0x0C;
        lf_checker_rt::callee_thiscall!(TEARDOWN, u32, obj);
        if rd16(obj.wrapping_add(6)) != 0 {
            let arr = rd32(obj);
            if arr != 0 {
                let heap_obj = rd32(lf_checker_rt::tls_slot(0).wrapping_add(8));
                let vtable = rd32(heap_obj);
                let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable.wrapping_add(FREE_SLOT)) as usize);
                free_mem(heap_obj, arr);
            }
        }
        0
    }
});
