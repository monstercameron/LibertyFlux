// original: 0x00642950 rage::ptxDomain::vf4

/// Set the three-word result buffer supplied as the first stack argument to all zero bits. The second stack argument and ECX are unused; return the output pointer.
lf_checker_rt::export!(thiscall, rw_00642950(_this: u32, out: *mut u32, _ignored: u32) -> *mut u32 {
    unsafe {
        const RESULT_WORDS: usize = 3;
        for index in 0..RESULT_WORDS {
            out.add(index).write_unaligned(0);
        }
    }
    out
});
