// original: 0x005C3650 ped_tls_object_slot_init_e

/// Allocates one TLS-owned object using allocator virtual slot +8, then calls the family initializer when allocation succeeds. The initializer result is written to this function's one-word global slot and returned in EAX; allocation failure writes and returns zero. The initializer reserves three uninitialized stack words, which the proof skips while comparing its this pointer and call order.
lf_checker_rt::export!(cdecl, rw_005c3650() -> u32 {
    unsafe {
        const GLOBAL_SLOT: u32 = 0x018b6f10;
        const ALLOCATOR_VTABLE: u32 = 0x00;
        const ALLOCATE_SLOT: u32 = 0x08;
        const INITIALIZER_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn call_allocator(target: u32, allocator: u32) -> u32 {
            let method: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(allocator, 0x1c, 0x10, 0)
        }

        let tls_context = lf_checker_rt::tls_slot(0);
        let allocator = read_word(tls_context + 8);
        let vtable = read_word(allocator + ALLOCATOR_VTABLE);
        let allocator_entry = read_word(vtable + ALLOCATE_SLOT);
        let allocated = call_allocator(allocator_entry, allocator);
        let initialized = if allocated == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(INITIALIZER_CALLEE, u32, allocated, 0, 0, 0)
        };
        lf_checker_rt::global::<u32>(GLOBAL_SLOT).write_unaligned(initialized);
        initialized
    }
});
