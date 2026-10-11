// original: 0x00C10C20 streaming_request_pair_vectors

/// Construct a paired-vector request record. The two source blocks are
/// copied into destination ranges +0x10 and +0x20, while +0/+4 contain the
/// request kind and current stream serial. The routine accepts two pointers
/// on the stack and returns with those eight bytes cleaned up.
lf_checker_rt::export!(thiscall, rw_00c10c20(dst: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const STREAM_SERIAL: u32 = 0x0117_35A4;
        const KIND: u8 = 0x74;
        const SERIAL: u32 = 4;
        const FIRST_VECTOR: u32 = 0x10;
        const SECOND_VECTOR: u32 = 0x20;
        let serial = lf_checker_rt::global::<u32>(STREAM_SERIAL).read_unaligned();
        (dst as *mut u8).write(KIND);
        ((dst + SERIAL) as *mut u32).write_unaligned(serial);
        (dst as *mut u8).add(1).write(0);
        for word in 0..4 {
            let a = ((first + word * 4) as *const u32).read_unaligned();
            let b = ((second + word * 4) as *const u32).read_unaligned();
            ((dst + FIRST_VECTOR + word * 4) as *mut u32).write_unaligned(a);
            ((dst + SECOND_VECTOR + word * 4) as *mut u32).write_unaligned(b);
        }
    }
    0
});
