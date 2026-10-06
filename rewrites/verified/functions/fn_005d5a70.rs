// original: 0x005d5a70 html_node_teardown (proposed)

/// Tear down the child list of a base HTML node.
///
/// Stamps the base vtable, then walks the `count` entries (`+0x10`, compared
/// unsigned) of the child array at `+0x0c`: each non-null child is destroyed
/// through its own virtual deleting destructor (slot `+0`, called with the
/// child in ECX and `1` on the stack) and every visited slot is cleared.
/// Afterwards frees the array itself through the thread allocator (TLS slot
/// 0: allocator at `+8`, free at vtable `+0x0c`) when the capacity word at
/// `+0x12` is non-zero and the array is non-null. Returns the free's answer
/// when it freed, else the entry count (zero when the loop was skipped).
///
/// Original: 0x005d5a70 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_005d5a70(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE0B94;
        const ARRAY_OFF: u32 = 0x0c;
        const COUNT_OFF: u32 = 0x10;
        const CAPACITY_OFF: u32 = 0x12;
        const FREE_OFF: u32 = 0x0c;

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

        wr32(this, lf_checker_rt::relocated(VTABLE));
        let count = rd16(this + COUNT_OFF);
        if count != 0 {
            let mut i = 0u32;
            while i < count {
                let array = rd32(this + ARRAY_OFF);
                let child = rd32(array + i * 4);
                if child != 0 {
                    let vtable = rd32(child);
                    let dtor: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rd32(vtable) as usize);
                    dtor(child, 1);
                }
                wr32(rd32(this + ARRAY_OFF) + i * 4, 0);
                i += 1;
            }
        }
        let capacity = rd16(this + CAPACITY_OFF);
        let array = rd32(this + ARRAY_OFF);
        let kept = if count != 0 { count } else { 0 };
        if capacity != 0 && array != 0 {
            let thread = lf_checker_rt::tls_slot(0);
            let allocator = rd32(thread + 8);
            let vtable = rd32(allocator);
            let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable + FREE_OFF) as usize);
            return free_fn(allocator, array);
        }
        kept
    }
});
