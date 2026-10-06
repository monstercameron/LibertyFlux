// original: 0x005d6470 html_table_grid_release (proposed)

/// Release the five owned grid buffers of an HTML table node.
///
/// Frees each of the pointers at `+0xec`, `+0xf0`, `+0xe8`, `+0xf4` and
/// `+0xf8` (in that order) through the thread allocator (reached from TLS
/// slot 0: allocator at `+8`, vtable at `+0`, free at vtable `+0x0c`, called
/// with the allocator in ECX and the pointer on the stack) when it is
/// non-null. The slots themselves are left untouched, and the return
/// register is residue (the last free's answer, or entry EAX when no slot
/// was set), so the proof checks the call sequence, not the return.
///
/// Original: 0x005d6470 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_005d6470(this: u32) -> u32 {
    unsafe {
        const SLOTS: [u32; 5] = [0xec, 0xf0, 0xe8, 0xf4, 0xf8];
        const FREE_OFF: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let thread = lf_checker_rt::tls_slot(0);
        let allocator = rd32(thread + 8);
        let vtable = rd32(allocator);
        let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + FREE_OFF) as usize);
        let mut i = 0;
        while i < SLOTS.len() {
            let ptr = rd32(this + SLOTS[i]);
            if ptr != 0 {
                free_fn(allocator, ptr);
            }
            i += 1;
        }
        0
    }
});
