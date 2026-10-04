// original: 0x0087b700 frame_node_attach (proposed)
/// Attach the frame-node behaviour table to an object, if one was given.
///
/// When `obj` is null there is nothing to attach to and the function just
/// returns it. Otherwise it stores the frame-node table address into the
/// object's first word and returns the object.
export!(cdecl, rw_0087b700(obj: u32) -> u32 {
    /// Frame-node behaviour table (file VA).
    const FRAME_TABLE: u32 = 0x00FE83D4;
    if obj != 0 {
        unsafe {
            (obj as *mut u32).write(relocated(FRAME_TABLE));
        }
    }
    obj
});
