// original: 0x005e4440 Impl_CONVERT_THEN_ADD_STRING_TO_HTML_SCRIPT_OBJECT
// rw_005e4440: append a string to an HTML script object's text buffer.
//
// Resolves pool slot `index` (dead slots address slot 0, matching the
// original's flag test), measures the source string, copies it with its
// terminator at the end of the live text, and advances the live length.
// Returns the source pointer just past the copied terminator.
export!(stdcall, rw_005e4440(index: u32, src: *const u8) -> u32 {
    unsafe {
        let bitmap = *global::<u32>(POOL_BITMAP) as *const u8;
        let entry: u32 = if *bitmap.add(index as usize) & SLOT_DEAD != 0 {
            0
        } else {
            let stride = *global::<u32>(POOL_STRIDE);
            let base = *global::<u32>(POOL_BASE);
            base.wrapping_add(stride.wrapping_mul(index))
        };
        let mut len = 0u32;
        while *src.add(len as usize) != 0 {
            len = len.wrapping_add(1);
        }
        let used = *(entry.wrapping_add(OBJ_LEN) as *const u32);
        let dst = (entry.wrapping_add(OBJ_TEXT).wrapping_add(used)) as *mut u8;
        let mut i = 0usize;
        loop {
            let b = *src.add(i);
            *dst.add(i) = b;
            i += 1;
            if b == 0 {
                break;
            }
        }
        *(entry.wrapping_add(OBJ_LEN) as *mut u32) = used.wrapping_add(len);
        (src as u32).wrapping_add(len).wrapping_add(1)
    }
});
