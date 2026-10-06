// original: 0x0087bfe0 crmt_node_base_init (proposed)
/// Initialise a blend-node object in place.
///
/// Clears the u16 at `this+0x04`, copies the low 16 bits of `word_arg` to the
/// u16 at `this+0x06`, zeroes the words at `+0x08..+0x1c`, stores the node
/// vtable pointer (file VA 0x00FE8504, relocated at load) at `+0x00`, and
/// zeroes the words at `+0x20..+0x2c`. The high 16 bits of `word_arg` are
/// ignored. Returns `this`.
///
/// Original: thiscall/1, no calls, no floating point.
export!(thiscall, rw_0087bfe0(this: u32, word_arg: u32) -> u32 {
    /// Node vtable (file VA; relocated at load).
    const NODE_VTABLE: u32 = 0x00FE8504;
    unsafe {
        let base = this as *mut u8;
        (base.add(0x04) as *mut u16).write_unaligned(0);
        (base.add(0x06) as *mut u16).write_unaligned((word_arg & 0xFFFF) as u16);
        for off in [0x08u32, 0x0C, 0x10, 0x14, 0x18, 0x1C] {
            (base.add(off as usize) as *mut u32).write_unaligned(0);
        }
        (base as *mut u32).write_unaligned(relocated(NODE_VTABLE));
        for off in [0x20u32, 0x24, 0x28, 0x2C] {
            (base.add(off as usize) as *mut u32).write_unaligned(0);
        }
        this
    }
});
