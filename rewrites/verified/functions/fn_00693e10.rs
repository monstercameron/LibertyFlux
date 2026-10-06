// original: 0x00693E10 comp_array_grow_slot (proposed)

//
// Appends a slot to the array at [obj] (count as a 16-bit word at
// [obj+4], capacity as a 16-bit word at [obj+6]): when count equals
// capacity, the capacity grows by 0x10, the intercepted allocator makes
// the new array, the old entries are copied over, and the old array is
// freed through the thread-local allocator (tls slot 0 -> [+8] ->
// vtable slot +0xc) when non-null. Returns the address of the new slot
// (base + count*4) and raises the count by one.
//
// Original: 0x00693E10 (thiscall, one unread stack argument,
// callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00693E10(obj: u32, _unused: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const ALLOC: u32 = 1;
        const FREE_SLOT: u32 = 0x0C;
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        let cap = rd16(obj.wrapping_add(6));
        if rd16(obj.wrapping_add(4)) == cap {
            let newcap = (cap as u32).wrapping_add(0x10) & 0xFFFF;
            wr16(obj.wrapping_add(6), newcap as u16);
            let newarr = lf_checker_rt::callee_thiscall!(ALLOC, u32, obj, newcap);
            let count = rd16(obj.wrapping_add(4)) as u32;
            let mut i = 0u32;
            while i < count {
                let v = rd32(rd32(obj).wrapping_add(i.wrapping_mul(4)));
                wr32(newarr.wrapping_add(i.wrapping_mul(4)), v);
                i = i.wrapping_add(1);
            }
            let old = rd32(obj);
            if old != 0 {
                let heap_obj = rd32(lf_checker_rt::tls_slot(0).wrapping_add(8));
                let vtable = rd32(heap_obj);
                let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable.wrapping_add(FREE_SLOT)) as usize);
                free_mem(heap_obj, old);
            }
            wr32(obj, newarr);
        }
        let count = rd16(obj.wrapping_add(4)) as u32;
        let slot = rd32(obj).wrapping_add(count.wrapping_mul(4));
        wr16(obj.wrapping_add(4), count.wrapping_add(1) as u16);
        slot
    }
});
