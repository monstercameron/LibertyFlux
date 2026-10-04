// original: 0x00bf1210 copy_indexed_triple_0
/// Copy one indexed triple from `this` to the start of `dst`.
///
/// Same table as `rw_bf11e0` but the destination is `dst+0/4/8`. Returns the
/// last copied word.
export!(thiscall, rw_bf1210(this_obj: u32, dst: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xffff) as u32;
        let base = this_obj + idx * 12;
        let w0 = *((base + 0x4c) as *const u32);
        let w1 = *((base + 0x50) as *const u32);
        let w2 = *((base + 0x54) as *const u32);
        *(dst as *mut u32) = w0;
        *((dst + 4) as *mut u32) = w1;
        *((dst + 8) as *mut u32) = w2;
        w2
    }
});
