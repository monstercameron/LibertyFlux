// original: 0x00C11320 streaming_request_create_with_modes

/// Build a five-argument request. A null object sets byte +1 to one and uses
/// u32::MAX at +8; a present object clears +1 and copies its id from +0x64.
/// The remaining inputs are narrowed bytes at +2 (high mode), +3 (low mode),
/// and +0x10 (class), plus a dword at +0x0c. Kind 0x7d and the current stream
/// serial complete the header.
lf_checker_rt::export!(thiscall, rw_00c11320(dst: u32, object: u32, request_id: u32, low_mode: u32, high_mode: u32, class: u32) -> u32 {
    unsafe {
        const STREAM_SERIAL: u32 = 0x0117_35A4;
        const KIND: u8 = 0x7d;
        let serial = lf_checker_rt::global::<u32>(STREAM_SERIAL).read_unaligned();
        (dst as *mut u8).write(KIND);
        ((dst + 4) as *mut u32).write_unaligned(serial);
        (dst as *mut u8).add(1).write(0);
        let object_id = if object == 0 {
            (dst as *mut u8).add(1).write(1);
            u32::MAX
        } else {
            ((object + 0x64) as *const u32).read_unaligned()
        };
        ((dst + 8) as *mut u32).write_unaligned(object_id);
        (dst as *mut u8).add(2).write(high_mode as u8);
        ((dst + 0x0c) as *mut u32).write_unaligned(request_id);
        (dst as *mut u8).add(3).write(low_mode as u8);
        (dst as *mut u8).add(0x10).write(class as u8);
    }
    0
});
