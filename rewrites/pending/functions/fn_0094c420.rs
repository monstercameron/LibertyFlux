// original: 0x0094c420 pair_array_init6
/// Initialise six key/value pairs to (empty, zero).
export!(thiscall, rw_0094c420(obj: *mut u32) -> u32 {
    unsafe {
        for i in 0..6usize {
            *obj.add(i * 2) = 0xFFFF_FFFF;
            *obj.add(i * 2 + 1) = 0;
        }
        0
    }
});
