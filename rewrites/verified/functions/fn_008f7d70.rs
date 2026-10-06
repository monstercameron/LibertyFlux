// original: 0x008f7d70 input_stack_push_slot (proposed)

/// Push a value onto the device's inline slot stack.
///
/// `obj` is the input device object. The dword at `+0x28` is the stack
/// depth (a signed count); the slots live at `+0x2C` upwards. The function
/// stores `v` at the slot the current depth points at and then increments
/// the depth. EAX still holds `v` at return. The contract pins the depth
/// to 0..3 so the indexed slot stays inside the fabricated object.
///
/// Thiscall: object in ECX, value as one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_008f7d70(obj: u32, v: u32) -> u32 {
    unsafe {
        const DEPTH_OFF: u32 = 0x28;
        const SLOTS_OFF: u32 = 0x2c;
        let depth = ((obj + DEPTH_OFF) as *const u32).read_unaligned();
        ((obj + SLOTS_OFF + depth.wrapping_mul(4)) as *mut u32).write_unaligned(v);
        ((obj + DEPTH_OFF) as *mut u32).write_unaligned(depth.wrapping_add(1));
        v
    }
});
