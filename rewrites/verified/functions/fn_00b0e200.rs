// original: 0x00B0E200 Pickup_ProcessSpecialStates

/// Visit all 1,500 pickup slots in ascending order and process the slot when
/// its type byte is either 0x16 or 0x17. Each matching slot is passed as the
/// this pointer to the cleanup helper. The pool uses a 0x50-byte stride and
/// the type byte is at offset 0x44.
///
/// Calling convention: cdecl with no arguments and no return value. Cleanup
/// calls occur in ascending slot order.
lf_checker_rt::export!(cdecl, rw_00B0E200() -> u32 {
    const TYPE_ARRAY_VA: u32 = 0x0161_56a4;
    const SLOT_STRIDE: u32 = 0x50;
    const TYPE_OFFSET: u32 = 0x44;
    const SLOT_COUNT: u32 = 1_500;
    const SPECIAL_TYPE: u8 = 0x16;
    const NETWORK_TYPE: u8 = 0x17;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    let first_type = lf_checker_rt::relocated(TYPE_ARRAY_VA);
    let mut slot_index = 0u32;
    while slot_index < SLOT_COUNT {
        let type_address = first_type.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
        let slot_type = unsafe { read_u8(type_address) };
        if slot_type == SPECIAL_TYPE || slot_type == NETWORK_TYPE {
            let pickup = type_address.wrapping_sub(TYPE_OFFSET);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, pickup);
        }
        slot_index += 1;
    }
    0
});
