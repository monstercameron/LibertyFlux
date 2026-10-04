// original: 0x0087bb20 mirror_node_init (proposed)
/// Initialise a mirror node in place, doing nothing for a null pointer.
export!(cdecl, rw_0087bb20(obj: u32) -> u32 {
    /// Node tag word stored at offset 4.
    const TAG: u32 = 0x00080000;
    /// Mirror-node behaviour table (file VA).
    const MIRROR_TABLE: u32 = 0x00FE844C;
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
            p.write(relocated(MIRROR_TABLE));
            p.add(8).write(0);
        }
    }
    obj
});
