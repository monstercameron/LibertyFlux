// original: 0x0087b8d0 capture_node_init (proposed)
/// Initialise a capture node in place, doing nothing for a null pointer.
///
/// The null-guarded form of the capture constructor: tag word, cleared
/// state, attached behaviour table, cleared flag byte. Returns the pointer.
export!(cdecl, rw_0087b8d0(obj: u32) -> u32 {
    /// Node tag word stored at offset 4.
    const TAG: u32 = 0x00140000;
    /// Capture-node behaviour table (file VA).
    const CAPTURE_TABLE: u32 = 0x00FE8410;
    if obj != 0 {
        unsafe {
            let p = obj as *mut u32;
            p.add(1).write(TAG);
            p.add(2).write(0);
            p.add(3).write(0);
            p.add(4).write(0);
            p.add(5).write(0);
            p.add(6).write(0);
            p.add(7).write(0);
            p.write(relocated(CAPTURE_TABLE));
            p.add(8).write(0);
            ((obj + 0x24) as *mut u8).write(0);
        }
    }
    obj
});
