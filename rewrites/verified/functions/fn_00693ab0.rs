// original: 0x00693AB0 comp_teardown_full (proposed)

//
// Full teardown of the component at `obj`: releases every live element of
// the array at [obj] (count as a 16-bit word at [obj+4]) through its
// vtable slot +0 with argument 1, frees the array through the thread-local
// allocator (tls slot 0 -> [+8] -> vtable slot +0xc) when non-null, and
// clears [obj]/[obj+4]. Then, when the byte at [obj+0x10] and the pointer
// at [obj+8] are both set, runs the intercepted resolver on that pointer,
// and when its word at +0x1E is also set runs the intercepted pair
// consumer with ([ptr+0x18], word); the pointer is freed and both fields
// cleared. Finally the byte at [obj+0x11] gates the same free-and-clear
// for [obj+0xC]. No return value.
//
// Original: 0x00693AB0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00693AB0(obj: u32) -> u32 {
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
        const RELEASE_SLOT: u32 = 0x00;
        const FREE_SLOT: u32 = 0x0C;
        const RESOLVER: u32 = 3;
        const PAIRWISE: u32 = 4;
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn free_ptr(p: u32) {
            unsafe {
                let heap_obj = rd32(lf_checker_rt::tls_slot(0).wrapping_add(8));
                let vtable = rd32(heap_obj);
                let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable.wrapping_add(FREE_SLOT)) as usize);
                free_mem(heap_obj, p);
            }
        }
        if 0u32 < rd16(obj.wrapping_add(4)) as u32 {
            let mut i = 0u32;
            loop {
                let elem = rd32(rd32(obj).wrapping_add(i.wrapping_mul(4)));
                if elem != 0 {
                    let vtable = rd32(elem);
                    let release: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rd32(vtable.wrapping_add(RELEASE_SLOT)) as usize);
                    release(elem, 1);
                }
                let count = rd16(obj.wrapping_add(4)) as i32;
                i = i.wrapping_add(1);
                if !((i as i32) < count) {
                    break;
                }
            }
        }
        let arr = rd32(obj);
        if arr != 0 {
            free_ptr(arr);
        }
        wr32(obj, 0);
        wr32(obj.wrapping_add(4), 0);
        if rd8(obj.wrapping_add(0x10)) != 0 {
            let p = rd32(obj.wrapping_add(8));
            if p != 0 {
                lf_checker_rt::callee_thiscall!(RESOLVER, u32, p);
                if rd16(p.wrapping_add(0x1E)) != 0 {
                    let w = rd16(p.wrapping_add(0x1E)) as u32;
                    let q = rd32(p.wrapping_add(0x18));
                    lf_checker_rt::callee_stdcall!(PAIRWISE, u32, q, w);
                }
                free_ptr(p);
                wr32(obj.wrapping_add(8), 0);
                wr8(obj.wrapping_add(0x10), 0);
            }
        }
        if rd8(obj.wrapping_add(0x11)) != 0 {
            let r = rd32(obj.wrapping_add(0x0C));
            if r != 0 {
                free_ptr(r);
                wr32(obj.wrapping_add(0x0C), 0);
                wr8(obj.wrapping_add(0x11), 0);
            }
        }
        0
    }
});
