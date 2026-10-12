// original: 0x0065F900 pack_sn_msg_session_attrs_changed

/// Packages session-attribute message fields and optionally reports their bit count rounded up to bytes.
/// The helper receives a seven-word local record: the message value, two fixed fields, three cleared
/// output/work words, and a final masked byte. Its signed count at byte offset 0x10 is rounded with
/// arithmetic shift after adding seven. A null output pointer is ignored; a failed helper writes zero.
/// The helper is an intercepted stdcall with one pointer argument, and this wrapper pops three stack
/// arguments. This proof sets the incoming frame byte at ESP-4 to zero because the original reads it
/// before writing the same masked byte back there.
lf_checker_rt::export!(thiscall, rw_0065f900(_object: *mut u8, message_value: u32, _unused: u32, output_size: *mut u32) -> u8 {
    const RECORD_TYPE: u32 = 0x1C68;
    const COUNT_OFFSET: usize = 4;
    let mut record = [message_value, 0, RECORD_TYPE, 0, 0, 0, 0];
    let record_pointer = record.as_mut_ptr() as u32;
    let succeeded = lf_checker_rt::callee_stdcall!(1, u8, record_pointer);
    if !output_size.is_null() {
        let byte_count = if succeeded == 0 {
            0
        } else {
            let count = record[COUNT_OFFSET];
            ((count as i32).wrapping_add(7) >> 3) as u32
        };
        unsafe { *output_size = byte_count; }
    }
    succeeded
});
