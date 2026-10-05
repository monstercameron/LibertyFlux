// original: 0x009426a0 streaming_struct_copy_48 (proposed)

/// Copy an 18-word descriptor from the source to the object.
///
/// Copies 0x48 bytes word by word from the stack argument to `this`.
/// Returns the last word copied in `eax`.
///
/// Original: 0x009426a0 (thiscall, one stack argument; callee pops 4).
lf_checker_rt::export!(thiscall, rw_009426a0(this: u32, src: u32) -> u32 {
    unsafe {
        const WORDS: u32 = 18;
        let mut last = 0u32;
        let mut i = 0u32;
        while i < WORDS {
            last = ((src + i * 4) as *const u32).read_unaligned();
            ((this + i * 4) as *mut u32).write_unaligned(last);
            i += 1;
        }
        last
    }
});
