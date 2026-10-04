// original: 0x00894230 codeaddr_switch_2
/// Map a 1-based slot byte to a handler code address.
///
/// Slots 1 through 24 each yield one fixed code address; any other
/// slot value (including 0) yields null. Only the low byte of the
/// argument selects the slot; higher bits are ignored.
const TABLE_00894230: [u32; 24] = [0x0089a600, 0x0089b640, 0x0089c2e0, 0x0089d530, 0x0089dd30, 0x0089e0c0, 0x0089eb60, 0x0089fe10, 0x008a0990, 0x008a1740, 0x008a2980, 0x008a2f80, 0x008a37b0, 0x008a3b70, 0x008973b0, 0x008a4280, 0x008a4550, 0x004016d0, 0x004016d0, 0x004016d0, 0x008a4d70, 0x008a5880, 0x008a5f90, 0x008a6aa0];

lf_k2_rt::export!(cdecl, rs17_00894230(slot: u32) -> u32 {
    let k = (slot & 0xFF) as u8;
    if (1..=24).contains(&k) {
        lf_k2_rt::relocated(TABLE_00894230[(k - 1) as usize])
    } else {
        0
    }
});
