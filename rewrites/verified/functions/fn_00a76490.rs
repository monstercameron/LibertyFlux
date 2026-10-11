// original: 0x00A76490 vehicle_flag_is_active

/// Return the low-byte true value when the model record has a nonzero field at
/// `+0xA8` and bit zero of its flag byte at `+0x22` is set. The original uses
/// the 32-bit thiscall register for the record pointer and reads no other
/// memory. The return channel is AL.
lf_checker_rt::export!(thiscall, rw_00a76490(this: u32) -> u8 {
    const MODEL_FIELD: u32 = 0xA8;
    const VEHICLE_FLAGS: u32 = 0x22;
    const ACTIVE_FLAG: u8 = 0x01;

    unsafe {
        let model_field = (this.wrapping_add(MODEL_FIELD) as *const u32).read_unaligned();
        if model_field == 0 {
            return 0;
        }
        let flags = (this.wrapping_add(VEHICLE_FLAGS) as *const u8).read();
        u8::from(flags & ACTIVE_FLAG != 0)
    }
});
