// original: 0x00B0BA30 Pickup_ProcessMatchingSlots

/// Scan the 1,500 fixed pickup slots in ascending order. For each slot whose
/// stored handle equals `handle`, call the cleanup helper with that slot's
/// zero-based index unless its type byte is the network-managed value 0x16.
/// Each slot is 0x50 bytes; the pool and field offsets are specific to the
/// original 32-bit layout. The function has no defined return value.
///
/// Calling convention: cdecl with one 32-bit handle argument. Calls to the
/// cleanup helper occur in ascending slot order.
lf_checker_rt::export!(cdecl, rw_00B0BA30(handle: u32) -> u32 {
    unsafe {
        const PICKUP_POOL_VA: u32 = 0x1615_660;
        const SLOT_STRIDE: u32 = 0x50;
        const SLOT_COUNT: u32 = 1_500;
        const HANDLE_OFFSET: u32 = 0x08;
        const TYPE_OFFSET: u32 = 0x44;
        const NETWORK_TYPE: u8 = 0x16;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let pool = lf_checker_rt::relocated(PICKUP_POOL_VA);
        let mut slot_index = 0u32;
        while slot_index < SLOT_COUNT {
            let slot = pool.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
            if read_u32(slot + HANDLE_OFFSET) == handle
                && read_u8(slot + TYPE_OFFSET) != NETWORK_TYPE
            {
                let _ = lf_checker_rt::callee_cdecl!(1, u32, slot_index);
            }
            slot_index += 1;
        }
    }
    0
});
