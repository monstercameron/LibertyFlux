// original: 0x00894010 codeaddr_switch_0
/// Map a 1-based slot byte to a handler code address.
///
/// Slots 1 through 24 each yield one fixed code address; any other
/// slot value (including 0) yields null. Only the low byte of the
/// argument selects the slot; higher bits are ignored.
const TABLE_00894010: [u32; 24] = [0x0089a5d0, 0x0089b610, 0x0089c2b0, 0x0089d4b0, 0x0089dd00, 0x0089e090, 0x0089eb30, 0x0089fdd0, 0x008a0960, 0x008a1710, 0x008a2950, 0x008a2f50, 0x008a3780, 0x008a3b60, 0x00897360, 0x008a4270, 0x008a4540, 0x004016a0, 0x004016a0, 0x008a4760, 0x008a4d40, 0x008a5850, 0x008a5f60, 0x008a6a30];

crate::rt::export!(cdecl, rs17_00894010(slot: u32) -> u32 {
    let k = (slot & 0xFF) as u8;
    if (1..=24).contains(&k) {
        crate::rt::relocated(TABLE_00894010[(k - 1) as usize])
    } else {
        0
    }
});
