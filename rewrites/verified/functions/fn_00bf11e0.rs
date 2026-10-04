// original: 0x00bf11e0 copy_indexed_triple_30
/// Copy one indexed triple from `this` to `dst+0x30`.
///
/// Copies the three words at `this + idx*12 + 0x4c/0x50/0x54` (`idx` is the
/// low word of the last argument) to `dst+0x30/0x34/0x38`. Returns the last
/// copied word.
export!(thiscall, rw_bf11e0(this_obj: u32, dst: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xffff) as u32;
        let base = this_obj + idx * 12;
        let w0 = *((base + 0x4c) as *const u32);
        let w1 = *((base + 0x50) as *const u32);
        let w2 = *((base + 0x54) as *const u32);
        *((dst + 0x30) as *mut u32) = w0;
        *((dst + 0x34) as *mut u32) = w1;
        *((dst + 0x38) as *mut u32) = w2;
        w2
    }
});
