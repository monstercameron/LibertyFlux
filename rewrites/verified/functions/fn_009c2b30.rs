// original: 0x009C2B30 stream_context_vector_copy

/// Copy the first three dwords of the context selected by index into
/// destination. If the table slot is null, write three zero dwords instead.
/// Return the destination pointer. The global table address is resolved from
/// the image base so the implementation works at any mapped address.
lf_checker_rt::export!(cdecl, rw_009c2b30(destination: u32, index: u32) -> u32 {
    unsafe {
        const CONTEXT_TABLE: u32 = 0x0129_3e58;
        const WORDS: usize = 3;
        let slot_va = CONTEXT_TABLE.wrapping_add(index.wrapping_mul(4));
        let context = lf_checker_rt::global::<u32>(slot_va).read_unaligned();
        let output = destination as usize as *mut u32;
        if context == 0 {
            for word in 0..WORDS {
                output.add(word).write_unaligned(0);
            }
        } else {
            let input = context as usize as *const u32;
            for word in 0..WORDS {
                output.add(word).write_unaligned(input.add(word).read_unaligned());
            }
        }
        destination
    }
});
