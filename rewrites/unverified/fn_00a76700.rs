// original: 0x00A76700 NativeImpl_GET_CHAR_LAST_DAMAGE_BONE

/// Resolve the requested value to a signed table index, then use the vehicle
/// object's virtual accessors to obtain the indexed record. A negative helper
/// result returns zero; a nonnegative result selects a 16-bit value at the
/// fixed 0xE0-byte record stride. Virtual calls are dispatched through the
/// object's planted function tables and use thiscall with the object in ECX.
unsafe fn read_word_at(address: u32, offset: u32) -> u32 {
    core::ptr::read_unaligned(address.wrapping_add(offset) as *const u32)
}

unsafe fn read_halfword_at(address: u32, offset: u32) -> u16 {
    core::ptr::read_unaligned(address.wrapping_add(offset) as *const u16)
}

unsafe fn call_virtual(object: u32, slot_offset: u32) -> u32 {
    let vtable = read_word_at(object, 0);
    let target = read_word_at(vtable, slot_offset);
    let method: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
    method(object)
}

lf_checker_rt::export!(cdecl, rw_00a76700(vehicle: u32, requested_value: u32) -> u32 {
    const ARRAY_OFFSET: u32 = 4;
    const VEHICLE_FALLBACK_OFFSET: u32 = 0x100;
    const RECORD_STRIDE: u32 = 0xE0;
    const RECORD_VALUE_OFFSET: u32 = 0x16;

    let signed_record_index = lf_checker_rt::callee_cdecl!(3, u32, vehicle, requested_value) as i32;
    if signed_record_index <= -1 {
        return 0;
    }

    unsafe {
        let first_accessor = call_virtual(vehicle, 0xA0);
        let first_result = if first_accessor != 0 {
            call_virtual(first_accessor, 0xE0)
        } else {
            read_word_at(vehicle, VEHICLE_FALLBACK_OFFSET)
        };
        if first_result == 0 {
            return 0;
        }

        let second_accessor = call_virtual(vehicle, 0xA0);
        let data_object = if second_accessor != 0 {
            call_virtual(second_accessor, 0xE0)
        } else {
            read_word_at(vehicle, VEHICLE_FALLBACK_OFFSET)
        };
        let record_array = read_word_at(read_word_at(data_object, ARRAY_OFFSET), 0);
        let record_offset = (signed_record_index as u32)
            .wrapping_mul(RECORD_STRIDE)
            .wrapping_add(RECORD_VALUE_OFFSET);
        read_halfword_at(record_array, record_offset) as u32
    }
});
