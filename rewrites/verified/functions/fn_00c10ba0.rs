// original: 0x00C10BA0 streaming_request_create_with_vector_and_object

/// Build a request record with two copied vectors and an optional stream id.
/// `dst` receives kind 0x78, the current stream serial, the optional object's
/// index at +0x64 (or u32::MAX), a request id at +0x0c, and four dwords from
/// `vector` at +0x10. `mode` is narrowed to one byte. This is a thiscall
/// routine with four stack arguments; vector components are copied as bits.
lf_checker_rt::export!(thiscall, rw_00c10ba0(dst: u32, request_id: u32, vector: u32, object: u32, mode: u32) -> u32 {
    unsafe {
        const STREAM_SERIAL: u32 = 0x0117_35A4;
        const KIND: u8 = 0x78;
        const SERIAL: u32 = 4;
        const OBJECT_ID: u32 = 8;
        const REQUEST_ID: u32 = 0x0c;
        const VECTOR: u32 = 0x10;
        let serial = lf_checker_rt::global::<u32>(STREAM_SERIAL).read_unaligned();
        (dst as *mut u8).write(KIND);
        ((dst + SERIAL) as *mut u32).write_unaligned(serial);
        let object_id = if object == 0 { u32::MAX } else { ((object + 0x64) as *const u32).read_unaligned() };
        ((dst + OBJECT_ID) as *mut u32).write_unaligned(object_id);
        ((dst + REQUEST_ID) as *mut u32).write_unaligned(request_id);
        (dst as *mut u8).add(2).write(0);
        for word in 0..4 {
            let bits = ((vector + word * 4) as *const u32).read_unaligned();
            ((dst + VECTOR + word * 4) as *mut u32).write_unaligned(bits);
        }
        (dst as *mut u8).add(1).write(mode as u8);
    }
    0
});
