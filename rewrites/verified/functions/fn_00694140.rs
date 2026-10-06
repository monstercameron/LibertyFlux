// original: 0x00694140 comp_teardown_vtables (proposed)

//
// Tears the object down: stores the intermediate vtable, runs the
// intercepted cleanup with ([obj+0x18] in ECX, [obj+0x14] on the stack),
// clears +0x18/+0x14, frees [obj+0xC] through the thread-local allocator
// (tls slot 0 -> [+8] -> vtable slot +0xc) when non-null, zeroes
// +0xC/+0x10/+4/+8, and stores the base vtable. The second free branch is
// dead (it tests AX just after AX was zeroed) and is omitted. No return
// value. Both vtable immediates are relocated.
//
// Original: 0x00694140 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00694140(obj: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const CLEANUP: u32 = 1;
        const FREE_SLOT: u32 = 0x0C;
        const V_MID_FILE_VA: u32 = 0xFE38DC;
        const V_BASE_FILE_VA: u32 = 0xFE38AC;
        let a = rd32(obj.wrapping_add(0x14));
        let cleanup_this = rd32(obj.wrapping_add(0x18));
        wr32(obj, lf_checker_rt::relocated(V_MID_FILE_VA));
        lf_checker_rt::callee_thiscall!(CLEANUP, u32, cleanup_this, a);
        wr32(obj.wrapping_add(0x18), 0);
        wr32(obj.wrapping_add(0x14), 0);
        let c = rd32(obj.wrapping_add(0x0C));
        if c != 0 {
            let heap_obj = rd32(lf_checker_rt::tls_slot(0).wrapping_add(8));
            let vtable = rd32(heap_obj);
            let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(FREE_SLOT)) as usize);
            free_mem(heap_obj, c);
        }
        wr32(obj.wrapping_add(0x0C), 0);
        wr32(obj.wrapping_add(0x10), 0);
        wr32(obj.wrapping_add(4), 0);
        wr32(obj.wrapping_add(8), 0);
        wr32(obj, lf_checker_rt::relocated(V_BASE_FILE_VA));
        0
    }
});
