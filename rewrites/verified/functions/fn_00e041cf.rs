// original: 0x00e041cf broadcast_ptr_slots
/// Stores the argument in the four global slots at 0x17AC298-0x17AC2A4.
///
/// Broadcasts one value over four adjacent slots, then returns it.
export!(cdecl, rw_00e041cf(value: u32) -> u32 {
    unsafe {
        *global::<u32>(0x17AC298) = value;
        *global::<u32>(0x17AC29C) = value;
        *global::<u32>(0x17AC2A0) = value;
        *global::<u32>(0x17AC2A4) = value;
        value
    }
});
