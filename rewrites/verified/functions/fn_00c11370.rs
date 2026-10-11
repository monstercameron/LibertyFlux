// original: 0x00C11370 streaming_request_set_byte_value

/// Build a one-byte request record with kind 0x7e, the current stream serial,
/// a zero state byte and the supplied byte at +2.
lf_checker_rt::export!(thiscall, rw_00c11370(dst: u32, value: u32) -> u32 {
    unsafe {
        const STREAM_SERIAL: u32 = 0x0117_35A4;
        const KIND: u8 = 0x7e;
        let serial = lf_checker_rt::global::<u32>(STREAM_SERIAL).read_unaligned();
        (dst as *mut u8).write(KIND);
        ((dst + 4) as *mut u32).write_unaligned(serial);
        (dst as *mut u8).add(1).write(0);
        (dst as *mut u8).add(2).write(value as u8);
    }
    0
});
