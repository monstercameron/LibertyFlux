// original: 0x0087c170 crmt_node_reinit (proposed)
/// Re-initialise an existing node object, doing nothing for null.
///
/// Runs the base initializer (intercepted callee 1, thiscall/1) with word 5
/// over a non-null `obj`, then stamps the blend-node vtable (file VA
/// 0x00FE8544, relocated at load) and clears the flag byte at `+0x30`. The
/// callee preserves ecx (verified against its code: it never writes ecx),
/// which the contract models so the stores land on the object. Computes no
/// return value.
///
/// Original: cdecl/1, one direct call, no floating point.
export!(cdecl, rw_0087c170(obj: u32) -> u32 {
    /// Blend-node vtable (file VA; relocated at load).
    const BLEND_VTABLE: u32 = 0x00FE8544;
    /// Word passed to the base initializer.
    const INIT_WORD: u32 = 5;
    /// Flag byte cleared after the stamp.
    const FLAG_OFF: u32 = 0x30;
    unsafe {
        if obj != 0 {
            callee_thiscall!(1, u32, obj, INIT_WORD);
            (obj as *mut u32).write_unaligned(relocated(BLEND_VTABLE));
            ((obj + FLAG_OFF) as *mut u8).write(0);
        }
        0
    }
});
