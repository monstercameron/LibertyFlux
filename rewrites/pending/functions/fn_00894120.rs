// original: 0x00894120 codeaddr_switch_1
/// Map a 1-based slot byte to a handler code address.
///
/// Slots 1 through 24 each yield one fixed code address; any other
/// slot value (including 0) yields null. Only the low byte of the
/// argument selects the slot; higher bits are ignored.
const TABLE_00894120: [u32; 24] = [0x0089a5e0, 0x0089b620, 0x0089c2c0, 0x0089d510, 0x0089dd10, 0x0089e0a0, 0x0089eb40, 0x0089fdf0, 0x008a0970, 0x008a1720, 0x008a2960, 0x008a2f60, 0x008a3790, 0x008a4d50, 0x0042c950, 0x0042c950, 0x0089e0a0, 0x0042c950, 0x0042c950, 0x0042c950, 0x008a4d50, 0x008a5860, 0x008a5f70, 0x008a6a80];

crate::rt::export!(cdecl, rs17_00894120(slot: u32) -> u32 {
    let k = (slot & 0xFF) as u8;
    if (1..=24).contains(&k) {
        crate::rt::relocated(TABLE_00894120[(k - 1) as usize])
    } else {
        0
    }
});
