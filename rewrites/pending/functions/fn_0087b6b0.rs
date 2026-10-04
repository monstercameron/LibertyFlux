// original: 0x0087b6b0 frame_node_init (proposed)
/// Initialise a frame node in place, doing nothing for a null pointer.
///
/// Stores the node's tag word, clears its state words, attaches the
/// frame-node behaviour table and clears the trailing flag byte. Returns
/// the pointer it was given.
export!(cdecl, rw_0087b6b0(obj: u32) -> u32 {
    /// Node tag word stored at offset 4.
    const TAG: u32 = 0x00090000;
    /// Frame-node behaviour table (file VA).
    const FRAME_TABLE: u32 = 0x00FE83D4;
    if obj != 0 {
        unsafe {
            let p = obj as *mut u32;
            p.add(1).write(TAG);
            p.add(2).write(0);
            p.add(3).write(0);
            p.add(4).write(0);
            p.add(5).write(0);
            p.add(6).write(0);
            p.write(relocated(FRAME_TABLE));
            p.add(7).write(0);
            ((obj + 0x20) as *mut u8).write(0);
        }
    }
    obj
});
