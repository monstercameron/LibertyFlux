// original: 0x00dda930 UIBasicClip::vf130

/// Copy one word from outside into the clip's submit part.
///
/// `this` is the clip object and `src` points at the word to copy. The
/// inner object at `this + SUBMIT (+0x1e0)` is loaded, the word at `*src` is
/// loaded, and it is stored at `inner + SUBMIT`; the copied word is returned
/// in EAX. No calls are made.
///
/// Original: thiscall, one stack word (the source pointer), callee pops it
/// (the callee pops 4 bytes), word result in EAX.
lf_checker_rt::export!(thiscall, rw_00dda930(this: u32, src: u32) -> u32 {
    const SUBMIT: u32 = 0x1e0;
    unsafe {
        let inner = ((this + SUBMIT) as *const u32).read_unaligned();
        let v = (src as *const u32).read_unaligned();
        ((inner + SUBMIT) as *mut u32).write_unaligned(v);
        v
    }
});
