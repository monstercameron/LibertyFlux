// original: 0x00B0C5C0 Pickup_SlotFromHandle

/// Resolve a pickup handle to its slot index. The low 16 bits select a slot
/// in the 0x50-byte pickup pool, and the high 16 bits must equal that slot's
/// generation word. The all-ones handle and any stale generation return
/// `u32::MAX`. The original performs no slot bounds check, so callers must
/// supply a valid pool index unless using the all-ones sentinel.
///
/// Calling convention: cdecl with one 32-bit handle; returns the index or the
/// all-ones invalid sentinel in EAX.
lf_checker_rt::export!(cdecl, rw_00B0C5C0(handle: u32) -> u32 {
    const INVALID_HANDLE: u32 = u32::MAX;
    const SLOT_STRIDE: u32 = 0x50;
    const GENERATION_WORD_VA: u32 = 0x0161_56a2;

    if handle == INVALID_HANDLE {
        return INVALID_HANDLE;
    }

    let slot_index = handle as u16 as u32;
    let requested_generation = (handle >> 16) as u16;
    let generation_address = lf_checker_rt::relocated(
        GENERATION_WORD_VA.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE)),
    );
    let current_generation = unsafe { (generation_address as *const u16).read_unaligned() };
    if current_generation == requested_generation {
        slot_index
    } else {
        INVALID_HANDLE
    }
});
