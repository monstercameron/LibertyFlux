// original: 0x0087c190 crmt_node_set_vtable (proposed)
/// Store the blend-node vtable pointer into an object, doing nothing for null.
///
/// When `obj` is non-null, writes the relocated address of the blend-node
/// vtable (file VA 0x00FE8544) to `obj+0x00` and returns `obj` unchanged.
/// A null argument takes no action and is returned as-is.
///
/// Original: cdecl/1, no calls, no floating point.
export!(cdecl, rw_0087c190(obj: u32) -> u32 {
    /// Blend-node vtable (file VA; relocated at load).
    const BLEND_VTABLE: u32 = 0x00FE8544;
    if obj != 0 {
        unsafe {
            (obj as *mut u32).write_unaligned(relocated(BLEND_VTABLE));
        }
    }
    obj
});
