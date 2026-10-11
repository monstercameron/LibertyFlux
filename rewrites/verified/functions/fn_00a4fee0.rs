// original: 0x00A4FEE0 vehicle_init_slots

/// Initializes all 256 vehicle slots in order by calling the slot helper with
/// the slot at offset 0x100 plus 0xE0 bytes per index. The helper receives nine
/// stack words, including a pointer to an otherwise unused local scratch area.
/// After the calls, the owner receives a zero count, a fixed sentinel, and a
/// cleared 4,096-word list. The final zero-argument helper is called before the
/// function returns true in AL. The owner layout and cleanup are 32-bit.
lf_checker_rt::export!(thiscall, rw_vehicle_init_slots(this: u32, _unused: u32) -> u8 {
    const SLOT_BASE: u32 = 0x0100;
    const SLOT_STRIDE: u32 = 0x00E0;
    const SLOT_COUNT: u32 = 256;
    const COUNT_FIELD: u32 = 0x12104;
    const SENTINEL_FIELD: u32 = 0x0E100;
    const LIST_BASE: u32 = 0x0E104;
    const LIST_WORDS: u32 = 4096;
    const SENTINEL: u32 = 0x000F423F;

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let mut scratch = [0u32; 8];
    let scratch_pointer = scratch.as_mut_ptr() as u32;
    let mut index = 0u32;
    while index < SLOT_COUNT {
        let slot = this
            .wrapping_add(SLOT_BASE)
            .wrapping_add(index.wrapping_mul(SLOT_STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(
            1, u32, slot, 0, scratch_pointer, 0, 0, 0, 0, 0, 0x64, 0
        );
        index += 1;
    }

    unsafe { write_u32(this.wrapping_add(COUNT_FIELD), 0) };
    unsafe { write_u32(this.wrapping_add(SENTINEL_FIELD), SENTINEL) };
    let mut word_index = 0u32;
    while word_index < LIST_WORDS {
        let address = this
            .wrapping_add(LIST_BASE)
            .wrapping_add(word_index.wrapping_mul(4));
        unsafe { write_u32(address, 0) };
        word_index += 1;
    }

    let _ = lf_checker_rt::callee_cdecl!(2, u32,);
    1
});
