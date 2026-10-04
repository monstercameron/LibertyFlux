// original: 0x00b431b0 dispatch_word_unless_idle
/// Dispatch a pending word to the shared worker unless it is idle-marked.
///
/// Yields 0xFF00 when the object's word is 0xFFFF (only the low byte is
/// cleared on that path), else tail-calls the dispatcher with the word value.
export!(cdecl, rw_b431b0(obj: u32) -> u32 {
    unsafe {
        let v = (obj as *const u16).byte_add(0x64).read();
        if v == 0xFFFF {
            return 0xFF00;
        }
        callee_cdecl!(2, u32, v as u32)
    }
});
