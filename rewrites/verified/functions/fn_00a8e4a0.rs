// original: 0x00a8e4a0 CInteriorInst::vf20

/// Copy the four words at +0x90..+0x9C out to the caller's buffer.
///
/// `this` points to the interior instance, `dst` receives four words (the
/// middle two move through vector registers in the original, which is
/// unobservable). eax holds the last word on return, as in the original.
///
/// Original: 0x00A8E4A0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8e4a0(this: u32, dst: u32) -> u32 {
    unsafe {
        const FIELD0: u32 = 0x90;
        const FIELD1: u32 = 0x94;
        const FIELD2: u32 = 0x98;
        const FIELD3: u32 = 0x9c;
        let w0 = ((this + FIELD0) as *const u32).read_unaligned();
        let w1 = ((this + FIELD1) as *const u32).read_unaligned();
        let w2 = ((this + FIELD2) as *const u32).read_unaligned();
        let w3 = ((this + FIELD3) as *const u32).read_unaligned();
        (dst as *mut u32).write_unaligned(w0);
        ((dst + 4) as *mut u32).write_unaligned(w1);
        ((dst + 8) as *mut u32).write_unaligned(w2);
        ((dst + 12) as *mut u32).write_unaligned(w3);
        w3
    }
});
