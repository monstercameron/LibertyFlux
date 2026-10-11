// original: 0x00A34130 CEntity::update_flag_state

/// Shifts the entity flags at +0x24 to test bit 25. When set, it writes
/// state 4 at +0x41 and one at +0xbe; otherwise it writes state 2 at +0x41.
/// The shifted flag word remains in EAX and is returned. This is a no-argument
/// thiscall method.
lf_checker_rt::export!(thiscall, rw_00a34130(this: u32) -> u32 {
    const ENTITY_FLAGS: u32 = 0x24;
    const ENTITY_STATE: u32 = 0x41;
    const CONTACT_MARKER: u32 = 0xbe;
    const TEST_BIT_SHIFT: u32 = 25;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        let shifted_flags = read_u32(this + ENTITY_FLAGS) >> TEST_BIT_SHIFT;
        if shifted_flags & 1 != 0 {
            write_u8(this + ENTITY_STATE, 4);
            write_u8(this + CONTACT_MARKER, 1);
        } else {
            write_u8(this + ENTITY_STATE, 2);
        }
        shifted_flags
    }
});
