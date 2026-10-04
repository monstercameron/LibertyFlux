// original: 0x00e5e780 network_global_swap
/// Save one global word aside and install a fixed handler pointer.
///
/// Loads the word at `0x017ad1b8` (old value), stores it into `0x0110ea04`,
/// then writes the constant `0x0110e9f8` into `0x017ad1b8`. Takes no arguments;
/// entry registers are ignored. Returns the previous word, as left in EAX.
export!(cdecl, rw_00e5e780() -> u32 {
    unsafe {
        let old = *global::<u32>(0x17AD1B8);
        *global::<u32>(0x110EA04) = old;
        *global::<u32>(0x17AD1B8) = relocated(0x110E9F8);
        old
    }
});
