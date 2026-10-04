// original: 0x0087b920 capture_node_attach (proposed)
/// Attach the capture-node behaviour table, doing nothing for null.
export!(cdecl, rw_0087b920(obj: u32) -> u32 {
    /// Capture-node behaviour table (file VA).
    const CAPTURE_TABLE: u32 = 0x00FE8410;
    if obj != 0 {
        unsafe {
            (obj as *mut u32).write(relocated(CAPTURE_TABLE));
        }
    }
    obj
});
