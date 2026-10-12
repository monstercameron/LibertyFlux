// original: 0x00C76D80 phbound_pool_store

/// Store a bound handle in pool slot a0. Slot zero is used when the flag byte for a0 has bit 7 set (a null write that faults); otherwise the slot is table base + stride * a0. Stores a1, notifies the watcher with (a0, a0) only when a1 is nonzero, and returns whether the slot reads back nonzero. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_c76d80(idx: u32, handle: u32) -> u32 {
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

        const POOL_GVA: u32 = 0x016EC774;
                let table = rd32(lf_checker_rt::relocated(POOL_GVA));
                let slot = if rd8(idx.wrapping_add(rd32(table.wrapping_add(4)))) & 0x80 != 0 {
                    0
                } else {
                    rd32(table).wrapping_add(rd32(table.wrapping_add(0xc)).wrapping_mul(idx))
                };
                wr32(slot, handle);
                if handle != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(1, u32, idx, idx);
                }
                u32::from(rd32(slot) != 0)
        }
});
