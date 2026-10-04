// original: 0x008d8140 remap_float_triple
// Runs each of three floats through a shared remap helper with a tag,
// storing each result back. No meaningful return value (the original
// leaves entry garbage in eax), so this returns 0.
export!(cdecl, rw_008d8140(vec: *mut u32, tag: u32) -> u32 {
    unsafe {
        for i in 0..3usize {
            let v = *vec.wrapping_add(i);
            let r: u32 = callee_cdecl!(1, u32, v, tag);
            *vec.wrapping_add(i) = r;
        }
        0
    }
});
