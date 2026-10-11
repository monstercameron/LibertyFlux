// original: 0x00C11290 streaming_request_compose_vector_and_weight

/// Build a composite streaming record. Its optional object id occupies +8;
/// scalar inputs occupy +0x0c, +0x10, and +0x14; a vector block is copied to
/// +0x20..+0x2c and a float bit pattern is copied to +0x14. Kind 0x80 and the
/// current serial are always written. The six stack arguments are thiscall.
lf_checker_rt::export!(thiscall, rw_00c11290(dst: u32, object: u32, tag: u32, mode: u32, vector: u32, weight: u32, request_id: u32) -> u32 {
    unsafe {
        const STREAM_SERIAL: u32 = 0x0117_35A4;
        const KIND: u8 = 0x80;
        let serial = lf_checker_rt::global::<u32>(STREAM_SERIAL).read_unaligned();
        (dst as *mut u8).write(KIND);
        ((dst + 4) as *mut u32).write_unaligned(serial);
        let object_id = if object == 0 { u32::MAX } else { ((object + 0x64) as *const u32).read_unaligned() };
        ((dst + 8) as *mut u32).write_unaligned(object_id);
        (dst as *mut u8).add(1).write(mode as u8);
        ((dst + 0x10) as *mut u32).write_unaligned(tag);
        (dst as *mut u8).add(2).write(0);
        for word in 0..4 {
            let bits = ((vector + word * 4) as *const u32).read_unaligned();
            ((dst + 0x20 + word * 4) as *mut u32).write_unaligned(bits);
        }
        ((dst + 0x14) as *mut u32).write_unaligned(weight);
        ((dst + 0x0c) as *mut u32).write_unaligned(request_id);
    }
    0
});
