// original: 0x0093f090 stream_selected_triplet (proposed)

/// Write the selected object's three position words into `buf`.
///
/// Calls the selected-slot getter: a null object zeroes the three words,
/// otherwise copies the words at offsets 0x30/0x34/0x38 of the inner
/// object found at offset `OBJ_INNER`. Returns `buf`.
///
/// Original: 0x0093f090 (cdecl, one stack word; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093f090(buf: u32) -> u32 {
    const SELECTED_GETTER: u32 = 1;
    const OBJ_INNER: u32 = 0x20;
    unsafe {
        let obj: u32 = lf_checker_rt::callee_cdecl!(SELECTED_GETTER, u32,);
        if obj == 0 {
            for off in [0u32, 4, 8] {
                (buf.wrapping_add(off) as *mut u32).write_unaligned(0);
            }
        } else {
            let inner = ((obj + OBJ_INNER) as *const u32).read_unaligned();
            for (i, off) in [0x30u32, 0x34, 0x38].iter().enumerate() {
                let w = ((inner + *off) as *const u32).read_unaligned();
                (buf.wrapping_add(i as u32 * 4) as *mut u32).write_unaligned(w);
            }
        }
        buf
    }
});
