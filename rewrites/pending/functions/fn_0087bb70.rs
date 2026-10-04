// original: 0x0087bb70 mirror_node_attach (proposed)
/// Attach the mirror-node behaviour table, doing nothing for null.
export!(cdecl, rw_0087bb70(obj: u32) -> u32 {
    /// Mirror-node behaviour table (file VA).
    const MIRROR_TABLE: u32 = 0x00FE844C;
    if obj != 0 {
        unsafe {
            (obj as *mut u32).write(relocated(MIRROR_TABLE));
        }
    }
    obj
});
