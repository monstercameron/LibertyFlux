// original: 0x00982540 audio_append_event_record
/// Append an event record to the global 256-entry audio table.
///
/// Does nothing once the count at 0x1161914 reaches 0x100. Otherwise stores
/// `a` at entry+0, `b` at entry+4, sets the flag byte at entry+8, and bumps
/// the count. Each entry is 12 bytes.
export!(cdecl, rw_00982540(a: u32, b: u32) -> () {
    unsafe {
        const COUNT: u32 = 0x1161914;
        const TABLE: u32 = 0x1161918;
        const CAP: u32 = 0x100;
        const ENTRY: u32 = 12;
        let count = global::<u32>(COUNT);
        if *count >= CAP {
            return;
        }
        let e = (*count) * ENTRY;
        *global::<u32>(TABLE.wrapping_add(e)) = a;
        *global::<u32>(TABLE.wrapping_add(e).wrapping_add(4)) = b;
        *global::<u8>(TABLE.wrapping_add(e).wrapping_add(8)) = 1;
        *count += 1;
    }
});
