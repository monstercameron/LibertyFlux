// original: 0x0069C9E0 rage::crAnimChannelCurveFloat::alloc_keys

/// Allocates a zeroed key array for a curve-float channel.
///
/// The stack argument is the signed element count. `count * 8` bytes are
/// allocated through the thread allocator reached as
/// `tls[0] -> [+8] -> vtable[+8]` (callee 1, thiscall: allocator, bytes,
/// `0x10`, `0`); a non-positive count (signed) skips the fill. Every
/// element's 8 bytes are then zeroed unless the element address itself is
/// null (which a null block yields for element 0 only). Returns the block.
///
/// Original: 0x0069C9E0 (stdcall, one stack word, callee pops 4).
lf_checker_rt::export!(stdcall, rw_0069C9E0(count: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const ELEM_SIZE: u32 = 8;
        const ALLOC_HINT: u32 = 0x10;
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
        let block = alloc(heap_obj, count.wrapping_mul(ELEM_SIZE), ALLOC_HINT, 0);
        if (count as i32) > 0 {
            let mut k: u32 = 0;
            while k < count {
                let elem = block.wrapping_add(k.wrapping_mul(ELEM_SIZE));
                if elem != 0 {
                    wr32(elem, 0);
                    wr32(elem.wrapping_add(4), 0);
                }
                k += 1;
            }
        }
        block
    }
});
