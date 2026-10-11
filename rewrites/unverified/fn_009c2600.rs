// original: 0x009C2600 stream_shared_vector4_store

/// Copy four float words from input into the shared streaming vector, set its
/// ready byte, and return input. The values are copied bit for bit, so signed
/// zero and NaN payloads are preserved without arithmetic.
lf_checker_rt::export!(cdecl, rw_009c2600(input: u32) -> u32 {
    unsafe {
        const READY_FLAG: u32 = 0x012f_b25c;
        const SHARED_VECTOR: u32 = 0x0103_e8b0;
        lf_checker_rt::global::<u8>(READY_FLAG).write(1);
        let source = input as usize as *const u32;
        for word in 0..4usize {
            let bits = source.add(word).read_unaligned();
            lf_checker_rt::global::<u32>(SHARED_VECTOR.wrapping_add((word as u32) * 4))
                .write_unaligned(bits);
        }
        input
    }
});
