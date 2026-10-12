// original: 0x00C7B380 CTaskComplexWaitForSeatToBeFree::clone

/// Clone a wait-for-seat task. Allocates a new task through the pool allocator named by the global allocator word; a null allocation yields a null clone. Otherwise builds it with the two-word scenario constructor from the fields at +0x14 and +0x18 and returns it. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_c7b380(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        const ALLOC_GVA: u32 = 0x0167E2A0;
                let fresh = lf_checker_rt::callee_thiscall!(1, u32, rd32(lf_checker_rt::relocated(ALLOC_GVA)));
                if fresh == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(2, u32, fresh, rd32(this.wrapping_add(0x14)), rd32(this.wrapping_add(0x18)))
        }
});
