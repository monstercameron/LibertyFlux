// original: 0x009e2680 audio_record_init
/// Initialise a small record: tag word -1, payload, zero flag.
/// Returns the record pointer. (thiscall/1)
export!(thiscall, rw_009e2680(this: *mut u8, payload: u32) -> u32 {
    unsafe {
        *(this.add(4) as *mut u32) = payload;
        *(this as *mut u32) = 0xFFFF_FFFF;
        *this.add(8) = 0;
        this as u32
    }
});
