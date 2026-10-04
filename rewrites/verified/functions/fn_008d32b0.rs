// original: 0x008d32b0 timer_set_clamped
/// Stores a timer value clamped to [0, 999_999_999] into the object's +0x5c8
/// slot. Returns the clamped value.
export!(thiscall, rw_008d32b0(this_: u32, value: i32) -> u32 {
    unsafe {
        const MAX_TIMER: i32 = 999_999_999;
        let clamped = value.clamp(0, MAX_TIMER);
        *((this_ + 0x5c8) as *mut u32) = clamped as u32;
        clamped as u32
    }
});
