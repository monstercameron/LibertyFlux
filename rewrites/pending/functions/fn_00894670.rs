// original: 0x00894670 codeaddr_switch_6
/// Map a 1-based slot byte to a handler code address.
///
/// Slots 1 through 24 each yield one fixed code address; any other
/// slot value (including 0) yields null. Only the low byte of the
/// argument selects the slot; higher bits are ignored.
const TABLE_00894670: [u32; 24] = [0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x0089fe30, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0, 0x008a5fb0];

crate::rt::export!(cdecl, rs17_00894670(slot: u32) -> u32 {
    let k = (slot & 0xFF) as u8;
    if (1..=24).contains(&k) {
        crate::rt::relocated(TABLE_00894670[(k - 1) as usize])
    } else {
        0
    }
});
