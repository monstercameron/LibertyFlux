// original: 0x00a95d50 streaming_slot_array_reset
/// Resets the global streaming-slot array to its empty state.
///
/// Walks 255 fixed-size records and, per record, clears the in-use byte,
/// stamps the id word to -1 and clears the trailing marker byte. Returns the
/// end pointer of the walk.
export!(cdecl, rw_00a95d50() -> u32 {
    unsafe {
        let mut p = relocated(0x12FB450);
        let end = relocated(0x13053B0);
        while (p as i32) < (end as i32) {
            *((p.wrapping_sub(0x88)) as *mut u8) = 0;
            *(p as *mut u32) = 0xFFFF_FFFF;
            *((p + 7) as *mut u8) = 0;
            p = p.wrapping_add(0xA0);
        }
        p
    }
});
