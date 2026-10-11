// original: 0x009BF080 ped_event_cleanup_reset
/// Resets an event object passed by pointer. If its related entity has the
/// selected type bits, clears bit 3 of the entity byte at offset 0xf19.
/// When the child slot at object offset 0x88 is nonnull, calls the scripted
/// thiscall release helper with the child as ECX and the address of the slot
/// as its stack argument, then clears the slot.
/// Always clears the dword at object offset 0xc4 and byte at 0xcc. The
/// function has no semantic return value.
lf_checker_rt::export!(cdecl, rw_009bf080(object: u32) -> () {
    const ENTITY_POINTER_OFFSET: u32 = 0x1C;
    const CHILD_POINTER_OFFSET: u32 = 0x88;
    const TYPE_BITS_OFFSET: u32 = 0x28;
    const ENTITY_FLAGS_OFFSET: u32 = 0xF19;
    const MATCHED_TYPE_BITS: u32 = 0x80;
    const TYPE_MASK: u32 = 0x3C0;
    const CLEAR_FLAG: u8 = 0x08;

    if object == 0 {
        return;
    }

    let entity = unsafe { ((object + ENTITY_POINTER_OFFSET) as *const u32).read() };
    if entity != 0 {
        let type_bits = unsafe { ((entity + TYPE_BITS_OFFSET) as *const u32).read() } & TYPE_MASK;
        if type_bits == MATCHED_TYPE_BITS {
            let flags_address = (entity + ENTITY_FLAGS_OFFSET) as *mut u8;
            let flags = unsafe { flags_address.read() };
            unsafe { flags_address.write(flags & !CLEAR_FLAG) };
        }
    }

    let child_slot = object + CHILD_POINTER_OFFSET;
    let child = unsafe { (child_slot as *const u32).read() };
    if child != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, child, child_slot);
        unsafe { (child_slot as *mut u32).write(0) };
    }
    unsafe {
        ((object + 0xC4) as *mut u32).write(0);
        ((object + 0xCC) as *mut u8).write(0);
    }
});
