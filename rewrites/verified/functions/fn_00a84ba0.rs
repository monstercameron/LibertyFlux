// original: 0x00A84BA0 CComplexNewExitVehicleTaskInfo::deserialize

/// Deserializes the vehicle-task base, reads one byte through the incoming
/// stream slot, and reads a typed vehicle value into offset 0x38. It copies
/// the flag's low bit into the low bit of the word at offset 0x30 while
/// preserving the remaining bits, then calls the exit-vehicle accessor. This
/// thiscall method consumes one stream pointer and returns that accessor's
/// EAX value.
lf_checker_rt::export!(thiscall, rw_00A84BA0(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_BYTE: u32 = 2;
    const READ_VEHICLE_VALUE: u32 = 3;
    const GET_EXIT_VEHICLE: u32 = 4;
    const FLAGS_OFFSET: u32 = 0x30;
    const VEHICLE_VALUE_OFFSET: u32 = 0x38;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);

    let mut stream_flag_word = stream;
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        (&mut stream_flag_word as *mut u32 as usize) as u32,
    );
    let _ = lf_checker_rt::callee_thiscall!(
        READ_VEHICLE_VALUE,
        u32,
        stream,
        task_info.wrapping_add(VEHICLE_VALUE_OFFSET),
    );

    let flags_address = task_info.wrapping_add(FLAGS_OFFSET);
    let old_flags = unsafe { core::ptr::read_unaligned(flags_address as *const u32) };
    let changed_low_bit = (u32::from(stream_flag_word as u8) ^ old_flags) & 1;
    unsafe { core::ptr::write_unaligned(flags_address as *mut u32, old_flags ^ changed_low_bit) };

    lf_checker_rt::callee_thiscall!(GET_EXIT_VEHICLE, u32, task_info)
});
