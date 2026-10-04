// original: 0x005e44a0 pool_append_converted_string
// rw_005e44a0: convert a string into a pool slot's text buffer.
//
// Resolves pool slot `index`, runs the source through the converter step,
// appends the converted bytes at the end of the live text, and advances the
// live length by twice the converted length. Returns the appender's answer.
export!(stdcall, rw_005e44a0(index: u32, arg: u32) -> u32 {
    unsafe {
        let bitmap = *global::<u32>(POOL_BITMAP) as *const u8;
        let entry: u32 = if *bitmap.add(index as usize) & SLOT_DEAD != 0 {
            0
        } else {
            let stride = *global::<u32>(POOL_STRIDE);
            let base = *global::<u32>(POOL_BASE);
            base.wrapping_add(stride.wrapping_mul(index))
        };
        let n: u32 = callee_cdecl!(1, u32, arg);
        let curlen = *(entry.wrapping_add(OBJ_LEN) as *const u32);
        let dest = entry.wrapping_add(OBJ_TEXT).wrapping_add(curlen);
        let r: u32 = callee_cdecl!(2, u32, dest, arg, 0xFFFFFFFF);
        *(entry.wrapping_add(OBJ_LEN) as *mut u32) =
            curlen.wrapping_add(n.wrapping_add(n));
        r
    }
});
