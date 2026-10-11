// original: 0x00C111B0 streaming_request_copy_three_byte_code

/// Initialize a byte-indexed request with an optional object id and a
/// three-byte code from `description`. The source dword is copied to +0x0c;
/// its bytes +4..+6 become request bytes +1..+3. A missing object stores
/// u32::MAX at +8. The receiver pointer is in ECX and two pointers are on
/// the stack.
lf_checker_rt::export!(thiscall, rw_00c111b0(dst: u32, object: u32, description: u32) -> u32 {
    unsafe {
        const STREAM_SERIAL: u32 = 0x0117_35A4;
        const KIND: u8 = 0x8f;
        let serial = lf_checker_rt::global::<u32>(STREAM_SERIAL).read_unaligned();
        (dst as *mut u8).write(KIND);
        ((dst + 4) as *mut u32).write_unaligned(serial);
        let object_id = if object == 0 { u32::MAX } else { ((object + 0x64) as *const u32).read_unaligned() };
        ((dst + 8) as *mut u32).write_unaligned(object_id);
        for i in 0..3 {
            let byte = ((description + 4 + i) as *const u8).read();
            (dst as *mut u8).add((1 + i) as usize).write(byte);
        }
        let source_word = (description as *const u32).read_unaligned();
        ((dst + 0x0c) as *mut u32).write_unaligned(source_word);
    }
    0
});
