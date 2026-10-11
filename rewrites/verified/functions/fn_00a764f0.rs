// original: 0x00A764F0 vehicle_record_lookup

/// Resolve an indexed record through the object's model-info pointer. A null
/// object, null model-info pointer, or zero field at model-info `+0x64` returns
/// `u32::MAX`. Otherwise a thiscall helper supplies a container whose array
/// at `+0xD4` holds record pointers; the selected record's signed 16-bit value
/// at `+0x0E` is sign-extended to the 32-bit result.
lf_checker_rt::export!(cdecl, rw_00a764f0(object: u32, index: u32) -> u32 {
    const MODEL_INFO: u32 = 0x7B4;
    const MODEL_FLAGS: u32 = 0x64;
    const RECORDS: u32 = 0xD4;
    const RECORD_ID: u32 = 0x0E;
    const CONTAINER_CALLEE: u32 = 1;

    unsafe {
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_i16(address: u32) -> i16 {
            unsafe { (address as *const i16).read_unaligned() }
        }

        let model_info = read_u32(object.wrapping_add(MODEL_INFO));
        if model_info == 0 || read_u32(model_info.wrapping_add(MODEL_FLAGS)) == 0 {
            return u32::MAX;
        }
        let container = lf_checker_rt::callee_thiscall!(CONTAINER_CALLEE, u32, model_info);
        let records = read_u32(container.wrapping_add(RECORDS));
        let record = read_u32(records.wrapping_add(index.wrapping_mul(4)));
        read_i16(record.wrapping_add(RECORD_ID)) as i32 as u32
    }
});
