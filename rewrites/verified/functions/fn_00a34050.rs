// original: 0x00A34050 CEntity::set_network_visibility

/// Calls the network object's virtual validation method, then sends the
/// visibility mode and value to the fixed setter when validation succeeds.
/// The network object is stored in the entity at +0x6c, and its vtable's
/// second slot is at +4. This is a 32-bit thiscall method with one incoming
/// stack word; the fixed helper receives `(network_object, 1, value)` and
/// its return value is returned. The null-network-object path is excluded by
/// the proof contract because the original preserves its incoming EAX there.
lf_checker_rt::export!(thiscall, rw_00a34050(this: u32, visibility: u32) -> u32 {
    const NETWORK_OBJECT: u32 = 0x6c;
    const VALIDATE_SLOT: u32 = 0x04;
    const VISIBILITY_MODE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let network_object = read_u32(this + NETWORK_OBJECT);
        if network_object == 0 {
            return 0;
        }

        let vtable = read_u32(network_object);
        let validate_address = read_u32(vtable + VALIDATE_SLOT);
        let validate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(validate_address as usize);
        if validate(network_object) == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, network_object, VISIBILITY_MODE, visibility)
        }
    }
});
