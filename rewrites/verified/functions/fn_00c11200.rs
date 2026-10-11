// original: 0x00C11200 streaming_request_set_object_and_mode

/// Initialize a short request record from an optional object's stream id and
/// one byte of mode. The record has kind 0x90, serial at +4, stream id at +8
/// (u32::MAX when the object is null), a cleared byte at +1, and mode at +2.
lf_checker_rt::export!(thiscall, rw_00c11200(dst: u32, object: u32, mode: u32) -> u32 {
    unsafe {
        const STREAM_SERIAL: u32 = 0x0117_35A4;
        const KIND: u8 = 0x90;
        let serial = lf_checker_rt::global::<u32>(STREAM_SERIAL).read_unaligned();
        (dst as *mut u8).write(KIND);
        ((dst + 4) as *mut u32).write_unaligned(serial);
        let object_id = if object == 0 { u32::MAX } else { ((object + 0x64) as *const u32).read_unaligned() };
        ((dst + 8) as *mut u32).write_unaligned(object_id);
        (dst as *mut u8).add(1).write(0);
        (dst as *mut u8).add(2).write(mode as u8);
    }
    0
});
