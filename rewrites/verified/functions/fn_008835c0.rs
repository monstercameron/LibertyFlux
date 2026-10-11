// original: 0x008835C0 streaming_claim_next_slot_bit

/// Mark and return the first unoccupied 32-slot group in the streaming
/// bitmap. The 16-bit group count, bitmap pointer, and slot base come from
/// relocated image globals. A zero count or an entirely occupied bitmap
/// returns zero; selecting group `i` writes its bit and returns
/// `base + i * 32` with 32-bit wrapping. The function brackets the scan with
/// the streaming lock constructor and destructor. The checker contract
/// scripts those calls and snapshots their two-word stack-local guard.
lf_checker_rt::export!(cdecl, rw_008835c0() -> u32 {
    unsafe {
        const LOCK_OBJECT_VA: u32 = 0x0115_40A0;
        const BITMAP_POINTER_VA: u32 = 0x0115_4098;
        const BITMAP_COUNT_VA: u32 = 0x0115_409E;
        const SLOT_BASE_VA: u32 = 0x0115_4064;
        const WORD_BITS: u32 = 32;
        const WORD_BYTES: u32 = 4;

        #[inline(always)]
        unsafe fn read_global_u32(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_global_u16(file_va: u32) -> u16 {
            unsafe { lf_checker_rt::global::<u16>(file_va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut lock_guard = [0u32; 2];
        let guard_address = lock_guard.as_mut_ptr() as u32;
        let lock_address = lf_checker_rt::relocated(LOCK_OBJECT_VA);
        let _lock_result = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            guard_address,
            lock_address
        );

        let group_count = read_global_u16(BITMAP_COUNT_VA) as u32;
        let bitmap = read_global_u32(BITMAP_POINTER_VA);
        let slot_base = read_global_u32(SLOT_BASE_VA);
        for group_index in 0..group_count {
            let word_address = bitmap.wrapping_add(
                (group_index / WORD_BITS).wrapping_mul(WORD_BYTES)
            );
            let occupied = read_word(word_address);
            let bit = 1u32 << (group_index & (WORD_BITS - 1));
            if occupied & bit == 0 {
                write_word(word_address, occupied | bit);
                let selected = slot_base.wrapping_add(group_index.wrapping_mul(WORD_BITS));
                let _unlock_result = lf_checker_rt::callee_thiscall!(2, u32, guard_address);
                return selected;
            }
        }

        let _unlock_result = lf_checker_rt::callee_thiscall!(2, u32, guard_address);
        0
    }
});
