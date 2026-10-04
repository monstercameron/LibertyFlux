// original: 0x0097ac20 audio_listener_find_free
/// Find the first free audio listener slot.
///
/// Scans the three global listener pointers and returns the first one
/// that is present and still fresh (state flag at +0x28 is 0),
/// or null when no slot qualifies.
export!(cdecl, rw_0097ac20() -> u32 {
    unsafe {
        let table = global::<u32>(0x12312D4);
        for i in 0..3usize {
            let e = *table.add(i);
            if e != 0 && *((e as *const u8).add(0x28)) == 0 {
                return e;
            }
        }
        0
    }
});
