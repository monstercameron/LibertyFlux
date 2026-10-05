// original: 0x0069CAA0 rage::crAnimChannelCurveFloat::free_keys

/// Frees a curve-float key array and each live element block.
///
/// The stack arguments are the array and the signed element count. Every
/// element's block pointer (at `array + 4 + i * 8`), when non-null, is
/// released, then the array itself when non-null. A non-positive count
/// (signed) skips the element loop. All releases go through the thread
/// allocator reached as `tls[0] -> [+8] -> vtable[+0xC]` (callee 1,
/// thiscall: allocator, block). A null array with a positive count faults
/// reading the first element pointer, identically on both sides. Returns
/// the last release answer.
///
/// Original: 0x0069CAA0 (stdcall, two stack words, callee pops 8).
lf_checker_rt::export!(stdcall, rw_0069CAA0(arr: u32, count: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        const ELEM_PTR_OFF: u32 = 4;
        const ELEM_SIZE: u32 = 8;
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
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
        let mut last: u32 = 0;
        if (count as i32) > 0 {
            let mut k: u32 = 0;
            while k < count {
                let p = rd32(arr.wrapping_add(ELEM_PTR_OFF).wrapping_add(k.wrapping_mul(ELEM_SIZE)));
                if p != 0 {
                    last = free(heap_obj, p);
                }
                k += 1;
            }
        }
        if arr != 0 {
            last = free(heap_obj, arr);
        }
        last
    }
});
