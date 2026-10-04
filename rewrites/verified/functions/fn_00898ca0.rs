// original: 0x00898ca0 audio_slot_alloc
/// Allocates one slot in a strided audio pool and links it into the table.
///
/// Fails with 0 when the used count has reached capacity. Otherwise bumps the
/// count, initialises the new fixed-stride element through the pool setup
/// call, records the element pointer and tag in the index table, and returns
/// the element pointer.
export!(thiscall, rw_00898ca0(this: u32, tag: u32, flags: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x70;
        let count = core::ptr::read_unaligned(this.wrapping_add(0x50) as *const u32);
        let cap = core::ptr::read_unaligned(this.wrapping_add(0x48) as *const u32);
        if count >= cap {
            return 0;
        }
        core::ptr::write_unaligned(
            this.wrapping_add(0x50) as *mut u32,
            count.wrapping_add(1),
        );
        let pool = core::ptr::read_unaligned(global::<u32>(0x115F810));
        let slot = pool.wrapping_add(count.wrapping_mul(STRIDE));
        callee_thiscall!(1, u32, slot, tag, this, flags);
        let table = core::ptr::read_unaligned(this.wrapping_add(0x4c) as *const u32);
        let cell = table.wrapping_add(count.wrapping_mul(8));
        core::ptr::write_unaligned(cell.wrapping_add(4) as *mut u32, tag);
        core::ptr::write_unaligned(cell as *mut u32, slot);
        slot
    }
});
