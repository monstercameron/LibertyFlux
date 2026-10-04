// original: 0x009e9220 ped_write_signature
/// Masks word 2 down to its top ten bits and writes the two-word
/// signature. The stack argument is ignored and no value is returned.
/// (thiscall, 1 arg.)
lf_checker_rt::export!(thiscall, rw_009e9220(this_ptr: u32, _unused: u32) -> u32 {
    unsafe {
        const W2_MASK: u32 = 0xFFFC0000;
        const SIG0: u32 = 0x8020001A;
        const SIG1: u32 = 0x10057D10;
        let w2 = (this_ptr.wrapping_add(8) as *const u32).read_unaligned();
        (this_ptr.wrapping_add(8) as *mut u32).write_unaligned(w2 & W2_MASK);
        (this_ptr as *mut u32).write_unaligned(SIG0);
        (this_ptr.wrapping_add(4) as *mut u32).write_unaligned(SIG1);
        0
    }
});
