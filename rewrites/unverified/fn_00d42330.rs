// original: 0x00D42330 task_node_44_init

/// Initialize the inline fields of a task node. The object arrives in ECX;
/// the original clears its links and counters, preserves only the high bits
/// of the word at +0x1c, sets the default duration to 1.0, sets the id fields
/// to all ones, and returns the object pointer in EAX.
lf_checker_rt::export!(thiscall, rw_D42330(this: u32) -> u32 {
    const LINK_A: u32 = 4;
    const LINK_B: u32 = 8;
    const NODE_FLAGS: u32 = 0x1c;
    const NODE_VALUE: u32 = 0x24;
    const NODE_AUX: u32 = 0x28;
    const NODE_BYTE: u32 = 0x14;
    const NODE_STATE: u32 = 0x18;
    const NODE_ID: u32 = 0x20;
    const NODE_TYPE: u32 = 0;
    const NODE_COUNT: u32 = 0x0c;
    const NODE_DURATION: u32 = 0x10;
    const PRESERVED_MASK: u32 = 0xffff_fc00;
    unsafe {
        core::ptr::write_unaligned((this + LINK_A) as *mut u32, 0);
        core::ptr::write_unaligned((this + LINK_B) as *mut u32, 0);
        let flags = (this + NODE_FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() & PRESERVED_MASK);
        core::ptr::write_unaligned((this + NODE_VALUE) as *mut u32, 0);
        core::ptr::write_unaligned((this + NODE_AUX) as *mut u32, 0);
        core::ptr::write((this + NODE_BYTE) as *mut u8, 0);
        core::ptr::write_unaligned((this + NODE_STATE) as *mut u32, 0);
        core::ptr::write_unaligned((this + NODE_ID) as *mut u32, u32::MAX);
        core::ptr::write_unaligned((this + NODE_TYPE) as *mut u32, u32::MAX);
        core::ptr::write_unaligned((this + NODE_COUNT) as *mut u32, 1);
        core::ptr::write_unaligned((this + NODE_DURATION) as *mut u32, 0x3f80_0000);
    }
    this
});
