// original: 0x0087b790 capture_node_construct (proposed)
/// Construct a capture node at the given address (thiscall constructor).
///
/// Stores the tag word, clears the state words, attaches the capture-node
/// behaviour table and clears the trailing flag byte. Returns `this`.
export!(thiscall, rw_0087b790(this: u32) -> u32 {
    /// Node tag word stored at offset 4.
    const TAG: u32 = 0x00140000;
    /// Capture-node behaviour table (file VA).
    const CAPTURE_TABLE: u32 = 0x00FE8410;
    unsafe {
        let p = this as *mut u32;
        p.add(1).write(TAG);
        p.add(2).write(0);
        p.add(3).write(0);
        p.add(4).write(0);
        p.add(5).write(0);
        p.add(6).write(0);
        p.add(7).write(0);
        p.write(relocated(CAPTURE_TABLE));
        p.add(8).write(0);
        ((this + 0x24) as *mut u8).write(0);
    }
    this
});
