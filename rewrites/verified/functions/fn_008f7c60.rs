// original: 0x008f7c60 input_stack_pop_slot (proposed)

/// Pop the top entry of the device's inline slot stack, clearing it.
///
/// `obj` is the input device object. The dword at `+0x28` is the stack
/// depth (a signed count the function decrements first); the slots live at
/// `+0x2C` upwards. The function decrements the depth, zeroes the slot the
/// new depth points at, and returns the new depth. The caller guarantees a
/// positive depth: with 0 the index would wrap and fault, so the contract
/// pins the depth to 1..4.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7c60(obj: u32) -> u32 {
    unsafe {
        const DEPTH_OFF: u32 = 0x28;
        const SLOTS_OFF: u32 = 0x2c;
        let depth = ((obj + DEPTH_OFF) as *const u32).read_unaligned();
        let depth = depth.wrapping_sub(1);
        ((obj + DEPTH_OFF) as *mut u32).write_unaligned(depth);
        ((obj + SLOTS_OFF + depth.wrapping_mul(4)) as *mut u32).write_unaligned(0);
        depth
    }
});
