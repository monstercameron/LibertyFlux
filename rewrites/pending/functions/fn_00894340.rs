// original: 0x00894340 codeaddr_switch_3
/// Map a 1-based slot byte to a handler code address.
///
/// Slots 1 through 24 each yield one fixed code address; any other
/// slot value (including 0) yields null. Only the low byte of the
/// argument selects the slot; higher bits are ignored.
const TABLE_00894340: [u32; 24] = [0x0089a610, 0x0089b650, 0x0089c2f0, 0x0089d580, 0x0089dd40, 0x0089e0d0, 0x0089eb70, 0x0089fe20, 0x008a09a0, 0x008a1750, 0x008a2990, 0x0089d580, 0x008a37c0, 0x008a4dc0, 0x008973c0, 0x008a4560, 0x008a4560, 0x0040bb50, 0x008a46c0, 0x0040bb50, 0x008a4dc0, 0x008a5890, 0x008a5fa0, 0x008a6af0];

crate::rt::export!(cdecl, rs17_00894340(slot: u32) -> u32 {
    let k = (slot & 0xFF) as u8;
    if (1..=24).contains(&k) {
        crate::rt::relocated(TABLE_00894340[(k - 1) as usize])
    } else {
        0
    }
});
