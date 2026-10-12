// original: 0x00C7B2D0 CTaskComplexStationaryScenario::clone

/// Clone a stationary-scenario task. Allocates a new task through the pool allocator named by the global allocator word, builds it with the stationary constructor from the fields at +0x14, +0x18, +0x1c and the bytes at +0x24, +0x59, +0x65, copies the word at +0x5c, and when the byte at +0x58 is nonzero also forwards the local point (float at +0x50, struct at +0x30) to the pose helper. A null allocation yields a null clone. Returns the clone. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_c7b2d0(this: u32) -> u32 {
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
                let mut clone = 0u32;
                if fresh != 0 {
                    clone = lf_checker_rt::callee_thiscall!(2, u32, fresh,
                        rd32(this.wrapping_add(0x14)), rd32(this.wrapping_add(0x18)),
                        rd32(this.wrapping_add(0x1c)), rd8(this.wrapping_add(0x24)) as u32,
                        rd8(this.wrapping_add(0x59)) as u32, rd8(this.wrapping_add(0x65)) as u32);
                }
                if rd8(this.wrapping_add(0x58)) != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(3, u32, clone,
                        this.wrapping_add(0x30), rd32(this.wrapping_add(0x50)));
                }
                wr32(clone.wrapping_add(0x5c), rd32(this.wrapping_add(0x5c)));
                clone
        }
});
