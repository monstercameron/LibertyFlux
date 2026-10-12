// original: 0x00D87780 input_stick_update_signs_and_emit

/// Updates the object's three direction fields from the low flag bit, stores
/// negative zero in the pending magnitude field, and returns early when the
/// disable byte is set, the linked state is null, or its mode mask is not
/// exactly 0x40. Otherwise it dispatches vtable slot 0x180 with five words in
/// the original stack order. The C++ routine is void; its incidental EAX is not
/// part of this proof, while all object writes and the outgoing call are checked.
lf_checker_rt::export!(cdecl, rw_00d87780(object: u32) -> () {
    const FLAGS: u32 = 0x2c;
    const DISABLE: u32 = 0x11a;
    const LINK: u32 = 0x16c;
    const MODE_FLAGS: u32 = 0x28;
    const POSITIVE_X: u32 = 0x1eb0;
    const FIXED_Y: u32 = 0x1eb4;
    const POSITIVE_Z: u32 = 0x1eb8;
    const PENDING_MAGNITUDE: u32 = 0x1ebc;
    const EMIT_DIRECTION: u32 = 1;

    let low_flags = unsafe {
        (object.wrapping_add(FLAGS) as *const u16).read_unaligned()
    };
    let positive = low_flags & 1 != 0;
    unsafe {
        (object.wrapping_add(POSITIVE_X) as *mut u32)
            .write_unaligned(if positive { 0x3f80_0000 } else { 0xbf80_0000 });
        (object.wrapping_add(FIXED_Y) as *mut u32).write_unaligned(0xbe99_999a);
        (object.wrapping_add(POSITIVE_Z) as *mut u32)
            .write_unaligned(if positive { 0x3f80_0000 } else { 0xbf80_0000 });
        (object.wrapping_add(PENDING_MAGNITUDE) as *mut u32).write_unaligned(0x8000_0000);
    }

    let disabled = unsafe {
        (object.wrapping_add(DISABLE) as *const u8).read_unaligned() & 1 != 0
    };
    if disabled { return; }

    let state = unsafe { (object.wrapping_add(LINK) as *const u32).read_unaligned() };
    if state == 0 { return; }
    let mode_flags = unsafe {
        (state.wrapping_add(MODE_FLAGS) as *const u32).read_unaligned()
    };
    if mode_flags & 0x3c0 != 0x40 { return; }

    let vtable = unsafe { (object as *const u32).read_unaligned() };
    let slot = unsafe { (vtable.wrapping_add(0x180) as *const u32).read_unaligned() };
    let callback: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(slot as usize) };
    let _ = callback(object, 0, 0, 1, 0, 0x33);
});
