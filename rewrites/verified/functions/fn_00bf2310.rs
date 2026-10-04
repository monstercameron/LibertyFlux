// original: 0x00bf2310 store_triple_to_table
/// Store a source triple into one indexed table slot of `this`.
///
/// Inverse of `rw_bf11e0`: writes `src+0x30/0x34/0x38` to
/// `this + idx*12 + 0x4c/0x50/0x54` (`idx` is the low word of the last
/// argument). Returns the last stored word.
export!(thiscall, rw_bf2310(this_obj: u32, src: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xffff) as u32;
        let base = this_obj + idx * 12;
        let w0 = *((src + 0x30) as *const u32);
        let w1 = *((src + 0x34) as *const u32);
        let w2 = *((src + 0x38) as *const u32);
        *((base + 0x4c) as *mut u32) = w0;
        *((base + 0x50) as *mut u32) = w1;
        *((base + 0x54) as *mut u32) = w2;
        w2
    }
});
