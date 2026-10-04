// original: 0x008d64c0 sample_pair_to_outs
// Samples a source record twice: the record's third word is remapped into
// the first out-slot, and the record pointer itself is converted into the
// second out-slot. No meaningful return value (the original leaves the
// second out-pointer in eax), so this returns 0.
export!(cdecl, rw_008d64c0(src: *const u32, out_a: *mut u32, out_b: *mut u32) -> u32 {
    unsafe {
        let s = *src.wrapping_add(2);
        let r1: u32 = callee_cdecl!(1, u32, s);
        *out_a = r1;
        let r2: u32 = callee_cdecl!(2, u32, src as u32);
        *out_b = r2;
        0
    }
});
