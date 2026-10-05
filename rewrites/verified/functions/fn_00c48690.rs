// original: 0x00c48690 ccamcinematic_copy_ctor (proposed)
/// Copy-construct a cinematic camera from `src`.
///
/// Runs the base copy routine (callee 1, called with `this` in ecx
/// and `src` on the stack), then copies the body: 21 words at
/// `BODY_A`, 21 words at `BODY_B`, 7 words of scalars, and 4 trailing
/// bytes. Returns `this`.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c48690(this: u32, src: u32) -> u32 {
    const BODY_A: u32 = 0x140;
    const BODY_WORDS: u32 = 0x15;
    const BODY_B: u32 = 0x194;
    const SCALARS: u32 = 0x1e8;
    const SCALAR_WORDS: u32 = 7;
    const TAIL: u32 = 0x204;
    const TAIL_BYTES: u32 = 4;
    const BASE_COPY: u32 = 1;
    unsafe {
        lf_checker_rt::callee_thiscall!(BASE_COPY, u32, this, src);
        let mut i = 0u32;
        while i < BODY_WORDS {
            let w = ((src + BODY_A + i * 4) as *const u32).read_unaligned();
            ((this + BODY_A + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
        i = 0;
        while i < BODY_WORDS {
            let w = ((src + BODY_B + i * 4) as *const u32).read_unaligned();
            ((this + BODY_B + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
        i = 0;
        while i < SCALAR_WORDS {
            let w = ((src + SCALARS + i * 4) as *const u32).read_unaligned();
            ((this + SCALARS + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
        i = 0;
        while i < TAIL_BYTES {
            let b = ((src + TAIL + i) as *const u8).read();
            ((this + TAIL + i) as *mut u8).write(b);
            i += 1;
        }
    }
    this
});
