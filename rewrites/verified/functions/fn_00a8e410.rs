// original: 0x00a8e410 CInteriorInst::vf27

/// Copy the eight words at +0xC0..+0xDC out to the caller's buffer.
///
/// `this` points to the interior instance, `dst` receives eight words (some
/// move through vector registers in the original, which is unobservable).
/// eax holds the last word on return, as in the original.
///
/// Original: 0x00A8E410 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8e410(this: u32, dst: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0xc0;
        let mut i = 0u32;
        while i < 8 {
            let w = ((this + BASE + i * 4) as *const u32).read_unaligned();
            ((dst + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
        ((dst + 28) as *const u32).read_unaligned()
    }
});
